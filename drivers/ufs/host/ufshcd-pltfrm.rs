//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/host/ufshcd-pltfrm.h
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
// Copyright (c) 2015, The Linux Foundation. All rights reserved.
//

pub const UFS_PWM_MODE: c_int = 1;
pub const UFS_HS_MODE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_host_params {
    pub /: *mut *mut u32 pwm_rx_gear; / pwm rx gear to work in,
    pub /: *mut *mut u32 pwm_tx_gear; / pwm tx gear to work in,
    pub /: *mut *mut u32 hs_rx_gear; / hs rx gear to work in,
    pub /: *mut *mut u32 hs_tx_gear; / hs tx gear to work in,
    pub /: *mut *mut u32 rx_lanes; / number of rx lanes,
    pub /: *mut *mut u32 tx_lanes; / number of tx lanes,
    pub /: *mut *mut u32 rx_pwr_pwm; / rx pwm working pwr,
    pub /: *mut *mut u32 tx_pwr_pwm; / tx pwm working pwr,
    pub /: *mut *mut u32 rx_pwr_hs; / rx hs working pwr,
    pub /: *mut *mut u32 tx_pwr_hs; / tx hs working pwr,
    pub /: *mut *mut u32 hs_rate; / rate A/B to work in HS,
    pub desired_working_mode: u32,
}

extern "C" {
    pub fn ufshcd_init_host_params(host_params: *mut ufs_host_params);
}
extern "C" {
    pub fn ufshcd_parse_gear_limits(hba: *mut ufs_hba, host_params: *mut ufs_host_params);
}
extern "C" {
    pub fn ufshcd_pltfrm_remove(pdev: *mut platform_device);
}
