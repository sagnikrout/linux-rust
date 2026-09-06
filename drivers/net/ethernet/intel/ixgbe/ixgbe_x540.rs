//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_x540.h
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
// Copyright(c) 1999 - 2024 Intel Corporation.

extern "C" {
    pub fn ixgbe_get_invariants_X540(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_reset_hw_X540(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_start_hw_X540(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_get_media_type_X540(hw: *mut ixgbe_hw) -> ixgbe_media_type;
}
extern "C" {
    pub fn ixgbe_blink_led_start_X540(hw: *mut ixgbe_hw, index: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_blink_led_stop_X540(hw: *mut ixgbe_hw, index: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_acquire_swfw_sync_X540(hw: *mut ixgbe_hw, mask: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_release_swfw_sync_X540(hw: *mut ixgbe_hw, mask: u32);
}
extern "C" {
    pub fn ixgbe_init_swfw_sync_X540(hw: *mut ixgbe_hw);
}
extern "C" {
    pub fn ixgbe_init_eeprom_params_X540(hw: *mut ixgbe_hw) -> c_int;
}
