//! Vectorized DensityFunction AST Bytecode Evaluator
//! Parses world generation noise graphs into a linear stack of SIMD instructions,
//! evaluating 16 coordinates in parallel with zero JVM GC pressure.

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimdOpcode {
    Constant = 0,
    Add = 1,
    Mul = 2,
    Min = 3,
    Max = 4,
    Clamp = 5,
    Abs = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SimdInstruction {
    pub opcode: SimdOpcode,
    pub src_reg_a: u8,
    pub src_reg_b: u8,
    pub dst_reg: u8,
    pub immediate: f32,
}

pub const MAX_SIMD_REGISTERS: usize = 16;
pub const SIMD_LANES: usize = 16;

#[derive(Default)]
pub struct SimdRegisterFile {
    pub registers: [[f32; SIMD_LANES]; MAX_SIMD_REGISTERS],
}

pub struct SimdAstProgram {
    pub instructions: Vec<SimdInstruction>,
}

impl SimdAstProgram {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn push(&mut self, instr: SimdInstruction) {
        self.instructions.push(instr);
    }

    pub fn execute(&self, rf: &mut SimdRegisterFile) {
        for instr in &self.instructions {
            let dst = (instr.dst_reg as usize) % MAX_SIMD_REGISTERS;
            let src_a = (instr.src_reg_a as usize) % MAX_SIMD_REGISTERS;
            let src_b = (instr.src_reg_b as usize) % MAX_SIMD_REGISTERS;

            match instr.opcode {
                SimdOpcode::Constant => {
                    let imm = if instr.immediate.is_nan() { 0.0 } else { instr.immediate };
                    rf.registers[dst].fill(imm);
                }
                SimdOpcode::Add => {
                    for i in 0..SIMD_LANES {
                        let res = rf.registers[src_a][i] + rf.registers[src_b][i];
                        rf.registers[dst][i] = if res.is_nan() { 0.0 } else { res };
                    }
                }
                SimdOpcode::Mul => {
                    for i in 0..SIMD_LANES {
                        let res = rf.registers[src_a][i] * rf.registers[src_b][i];
                        rf.registers[dst][i] = if res.is_nan() { 0.0 } else { res };
                    }
                }
                SimdOpcode::Min => {
                    for i in 0..SIMD_LANES {
                        rf.registers[dst][i] = rf.registers[src_a][i].min(rf.registers[src_b][i]);
                    }
                }
                SimdOpcode::Max => {
                    for i in 0..SIMD_LANES {
                        rf.registers[dst][i] = rf.registers[src_a][i].max(rf.registers[src_b][i]);
                    }
                }
                SimdOpcode::Clamp => {
                    let min_raw = instr.immediate;
                    let max_raw = rf.registers[src_b][0];
                    let lo = if min_raw.is_nan() { -1e6 } else { min_raw };
                    let hi = if max_raw.is_nan() { 1e6 } else { max_raw };
                    let (safe_lo, safe_hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };

                    for i in 0..SIMD_LANES {
                        let val = rf.registers[src_a][i];
                        rf.registers[dst][i] = if val.is_nan() { 0.0 } else { val.clamp(safe_lo, safe_hi) };
                    }
                }
                SimdOpcode::Abs => {
                    for i in 0..SIMD_LANES {
                        rf.registers[dst][i] = rf.registers[src_a][i].abs();
                    }
                }
            }
        }
    }
}

// C-ABI Exports
#[no_mangle]
pub extern "C" fn hz_ast_program_create() -> *mut SimdAstProgram {
    Box::into_raw(Box::new(SimdAstProgram::new()))
}

#[no_mangle]
pub extern "C" fn hz_ast_program_destroy(program: *mut SimdAstProgram) {
    if !program.is_null() {
        unsafe {
            drop(Box::from_raw(program));
        }
    }
}

#[no_mangle]
pub extern "C" fn hz_ast_program_push(
    program: *mut SimdAstProgram,
    opcode: u8,
    src_a: u8,
    src_b: u8,
    dst: u8,
    immediate: f32,
) -> i32 {
    if program.is_null() || opcode > 6 {
        return -1;
    }
    let program = unsafe { &mut *program };
    let op = match opcode {
        0 => SimdOpcode::Constant,
        1 => SimdOpcode::Add,
        2 => SimdOpcode::Mul,
        3 => SimdOpcode::Min,
        4 => SimdOpcode::Max,
        5 => SimdOpcode::Clamp,
        _ => SimdOpcode::Abs,
    };
    program.push(SimdInstruction {
        opcode: op,
        src_reg_a: src_a,
        src_reg_b: src_b,
        dst_reg: dst,
        immediate,
    });
    0
}

#[no_mangle]
pub extern "C" fn hz_ast_program_execute(
    program: *const SimdAstProgram,
    rf: *mut SimdRegisterFile,
) -> i32 {
    if program.is_null() || rf.is_null() {
        return -1;
    }
    let program = unsafe { &*program };
    let rf = unsafe { &mut *rf };
    program.execute(rf);
    0
}