//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/chrp/smp.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Smp support for CHRP machines.
//
// Written by Cort Dougan (cort@cs.nmt.edu) borrowing a great
// deal of code from the sparc and intel versions.
//
// Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
//

#[no_mangle]
unsafe extern "C" fn smp_chrp_kick_cpu(nr: c_int) -> c_int {
    static int smp_chrp_kick_cpu(int nr)
    {
// (unsigned long *)KERNELBASE = nr;
    asm volatile("dcbf 0,%0"::"r"(KERNELBASE):"memory");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smp_chrp_setup_cpu(cpu_nr: c_int) {
    static void smp_chrp_setup_cpu(int cpu_nr)
    {
    mpic_setup_this_cpu();
    }
// CHRP with openpic
    struct smp_ops_t chrp_smp_ops = {
    .cause_nmi_ipi = core::ptr::null_mut(),
    .message_pass = smp_mpic_message_pass,
    .probe = smp_mpic_probe,
    .kick_cpu = smp_chrp_kick_cpu,
    .setup_cpu = smp_chrp_setup_cpu,
    .give_timebase = rtas_give_timebase,
    .take_timebase = rtas_take_timebase,
    };
