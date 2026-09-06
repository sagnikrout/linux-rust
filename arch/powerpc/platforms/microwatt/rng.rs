//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/microwatt/rng.c
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
// Derived from arch/powerpc/platforms/powernv/rng.c, which is:
// Copyright 2013, Michael Ellerman, IBM Corporation.
//

pub const DARN_ERR: c_uint = 0xFFFFFFFFFFFFFFFFul;
#[no_mangle]
unsafe extern "C" fn microwatt_get_random_darn(v: *mut c_ulong) -> c_int {
    static int microwatt_get_random_darn(unsigned long *v)
    {
    unsigned long val;
// Using DARN with L=1 - 64-bit conditioned random number
    asm volatile(PPC_DARN(%0, 1) : "=r"(val));
    if (val == DARN_ERR)
    return 0;
// v = val;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn microwatt_rng_init() -> void __init {
    void __init microwatt_rng_init(void)
    {
    unsigned long val;
    int i;
    for (i = 0; i < 10; i++) {
    if (microwatt_get_random_darn(&val)) {
    ppc_md.get_random_seed = microwatt_get_random_darn;
    return;
    }
    }
    }
