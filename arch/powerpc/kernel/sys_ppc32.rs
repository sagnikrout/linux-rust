//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/sys_ppc32.c
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
// sys_ppc32.c: 32-bit system calls with complex calling conventions.
//
// Copyright (C) 2001 IBM
// Copyright (C) 1997,1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
// Copyright (C) 1997 David S. Miller (davem@caip.rutgers.edu)
//
// 32-bit system calls with 64-bit arguments pass those in register pairs.
// This must be specially dealt with on 64-bit kernels. The compat_arg_u64_dual
// in generic compat syscalls is not always usable because the register
// pairing is constrained depending on preceding arguments.
//
// An analogous problem exists on 32-bit kernels with ARCH_HAS_SYSCALL_WRAPPER,
// the defined system call functions take the pt_regs as an argument, and there
// is a mapping macro which maps registers to arguments
// (SC_POWERPC_REGS_TO_ARGS) which also does not deal with these 64-bit
// arguments.
//
// This file contains these system calls.
//

    PPC32_SYSCALL_DEFINE6(ppc_pread64,
    unsigned int, fd,
    char __user *, ubuf, compat_size_t, count,
    u32, reg6, u32, pos1, u32, pos2)
    {
    return ksys_pread64(fd, ubuf, count, merge_64(pos1, pos2));
    }
    PPC32_SYSCALL_DEFINE6(ppc_pwrite64,
    unsigned int, fd,
    const char __user *, ubuf, compat_size_t, count,
    u32, reg6, u32, pos1, u32, pos2)
    {
    return ksys_pwrite64(fd, ubuf, count, merge_64(pos1, pos2));
    }
    PPC32_SYSCALL_DEFINE5(ppc_readahead,
    int, fd, u32, r4,
    u32, offset1, u32, offset2, u32, count)
    {
    return ksys_readahead(fd, merge_64(offset1, offset2), count);
    }
    PPC32_SYSCALL_DEFINE4(ppc_truncate64,
    const char __user *, path, u32, reg4,
    unsigned long, len1, unsigned long, len2)
    {
    return ksys_truncate(path, merge_64(len1, len2));
    }
    PPC32_SYSCALL_DEFINE4(ppc_ftruncate64,
    unsigned int, fd, u32, reg4,
    unsigned long, len1, unsigned long, len2)
    {
    return ksys_ftruncate(fd, merge_64(len1, len2), FTRUNCATE_LFS);
    }
    PPC32_SYSCALL_DEFINE6(ppc32_fadvise64,
    int, fd, u32, unused, u32, offset1, u32, offset2,
    size_t, len, int, advice)
    {
    return ksys_fadvise64_64(fd, merge_64(offset1, offset2), len,
    advice);
    }
    PPC32_SYSCALL_DEFINE6(ppc_sync_file_range2,
    int, fd, unsigned int, flags,
    unsigned int, offset1, unsigned int, offset2,
    unsigned int, nbytes1, unsigned int, nbytes2)
    {
    let mut offset: loff_t = merge_64(offset1, offset2);
    let mut nbytes: loff_t = merge_64(nbytes1, nbytes2);
    return ksys_sync_file_range(fd, offset, nbytes, flags);
    }

    SYSCALL_DEFINE6(ppc_fallocate,
    int, fd, int, mode,
    u32, offset1, u32, offset2, u32, len1, u32, len2)
    {
    return ksys_fallocate(fd, mode,
    merge_64(offset1, offset2),
    merge_64(len1, len2));
    }
