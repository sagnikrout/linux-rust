//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/stm32-lptimer.h
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
// STM32 Low-Power Timer parent driver.
// Copyright (C) STMicroelectronics 2017
// Author: Fabrice Gasnier <fabrice.gasnier@st.com>
// Inspired by Benjamin Gaignard's stm32-timers driver
//

pub const STM32_LPTIM_ISR: c_uint = 0x00	/* Interrupt and Status Reg  */;
pub const STM32_LPTIM_ICR: c_uint = 0x04	/* Interrupt Clear Reg       */;
pub const STM32_LPTIM_IER: c_uint = 0x08	/* Interrupt Enable Reg      */;
pub const STM32_LPTIM_CFGR: c_uint = 0x0C	/* Configuration Reg         */;
pub const STM32_LPTIM_CR: c_uint = 0x10	/* Control Reg               */;
pub const STM32_LPTIM_CMP: c_uint = 0x14	/* Compare Reg (MP25 CCR1)   */;
pub const STM32_LPTIM_ARR: c_uint = 0x18	/* Autoreload Reg            */;
pub const STM32_LPTIM_CNT: c_uint = 0x1C	/* Counter Reg               */;
pub const STM32_LPTIM_CCMR1: c_uint = 0x2C	/* Capture/Compare Mode MP25 */;
pub const STM32_LPTIM_CCR2: c_uint = 0x34	/* Compare Reg2 MP25         */;
pub const STM32_LPTIM_HWCFGR2: c_uint = 0x3EC	/* Hardware configuration register 2 - MP25 */;
pub const STM32_LPTIM_HWCFGR1: c_uint = 0x3F0	/* Hardware configuration register 1 - MP15 */;
pub const STM32_LPTIM_VERR: c_uint = 0x3F4	/* Version identification register - MP15 */;
// STM32_LPTIM_ISR - bit fields

// STM32_LPTIM_ICR - bit fields

// STM32_LPTIM_IER - bit fields

// STM32_LPTIM_CR - bit fields

// STM32_LPTIM_CFGR - bit fields

// STM32_LPTIM_CKPOL
pub const STM32_LPTIM_CKPOL_RISING_EDGE: c_int = 0;
pub const STM32_LPTIM_CKPOL_FALLING_EDGE: c_int = 1;
pub const STM32_LPTIM_CKPOL_BOTH_EDGES: c_int = 2;
// STM32_LPTIM_ARR
pub const STM32_LPTIM_MAX_ARR: c_uint = 0xFFFF;
// STM32_LPTIM_CCMR1

// STM32_LPTIM_HWCFGR1

// STM32_LPTIM_HWCFGR2

// STM32_LPTIM_VERR
pub const STM32_LPTIM_VERR_23: c_uint = 0x23	/* STM32MP25 */;
//
// struct stm32_lptimer - STM32 Low-Power Timer data assigned by parent device
// @clk: clock reference for this instance
// @regmap: register map reference for this instance
// @has_encoder: indicates this Low-Power Timer supports encoder mode
// @num_cc_chans: indicates the number of capture/compare channels
// @version: indicates the major and minor revision of the controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_lptimer {
    pub clk: *mut clk,
    pub regmap: *mut regmap,
    pub has_encoder: bool,
    pub num_cc_chans: c_uint,
    pub version: u32,
}
