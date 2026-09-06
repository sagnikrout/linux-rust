//! Automatically rewritten from C to Rust
//! Source: arch/riscv/lib/delay.c
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
// Copyright (C) 2012 Regents of the University of California
//

//
// This is copies from arch/arm/include/asm/delay.h
//
// Loop (or tick) based delay:
//
// loops = loops_per_jiffy * jiffies_per_sec * delay_us / us_per_sec
//
// where:
//
// jiffies_per_sec = HZ
// us_per_sec = 1000000
//
// Therefore the constant part is HZ / 1000000 which is a small
// fractional number. To make this usable with integer math, we
// scale up this constant by 2^31, perform the actual multiplication,
// and scale the result back down by 2^31 with a simple shift:
//
// loops = (loops_per_jiffy * delay_us * UDELAY_MULT) >> 31
//
// where:
//
// UDELAY_MULT = 2^31 * HZ / 1000000
// = (2^31 / 1000000) * HZ
// = 2147.483648 * HZ
// = 2147 * HZ + 483648 * HZ / 1000000
//
// 31 is the biggest scale shift value that won't overflow 32 bits for
// delay_us * UDELAY_MULT assuming HZ <= 1000 and delay_us <= 2000.
//
pub const MAX_UDELAY_US: c_int = 2000;
pub const MAX_UDELAY_HZ: c_int = 1000;

pub const UDELAY_SHIFT: c_int = 31;

//
// RISC-V supports both UDELAY and NDELAY.  This is largely the same as above,
// but with different constants.  I added 10 bits to the shift to get this, but
// the result is that I need a 64-bit multiply, which is slow on 32-bit
// platforms.
//
// NDELAY_MULT = 2^41 * HZ / 1000000000
// = (2^41 / 1000000000) * HZ
// = 2199.02325555 * HZ
// = 2199 * HZ + 23255550 * HZ / 1000000000
//
// The maximum here is to avoid 64-bit overflow, but it isn't checked as it
// won't happen.
//

pub const NDELAY_SHIFT: c_int = 41;

#[no_mangle]
pub unsafe extern "C" fn __delay(cycles: c_ulong) {
    void __delay(unsigned long cycles)
    {
    let mut t0: u64 = get_cycles();
    while ((unsigned long)(get_cycles() - t0) < cycles)
    cpu_relax();
    }
    EXPORT_SYMBOL(__delay);
#[no_mangle]
pub unsafe extern "C" fn udelay(usecs: c_ulong) {
    void udelay(unsigned long usecs)
    {
    let mut ucycles: u64 = (u64)usecs * lpj_fine * UDELAY_MULT;
    u64 n;
    if (unlikely(usecs > MAX_UDELAY_US)) {
    n = (u64)usecs * riscv_timebase;
    do_div(n, 1000000);
    __delay(n);
    return;
    }
    __delay(ucycles >> UDELAY_SHIFT);
    }
    EXPORT_SYMBOL(udelay);
#[no_mangle]
pub unsafe extern "C" fn ndelay(nsecs: c_ulong) {
    void ndelay(unsigned long nsecs)
    {
//
// This doesn't bother checking for overflow, as it won't happen (it's
// an hour) of delay.
//
    let mut ncycles: c_ulonglong = nsecs * lpj_fine * NDELAY_MULT;
    __delay(ncycles >> NDELAY_SHIFT);
    }
    EXPORT_SYMBOL(ndelay);
#[no_mangle]
pub unsafe extern "C" fn delay_read_timer(timer_val: *mut c_ulong) -> bool {
    bool delay_read_timer(unsigned long *timer_val)
    {
// timer_val = get_cycles();
    return true;
    }
