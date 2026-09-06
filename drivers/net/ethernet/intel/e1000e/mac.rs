//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000e/mac.h
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
extern "C" {
    pub fn e1000e_blink_led_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_check_for_copper_link(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_check_for_fiber_link(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_check_for_serdes_link(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_cleanup_led_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_config_fc_after_link_up(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_disable_pcie_master(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_force_mac_fc(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_get_auto_rd_done(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_get_bus_info_pcie(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000_set_lan_id_single_port(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000e_get_hw_semaphore(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_id_led_init_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_led_on_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_led_off_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_set_fc_watermarks(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_setup_fiber_serdes_link(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_setup_led_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_setup_link_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_clear_hw_cntrs_base(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_clear_vfta_generic(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000e_init_rx_addrs(hw: *mut e1000_hw, rar_count: u16);
}
extern "C" {
    pub fn e1000e_put_hw_semaphore(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_check_alt_mac_addr_generic(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn e1000e_reset_adaptive(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000e_set_pcie_no_snoop(hw: *mut e1000_hw, no_snoop: u32);
}
extern "C" {
    pub fn e1000e_update_adaptive(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000_write_vfta_generic(hw: *mut e1000_hw, offset: u32, value: u32);
}
extern "C" {
    pub fn e1000_set_lan_id_multi_port_pcie(hw: *mut e1000_hw);
}
extern "C" {
    pub fn e1000e_rar_get_count_generic(hw: *mut e1000_hw) -> u32;
}
extern "C" {
    pub fn e1000e_rar_set_generic(hw: *mut e1000_hw, addr: *mut u8, index: u32) -> c_int;
}
extern "C" {
    pub fn e1000e_config_collision_dist_generic(hw: *mut e1000_hw);
}
