//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/rdrand.c
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
// This file is part of the Linux kernel.
//
// Copyright (c) 2011, Intel Corporation
// Authors: Fenghua Yu <fenghua.yu@intel.com>,
// H. Peter Anvin <hpa@linux.intel.com>
//

//
// RDRAND has Built-In-Self-Test (BIST) that runs on every invocation.
// Run the instruction a few times as a sanity check. Also make sure
// it's not outputting the same value over and over, which has happened
// as a result of past CPU bugs.
//
// If it fails, it is simple to disable RDRAND and RDSEED here.
//
#[no_mangle]
pub unsafe extern "C" fn x86_init_rdrand(c: *mut cpuinfo_x86) {
    void x86_init_rdrand(struct cpuinfo_x86 *c)
    {
    enum { SAMPLES = 8, MIN_CHANGE = 5 };
    unsigned long sample, prev;
    let mut failure: bool = false;
    size_t i, changed;
    if (!cpu_has(c, X86_FEATURE_RDRAND))
    return;
    for (changed = 0, i = 0; i < SAMPLES; ++i) {
    if (!rdrand_long(&sample)) {
    failure = true;
    break;
    }
    changed += i && sample != prev;
    prev = sample;
    }
    if (changed < MIN_CHANGE)
    failure = true;
    if (failure) {
    clear_cpu_cap(c, X86_FEATURE_RDRAND);
    clear_cpu_cap(c, X86_FEATURE_RDSEED);
    pr_emerg("RDRAND is not reliable on this platform; disabling.\n");
    }
    }
