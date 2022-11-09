extern crate elf;
use crate::const_var::*;
use crate::utils::{align_down, align_up};
use elf::File;
use libc;
use std::os::unix::fs::FileExt;
use std::slice;

#[derive(Debug)]
pub struct MappedElf {
    pub entry: u64,
    pub phdr_addr: u64,
    pub ph_count: u16,
    pub ph_entsize: u16,
    pub program_addr_end: u64,
    pub exec_addr_min: u64,
    pub exec_addr_max: u64,
    pub is_float: bool,
}

pub fn read_elf(file_path: &String) -> MappedElf {
    let binding = std::fs::read(file_path).expect("Could not read file.");
    let file_data = binding.as_slice();
    let mut file = File::open_stream(file_data).expect("Could not parse ELF Header");
    let elf_hdr = file.ehdr;
    assert_eq!(elf_hdr.e_machine, elf::gabi::EM_RISCV, "only support riscv");
    assert_eq!(elf_hdr.class, elf::Class::ELF64, "only support 64bit");
    assert_eq!(elf_hdr.e_type, elf::gabi::ET_EXEC, "only support static");
    let elf_flag = elf_hdr.e_flags;
    assert_eq!(elf_flag & EF_RISCV_RVC, 0, "RVC not support");
    assert_eq!(elf_flag & EF_RISCV_RVE, 0, "RVE not support");
    assert_eq!(elf_flag & EF_RISCV_TSO, 0, "TSO not support");
    let is_float = (elf_flag & EF_RISCV_FLOAT_ABI_QUAD) != 0
        || (elf_flag & EF_RISCV_FLOAT_ABI_DOUBLE) != 0
        || (elf_flag & EF_RISCV_FLOAT_ABI_SINGLE) != 0;
    let mut exec_addr_min = u64::max_value();
    let mut exec_addr_max = 0;

    for section_hdr in file.section_headers().unwrap() {
        if (section_hdr.sh_flags & elf::gabi::SHF_EXECINSTR as u64) != 0 {
            if exec_addr_min > section_hdr.sh_addr {
                exec_addr_min = section_hdr.sh_addr;
            }
            if exec_addr_max < (section_hdr.sh_addr + section_hdr.sh_size) {
                exec_addr_max = section_hdr.sh_addr + section_hdr.sh_size;
            }
        }
    }

    let mut load_addr_min = u64::max_value();
    let mut load_addr_max = 0;
    let mut elf_start_addr = 0;
    let mut segments = Vec::new();
    for program_hdr in file.segments().unwrap() {
        match program_hdr.p_type {
            elf::gabi::PT_LOAD => {
                // first segment addr is the lowest
                if elf_start_addr == 0 {
                    elf_start_addr = program_hdr.p_vaddr - program_hdr.p_offset;
                    assert!(elf_start_addr != 0);
                }
                if load_addr_min > program_hdr.p_vaddr {
                    load_addr_min = program_hdr.p_vaddr;
                }
                if load_addr_max < (program_hdr.p_vaddr + program_hdr.p_memsz) {
                    load_addr_max = program_hdr.p_vaddr + program_hdr.p_memsz;
                }
                segments.push((
                    program_hdr.p_vaddr,
                    program_hdr.p_offset,
                    program_hdr.p_filesz,
                ));
            }
            elf::gabi::PT_INTERP => {
                panic!("is dynamic link");
            }
            _ => {}
        }
    }
    assert!(
        load_addr_max < TRANSLATOR_BASE,
        "segment addr too high or TRANSLATOR_BASE too low"
    );
    let phdr_addr = elf_start_addr + elf_hdr.e_phoff;

    let program_addr_start = align_down(load_addr_min, 4096);
    let program_addr_end = align_up(load_addr_max, 4096);
    let elf = unsafe {
        libc::mmap(
            program_addr_start as *mut libc::c_void,
            (program_addr_end - program_addr_start) as usize,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_FIXED_NOREPLACE | libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        ) as u64
    };
    assert_eq!(elf, program_addr_start, "map error");

    let file = std::fs::File::open(file_path).expect("Could not read file.");

    for segment in segments {
        let (p_vaddr, p_offset, p_filesz) = segment;
        file.read_at(
            unsafe { slice::from_raw_parts_mut(p_vaddr as *mut u8, p_filesz as usize) },
            p_offset,
        )
        .expect("copy segment fail");
    }
    return MappedElf {
        entry: elf_hdr.e_entry,
        phdr_addr: phdr_addr,
        ph_count: elf_hdr.e_phnum,
        ph_entsize: elf_hdr.e_phentsize,
        program_addr_end: program_addr_end,
        exec_addr_min: exec_addr_min,
        exec_addr_max: exec_addr_max,
        is_float: is_float,
    };
}
