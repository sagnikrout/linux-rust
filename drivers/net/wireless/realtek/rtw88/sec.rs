//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/sec.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2018-2019  Realtek Corporation
//
pub const RTW_SEC_CMD_REG: c_uint = 0x670;
pub const RTW_SEC_WRITE_REG: c_uint = 0x674;
pub const RTW_SEC_READ_REG: c_uint = 0x678;
pub const RTW_SEC_CONFIG: c_uint = 0x680;
pub const RTW_SEC_CAM_ENTRY_SHIFT: c_int = 3;
pub const RTW_SEC_DEFAULT_KEY_NUM: c_int = 4;

extern "C" {
    pub fn rtw_sec_get_free_cam(sec: *mut rtw_sec_desc) -> c_int;
}
extern "C" {
    pub fn rtw_sec_cam_pg_backup(rtwdev: *mut rtw_dev, used_cam: *mut u8) -> u8;
}
extern "C" {
    pub fn rtw_sec_enable_sec_engine(rtwdev: *mut rtw_dev);
}
