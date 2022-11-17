use crate::registers::REG;

#[repr(C)]
pub union RISC_SRC3 {
    imm: i64,
    src3: (u32, u32), // reg_src3, rounding_mode
}

impl Default for RISC_SRC3 {
    fn default() -> Self {
        RISC_SRC3 { imm: 0 }
    }
}

#[derive(Default)]
pub struct RiscInst {
    pub addr: u64,
    pub mnem: u32,   // todo: what is it?
    pub optype: u32, // todo:  what is it?
    pub reg_src_1: REG,
    pub reg_src_2: REG,
    pub reg_dest: REG,
    pub reg_src_3: RISC_SRC3,
}
