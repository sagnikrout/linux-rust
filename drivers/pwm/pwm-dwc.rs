//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pwm/pwm-dwc.h
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
// DesignWare PWM Controller driver
//
// Copyright (C) 2018-2020 Intel Corporation
//
// Author: Felipe Balbi (Intel)
// Author: Jarkko Nikula <jarkko.nikula@linux.intel.com>
// Author: Raymond Tan <raymond.tan@intel.com>
//

pub const DWC_TIMERS_INT_STS: c_uint = 0xa0;
pub const DWC_TIMERS_EOI: c_uint = 0xa4;
pub const DWC_TIMERS_RAW_INT_STS: c_uint = 0xa8;
pub const DWC_TIMERS_COMP_VERSION: c_uint = 0xac;
pub const DWC_TIMERS_TOTAL: c_int = 8;
// Timer Control Register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc_pwm_info {
    pub nr: c_uint,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc_pwm_drvdata {
    pub info: *const dwc_pwm_info,
    pub io_base: *mut void __iomem,
    pub chips: [*mut pwm_chip; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc_pwm_ctx {
    pub cnt: u32,
    pub cnt2: u32,
    pub ctrl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc_pwm {
    pub base: *mut void __iomem,
    pub clk_ns: c_uint,
    pub ctx: [dwc_pwm_ctx; DWC_TIMERS_TOTAL],
}

extern "C" {
    pub fn pwmchip_get_drvdata(_arg: chip) -> return;
}
extern "C" {
    pub fn readl(offset: dwc->base +) -> return;
}
