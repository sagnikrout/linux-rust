//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/fsl-dcu/fsl_tcon.h
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
// Copyright 2015 Toradex AG
//
// Stefan Agner <stefan@agner.ch>
//
// Freescale TCON device driver
//

pub const FSL_TCON_CTRL1: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_tcon {
    pub regs: *mut regmap,
    pub ipg_clk: *mut clk,
}

extern "C" {
    pub fn fsl_tcon_free(tcon: *mut fsl_tcon);
}
extern "C" {
    pub fn fsl_tcon_bypass_disable(tcon: *mut fsl_tcon);
}
extern "C" {
    pub fn fsl_tcon_bypass_enable(tcon: *mut fsl_tcon);
}
