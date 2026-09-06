//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/44x/idle.c
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
// Copyright 2008 IBM Corp.
//
// Based on arch/powerpc/platforms/pasemi/idle.c:
// Copyright (C) 2006-2007 PA Semi, Inc
//
// Added by: Jerone Young <jyoung5@us.ibm.com>
//

    static int mode_spin;
#[no_mangle]
unsafe extern "C" fn ppc44x_idle() {
    static void ppc44x_idle(void)
    {
    unsigned long msr_save;
    msr_save = mfmsr();
// set wait state MSR
    mtmsr(msr_save|MSR_WE|MSR_EE|MSR_CE|MSR_DE);
    isync();
// return to initial state
    mtmsr(msr_save);
    isync();
    }
#[no_mangle]
unsafe extern "C" fn ppc44x_idle_init() -> int __init {
    static int __init ppc44x_idle_init(void)
    {
    if (!mode_spin) {
// If we are not setting spin mode
    then we set to wait mode */
    ppc_md.power_save = &ppc44x_idle;
    }
    return 0;
    }
    arch_initcall(ppc44x_idle_init);
#[no_mangle]
unsafe extern "C" fn idle_param(p: *mut c_char) -> int __init {
    static int __init idle_param(char *p)
    {
    if (!strcmp("spin", p)) {
    mode_spin = 1;
    ppc_md.power_save = core::ptr::null_mut();
    }
    return 0;
    }
    early_param("idle", idle_param);
