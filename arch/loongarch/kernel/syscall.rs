//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/syscall.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0+
//
// Author: Hanlu Li <lihanlu@loongson.cn>
// Huacai Chen <chenhuacai@loongson.cn>
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    SYSCALL_DEFINE6(mmap, unsigned long, addr, unsigned long, len, unsigned long,
    prot, unsigned long, flags, unsigned long, fd, unsigned long, offset)
    {
    if (offset & ~PAGE_MASK)
    return -EINVAL;
    return ksys_mmap_pgoff(addr, len, prot, flags, fd, offset >> PAGE_SHIFT);
    }
    SYSCALL_DEFINE6(mmap2, unsigned long, addr, unsigned long, len, unsigned long,
    prot, unsigned long, flags, unsigned long, fd, unsigned long, offset)
    {
    if (offset & (~PAGE_MASK >> 12))
    return -EINVAL;
    return ksys_mmap_pgoff(addr, len, prot, flags, fd, offset >> (PAGE_SHIFT - 12));
    }
    void *sys_call_table[__NR_syscalls] = {
    [0 ... __NR_syscalls - 1] = sys_ni_syscall,

    };
    typedef long (*sys_call_fn)(unsigned long, unsigned long,
    unsigned long, unsigned long, unsigned long, unsigned long);
#[no_mangle]
pub unsafe extern "C" fn do_syscall(regs: *mut pt_regs) -> void noinstr __no_stack_protector {
    void noinstr __no_stack_protector do_syscall(struct pt_regs *regs)
    {
    sys_call_fn syscall_fn;
    unsigned long nr;
    nr = regs.regs[11];
// Set for syscall restarting
    if (nr < NR_syscalls)
    regs.regs[0] = nr + 1;
    regs.csr_era += 4;
    regs.orig_a0 = regs.regs[4];
    regs.regs[4] = -ENOSYS;
    if (likely(syscall_enter_from_user_mode_randomize_stack(regs, &nr))) {
    if (nr < NR_syscalls) {
    syscall_fn = sys_call_table[array_index_nospec(nr, NR_syscalls)];
    regs.regs[4] = syscall_fn(regs.orig_a0, regs.regs[5], regs.regs[6],
    regs.regs[7], regs.regs[8], regs.regs[9]);
    }
    }
    syscall_exit_to_user_mode(regs);
    }
