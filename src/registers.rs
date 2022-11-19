use crate::const_var::*;
use numeric_enum_macro::numeric_enum;

numeric_enum! {
    #[repr(u32)]
    #[allow(non_camel_case_types)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum REG {
        zero=0,
        ra=1, sp=2, gp=3, tp=4, t0=5, t1=6, t2=7,
        s0=8, // s0=fp
        s1=9, a0=10, a1=11, a2=12, a3=13, a4=14, a5=15, a6=16,
        a7=17, s2=18, s3=19, s4=20, s5=21, s6=22, s7=23, s8=24,
        s9=25, s10=26, s11=27, t3=28, t4=29, t5=30, t6=31, pc=32,
        invalid=33
    }
}

impl Default for REG {
    fn default() -> Self {
        REG::invalid
    }
}

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
    pub fn set_reg(&mut self, reg: REG, val: u64) {
        if reg == REG::zero {
            panic!("try to set zero reg")
        }
        self.general_regs[reg as usize] = val
    }
}
