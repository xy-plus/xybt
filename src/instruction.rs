use crate::registers::RiscReg;
use numeric_enum_macro::numeric_enum;

numeric_enum! {
    #[repr(u32)]
    #[allow(non_camel_case_types)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum OpCode {
        LOAD = 0,
        LOAD_FP = 1,
        MISC_MEM = 3,
        OP_IMM = 4,
        AUIPC = 5,
        OP_IMM_32 = 6,
        STORE = 8,
        STORE_FP = 9,
        AMO = 11,
        OP = 12,
        LUI = 13,
        OP_32 = 14,
        MADD = 16,
        MSUB = 17,
        NMSUB = 18,
        NMADD = 19,
        OP_FP = 20,
        BRANCH = 24,
        JALR = 25,
        JAL = 27,
        SYSTEM = 28,
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum MNEM {
    INVALID_MNEM,

    //---RV32I---
    LUI,   //load upper Imm
    AUIPC, //register = Imm

    //control flow
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,

    //load & store
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,

    //Arithmetic
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,

    //??????
    FENCE,
    ECALL,
    EBREAK,
    FENCE_I,

    //CSRR
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,
    MANUAL_CSRR,

    //---RV64I---
    //load & store
    LWU,
    LD,
    SD,

    //Arithmetic
    //SLLI,
    //SRLI,
    //SRAI,
    ADDIW,
    SLLIW,
    SRLIW,
    SRAIW,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,

    //---RV32M---
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,

    //---RV64M---
    MULW,
    DIVW,
    DIVUW,
    REMW,
    REMUW,

    //---RV32A---
    LRW,
    SCW,
    AMOSWAPW,
    AMOADDW,
    AMOXORW,
    AMOANDW,
    AMOORW,
    AMOMINW,
    AMOMAXW,
    AMOMINUW,
    AMOMAXUW,

    //---RV64A---
    LRD,
    SCD,
    AMOSWAPD,
    AMOADDD,
    AMOXORD,
    AMOANDD,
    AMOORD,
    AMOMIND,
    AMOMAXD,
    AMOMINUD,
    AMOMAXUD,

    //---RV32F---
    FLW,
    FSW,
    FMADDS,
    FMSUBS,
    FNMSUBS,
    FNMADDS,
    FADDS,
    FSUBS,
    FMULS,
    FDIVS,
    FSQRTS,
    FSGNJS,
    FSGNJNS,
    FSGNJXS,
    FMINS,
    FMAXS,
    FCVTWS,
    FCVTWUS,
    FMVXW,
    FEQS,
    FLTS,
    FLES,
    FCLASSS,
    FCVTSW,
    FCVTSWU,
    FMVWX,

    //---RV64F---
    FCVTLS,
    FCVTLUS,
    FCVTSL,
    FCVTSLU,

    //---RV32D--- + //---RV64D--- //reordering for easier parser design
    FLD,
    FSD,
    FMADDD,
    FMSUBD,
    FNMSUBD,
    FNMADDD,
    FADDD,
    FSUBD,
    FMULD,
    FDIVD,
    FSQRTD,
    FSGNJD,
    FSGNJND,
    FSGNJXD,
    FMIND,
    FMAXD,
    FCVTWD,
    FCVTWUD,
    FMVXD,
    FEQD,
    FLTD,
    FLED,
    FCLASSD,
    FCVTDW,
    FCVTDWU,
    FMVDX,
    FCVTLD,
    FCVTLUD,
    FCVTDL,
    FCVTDLU,
    FCVTSD,
    FCVTDS,

    //---PSEUDO---
    PC_NEXT_INST,
    SILENT_NOP,
    PATTERN_EMIT,

    //To always have a count, don't insert below here
    LAST_MNEM,
}

impl Default for MNEM {
    fn default() -> Self {
        MNEM::INVALID_MNEM
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum OpType {
    REG_REG,
    IMMEDIATE,
    UPPER_IMMEDIATE,
    STORE,
    BRANCH,
    JUMP,
    SYSTEM,
    FLOAT,
    INVALID_INSTRUCTION,
    INVALID_BLOCK,
    PSEUDO,
}

impl Default for OpType {
    fn default() -> Self {
        OpType::INVALID_INSTRUCTION
    }
}

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

// #[derive(Default, Debug)]
// pub struct RiscSrc3 {
//     src3: (u32, u32), // reg_src3, rounding_mode
// }

#[derive(Default, Debug)]
pub struct RiscInst {
    pub addr: u64,
    pub mnem: MNEM, // todo: what is it?
    pub optype: OpType,
    pub reg_src_1: RiscReg,
    pub reg_src_2: RiscReg,
    pub reg_dest: RiscReg,
    pub imm: i64,
    pub reg_src_3: RiscReg,
    pub rounding_mode: u32,
}

pub struct RawRiscInst(pub u32);

impl RawRiscInst {
    // extract rd register number bit[11:7]
    pub fn extract_rd(&self) -> RiscReg {
        return RiscReg::try_from(self.0 >> 7 & 0x1f).unwrap();
    }

    // extract rs1 register number bit[19:15]
    pub fn extract_rs1(&self) -> RiscReg {
        return RiscReg::try_from(self.0 >> 15 & 0x1f).unwrap();
    }

    // extract rs2 register number bit[24:20]
    pub fn extract_rs2(&self) -> RiscReg {
        return RiscReg::try_from(self.0 >> 20 & 0x1f).unwrap();
    }

    // extract rs3 register number bit[31:27]
    pub fn extract_rs3(&self) -> RiscReg {
        return RiscReg::try_from(self.0 >> 27 & 0x1f).unwrap();
    }

    pub fn extract_op(&self) -> OpCode {
        return OpCode::try_from(self.0 >> 2 & 0x1f).unwrap();
    }

    // extract func3 bit [14:12]
    pub fn extract_funct3(&self) -> u32 {
        return self.0 >> 12 & 0x7;
    }

    // extract I-Type immediate bit[31:20]
    pub fn extract_imm_I(&self) -> i32 {
        // sign extend!
        return self.0 as i32 >> 20;
    }

    // extract U-Type immediate bit[31:12] -> mask lower 12 bit [11:0] with zeros
    pub fn extract_imm_U(&self) -> i32 {
        // sign extend!
        return self.0 as i32 & !(0xfff);
    }
}
