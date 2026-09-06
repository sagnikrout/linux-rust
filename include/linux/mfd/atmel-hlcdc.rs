//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/atmel-hlcdc.h
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
// Copyright (C) 2014 Free Electrons
// Copyright (C) 2014 Atmel
//
// Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
//

pub const ATMEL_HLCDC_EN: c_uint = 0x20;
pub const ATMEL_HLCDC_DIS: c_uint = 0x24;
pub const ATMEL_HLCDC_SR: c_uint = 0x28;
pub const ATMEL_HLCDC_IER: c_uint = 0x2c;
pub const ATMEL_HLCDC_IDR: c_uint = 0x30;
pub const ATMEL_HLCDC_IMR: c_uint = 0x34;
pub const ATMEL_HLCDC_ISR: c_uint = 0x38;
pub const ATMEL_XLCDC_ATTRE: c_uint = 0x3c;

pub const ATMEL_HLCDC_CLKDIV_SHFT: c_int = 16;

//
// Structure shared by the MFD device and its subdevices.
//
// @regmap: register map used to access HLCDC IP registers
// @periph_clk: the hlcdc peripheral clock
// @sys_clk: the hlcdc system clock
// @slow_clk: the system slow clk
// @irq: the hlcdc irq
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc {
    pub regmap: *mut regmap,
    pub lvds_pll_clk: *mut clk,
    pub periph_clk: *mut clk,
    pub sys_clk: *mut clk,
    pub slow_clk: *mut clk,
    pub irq: c_int,
}
