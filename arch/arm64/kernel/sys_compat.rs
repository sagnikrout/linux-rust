//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/sys_compat.c
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
// Based on arch/arm/kernel/sys_arm.c
//
// Copyright (C) People who wrote linux/arch/i386/kernel/sys_i386.c
// Copyright (C) 1995, 1996 Russell King.
// Copyright (C) 2012 ARM Ltd.
//

    static long
    __do_compat_cache_op(unsigned long start, unsigned long end)
    {
    long ret;
    do {
    let mut chunk: c_ulong = min(PAGE_SIZE, end - start);
    if (fatal_signal_pending(current))
    return 0;
    if (cpus_have_final_cap(ARM64_WORKAROUND_1542419)) {
//
// The workaround requires an inner-shareable tlbi.
// We pick the reserved-ASID to minimise the impact.
//
    __tlbi(aside1is, 0UL);
    __tlbi_sync_s1ish(current.mm);
    }
    ret = caches_clean_inval_user_pou(start, start + chunk);
    if (ret)
    return ret;
    cond_resched();
    start += chunk;
    } while (start < end);
    return 0;
    }
    static inline long
    do_compat_cache_op(unsigned long start, unsigned long end, int flags)
    {
    if (end < start || flags)
    return -EINVAL;
    if (!access_ok((const void __user *)start, end - start))
    return -EFAULT;
    return __do_compat_cache_op(start, end);
    }
//
// Handle all unrecognised system calls.
//
#[no_mangle]
pub unsafe extern "C" fn compat_arm_syscall(regs: *mut pt_regs, scno: c_int) -> c_long {
    long compat_arm_syscall(struct pt_regs *regs, int scno)
    {
    unsigned long addr;
    switch (scno) {
//
// Flush a region from virtual address 'r0' to virtual address 'r1'
// _exclusive_.  There is no alignment requirement on either address;
// user space does not need to know the hardware cache layout.
//
// r2 contains flags.  It should ALWAYS be passed as ZERO until it
// is defined to be something else.  For now we ignore it, but may
// the fires of hell burn in your belly if you break this rule. ;)
//
// (at a later date, we may want to allow this call to not flush
// various aspects of the cache.  Passing '0' will guarantee that
// everything necessary gets flushed to maintain consistency in
// the specified region).
//
    case __ARM_NR_compat_cacheflush:
    return do_compat_cache_op(regs.regs[0], regs.regs[1], regs.regs[2]);
    case __ARM_NR_compat_set_tls:
    current.thread.uw.tp_value = regs.regs[0];
//
// Protect against register corruption from context switch.
// See comment in tls_thread_flush.
//
    barrier();
    write_sysreg(regs.regs[0], tpidrro_el0);
    return 0;
    default:
//
// Calls 0xf0xxx..0xf07ff are defined to return -ENOSYS
// if not implemented, rather than raising SIGILL. This
// way the calling program can gracefully determine whether
// a feature is supported.
//
    if (scno < __ARM_NR_COMPAT_END)
    return -ENOSYS;
    break;
    }
    addr = instruction_pointer(regs) - (compat_thumb_mode(regs) ? 2 : 4);
    arm64_notify_die("Oops - bad compat syscall(2)", regs,
    SIGILL, ILL_ILLTRP, addr, 0);
    return 0;
    }
