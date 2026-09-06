//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/debug.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_debug_mask {
    RTW_DBG_PCI		= 0x00000001,
    RTW_DBG_TX		= 0x00000002,
    RTW_DBG_RX		= 0x00000004,
    RTW_DBG_PHY		= 0x00000008,
    RTW_DBG_FW		= 0x00000010,
    RTW_DBG_EFUSE		= 0x00000020,
    RTW_DBG_COEX		= 0x00000040,
    RTW_DBG_RFK		= 0x00000080,
    RTW_DBG_REGD		= 0x00000100,
    RTW_DBG_DEBUGFS		= 0x00000200,
    RTW_DBG_PS		= 0x00000400,
    RTW_DBG_BF		= 0x00000800,
    RTW_DBG_WOW		= 0x00001000,
    RTW_DBG_CFO		= 0x00002000,
    RTW_DBG_PATH_DIV	= 0x00004000,
    RTW_DBG_ADAPTIVITY	= 0x00008000,
    RTW_DBG_HW_SCAN		= 0x00010000,
    RTW_DBG_STATE		= 0x00020000,
    RTW_DBG_SDIO		= 0x00040000,
    RTW_DBG_USB		= 0x00080000,

    RTW_DBG_UNEXP		= 0x80000000,
    RTW_DBG_ALL		= 0xffffffff
}

extern "C" {
    pub fn rtw_debugfs_init(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_debugfs_deinit(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_debugfs_get_simple_phy_info(m: *mut seq_file);
}

