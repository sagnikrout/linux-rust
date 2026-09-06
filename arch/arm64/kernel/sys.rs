//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/sys.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// AArch64-specific system calls implementation
//
// Copyright (C) 2012 ARM Ltd.
// Author: Catalin Marinas <catalin.marinas@arm.com>
//

    SYSCALL_DEFINE6(mmap, unsigned long, addr, unsigned long, len,
    unsigned long, prot, unsigned long, flags,
    unsigned long, fd, unsigned long, off)
    {
    if (offset_in_page(off) != 0)
    return -EINVAL;
    return ksys_mmap_pgoff(addr, len, prot, flags, fd, off >> PAGE_SHIFT);
    }
    SYSCALL_DEFINE1(arm64_personality, unsigned int, personality)
    {
    if (personality(personality) == PER_LINUX32 &&
    !system_supports_32bit_el0())
    return -EINVAL;
    return ksys_personality(personality);
    }
    asmlinkage long sys_ni_syscall(void);
#[no_mangle]
pub unsafe extern "C" fn __arm64_sys_ni_syscall(__unused: *const pt_regs) -> asmlinkage long {
    asmlinkage long __arm64_sys_ni_syscall(const struct pt_regs *__unused)
    {
    return sys_ni_syscall();
    }
//
// Wrappers to pass the pt_regs argument.
//

    const syscall_fn_t sys_call_table[__NR_syscalls] = {
    [0 ... __NR_syscalls - 1] = __arm64_sys_ni_syscall,

    };
