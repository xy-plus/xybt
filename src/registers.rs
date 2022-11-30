use crate::const_var::*;
use crate::instruction::RiscInst;
use iced_x86::Register;
use numeric_enum_macro::numeric_enum;

numeric_enum! {
    #[repr(u32)]
    #[allow(non_camel_case_types)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum RiscReg {
        zero=0,
        ra=1, sp=2, gp=3, tp=4, t0=5, t1=6, t2=7,
        s0=8, // s0=fp
        s1=9, a0=10, a1=11, a2=12, a3=13, a4=14, a5=15, a6=16,
        a7=17, s2=18, s3=19, s4=20, s5=21, s6=22, s7=23, s8=24,
        s9=25, s10=26, s11=27, t3=28, t4=29, t5=30, t6=31, pc=32,
        invalid=33
    }
}

impl Default for RiscReg {
    fn default() -> Self {
        RiscReg::invalid
    }
}

// #[repr(u32)]
// #[allow(non_camel_case_types)]
// // #[allow(dead_code)]
// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum X86Reg {
//     AX = 0x100,
//     CX,
//     DX,
//     BX,
//     SP,
//     BP,
//     SI,
//     DI,
//     R8,
//     R9,
//     R10,
//     R11,
//     R12,
//     R13,
//     R14,
//     R15,
//     IP,
//     AH = 0x204,
//     CH,
//     DH,
//     BH,
//     ES = 0x300,
//     CS,
//     SS,
//     DS,
//     FS,
//     GS,
//     ST0 = 0x400,
//     ST1,
//     ST2,
//     ST3,
//     ST4,
//     ST5,
//     ST6,
//     ST7,
//     MM0 = 0x500,
//     MM1,
//     MM2,
//     MM3,
//     MM4,
//     MM5,
//     MM6,
//     MM7,
//     XMM0 = 0x600,
//     XMM1,
//     XMM2,
//     XMM3,
//     XMM4,
//     XMM5,
//     XMM6,
//     XMM7,
//     XMM8,
//     XMM9,
//     XMM10,
//     XMM11,
//     XMM12,
//     XMM13,
//     XMM14,
//     XMM15,
//     zero,
//     invalid,
// }

// impl Default for X86Reg {
//     fn default() -> Self {
//         X86Reg::invalid
//     }
// }

#[derive(Debug)]
pub struct REGISTERS {
    general_regs: [u64; GENERAL_REGS_NUM + 1], // todo: find out why plus 1
}

impl REGISTERS {
    pub fn new() -> Self {
        return Self {
            general_regs: [0; GENERAL_REGS_NUM + 1],
        };
    }
    pub fn set_reg(&mut self, reg: RiscReg, val: u64) {
        if reg == RiscReg::zero {
            panic!("try to set zero reg")
        }
        self.general_regs[reg as usize] = val
    }
}

pub struct RegInfo {
    pub gp_map: [Register; GENERAL_REGS_NUM],
    pub gp_mapped: [bool; GENERAL_REGS_NUM],
    pub gp: REGISTERS,
    pub replacement_content: [RiscReg; REPLACE_REGS_NUM],
    pub replacement_recency: [i32; REPLACE_REGS_NUM],
    pub current_recency: i32,
    pub inst_mem_addr: u64,
}

impl RegInfo {
    pub fn get_rs1(&self, inst: &RiscInst) -> Register {
        // println!("{:#?}", inst.reg_src_1);
        // if inst.reg_src_1
        if self.gp_mapped[inst.reg_src_1 as usize] {
            return self.gp_map[inst.reg_src_1 as usize];
        } else {
            unimplemented!()
        }
    }

    pub fn get_rd(&self, inst: &RiscInst) -> Register {
        if self.gp_mapped[inst.reg_dest as usize] {
            return self.gp_map[inst.reg_dest as usize];
        } else {
            unimplemented!()
        }
    }

    pub fn get_gp_map_reg(&self, risc_reg: RiscReg) -> Register {
        assert!(self.gp_mapped[risc_reg as usize]);
        return self.gp_map[risc_reg as usize];
    }
}

pub fn init_map_context(gp: REGISTERS, inst_mem_addr: u64) -> RegInfo {
    let mut gp_map = [Register::default(); GENERAL_REGS_NUM];
    let mut gp_mapped = [false; GENERAL_REGS_NUM];
    let mut replacement_content = [RiscReg::default(); REPLACE_REGS_NUM];
    let mut replacement_recency = [0; REPLACE_REGS_NUM];
    let mut current_recency = 1;
    const map_pair: [(RiscReg, Register); 13] = [
        (RiscReg::a5, Register::RBX),
        (RiscReg::a4, Register::RBP),
        (RiscReg::a3, Register::RSI),
        (RiscReg::a0, Register::RDI),
        (RiscReg::s0, Register::R8),
        (RiscReg::sp, Register::R9),
        (RiscReg::a2, Register::R10),
        (RiscReg::a1, Register::R11),
        (RiscReg::s1, Register::R12),
        (RiscReg::ra, Register::R13),
        (RiscReg::a7, Register::R14),
        (RiscReg::s2, Register::R15),
        (RiscReg::zero, Register::None),
    ];
    for (risc_reg, x86_reg) in &map_pair {
        gp_map[*risc_reg as usize] = *x86_reg;
        gp_mapped[*risc_reg as usize] = true;
    }
    let mut reg_info = RegInfo {
        gp_map: gp_map,
        gp_mapped: gp_mapped,
        gp: gp,
        replacement_content: replacement_content,
        replacement_recency: replacement_recency,
        current_recency: current_recency,
        inst_mem_addr: inst_mem_addr,
    };
    return reg_info;
}
