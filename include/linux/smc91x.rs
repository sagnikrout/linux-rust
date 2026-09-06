//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/smc91x.h
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
// These bits define which access sizes a platform can support, rather
// than the maximal access size.  So, if your platform can do 16-bit
// and 32-bit accesses to the SMC91x device, but not 8-bit, set both
// SMC91X_USE_16BIT and SMC91X_USE_32BIT.
//
// The SMC91x driver requires at least one of SMC91X_USE_8BIT or
// SMC91X_USE_16BIT to be supported - just setting SMC91X_USE_32BIT is
// an invalid configuration.
//

// two bits for IO_SHIFT, let's hope later designs will keep this sane

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc91x_platdata {
    pub flags: c_ulong,
    pub leda: c_uchar,
    pub ledb: c_uchar,
    pub /: *mut *mut *mut bool pxa_u16_align4; / PXA buggy u16 writes on 4n+2 addresses,
}
