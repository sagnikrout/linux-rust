//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/debug.h
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
// Copyright(c) 2009-2012  Realtek Corporation.
// --------------------------------------------------------------
//
// Fatal bug.
// For example, Tx/Rx/IO locked up,
// memory access violation,
// resource allocation failed,
// unexpected HW behavior, HW BUG
// and so on.
//
// #define DBG_EMERG			0
//
// Abnormal, rare, or unexpeted cases.
// For example, Packet/IO Ctl canceled,
// device suprisely unremoved and so on.
//
pub const DBG_WARNING: c_int = 2;
//
// Normal case driver developer should
// open, we can see link status like
// assoc/AddBA/DHCP/adapter start and
// so on basic and useful infromations.
//
pub const DBG_DMESG: c_int = 3;
//
// Normal case with useful information
// about current SW or HW state.
// For example, Tx/Rx descriptor to fill,
// Tx/Rx descriptor completed status,
// SW protocol state change, dynamic
// mechanism state change and so on.
//
pub const DBG_LOUD: c_int = 4;
//
// Normal case with detail execution
// flow or information.
//
pub const DBG_TRACE: c_int = 5;
// --------------------------------------------------------------

// --------------------------------------------------------------
// Define EEPROM and EFUSE  check module bit

// Define init check for module bit

// Define PHY-BB/RF/MAC check module bit

// Define Dynamic Mechanism check module bit --> FDM

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbgp_flag_e {
    FQOS = 0,
    FTX = 1,
    FRX = 2,
    FSEC = 3,
    FMGNT = 4,
    FMLME = 5,
    FRESOURCE = 6,
    FBEACON = 7,
    FISR = 8,
    FPHY = 9,
    FMP = 10,
    FEEPROM = 11,
    FPWR = 12,
    FDM = 13,
    FDBGCTRL = 14,
    FC2H = 15,
    FBT = 16,
    FINIT = 17,
    FIOCTL = 18,
    DBGP_TYPE_MAX
}

extern "C" {
    pub fn rtl_debug_add_one(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_debug_remove_one(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_debugfs_add_topdir();
}
extern "C" {
    pub fn rtl_debugfs_remove_topdir();
}

// Macro flag: #define rtl_debug_add_one(hw)
// Macro flag: #define rtl_debug_remove_one(hw)
// Macro flag: #define rtl_debugfs_add_topdir()
// Macro flag: #define rtl_debugfs_remove_topdir()

