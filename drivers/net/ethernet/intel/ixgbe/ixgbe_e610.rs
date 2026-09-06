//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_e610.h
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
// Copyright(c) 2024 Intel Corporation.

extern "C" {
    pub fn ixgbe_aci_check_event_pending(hw: *mut ixgbe_hw) -> bool;
}
extern "C" {
    pub fn ixgbe_fill_dflt_direct_cmd_desc(desc: *mut libie_aq_desc, opcode: u16);
}
extern "C" {
    pub fn ixgbe_release_res(hw: *mut ixgbe_hw, res: libie_aq_res_id);
}
extern "C" {
    pub fn ixgbe_get_caps(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_aci_disable_rxen(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_aci_set_link_restart_an(hw: *mut ixgbe_hw, ena_link: bool) -> c_int;
}
extern "C" {
    pub fn ixgbe_update_link_info(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_get_link_status(hw: *mut ixgbe_hw, link_up: *mut bool) -> c_int;
}
extern "C" {
    pub fn ixgbe_aci_set_event_mask(hw: *mut ixgbe_hw, port_num: u8, mask: u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_configure_lse(hw: *mut ixgbe_hw, activate: bool, mask: u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_aci_set_port_id_led(hw: *mut ixgbe_hw, orig_mode: bool) -> c_int;
}
extern "C" {
    pub fn ixgbe_get_media_type_e610(hw: *mut ixgbe_hw) -> ixgbe_media_type;
}
extern "C" {
    pub fn ixgbe_setup_fc_e610(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_fc_autoneg_e610(hw: *mut ixgbe_hw);
}
extern "C" {
    pub fn ixgbe_disable_rx_e610(hw: *mut ixgbe_hw);
}
extern "C" {
    pub fn ixgbe_init_phy_ops_e610(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_identify_phy_e610(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_identify_module_e610(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_setup_phy_link_e610(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_setup_eee_e610(hw: *mut ixgbe_hw, enable_eee: bool) -> c_int;
}
extern "C" {
    pub fn ixgbe_set_phy_power_e610(hw: *mut ixgbe_hw, on: bool) -> c_int;
}
extern "C" {
    pub fn ixgbe_enter_lplu_e610(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_init_eeprom_params_e610(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_release_nvm(hw: *mut ixgbe_hw);
}
extern "C" {
    pub fn ixgbe_nvm_validate_checksum(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_get_inactive_nvm_ver(hw: *mut ixgbe_hw, nvm: *mut ixgbe_nvm_info) -> c_int;
}
extern "C" {
    pub fn ixgbe_read_sr_word_aci(hw: *mut ixgbe_hw, offset: u16, data: *mut u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_read_ee_aci_e610(hw: *mut ixgbe_hw, offset: u16, data: *mut u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_validate_eeprom_checksum_e610(hw: *mut ixgbe_hw, checksum_val: *mut u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_reset_hw_e610(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_get_flash_data(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_aci_nvm_update_empr(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_aci_erase_nvm(hw: *mut ixgbe_hw, module_typeid: u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_fwlog_init(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_fwlog_deinit(hw: *mut ixgbe_hw);
}
