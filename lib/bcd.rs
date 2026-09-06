//! Automatically rewritten from C to Rust
//! Source: lib/bcd.c
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

#[no_mangle]
pub unsafe extern "C" fn _bcd2bin(val: c_uchar) -> unsigned {
    unsigned _bcd2bin(unsigned char val)
    {
    return (val & 0x0f) + (val >> 4) * 10;
    }
    EXPORT_SYMBOL(_bcd2bin);
#[no_mangle]
pub unsafe extern "C" fn _bin2bcd(val: unsigned) -> c_uchar {
    unsigned char _bin2bcd(unsigned val)
    {
    let mut t: c_uint = (val * 103) >> 10;
    return (t << 4) | (val - t * 10);
    }
    EXPORT_SYMBOL(_bin2bcd);
