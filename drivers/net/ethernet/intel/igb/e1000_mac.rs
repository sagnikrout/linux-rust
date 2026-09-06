//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igb/e1000_mac.h
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
// Copyright(c) 2007 - 2018 Intel Corporation.

// Functions that should not be called directly from drivers but can be used
// by other files in this 'shared code'
//
extern "C" {
    pub fn igb_blink_led(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_check_for_copper_link(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_cleanup_led(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_config_fc_after_link_up(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_disable_pcie_master(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_force_mac_fc(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_auto_rd_done(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_bus_info_pcie(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_hw_semaphore(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_id_led_init(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_led_off(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_setup_link(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_validate_mdi_setting(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_clear_hw_cntrs_base(hw: *mut e1000_hw);
}
extern "C" {
    pub fn igb_clear_vfta(hw: *mut e1000_hw);
}
extern "C" {
    pub fn igb_write_vfta(hw: *mut e1000_hw, offset: u32, value: u32);
}
extern "C" {
    pub fn igb_config_collision_dist(hw: *mut e1000_hw);
}
extern "C" {
    pub fn igb_init_rx_addrs(hw: *mut e1000_hw, rar_count: u16);
}
extern "C" {
    pub fn igb_mta_set(hw: *mut e1000_hw, hash_value: u32);
}
extern "C" {
    pub fn igb_put_hw_semaphore(hw: *mut e1000_hw);
}
extern "C" {
    pub fn igb_rar_set(hw: *mut e1000_hw, addr: *mut u8, index: u32);
}
extern "C" {
    pub fn igb_check_alt_mac_addr(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_enable_mng_pass_thru(hw: *mut e1000_hw) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_mng_mode {
    e1000_mng_mode_none = 0,
    e1000_mng_mode_asf,
    e1000_mng_mode_pt,
    e1000_mng_mode_ipmi,
    e1000_mng_mode_host_if_only
}

pub const E1000_FACTPS_MNGCG: c_uint = 0x20000000;
pub const E1000_FWSM_MODE_MASK: c_uint = 0xE;
pub const E1000_FWSM_MODE_SHIFT: c_int = 1;
pub const E1000_MNG_DHCP_COOKIE_STATUS_VLAN: c_uint = 0x2;
