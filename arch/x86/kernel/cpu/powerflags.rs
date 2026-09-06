//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/powerflags.c
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
// Strings for the various x86 power flags
//
// This file must not contain any executable code.
//

    const char *const x86_power_flags[32] = {
    "ts",	/* temperature sensor */
    "fid",  /* frequency id control */
    "vid",  /* voltage id control */
    "ttp",  /* thermal trip */
    "tm",	/* hardware thermal control */
    "stc",	/* software thermal control */
    "100mhzsteps", /* 100 MHz multiplier control */
    "hwpstate", /* hardware P-state control */
    "",	/* tsc invariant mapped to constant_tsc */
    "cpb",  /* core performance boost */
    "eff_freq_ro", /* Readonly aperf/mperf */
    "proc_feedback", /* processor feedback interface */
    "acc_power", /* accumulated power mechanism */
    };
