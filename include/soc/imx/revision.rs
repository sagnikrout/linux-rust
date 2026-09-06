//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/imx/revision.h
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
// Copyright 2015 Linaro Ltd.
//
pub const IMX_CHIP_REVISION_1_0: c_uint = 0x10;
pub const IMX_CHIP_REVISION_1_1: c_uint = 0x11;
pub const IMX_CHIP_REVISION_1_2: c_uint = 0x12;
pub const IMX_CHIP_REVISION_1_3: c_uint = 0x13;
pub const IMX_CHIP_REVISION_1_4: c_uint = 0x14;
pub const IMX_CHIP_REVISION_1_5: c_uint = 0x15;
pub const IMX_CHIP_REVISION_2_0: c_uint = 0x20;
pub const IMX_CHIP_REVISION_2_1: c_uint = 0x21;
pub const IMX_CHIP_REVISION_2_2: c_uint = 0x22;
pub const IMX_CHIP_REVISION_2_3: c_uint = 0x23;
pub const IMX_CHIP_REVISION_3_0: c_uint = 0x30;
pub const IMX_CHIP_REVISION_3_1: c_uint = 0x31;
pub const IMX_CHIP_REVISION_3_2: c_uint = 0x32;
pub const IMX_CHIP_REVISION_3_3: c_uint = 0x33;
pub const IMX_CHIP_REVISION_UNKNOWN: c_uint = 0xff;
extern "C" {
    pub fn mx25_revision() -> c_int;
}
extern "C" {
    pub fn mx27_revision() -> c_int;
}
extern "C" {
    pub fn mx31_revision() -> c_int;
}
extern "C" {
    pub fn mx35_revision() -> c_int;
}
extern "C" {
    pub fn mx51_revision() -> c_int;
}
extern "C" {
    pub fn mx53_revision() -> c_int;
}
extern "C" {
    pub fn imx_get_soc_revision() -> c_uint;
}
extern "C" {
    pub fn imx_print_silicon_rev(cpu: *const c_char, srev: c_int);
}
