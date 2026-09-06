//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pasemi/idle.c
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
// Copyright (C) 2006-2007 PA Semi, Inc
//
// Maintained by: Olof Johansson <olof@lixom.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sleep_mode {
    pub name: *mut c_char,
    pub (*entry)(void): *mut c_void,
}

    static struct sleep_mode modes[] = {
    { .name = "spin", .entry = &idle_spin },
    { .name = "doze", .entry = &idle_doze },
    };
    let mut current_mode: static int = 0;
#[no_mangle]
unsafe extern "C" fn pasemi_system_reset_exception(regs: *mut pt_regs) -> c_int {
    static int pasemi_system_reset_exception(struct pt_regs *regs)
    {
// If we were woken up from power savings, we need to return
// to the calling function, since nip is not saved across
// all modes.
//
    if (regs.msr & SRR1_WAKEMASK)
    regs_set_return_ip(regs, regs.link);
    switch (regs.msr & SRR1_WAKEMASK) {
    case SRR1_WAKEDEC:
    set_dec(1);
    break;
    case SRR1_WAKEEE:
//
// Handle these when interrupts get re-enabled and we take
// them as regular exceptions. We are in an NMI context
// and can't handle these here.
//
    break;
    default:
// do system reset
    return 0;
    }
// Set higher astate since we come out of power savings at 0
    restore_astate(hard_smp_processor_id());
// everything handled
    regs_set_recoverable(regs);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn pasemi_idle_init() -> int __init {
    static int __init pasemi_idle_init(void)
    {

    pr_warn("No cpufreq driver, powersavings modes disabled\n");
    current_mode = 0;

    ppc_md.system_reset_exception = pasemi_system_reset_exception;
    ppc_md.power_save = modes[current_mode].entry;
    pr_info("Using PA6T idle loop (%s)\n", modes[current_mode].name);
    return 0;
    }
    machine_late_initcall(pasemi, pasemi_idle_init);
#[no_mangle]
unsafe extern "C" fn idle_param(p: *mut c_char) -> int __init {
    static int __init idle_param(char *p)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(modes); i++) {
    if (!strcmp(modes[i].name, p)) {
    current_mode = i;
    break;
    }
    }
    return 0;
    }
    early_param("idle", idle_param);
