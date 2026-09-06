//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igb/e1000_phy.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_ms_type {
    e1000_ms_hw_default = 0,
    e1000_ms_force_master,
    e1000_ms_force_slave,
    e1000_ms_auto
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_smart_speed {
    e1000_smart_speed_default = 0,
    e1000_smart_speed_on,
    e1000_smart_speed_off
}

extern "C" {
    pub fn igb_check_downshift(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_check_reset_block(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_copper_link_setup_igp(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_copper_link_setup_m88(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_copper_link_setup_m88_gen2(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_phy_force_speed_duplex_igp(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_phy_force_speed_duplex_m88(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_cable_length_m88(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_cable_length_m88_gen2(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_cable_length_igp_2(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_phy_id(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_phy_info_igp(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_phy_info_m88(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_phy_sw_reset(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_phy_hw_reset(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_read_phy_reg_igp(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_set_d3_lplu_state(hw: *mut e1000_hw, active: bool) -> i32;
}
extern "C" {
    pub fn igb_setup_copper_link(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_write_phy_reg_igp(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn igb_power_up_phy_copper(hw: *mut e1000_hw);
}
extern "C" {
    pub fn igb_power_down_phy_copper(hw: *mut e1000_hw);
}
extern "C" {
    pub fn igb_phy_init_script_igp3(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_initialize_M88E1512_phy(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_initialize_M88E1543_phy(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_read_phy_reg_mdic(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_write_phy_reg_mdic(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn igb_read_phy_reg_i2c(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_write_phy_reg_i2c(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn igb_read_sfp_data_byte(hw: *mut e1000_hw, offset: u16, data: *mut u8) -> i32;
}
extern "C" {
    pub fn igb_copper_link_setup_82580(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_phy_info_82580(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_phy_force_speed_duplex_82580(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_get_cable_length_82580(hw: *mut e1000_hw) -> i32;
}
extern "C" {
    pub fn igb_read_phy_reg_82580(hw: *mut e1000_hw, offset: u32, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igb_write_phy_reg_82580(hw: *mut e1000_hw, offset: u32, data: u16) -> i32;
}
extern "C" {
    pub fn igb_check_polarity_m88(hw: *mut e1000_hw) -> i32;
}
// IGP01E1000 Specific Registers
pub const IGP01E1000_PHY_PORT_CONFIG: c_uint = 0x10 /* Port Config */;
pub const IGP01E1000_PHY_PORT_STATUS: c_uint = 0x11 /* Status */;
pub const IGP01E1000_PHY_PORT_CTRL: c_uint = 0x12 /* Control */;
pub const IGP01E1000_PHY_LINK_HEALTH: c_uint = 0x13 /* PHY Link Health */;
pub const IGP02E1000_PHY_POWER_MGMT: c_uint = 0x19 /* Power Management */;
pub const IGP01E1000_PHY_PAGE_SELECT: c_uint = 0x1F /* Page Select */;
pub const IGP01E1000_PHY_PCS_INIT_REG: c_uint = 0x00B4;
pub const IGP01E1000_PHY_POLARITY_MASK: c_uint = 0x0078;
pub const IGP01E1000_PSCR_AUTO_MDIX: c_uint = 0x1000;
pub const IGP01E1000_PSCR_FORCE_MDI_MDIX: c_uint = 0x2000 /* 0=MDI, 1=MDIX */;
pub const IGP01E1000_PSCFR_SMART_SPEED: c_uint = 0x0080;
pub const I82580_ADDR_REG: c_int = 16;
pub const I82580_CFG_REG: c_int = 22;

pub const I82580_CTRL_REG: c_int = 23;

// 82580 specific PHY registers
pub const I82580_PHY_CTRL_2: c_int = 18;
pub const I82580_PHY_LBK_CTRL: c_int = 19;
pub const I82580_PHY_STATUS_2: c_int = 26;
pub const I82580_PHY_DIAG_STATUS: c_int = 31;
// I82580 PHY Status 2
pub const I82580_PHY_STATUS2_REV_POLARITY: c_uint = 0x0400;
pub const I82580_PHY_STATUS2_MDIX: c_uint = 0x0800;
pub const I82580_PHY_STATUS2_SPEED_MASK: c_uint = 0x0300;
pub const I82580_PHY_STATUS2_SPEED_1000MBPS: c_uint = 0x0200;
pub const I82580_PHY_STATUS2_SPEED_100MBPS: c_uint = 0x0100;
// I82580 PHY Control 2
pub const I82580_PHY_CTRL2_MANUAL_MDIX: c_uint = 0x0200;
pub const I82580_PHY_CTRL2_AUTO_MDI_MDIX: c_uint = 0x0400;
pub const I82580_PHY_CTRL2_MDIX_CFG_MASK: c_uint = 0x0600;
// I82580 PHY Diagnostics Status
pub const I82580_DSTATUS_CABLE_LENGTH: c_uint = 0x03FC;
pub const I82580_DSTATUS_CABLE_LENGTH_SHIFT: c_int = 2;
// 82580 PHY Power Management
pub const E1000_82580_PHY_POWER_MGMT: c_uint = 0xE14;
pub const E1000_82580_PM_SPD: c_uint = 0x0001 /* Smart Power Down */;
pub const E1000_82580_PM_D0_LPLU: c_uint = 0x0002 /* For D0a states */;
pub const E1000_82580_PM_D3_LPLU: c_uint = 0x0004 /* For all other states */;
pub const E1000_82580_PM_GO_LINKD: c_uint = 0x0020 /* Go Link Disconnect */;
// Enable flexible speed on link-up
pub const IGP02E1000_PM_D0_LPLU: c_uint = 0x0002 /* For D0a states */;
pub const IGP02E1000_PM_D3_LPLU: c_uint = 0x0004 /* For all other states */;
pub const IGP01E1000_PLHR_SS_DOWNGRADE: c_uint = 0x8000;
pub const IGP01E1000_PSSR_POLARITY_REVERSED: c_uint = 0x0002;
pub const IGP01E1000_PSSR_MDIX: c_uint = 0x0800;
pub const IGP01E1000_PSSR_SPEED_MASK: c_uint = 0xC000;
pub const IGP01E1000_PSSR_SPEED_1000MBPS: c_uint = 0xC000;
pub const IGP02E1000_PHY_CHANNEL_NUM: c_int = 4;
pub const IGP02E1000_PHY_AGC_A: c_uint = 0x11B1;
pub const IGP02E1000_PHY_AGC_B: c_uint = 0x12B1;
pub const IGP02E1000_PHY_AGC_C: c_uint = 0x14B1;
pub const IGP02E1000_PHY_AGC_D: c_uint = 0x18B1;

pub const IGP02E1000_AGC_LENGTH_MASK: c_uint = 0x7F;
pub const IGP02E1000_AGC_RANGE: c_int = 15;
pub const E1000_CABLE_LENGTH_UNDEFINED: c_uint = 0xFF;
// SFP modules ID memory locations
pub const E1000_SFF_IDENTIFIER_OFFSET: c_uint = 0x00;
pub const E1000_SFF_IDENTIFIER_SFF: c_uint = 0x02;
pub const E1000_SFF_IDENTIFIER_SFP: c_uint = 0x03;
pub const E1000_SFF_ETH_FLAGS_OFFSET: c_uint = 0x06;
// Flags for SFP modules compatible with ETH up to 1Gb
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_sfp_flags {
    pub e1000_base_sx:1: u8,
    pub e1000_base_lx:1: u8,
    pub e1000_base_cx:1: u8,
    pub e1000_base_t:1: u8,
    pub e100_base_lx:1: u8,
    pub e100_base_fx:1: u8,
    pub e10_base_bx10:1: u8,
    pub e10_base_px:1: u8,
}
