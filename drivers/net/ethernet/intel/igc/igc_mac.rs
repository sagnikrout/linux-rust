//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igc/igc_mac.h
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

// forward declaration
extern "C" {
    pub fn igc_disable_pcie_master(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_check_for_copper_link(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_config_fc_after_link_up(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_force_mac_fc(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_init_rx_addrs(hw: *mut igc_hw, rar_count: u16);
}
extern "C" {
    pub fn igc_setup_link(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_clear_hw_cntrs_base(hw: *mut igc_hw);
}
extern "C" {
    pub fn igc_get_auto_rd_done(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_put_hw_semaphore(hw: *mut igc_hw);
}
extern "C" {
    pub fn igc_rar_set(hw: *mut igc_hw, addr: *mut u8, index: u32);
}
extern "C" {
    pub fn igc_config_collision_dist(hw: *mut igc_hw);
}
extern "C" {
    pub fn igc_enable_mng_pass_thru(hw: *mut igc_hw) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igc_mng_mode {
    igc_mng_mode_none = 0,
    igc_mng_mode_asf,
    igc_mng_mode_pt,
    igc_mng_mode_ipmi,
    igc_mng_mode_host_if_only
}
