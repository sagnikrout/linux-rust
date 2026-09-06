//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/pxa_sdhci.h
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
// include/linux/platform_data/pxa_sdhci.h
//
// Copyright 2010 Marvell
// Zhangfei Gao <zhangfei.gao@marvell.com>
//
// PXA Platform - SDHCI platform data definitions
//
// pxa specific flag
// Require clock free running

// card always wired to host, like on-chip emmc

// Board design supports 8-bit data on SD/SDIO BUS

//
// struct pxa_sdhci_platdata() - Platform device data for PXA SDHCI
// @flags: flags for platform requirement
// @clk_delay_cycles:
// mmp2: each step is roughly 100ps, 5bits width
// pxa910: each step is 1ns, 4bits width
// @clk_delay_sel: select clk_delay, used on pxa910
// 0: choose feedback clk
// 1: choose feedback clk + delay value
// 2: choose internal clk
// @clk_delay_enable: enable clk_delay or not, used on pxa910
// @max_speed: the maximum speed supported
// @host_caps: Standard MMC host capabilities bit field.
// @quirks: quirks of platfrom
// @quirks2: quirks2 of platfrom
// @pm_caps: pm_caps of platfrom
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_pxa_platdata {
    pub flags: c_uint,
    pub clk_delay_cycles: c_uint,
    pub clk_delay_sel: c_uint,
    pub clk_delay_enable: bool,
    pub max_speed: c_uint,
    pub host_caps: u32,
    pub host_caps2: u32,
    pub quirks: c_uint,
    pub quirks2: c_uint,
    pub pm_caps: c_uint,
}
