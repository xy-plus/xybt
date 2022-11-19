use crate::instruction::{OpCode, OpType, RawRiscInst, RiscInst, MNEM};
use crate::registers::REG;

pub fn parse_instruction(parse_buf: &mut RiscInst) {
    let raw_inst = unsafe { RawRiscInst(*(parse_buf.addr as *const u32)) };
    parse_buf.reg_dest = raw_inst.extract_rd();
    parse_buf.reg_src_1 = raw_inst.extract_rs1();
    let opcode = raw_inst.extract_op();
    match opcode {
        OpCode::OP_IMM => {
            parse_buf.optype = OpType::IMMEDIATE;
            match raw_inst.extract_funct3() {
                0 => {
                    parse_buf.mnem = MNEM::ADDI;
                    parse_buf.imm = raw_inst.extract_imm_I() as i64;
                }
                mismatch_opcode => {
                    panic!("mismatch opcode {}", mismatch_opcode);
                }
            }
        }
        OpCode::AUIPC => {
            parse_buf.optype = OpType::IMMEDIATE;
            parse_buf.mnem = MNEM::AUIPC;
            parse_buf.reg_src_1 = REG::invalid;
            parse_buf.imm = raw_inst.extract_imm_U() as i64;
        }
        OpCode::SYSTEM => {
            parse_buf.optype = OpType::SYSTEM;
            parse_buf.imm = raw_inst.extract_imm_I() as i64;
            match raw_inst.extract_funct3() {
                0 => {
                    if raw_inst.0 & (1 << 20) != 0 {
                        parse_buf.mnem = MNEM::EBREAK;
                    } else {
                        parse_buf.mnem = MNEM::ECALL;
                    }
                }
                mismatch_opcode => {
                    panic!("mismatch opcode {}", mismatch_opcode);
                }
            }
        }
        mismatch_opcode => {
            unimplemented!("mismatch opcode {:#?}", mismatch_opcode);
        }
    }
}
