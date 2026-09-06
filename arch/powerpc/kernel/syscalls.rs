//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/syscalls.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Implementation of various system calls for Linux/PowerPC
//
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Derived from "arch/i386/kernel/sys_i386.c"
// Adapted from the i386 version by Gary Thomas
// Modified by Cort Dougan (cort@cs.nmt.edu)
// and Paul Mackerras (paulus@cs.anu.edu.au).
//
// This file contains various random system calls that
// have a non-standard calling sequence on the Linux/PPC
// platform.
//

    static long do_mmap2(unsigned long addr, size_t len,
    unsigned long prot, unsigned long flags,
    unsigned long fd, unsigned long off, int shift)
    {
    if (!arch_validate_prot(prot, addr))
    return -EINVAL;
    if (!IS_ALIGNED(off, 1 << shift))
    return -EINVAL;
    return ksys_mmap_pgoff(addr, len, prot, flags, fd, off >> shift);
    }
    SYSCALL_DEFINE6(mmap2, unsigned long, addr, size_t, len,
    unsigned long, prot, unsigned long, flags,
    unsigned long, fd, unsigned long, pgoff)
    {
    return do_mmap2(addr, len, prot, flags, fd, pgoff, PAGE_SHIFT-12);
    }

    COMPAT_SYSCALL_DEFINE6(mmap2,
    unsigned long, addr, size_t, len,
    unsigned long, prot, unsigned long, flags,
    unsigned long, fd, unsigned long, off_4k)
    {
    return do_mmap2(addr, len, prot, flags, fd, off_4k, PAGE_SHIFT-12);
    }

    SYSCALL_DEFINE6(mmap, unsigned long, addr, size_t, len,
    unsigned long, prot, unsigned long, flags,
    unsigned long, fd, off_t, offset)
    {
    return do_mmap2(addr, len, prot, flags, fd, offset, PAGE_SHIFT);
    }

#[no_mangle]
unsafe extern "C" fn do_ppc64_personality(personality: c_ulong) -> c_long {
    static long do_ppc64_personality(unsigned long personality)
    {
    long ret;
    if (personality(current.personality) == PER_LINUX32
    && personality(personality) == PER_LINUX)
    personality = (personality & ~PER_MASK) | PER_LINUX32;
    ret = ksys_personality(personality);
    if (personality(ret) == PER_LINUX32)
    ret = (ret & ~PER_MASK) | PER_LINUX;
    return ret;
    }
    SYSCALL_DEFINE1(ppc64_personality, unsigned long, personality)
    {
    return do_ppc64_personality(personality);
    }

    COMPAT_SYSCALL_DEFINE1(ppc64_personality, unsigned long, personality)
    {
    return do_ppc64_personality(personality);
    }

    SYSCALL_DEFINE6(ppc_fadvise64_64,
    int, fd, int, advice, u32, offset_high, u32, offset_low,
    u32, len_high, u32, len_low)
    {
    return ksys_fadvise64_64(fd, merge_64(offset_high, offset_low),
    merge_64(len_high, len_low), advice);
    }
    SYSCALL_DEFINE0(switch_endian)
    {
    struct thread_info *ti;
    regs_set_return_msr(current.thread.regs,
    current.thread.regs.msr ^ MSR_LE);
//
// Set TIF_RESTOREALL so that r3 isn't clobbered on return to
// userspace. That also has the effect of restoring the non-volatile
// GPRs, so we saved them on the way in here.
//
    ti = current_thread_info();
    ti.flags |= _TIF_RESTOREALL;
    return 0;
    }
