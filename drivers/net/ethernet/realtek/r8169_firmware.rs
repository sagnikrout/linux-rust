//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/realtek/r8169_firmware.h
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
// r8169_firmware.h: RealTek 8169/8168/8101 ethernet driver.
//
// Copyright (c) 2002 ShuChen <shuchen@realtek.com.tw>
// Copyright (c) 2003 - 2007 Francois Romieu <romieu@fr.zoreil.com>
// Copyright (c) a lot of people too. Please respect their work.
//
// See MAINTAINERS file for support contact information.
//

extern "C" {
    pub fn void(tp: *mut *mut rtl_fw_write_t)(struct rtl8169_private, reg: c_int, val: c_int) -> typedef;
}
extern "C" {
    pub fn int(tp: *mut *mut rtl_fw_read_t)(struct rtl8169_private, reg: c_int) -> typedef;
}
pub const RTL_VER_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_fw {
    pub phy_write: rtl_fw_write_t,
    pub phy_read: rtl_fw_read_t,
    pub mac_mcu_write: rtl_fw_write_t,
    pub mac_mcu_read: rtl_fw_read_t,
    pub fw: *const firmware,
    pub fw_name: *const c_char,
    pub dev: *mut device,
    pub version: [c_char; RTL_VER_SIZE],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_fw_phy_action {
    pub code: *mut __le32,
    pub size: usize,
    pub phy_action: },
}

extern "C" {
    pub fn rtl_fw_request_firmware(rtl_fw: *mut rtl_fw) -> c_int;
}
extern "C" {
    pub fn rtl_fw_release_firmware(rtl_fw: *mut rtl_fw);
}
extern "C" {
    pub fn rtl_fw_write_firmware(tp: *mut rtl8169_private, rtl_fw: *mut rtl_fw);
}
