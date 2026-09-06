//! Automatically rewritten from C to Rust
//! Source: arch/s390/lib/delay.c
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
// Precise Delay Loops for S390
//
// Copyright IBM Corp. 1999, 2008
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
//

#[no_mangle]
pub unsafe extern "C" fn __delay(loops: c_ulong) {
    void __delay(unsigned long loops)
    {
//
// Loop 'loops' times. Callers must not assume a specific
// amount of time passes before this function returns.
//
    asm volatile("0: brct %0,0b" : : "d" ((loops/2) + 1));
    }
    EXPORT_SYMBOL(__delay);
#[no_mangle]
unsafe extern "C" fn delay_loop(delta: c_ulong) {
    static void delay_loop(unsigned long delta)
    {
    unsigned long end;
    end = get_tod_clock_monotonic() + delta;
    while (!tod_after(get_tod_clock_monotonic(), end))
    cpu_relax();
    }
#[no_mangle]
pub unsafe extern "C" fn __udelay(usecs: c_ulong) {
    void __udelay(unsigned long usecs)
    {
    delay_loop(usecs << 12);
    }
    EXPORT_SYMBOL(__udelay);
#[no_mangle]
pub unsafe extern "C" fn __ndelay(nsecs: c_ulong) {
    void __ndelay(unsigned long nsecs)
    {
    nsecs <<= 9;
    do_div(nsecs, 125);
    delay_loop(nsecs);
    }
    EXPORT_SYMBOL(__ndelay);
