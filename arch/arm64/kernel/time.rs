//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/time.c
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
// Based on arch/arm/kernel/time.c
//
// Copyright (C) 1991, 1992, 1995  Linus Torvalds
// Modifications for ARM (C) 1994-2001 Russell King
// Copyright (C) 2012 ARM Ltd.
//

#[no_mangle]
unsafe extern "C" fn profile_pc_cb(arg: *mut c_void, pc: c_ulong) -> bool {
    static bool profile_pc_cb(void *arg, unsigned long pc)
    {
    unsigned long *prof_pc = arg;
    if (in_lock_functions(pc))
    return true;
// prof_pc = pc;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn profile_pc(regs: *mut pt_regs) -> c_ulong {
    unsigned long profile_pc(struct pt_regs *regs)
    {
    let mut prof_pc: c_ulong = 0;
    arch_stack_walk(profile_pc_cb, &prof_pc, current, regs);
    return prof_pc;
    }
    EXPORT_SYMBOL(profile_pc);
#[no_mangle]
pub unsafe extern "C" fn time_init() -> void __init {
    void __init time_init(void)
    {
    u32 arch_timer_rate;
    of_clk_init(core::ptr::null_mut());
    timer_probe();
    tick_setup_hrtimer_broadcast();
    arch_timer_rate = arch_timer_get_rate();
    if (!arch_timer_rate)
    panic("Unable to initialise architected timer.\n");
// Calibrate the delay loop directly
    lpj_fine = arch_timer_rate / HZ;
    pv_time_init();
    }
