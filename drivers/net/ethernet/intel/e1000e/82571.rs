//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000e/82571.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.
pub const ID_LED_RESERVED_F746: c_uint = 0xF746;

pub const E1000_GCR_L1_ACT_WITHOUT_L0S_RX: c_uint = 0x08000000;

// Intr Throttling - RW

pub const E1000_EIAC_82574: c_uint = 0x000DC	/* Ext. Interrupt Auto Clear - RW */;
pub const E1000_EIAC_MASK_82574: c_uint = 0x01F00000;
pub const E1000_IVAR_INT_ALLOC_VALID: c_uint = 0x8;
// Manageability Operation Mode mask
pub const E1000_NVM_INIT_CTRL2_MNGM: c_uint = 0x6000;
pub const E1000_BASE1000T_STATUS: c_int = 10;
pub const E1000_IDLE_ERROR_COUNT_MASK: c_uint = 0xFF;
pub const E1000_RECEIVE_ERROR_COUNTER: c_int = 21;
pub const E1000_RECEIVE_ERROR_MAX: c_uint = 0xFFFF;
extern "C" {
    pub fn e1000_check_phy_82574(hw: *mut e1000_hw) -> bool;
}
extern "C" {
    pub fn e1000e_get_laa_state_82571(hw: *mut e1000_hw) -> bool;
}
extern "C" {
    pub fn e1000e_set_laa_state_82571(hw: *mut e1000_hw, state: bool);
}
