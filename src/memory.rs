use crate::const_var::*;
use crate::utils::align_up;
use crate::MappedElf;
use auxv;
use libc;
use std::mem::size_of;

fn alloc_stack() -> u64 {
    let user_stack_start_addr = unsafe {
        libc::mmap(
            USER_STACK_START_ADDR as *mut libc::c_void,
            (USER_STACK_SIZE + GUARD_PAGE_SIZE) as usize,
            libc::PROT_WRITE | libc::PROT_READ,
            libc::MAP_ANONYMOUS | libc::MAP_STACK | libc::MAP_PRIVATE | libc::MAP_FIXED_NOREPLACE,
            -1,
            0,
        ) as u64
    };
    assert_eq!(
        user_stack_start_addr, USER_STACK_START_ADDR,
        "map satck fail"
    );
    unsafe {
        libc::mprotect(
            user_stack_start_addr as *mut libc::c_void,
            GUARD_PAGE_SIZE as usize,
            libc::PROT_NONE,
        );
    }
    let user_stack_top_addr = user_stack_start_addr + USER_STACK_SIZE + GUARD_PAGE_SIZE;
    return user_stack_top_addr;
}

pub fn create_user_stack(
    mapped_elf: MappedElf,
    user_args: &Vec<String>,
    env_vars: &Vec<String>,
) -> u64 {
    let stack_start_addr = alloc_stack();

    // copy info to stack
    let args_size = size_of::<auxv::AuxvPair>() * AUXC      // auxv size
        + size_of::<*const *const char>() * (env_vars.len() + 1)    // envp size
        + size_of::<*const *const char>() * (user_args.len() + 1)   // argv size
        + size_of::<i32>(); // argc size
    let stack_align_offset = align_up(args_size as u64, 16) - args_size as u64;
    let stack_start_addr = stack_start_addr - stack_align_offset;
    unsafe {
        // copy auxv
        let stack_ptr = stack_start_addr as *mut auxv::AuxvPair;
        stack_ptr.sub(1).write(auxv::AuxvPair {
            key: libc::AT_NULL,
            value: 0,
        });
        stack_ptr.sub(2).write(auxv::AuxvPair {
            key: libc::AT_ENTRY,
            value: mapped_elf.entry,
        });
        stack_ptr.sub(3).write(auxv::AuxvPair {
            key: libc::AT_PHDR,
            value: mapped_elf.phdr_addr,
        });
        stack_ptr.sub(4).write(auxv::AuxvPair {
            key: libc::AT_PHNUM,
            value: mapped_elf.ph_count as u64,
        });
        stack_ptr.sub(5).write(auxv::AuxvPair {
            key: libc::AT_PHENT,
            value: mapped_elf.ph_entsize as u64,
        });
        stack_ptr.sub(6).write(auxv::AuxvPair {
            key: libc::AT_UID,
            value: libc::getauxval(libc::AT_UID),
        });
        stack_ptr.sub(7).write(auxv::AuxvPair {
            key: libc::AT_GID,
            value: libc::getauxval(libc::AT_GID),
        });
        stack_ptr.sub(8).write(auxv::AuxvPair {
            key: libc::AT_EGID,
            value: libc::getauxval(libc::AT_EGID),
        });
        stack_ptr.sub(9).write(auxv::AuxvPair {
            key: libc::AT_EUID,
            value: libc::getauxval(libc::AT_EUID),
        });
        stack_ptr.sub(10).write(auxv::AuxvPair {
            key: libc::AT_CLKTCK,
            value: libc::getauxval(libc::AT_CLKTCK),
        });
        stack_ptr.sub(11).write(auxv::AuxvPair {
            key: libc::AT_RANDOM,
            value: libc::getauxval(libc::AT_RANDOM),
        });
        stack_ptr.sub(12).write(auxv::AuxvPair {
            key: libc::AT_SECURE,
            value: 0,
        });
        stack_ptr.sub(13).write(auxv::AuxvPair {
            key: libc::AT_PAGESZ,
            value: 4096,
        });
        stack_ptr.sub(14).write(auxv::AuxvPair {
            key: libc::AT_HWCAP,
            value: 0,
        });
        stack_ptr.sub(15).write(auxv::AuxvPair {
            key: libc::AT_HWCAP2,
            value: 0,
        });
        stack_ptr.sub(16).write(auxv::AuxvPair {
            key: libc::AT_EXECFN,
            value: user_args[0].as_ptr() as u64,
        });

        // copy envp
        let stack_ptr = stack_ptr.sub(AUXC) as u64 as *mut *const u8;
        stack_ptr.sub(1).write(std::ptr::null_mut());
        for i in 0..env_vars.len() {
            stack_ptr
                .sub(i + 2)
                .write(env_vars[env_vars.len() - 1 - i].as_ptr());
        }

        // copy argv
        let stack_ptr = stack_ptr.sub(env_vars.len() + 1) as u64 as *mut *const u8;
        stack_ptr.sub(1).write(std::ptr::null_mut());
        for i in 0..user_args.len() {
            stack_ptr
                .sub(i + 2)
                .write(user_args[user_args.len() - 1 - i].as_ptr());
        }

        // copy argc
        let stack_ptr = stack_ptr.sub(user_args.len() + 1) as u64 as *mut i32;
        stack_ptr.sub(1).write(user_args.len() as i32);
        let stack_top = stack_ptr.sub(1) as u64;
        return stack_top;
    }
}

pub fn setup_inst_mem() -> u64 {
    let addr = TRANSLATOR_BASE - STACK_OFFSET;
    let inst_mem_addr = unsafe {
        libc::mmap(
            addr as *mut libc::c_void,
            STACK_OFFSET as usize,
            libc::PROT_WRITE | libc::PROT_READ | libc::PROT_EXEC,
            libc::MAP_FIXED_NOREPLACE
                | libc::MAP_ANONYMOUS
                | libc::MAP_PRIVATE
                | libc::MAP_NORESERVE,
            -1,
            0,
        ) as u64
    };
    assert_eq!(addr, inst_mem_addr);
    return inst_mem_addr;
}
