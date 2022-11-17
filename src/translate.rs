use crate::const_var::*;
use crate::instruction::RiscInst;
use crate::parser::parse_instruction;
use libc;
use std::mem::size_of;

pub fn translate_block(risc_addr: u64, c_info: u64) {
    let max_count = BLOCK_CACHE_SIZE;
    let block_cache_addr = unsafe {
        libc::mmap(
            libc::PT_NULL as *mut libc::c_void,
            max_count * size_of::<RiscInst>(),
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
            -1,
            0,
        )
    } as u64;

    // todo: float block
    let _instructions_in_block = parse_block(risc_addr, block_cache_addr, max_count, c_info);
}

fn parse_block(risc_addr: u64, parse_buf: u64, _max_count: usize, _c_info: u64) {
    println!("{:#?}", size_of::<RiscInst>());
    let parse_buf = parse_buf as *mut RiscInst;
    unsafe {
        parse_buf.write(RiscInst {
            addr: risc_addr,
            ..Default::default()
        });
        parse_instruction(parse_buf);
    }
}
