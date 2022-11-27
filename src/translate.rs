use crate::const_var::*;
use crate::instruction::{OpType, RiscInst, MNEM};
use crate::parser::parse_instruction;
use crate::registers::RegInfo;
use iced_x86::{
    BlockEncoder, BlockEncoderOptions, Code, Decoder, DecoderOptions, Formatter, GasFormatter,
    IcedError, Instruction, InstructionBlock, MemoryOperand, Register,
};
use std::mem::size_of;

fn translate_risc_instr(inst: &RiscInst, c_info: &RegInfo) {
    match &inst.mnem {
        MNEM::ADDI => {
            let reg_src1 = c_info.get_rs1(inst);
            let reg_dest = c_info.get_rd(inst);
            println!("{:#?}", inst);
            if reg_src1 == Register::None {
                let instructions = vec![
                    // Instruction::with2(Code::Mov_r64_imm64, reg_dest, inst.imm).unwrap(),
                    Instruction::with2(Code::Mov_rm64_imm32, reg_dest, inst.imm).unwrap(),
                    Instruction::with2(Code::Mov_rm64_imm32, reg_dest, inst.imm).unwrap(),
                ];
                let target_rip = c_info.inst_mem_addr;
                let block = InstructionBlock::new(&instructions, target_rip);
                let result = match BlockEncoder::encode(64, block, BlockEncoderOptions::NONE) {
                    Err(error) => panic!("Failed to encode it: {}", error),
                    Ok(result) => result,
                };
                let bytes = result.code_buffer;
                let bytes_code = &bytes[0..bytes.len()];
                let mut decoder =
                    Decoder::with_ip(64, bytes_code, target_rip, DecoderOptions::NONE);
                let mut formatter = GasFormatter::new();
                formatter.options_mut().set_first_operand_char_index(8);
                let mut output = String::new();
                for instruction in &mut decoder {
                    output.clear();
                    formatter.format(&instruction, &mut output);
                    println!("{:016X} {}", instruction.ip(), output);
                }
                println!("{:#x?}", bytes_code);
            } else {
                unimplemented!();
            }
            todo!();
            // panic!("t")
            // println!("{:#?}, {:#?}", reg_src1, reg_dest);
            // let mut instructions = vec![];
        }
        mismatch_optype => {
            panic!("mismatch optype {:?}", mismatch_optype);
        }
    }
}

fn translate_block_instructions(
    block_cache: &Vec<RiscInst>,
    instructions_in_block: u64,
    c_info: &RegInfo,
) {
    // init_block(c_info->r_info);  // todo: use it
    // todo: opt inst
    for i in 0..instructions_in_block {
        translate_risc_instr(&block_cache[i as usize], c_info);
    }
}
pub fn translate_block(risc_addr: u64, c_info: &RegInfo) {
    const max_count: usize = BLOCK_CACHE_SIZE;
    let mut block_cache: Vec<RiscInst> = Vec::new();
    block_cache.resize_with(max_count, Default::default);
    // todo: float block
    let instructions_in_block = parse_block(risc_addr, &mut block_cache, max_count, c_info);
    let block = translate_block_instructions(&block_cache, instructions_in_block, c_info);
}

fn parse_block(
    risc_addr: u64,
    parse_buf: &mut Vec<RiscInst>,
    max_count: usize,
    _c_info: &RegInfo,
) -> u64 {
    println!("{:#?}", size_of::<RiscInst>());
    // todo: why -2 ?
    let mut curr_addr = risc_addr;
    let mut instructions_in_block = 0;

    for parse_pos in 0..(max_count - 2) {
        parse_buf[parse_pos].addr = curr_addr;
        parse_instruction(&mut parse_buf[parse_pos]);
        match &parse_buf[parse_pos].optype {
            OpType::IMMEDIATE => {
                curr_addr += 4;
                instructions_in_block += 1;
            }
            OpType::SYSTEM => match &parse_buf[parse_pos].mnem {
                MNEM::ECALL => {
                    instructions_in_block += 1;
                    return instructions_in_block;
                }
                mismatch_optype => {
                    panic!("mismatch optype {:?}", mismatch_optype);
                }
            },
            mismatch_optype => {
                panic!("mismatch optype {:?}", mismatch_optype);
            }
        }
    }
    return instructions_in_block;
}
