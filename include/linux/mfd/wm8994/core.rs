//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8994/core.h
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
// include/linux/mfd/wm8994/core.h -- Core interface for WM8994
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm8994_type {
    WM8994 = 0,
    WM8958 = 1,
    WM1811 = 2,
}

pub const WM8994_NUM_GPIO_REGS: c_int = 11;
pub const WM8994_NUM_LDO_REGS: c_int = 2;
pub const WM8994_NUM_IRQ_REGS: c_int = 2;
pub const WM8994_IRQ_TEMP_SHUT: c_int = 0;
pub const WM8994_IRQ_MIC1_DET: c_int = 1;
pub const WM8994_IRQ_MIC1_SHRT: c_int = 2;
pub const WM8994_IRQ_MIC2_DET: c_int = 3;
pub const WM8994_IRQ_MIC2_SHRT: c_int = 4;
pub const WM8994_IRQ_FLL1_LOCK: c_int = 5;
pub const WM8994_IRQ_FLL2_LOCK: c_int = 6;
pub const WM8994_IRQ_SRC1_LOCK: c_int = 7;
pub const WM8994_IRQ_SRC2_LOCK: c_int = 8;
pub const WM8994_IRQ_AIF1DRC1_SIG_DET: c_int = 9;
pub const WM8994_IRQ_AIF1DRC2_SIG_DET: c_int = 10;
pub const WM8994_IRQ_AIF2DRC_SIG_DET: c_int = 11;
pub const WM8994_IRQ_FIFOS_ERR: c_int = 12;
pub const WM8994_IRQ_WSEQ_DONE: c_int = 13;
pub const WM8994_IRQ_DCS_DONE: c_int = 14;
pub const WM8994_IRQ_TEMP_WARN: c_int = 15;
// GPIOs in the chip are numbered from 1-11

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8994 {
    pub pdata: wm8994_pdata,
    pub type: wm8994_type,
    pub revision: c_int,
    pub cust_id: c_int,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub ldo_ena_always_driven: bool,
    pub gpio_base: c_int,
    pub irq_base: c_int,
    pub irq: c_int,
    pub irq_data: *mut regmap_irq_chip_data,
    pub edge_irq: *mut irq_domain,
// Used over suspend/resume
    pub suspended: bool,
    pub dbvdd: *mut regulator_dev,
    pub num_supplies: c_int,
    pub supplies: *mut regulator_bulk_data,
}

// Device I/O API
extern "C" {
    pub fn regmap_write(_arg: wm8994->regmap, _arg: reg, _arg: val) -> return;
}
extern "C" {
    pub fn regmap_bulk_read(_arg: wm8994->regmap, _arg: reg, _arg: buf, _arg: count) -> return;
}
extern "C" {
    pub fn regmap_raw_write(_arg: wm8994->regmap, _arg: reg, _arg: buf, sizeof(u16): *mut *mut count) -> return;
}
extern "C" {
    pub fn regmap_update_bits(_arg: wm8994->regmap, _arg: reg, _arg: mask, _arg: val) -> return;
}
// Helper to save on boilerplate
extern "C" {
    pub fn wm8994_irq_init(wm8994: *mut wm8994) -> c_int;
}
extern "C" {
    pub fn wm8994_irq_exit(wm8994: *mut wm8994);
}
