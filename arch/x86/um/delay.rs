//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/delay.c
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
// Copyright (C) 2011 Richard Weinberger <richrd@nod.at>
// Mostly copied from arch/x86/lib/delay.c
//

#[no_mangle]
pub unsafe extern "C" fn __delay(loops: c_ulong) {
    void __delay(unsigned long loops)
    {
    asm volatile(
    "test %0,%0\n"
    "jz 3f\n"
    "jmp 1f\n"
    ".align 16\n"
    "1: jmp 2f\n"
    ".align 16\n"
    "2: dec %0\n"
    " jnz 2b\n"
    "3: dec %0\n"
    : /* we don't need output */
    : "a" (loops)
    );
    }
    EXPORT_SYMBOL(__delay);
#[no_mangle]
pub unsafe extern "C" fn __const_udelay(xloops: c_ulong) {
    inline void __const_udelay(unsigned long xloops)
    {
    int d0;
    xloops *= 4;
    asm("mull %%edx"
    : "=d" (xloops), "=&a" (d0)
    : "1" (xloops), "0"
    (loops_per_jiffy * (HZ/4)));
    __delay(++xloops);
    }
    EXPORT_SYMBOL(__const_udelay);
#[no_mangle]
pub unsafe extern "C" fn __udelay(usecs: c_ulong) {
    void __udelay(unsigned long usecs)
    {
    __const_udelay(usecs * 0x000010c7); /* 2**32 / 1000000 (rounded up) */
    }
    EXPORT_SYMBOL(__udelay);
#[no_mangle]
pub unsafe extern "C" fn __ndelay(nsecs: c_ulong) {
    void __ndelay(unsigned long nsecs)
    {
    __const_udelay(nsecs * 0x00005); /* 2**32 / 1000000000 (rounded up) */
    }
    EXPORT_SYMBOL(__ndelay);
