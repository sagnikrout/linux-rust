//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/samsung/phy-samsung-usb2.h
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
// Samsung SoC USB 1.1/2.0 PHY driver
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// Author: Kamil Debski <k.debski@samsung.com>
//

pub const KHZ: c_int = 1000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_usb2_phy_instance {
    pub cfg: *const samsung_usb2_common_phy,
    pub phy: *mut phy,
    pub drv: *mut samsung_usb2_phy_driver,
    pub int_cnt: c_int,
    pub ext_cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_usb2_phy_driver {
    pub cfg: *const samsung_usb2_phy_config,
    pub clk: *mut clk,
    pub ref_clk: *mut clk,
    pub vbus: *mut regulator,
    pub ref_rate: c_ulong,
    pub ref_reg_val: u32,
    pub dev: *mut device,
    pub reg_phy: *mut void __iomem,
    pub reg_pmu: *mut regmap,
    pub reg_sys: *mut regmap,
    pub lock: spinlock_t,
    pub instances: [samsung_usb2_phy_instance; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_usb2_common_phy {
    pub ): *mut *mut int (power_on)(struct samsung_usb2_phy_instance,
    pub ): *mut *mut int (power_off)(struct samsung_usb2_phy_instance,
    pub id: c_uint,
    pub label: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_usb2_phy_config {
    pub phys: *const samsung_usb2_common_phy,
    pub ): *mut *mut int (rate_to_clk)(unsigned long, u32,
    pub num_phys: c_uint,
    pub has_mode_switch: bool,
    pub has_refclk_sel: bool,
}
