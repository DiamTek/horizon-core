//! NUMA-aware Work-Stealing Thread Scheduler and Rayon Orchestrator.

use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

static GLOBAL_NUMA_SCHEDULER: OnceLock<HorizonNumaScheduler> = OnceLock::new();

pub struct NumaWorkerTopology {
    pub node_id: usize,
    pub pool: ThreadPool,
}

pub struct HorizonNumaScheduler {
    nodes: Vec<NumaWorkerTopology>,
    dispatch_counter: AtomicUsize,
}

impl HorizonNumaScheduler {
    pub fn new() -> Self {
        let num_cpus = num_cpus::get().max(1);
        let pool = ThreadPoolBuilder::new()
            .num_threads(num_cpus)
            .thread_name(|idx| format!("horizon-core-worker-{}", idx))
            .build()
            .or_else(|_| {
                ThreadPoolBuilder::new()
                    .num_threads(1)
                    .thread_name(|_| "horizon-core-worker-fallback".to_string())
                    .build()
            })
            .unwrap_or_else(|_| {
                ThreadPoolBuilder::new()
                    .use_current_thread()
                    .build()
                    .unwrap()
            });

        Self {
            nodes: vec![NumaWorkerTopology {
                node_id: 0,
                pool,
            }],
            dispatch_counter: AtomicUsize::new(0),
        }
    }

    pub fn global() -> &'static Self {
        GLOBAL_NUMA_SCHEDULER.get_or_init(Self::new)
    }

    pub fn dispatch<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let idx = self.dispatch_counter.fetch_add(1, Ordering::Relaxed) % self.nodes.len();
        self.nodes[idx].pool.spawn(task);
    }
}

#[no_mangle]
pub extern "C" fn hz_scheduler_init() -> i32 {
    let _ = HorizonNumaScheduler::global();
    0
}

#[no_mangle]
pub extern "C" fn hz_scheduler_worker_count() -> u32 {
    num_cpus::get() as u32
}