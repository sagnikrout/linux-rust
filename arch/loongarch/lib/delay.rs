//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/lib/delay.c
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

#[no_mangle]
pub unsafe extern "C" fn __delay(cycles: c_ulong) {
    void __delay(unsigned long cycles)
    {
    let mut t0: u64 = get_cycles();
    while ((unsigned long)(get_cycles() - t0) < cycles)
    cpu_relax();
    }
    EXPORT_SYMBOL(__delay);
//
// Division by multiplication: you don't have to worry about
// loss of precision.
//
// Use only for very small delays ( < 1 msec).	Should probably use a
// lookup table, really, as the multiplications take much too long with
// short delays.  This is a "reasonable" implementation, though (and the
// first constant multiplications gets optimized away if the delay is
// a constant)
//
#[no_mangle]
pub unsafe extern "C" fn __udelay(us: c_ulong) {
    void __udelay(unsigned long us)
    {
    __delay((us * 0x000010c7ull * HZ * lpj_fine) >> 32);
    }
    EXPORT_SYMBOL(__udelay);
#[no_mangle]
pub unsafe extern "C" fn __ndelay(ns: c_ulong) {
    void __ndelay(unsigned long ns)
    {
    __delay((ns * 0x00000005ull * HZ * lpj_fine) >> 32);
    }
    EXPORT_SYMBOL(__ndelay);
