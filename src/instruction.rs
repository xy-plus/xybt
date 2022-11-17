use crate::registers::REG;

// todo: need union?
// #[repr(C)]
// #[derive(Debug)]
// pub union RiscSrc3 {
//     imm: i64,
//     src3: (u32, u32), // reg_src3, rounding_mode
// }

// impl Default for RiscSrc3 {
//     fn default() -> Self {
//         RiscSrc3 { imm: 0 }
//     }
// }

#[derive(Default, Debug)]
pub struct RiscSrc3 {
    imm: i64,
    src3: (u32, u32), // reg_src3, rounding_mode
}

#[derive(Default, Debug)]
pub struct RiscInst {
    pub addr: u64,
    pub mnem: u32,   // todo: what is it?
    pub optype: u32, // todo:  what is it?
    pub reg_src_1: REG,
    pub reg_src_2: REG,
    pub reg_dest: REG,
    pub reg_src_3: RiscSrc3,
}
