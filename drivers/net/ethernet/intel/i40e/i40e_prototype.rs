//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_prototype.h
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
// Copyright(c) 2013 - 2021 Intel Corporation.

// Prototypes for shared code functions that are not in
// the standard function pointer structures.  These are
// mostly because they are needed even before the init
// has happened and will assist in the early SW and FW
// setup.
//
// adminq functions
extern "C" {
    pub fn i40e_init_adminq(hw: *mut i40e_hw) -> c_int;
}
extern "C" {
    pub fn i40e_shutdown_adminq(hw: *mut i40e_hw);
}
// debug function for adminq
extern "C" {
    pub fn i40e_check_asq_alive(hw: *mut i40e_hw) -> bool;
}
extern "C" {
    pub fn i40e_aq_queue_shutdown(hw: *mut i40e_hw, unloading: bool) -> c_int;
}
extern "C" {
    pub fn i40e_led_get(hw: *mut i40e_hw) -> u32;
}
extern "C" {
    pub fn i40e_led_set(hw: *mut i40e_hw, mode: u32, blink: bool);
}
// admin send queue commands
// cmd_details);
// i40e_common
extern "C" {
    pub fn i40e_init_shared_code(hw: *mut i40e_hw) -> c_int;
}
extern "C" {
    pub fn i40e_pf_reset(hw: *mut i40e_hw) -> c_int;
}
extern "C" {
    pub fn i40e_clear_hw(hw: *mut i40e_hw);
}
extern "C" {
    pub fn i40e_clear_pxe_mode(hw: *mut i40e_hw);
}
extern "C" {
    pub fn i40e_get_link_status(hw: *mut i40e_hw, link_up: *mut bool) -> c_int;
}
extern "C" {
    pub fn i40e_update_link_info(hw: *mut i40e_hw) -> c_int;
}
extern "C" {
    pub fn i40e_get_mac_addr(hw: *mut i40e_hw, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn i40e_get_port_mac_addr(hw: *mut i40e_hw, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn i40e_get_pba_string(hw: *mut i40e_hw);
}
extern "C" {
    pub fn i40e_pre_tx_queue_cfg(hw: *mut i40e_hw, queue: u32, enable: bool);
}
// prototype for functions used for NVM access
extern "C" {
    pub fn i40e_init_nvm(hw: *mut i40e_hw) -> c_int;
}
extern "C" {
    pub fn i40e_release_nvm(hw: *mut i40e_hw);
}
extern "C" {
    pub fn i40e_update_nvm_checksum(hw: *mut i40e_hw) -> c_int;
}
extern "C" {
    pub fn i40e_nvmupd_clear_wait_state(hw: *mut i40e_hw);
}
extern "C" {
    pub fn i40e_set_pci_config_data(hw: *mut i40e_hw, link_status: u16);
}
extern "C" {
    pub fn i40e_set_mac_type(hw: *mut i40e_hw) -> c_int;
}
//
// i40e_virtchnl_link_speed - Convert AdminQ link_speed to virtchnl definition
// @link_speed: the speed to convert
//
// Returns the link_speed in terms of the virtchnl interface, for use in
// converting link_speed as reported by the AdminQ into the format used for
// talking to virtchnl devices. If we can't represent the link speed properly,
// report LINK_SPEED_UNKNOWN.
//
// prototype for functions used for SW locks
// i40e_common for VF drivers
extern "C" {
    pub fn i40e_read_rx_ctl(hw: *mut i40e_hw, reg_addr: u32) -> u32;
}
extern "C" {
    pub fn i40e_write_rx_ctl(hw: *mut i40e_hw, reg_addr: u32, reg_val: u32);
}
// Convenience wrappers for most common use case

extern "C" {
    pub fn i40e_get_phy_address(hw: *mut i40e_hw, dev_num: u8) -> u8;
}
// i40e_ddp
extern "C" {
    pub fn i40e_ddp_flash(netdev: *mut net_device, flash: *mut ethtool_flash) -> c_int;
}
// Firmware and AdminQ version check helpers
//
// i40e_is_aq_api_ver_ge
// @hw: pointer to i40e_hw structure
// @maj: API major value to compare
// @min: API minor value to compare
//
// Assert whether current HW API version is greater/equal than provided.
//
// i40e_is_aq_api_ver_lt
// @hw: pointer to i40e_hw structure
// @maj: API major value to compare
// @min: API minor value to compare
//
// Assert whether current HW API version is less than provided.
//
// i40e_is_fw_ver_ge
// @hw: pointer to i40e_hw structure
// @maj: API major value to compare
// @min: API minor value to compare
//
// Assert whether current firmware version is greater/equal than provided.
//
// i40e_is_fw_ver_lt
// @hw: pointer to i40e_hw structure
// @maj: API major value to compare
// @min: API minor value to compare
//
// Assert whether current firmware version is less than provided.
//
// i40e_is_fw_ver_eq
// @hw: pointer to i40e_hw structure
// @maj: API major value to compare
// @min: API minor value to compare
//
// Assert whether current firmware version is equal to provided.
//
