//! Bandwidth Governor and Frustum-Weighted LOD Prioritization
//! Implements token-bucket rate limiting (15 KB/s sustained, 30 KB/s burst)
//! and angularly-weighted positive-definite priority sorting without division-by-zero or inverted sorting.

use core::sync::atomic::{AtomicI64, AtomicU64, Ordering};

pub const DEFAULT_SUSTAINED_BYTES_PER_SEC: u64 = 15 * 1024; // 15 KB/s
pub const DEFAULT_BURST_CAPACITY_BYTES: u64 = 30 * 1024;    // 30 KB/s burst

/// Leaky-bucket / token-bucket rate limiter per network peer.
pub struct BandwidthGovernor {
    sustained_rate_bytes_per_sec: u64,
    burst_capacity_bytes: u64,
    available_tokens: AtomicI64,
    last_update_timestamp_ns: AtomicU64,
}

impl BandwidthGovernor {
    pub fn new(sustained_rate: u64, burst_capacity: u64) -> Self {
        Self {
            sustained_rate_bytes_per_sec: sustained_rate,
            burst_capacity_bytes: burst_capacity,
            available_tokens: AtomicI64::new(burst_capacity as i64),
            last_update_timestamp_ns: AtomicU64::new(0),
        }
    }

    /// Attempts to consume `bytes` from the bandwidth quota. Returns true if permitted.
    pub fn try_consume(&self, bytes: usize, current_time_ns: u64) -> bool {
        let last_time = self.last_update_timestamp_ns.load(Ordering::Acquire);
        if last_time > 0 && current_time_ns > last_time {
            let elapsed_sec = (current_time_ns - last_time) as f64 / 1_000_000_000.0;
            let refilled = (elapsed_sec * self.sustained_rate_bytes_per_sec as f64) as i64;
            if refilled > 0 {
                let current = self.available_tokens.load(Ordering::Acquire);
                let new_tokens = (current + refilled).min(self.burst_capacity_bytes as i64);
                self.available_tokens.store(new_tokens, Ordering::Release);
                self.last_update_timestamp_ns.store(current_time_ns, Ordering::Release);
            }
        } else if last_time == 0 {
            self.last_update_timestamp_ns.store(current_time_ns, Ordering::Release);
        }

        let needed = bytes as i64;
        let mut tokens = self.available_tokens.load(Ordering::Acquire);
        loop {
            if tokens < needed {
                return false;
            }
            match self.available_tokens.compare_exchange_weak(
                tokens,
                tokens - needed,
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(actual) => tokens = actual,
            }
        }
    }
}

/// Computes angularly weighted positive-definite LOD priority:
/// Delta = P_chunk - P_player
/// Weight_angle = max(0.0, V_look . (Delta / (||Delta|| + epsilon)))
/// Priority = (alpha * Weight_angle + (1.0 - alpha)) / (||Delta|| + epsilon)
/// Guarantees chunks in front receive 4x priority over chunks behind, with zero division-by-zero crashes.
pub fn compute_frustum_priority(
    chunk_x: f32,
    chunk_z: f32,
    player_x: f32,
    player_z: f32,
    look_x: f32,
    look_z: f32,
) -> f32 {
    const ALPHA: f32 = 0.75;
    const EPSILON: f32 = 0.001;

    let delta_x = chunk_x - player_x;
    let delta_z = chunk_z - player_z;
    let dist_sq = delta_x * delta_x + delta_z * delta_z;
    let dist = dist_sq.sqrt();

    let inv_dist = 1.0 / (dist + EPSILON);
    let norm_dx = delta_x * inv_dist;
    let norm_dz = delta_z * inv_dist;

    let dot = look_x * norm_dx + look_z * norm_dz;
    let weight_angle = dot.max(0.0);

    (ALPHA * weight_angle + (1.0 - ALPHA)) * inv_dist
}

// C-ABI Exports
#[no_mangle]
pub extern "C" fn hz_bandwidth_governor_create(sustained: u64, burst: u64) -> *mut BandwidthGovernor {
    Box::into_raw(Box::new(BandwidthGovernor::new(sustained, burst)))
}

#[no_mangle]
pub extern "C" fn hz_bandwidth_governor_destroy(gov: *mut BandwidthGovernor) {
    if !gov.is_null() {
        unsafe { drop(Box::from_raw(gov)); }
    }
}

#[no_mangle]
pub extern "C" fn hz_bandwidth_governor_try_consume(
    gov: *mut BandwidthGovernor,
    bytes: usize,
    now_ns: u64,
) -> i32 {
    if gov.is_null() {
        return -1;
    }
    let gov = unsafe { &*gov };
    if gov.try_consume(bytes, now_ns) { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn hz_frustum_lod_priority(
    chunk_x: f32,
    chunk_z: f32,
    player_x: f32,
    player_z: f32,
    look_x: f32,
    look_z: f32,
) -> f32 {
    compute_frustum_priority(chunk_x, chunk_z, player_x, player_z, look_x, look_z)
}