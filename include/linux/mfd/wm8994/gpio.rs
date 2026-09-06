//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8994/gpio.h
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
// include/linux/mfd/wm8994/gpio.h - GPIO configuration for WM8994
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
pub const WM8994_GPIO_MAX: c_int = 11;
pub const WM8994_GP_FN_PIN_SPECIFIC: c_int = 0;
pub const WM8994_GP_FN_GPIO: c_int = 1;
pub const WM8994_GP_FN_SDOUT: c_int = 2;
pub const WM8994_GP_FN_IRQ: c_int = 3;
pub const WM8994_GP_FN_TEMPERATURE: c_int = 4;
pub const WM8994_GP_FN_MICBIAS1_DET: c_int = 5;
pub const WM8994_GP_FN_MICBIAS1_SHORT: c_int = 6;
pub const WM8994_GP_FN_MICBIAS2_DET: c_int = 7;
pub const WM8994_GP_FN_MICBIAS2_SHORT: c_int = 8;
pub const WM8994_GP_FN_FLL1_LOCK: c_int = 9;
pub const WM8994_GP_FN_FLL2_LOCK: c_int = 10;
pub const WM8994_GP_FN_SRC1_LOCK: c_int = 11;
pub const WM8994_GP_FN_SRC2_LOCK: c_int = 12;
pub const WM8994_GP_FN_DRC1_ACT: c_int = 13;
pub const WM8994_GP_FN_DRC2_ACT: c_int = 14;
pub const WM8994_GP_FN_DRC3_ACT: c_int = 15;
pub const WM8994_GP_FN_WSEQ_STATUS: c_int = 16;
pub const WM8994_GP_FN_FIFO_ERROR: c_int = 17;
pub const WM8994_GP_FN_OPCLK: c_int = 18;
pub const WM8994_GP_FN_THW: c_int = 19;
pub const WM8994_GP_FN_DCS_DONE: c_int = 20;
pub const WM8994_GP_FN_FLL1_OUT: c_int = 21;
pub const WM8994_GP_FN_FLL2_OUT: c_int = 22;
pub const WM8994_GPN_DIR: c_uint = 0x8000  /* GPN_DIR */;
pub const WM8994_GPN_DIR_MASK: c_uint = 0x8000  /* GPN_DIR */;

pub const WM8994_GPN_PU: c_uint = 0x4000  /* GPN_PU */;
pub const WM8994_GPN_PU_MASK: c_uint = 0x4000  /* GPN_PU */;

pub const WM8994_GPN_PD: c_uint = 0x2000  /* GPN_PD */;
pub const WM8994_GPN_PD_MASK: c_uint = 0x2000  /* GPN_PD */;

pub const WM8994_GPN_POL: c_uint = 0x0400  /* GPN_POL */;
pub const WM8994_GPN_POL_MASK: c_uint = 0x0400  /* GPN_POL */;

pub const WM8994_GPN_OP_CFG: c_uint = 0x0200  /* GPN_OP_CFG */;
pub const WM8994_GPN_OP_CFG_MASK: c_uint = 0x0200  /* GPN_OP_CFG */;

pub const WM8994_GPN_DB: c_uint = 0x0100  /* GPN_DB */;
pub const WM8994_GPN_DB_MASK: c_uint = 0x0100  /* GPN_DB */;

pub const WM8994_GPN_LVL: c_uint = 0x0040  /* GPN_LVL */;
pub const WM8994_GPN_LVL_MASK: c_uint = 0x0040  /* GPN_LVL */;

pub const WM8994_GPN_FN_MASK: c_uint = 0x001F  /* GPN_FN - [4:0] */;

