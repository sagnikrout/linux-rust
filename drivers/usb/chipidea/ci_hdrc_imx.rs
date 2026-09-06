//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/chipidea/ci_hdrc_imx.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2012 Freescale Semiconductor, Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_usbmisc_data {
    pub dev: *mut device,
    pub index: c_int,
    pub /: *mut *mut unsigned int disable_oc:1; / over current detect disabled,
// true if over-current polarity is active low
    pub oc_pol_active_low:1: c_uint,
// true if dt specifies polarity
    pub oc_pol_configured:1: c_uint,
    pub /: *mut *mut unsigned int pwr_pol:1; / power polarity,
    pub /: *mut *mut unsigned int evdo:1; / set external vbus divider option,
    pub /: *mut *mut unsigned int ulpi:1; / connected to an ULPI phy,
    pub /: *mut *mut unsigned int hsic:1; / HSIC controller,
    pub /: *mut *mut unsigned int ext_id:1; / ID from exteranl event,
    pub /: *mut *mut unsigned int ext_vbus:1; / Vbus from exteranl event,
    pub usb_phy: *mut usb_phy,
    pub /: *mut *mut usb_dr_mode available_role; / runtime usb dr mode,
    pub emp_curr_control: c_int,
    pub dc_vol_level_adjust: c_int,
    pub rise_fall_time_adjust: c_int,
}

extern "C" {
    pub fn imx_usbmisc_init(data: *mut imx_usbmisc_data) -> c_int;
}
extern "C" {
    pub fn imx_usbmisc_init_post(data: *mut imx_usbmisc_data) -> c_int;
}
extern "C" {
    pub fn imx_usbmisc_hsic_set_connect(data: *mut imx_usbmisc_data) -> c_int;
}
extern "C" {
    pub fn imx_usbmisc_charger_detection(data: *mut imx_usbmisc_data, connect: bool) -> c_int;
}
extern "C" {
    pub fn imx_usbmisc_suspend(data: *mut imx_usbmisc_data, wakeup: bool) -> c_int;
}
extern "C" {
    pub fn imx_usbmisc_resume(data: *mut imx_usbmisc_data, wakeup: bool) -> c_int;
}
extern "C" {
    pub fn imx_usbmisc_pullup(data: *mut imx_usbmisc_data, on: bool) -> c_int;
}
