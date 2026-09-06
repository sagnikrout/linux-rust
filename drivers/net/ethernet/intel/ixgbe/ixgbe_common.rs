//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_common.h
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
    pub fn ixgbe_get_pcie_msix_count_generic(hw: *mut ixgbe_hw) -> u16;
}
extern "C" {
    pub fn ixgbe_init_hw_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_start_hw_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_start_hw_gen2(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_clear_hw_cntrs_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_get_mac_addr_generic(hw: *mut ixgbe_hw, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn ixgbe_convert_bus_width(link_status: u16) -> ixgbe_bus_width;
}
extern "C" {
    pub fn ixgbe_convert_bus_speed(link_status: u16) -> ixgbe_bus_speed;
}
extern "C" {
    pub fn ixgbe_get_bus_info_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_set_lan_id_multi_port_pcie(hw: *mut ixgbe_hw);
}
extern "C" {
    pub fn ixgbe_stop_adapter_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_led_on_generic(hw: *mut ixgbe_hw, index: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_led_off_generic(hw: *mut ixgbe_hw, index: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_init_led_link_act_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_init_eeprom_params_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_write_eeprom_generic(hw: *mut ixgbe_hw, offset: u16, data: u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_read_eerd_generic(hw: *mut ixgbe_hw, offset: u16, data: *mut u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_write_eewr_generic(hw: *mut ixgbe_hw, offset: u16, data: u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_calc_eeprom_checksum_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_update_eeprom_checksum_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_clear_rar_generic(hw: *mut ixgbe_hw, index: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_init_rx_addrs_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_enable_mc_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_disable_mc_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_disable_rx_buff_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_enable_rx_buff_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_enable_rx_dma_generic(hw: *mut ixgbe_hw, regval: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_fc_enable_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_setup_fc_generic(: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_device_supports_autoneg_fc(hw: *mut ixgbe_hw) -> bool;
}
extern "C" {
    pub fn ixgbe_fc_autoneg(hw: *mut ixgbe_hw);
}
extern "C" {
    pub fn ixgbe_acquire_swfw_sync(hw: *mut ixgbe_hw, mask: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_release_swfw_sync(hw: *mut ixgbe_hw, mask: u32);
}
extern "C" {
    pub fn ixgbe_get_san_mac_addr_generic(hw: *mut ixgbe_hw, san_mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn ixgbe_set_vmdq_generic(hw: *mut ixgbe_hw, rar: u32, vmdq: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_set_vmdq_san_mac_generic(hw: *mut ixgbe_hw, vmdq: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_clear_vmdq_generic(hw: *mut ixgbe_hw, rar: u32, vmdq: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_init_uta_tables_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_clear_vfta_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn prot_autoc_read_generic(hw: *mut ixgbe_hw, : *mut bool, reg_val: *mut u32) -> c_int;
}
extern "C" {
    pub fn prot_autoc_write_generic(hw: *mut ixgbe_hw, reg_val: u32, locked: bool) -> c_int;
}
extern "C" {
    pub fn ixgbe_blink_led_start_generic(hw: *mut ixgbe_hw, index: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_blink_led_stop_generic(hw: *mut ixgbe_hw, index: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_set_mac_anti_spoofing(hw: *mut ixgbe_hw, enable: bool, vf: c_int);
}
extern "C" {
    pub fn ixgbe_set_vlan_anti_spoofing(hw: *mut ixgbe_hw, enable: bool, vf: c_int);
}
extern "C" {
    pub fn ixgbe_get_device_caps_generic(hw: *mut ixgbe_hw, device_caps: *mut u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_calculate_checksum(buffer: *mut u8, length: u32) -> u8;
}
extern "C" {
    pub fn ixgbe_hic_unlocked(hw: *mut ixgbe_hw, buffer: *mut u32, len: u32, timeout: u32) -> c_int;
}
extern "C" {
    pub fn ixgbe_clear_tx_pending(hw: *mut ixgbe_hw);
}
extern "C" {
    pub fn ixgbe_mng_present(hw: *mut ixgbe_hw) -> bool;
}
extern "C" {
    pub fn ixgbe_mng_enabled(hw: *mut ixgbe_hw) -> bool;
}
pub const IXGBE_I2C_THERMAL_SENSOR_ADDR: c_uint = 0xF8;
pub const IXGBE_EMC_INTERNAL_DATA: c_uint = 0x00;
pub const IXGBE_EMC_INTERNAL_THERM_LIMIT: c_uint = 0x20;
pub const IXGBE_EMC_DIODE1_DATA: c_uint = 0x01;
pub const IXGBE_EMC_DIODE1_THERM_LIMIT: c_uint = 0x19;
pub const IXGBE_EMC_DIODE2_DATA: c_uint = 0x23;
pub const IXGBE_EMC_DIODE2_THERM_LIMIT: c_uint = 0x1A;
pub const IXGBE_EMC_DIODE3_DATA: c_uint = 0x2A;
pub const IXGBE_EMC_DIODE3_THERM_LIMIT: c_uint = 0x30;
extern "C" {
    pub fn ixgbe_get_thermal_sensor_data_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_init_thermal_sensor_thresh_generic(hw: *mut ixgbe_hw) -> c_int;
}
extern "C" {
    pub fn ixgbe_disable_rx_generic(hw: *mut ixgbe_hw);
}
extern "C" {
    pub fn ixgbe_enable_rx_generic(hw: *mut ixgbe_hw);
}
pub const IXGBE_FAILED_READ_RETRIES: c_int = 5;
pub const IXGBE_FAILED_READ_REG: c_uint = 0xffffffffU;
pub const IXGBE_FAILED_READ_CFG_DWORD: c_uint = 0xffffffffU;
pub const IXGBE_FAILED_READ_CFG_WORD: c_uint = 0xffffU;
extern "C" {
    pub fn ixgbe_read_pci_cfg_word(hw: *mut ixgbe_hw, reg: u32) -> u16;
}
extern "C" {
    pub fn ixgbe_write_pci_cfg_word(hw: *mut ixgbe_hw, reg: u32, value: u16);
}
extern "C" {
    pub fn unlikely(_arg: !addr) -> return;
}

extern "C" {
    pub fn ixgbe_read_reg(hw: *mut ixgbe_hw, reg: u32) -> u32;
}

