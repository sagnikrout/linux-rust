//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/vpe/csc.h
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
// Copyright (c) 2013 Texas Instruments Inc.
//
// David Griego, <dagriego@biglakesoftware.com>
// Dale Farnsworth, <dale@farnsworth.org>
// Archit Taneja, <archit@ti.com>
//
// VPE color space converter regs
pub const CSC_CSC00: c_uint = 0x00;
pub const CSC_A0_MASK: c_uint = 0x1fff;
pub const CSC_A0_SHIFT: c_int = 0;
pub const CSC_B0_MASK: c_uint = 0x1fff;
pub const CSC_B0_SHIFT: c_int = 16;
pub const CSC_CSC01: c_uint = 0x04;
pub const CSC_C0_MASK: c_uint = 0x1fff;
pub const CSC_C0_SHIFT: c_int = 0;
pub const CSC_A1_MASK: c_uint = 0x1fff;
pub const CSC_A1_SHIFT: c_int = 16;
pub const CSC_CSC02: c_uint = 0x08;
pub const CSC_B1_MASK: c_uint = 0x1fff;
pub const CSC_B1_SHIFT: c_int = 0;
pub const CSC_C1_MASK: c_uint = 0x1fff;
pub const CSC_C1_SHIFT: c_int = 16;
pub const CSC_CSC03: c_uint = 0x0c;
pub const CSC_A2_MASK: c_uint = 0x1fff;
pub const CSC_A2_SHIFT: c_int = 0;
pub const CSC_B2_MASK: c_uint = 0x1fff;
pub const CSC_B2_SHIFT: c_int = 16;
pub const CSC_CSC04: c_uint = 0x10;
pub const CSC_C2_MASK: c_uint = 0x1fff;
pub const CSC_C2_SHIFT: c_int = 0;
pub const CSC_D0_MASK: c_uint = 0x0fff;
pub const CSC_D0_SHIFT: c_int = 16;
pub const CSC_CSC05: c_uint = 0x14;
pub const CSC_D1_MASK: c_uint = 0x0fff;
pub const CSC_D1_SHIFT: c_int = 0;
pub const CSC_D2_MASK: c_uint = 0x0fff;
pub const CSC_D2_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csc_data {
    pub base: *mut void __iomem,
    pub res: *mut resource,
    pub pdev: *mut platform_device,
}

extern "C" {
    pub fn csc_dump_regs(csc: *mut csc_data);
}
extern "C" {
    pub fn csc_set_coeff_bypass(csc: *mut csc_data, csc_reg5: *mut u32);
}
