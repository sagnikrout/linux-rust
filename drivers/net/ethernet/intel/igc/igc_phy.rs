//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igc/igc_phy.h
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
// Copyright (c)  2018 Intel Corporation

extern "C" {
    pub fn igc_check_reset_block(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_phy_hw_reset(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_get_phy_id(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_check_downshift(hw: *mut igc_hw);
}
extern "C" {
    pub fn igc_setup_copper_link(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_power_up_phy_copper(hw: *mut igc_hw);
}
extern "C" {
    pub fn igc_power_down_phy_copper(hw: *mut igc_hw);
}
extern "C" {
    pub fn igc_write_phy_reg_gpy(hw: *mut igc_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn igc_read_phy_reg_gpy(hw: *mut igc_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igc_read_phy_fw_version(hw: *mut igc_hw) -> u16;
}
extern "C" {
    pub fn igc_force_speed_duplex(hw: *mut igc_hw) -> i32;
}
