//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/pcs/pcs-xpcs.h
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
//
// Copyright (c) 2020 Synopsys, Inc. and/or its affiliates.
// Synopsys DesignWare XPCS helpers
//
// Author: Jose Abreu <Jose.Abreu@synopsys.com>
//

// Vendor regs access

// VR_XS_PCS

pub const DW_VR_XS_PCS_DIG_CTRL1: c_uint = 0x0000;

pub const DW_VR_XS_PCS_DIG_STS: c_uint = 0x0010;

// SR_MII

// SR_AN
pub const DW_SR_AN_ADV1: c_uint = 0x10;
pub const DW_SR_AN_ADV2: c_uint = 0x11;
pub const DW_SR_AN_ADV3: c_uint = 0x12;
// Clause 73 Defines
// AN_LP_ABL1

pub const DW_C73_AN_ADV_SF: c_uint = 0x1;
// AN_LP_ABL2

// AN_LP_ABL3

// Clause 37 Defines
// VR MII MMD registers offsets
pub const DW_VR_MII_DIG_CTRL1: c_uint = 0x8000;

pub const DW_VR_MII_AN_CTRL: c_uint = 0x8001;

pub const DW_VR_MII_TX_CONFIG_PHY_SIDE_SGMII: c_uint = 0x1;
pub const DW_VR_MII_TX_CONFIG_MAC_SIDE_SGMII: c_uint = 0x0;

pub const DW_VR_MII_PCS_MODE_C37_1000BASEX: c_uint = 0x0;
pub const DW_VR_MII_PCS_MODE_C37_SGMII: c_uint = 0x2;

pub const DW_VR_MII_AN_INTR_STS: c_uint = 0x8002;

pub const DW_VR_MII_C37_ANSGM_SP_10: c_uint = 0x0;
pub const DW_VR_MII_C37_ANSGM_SP_100: c_uint = 0x1;
pub const DW_VR_MII_C37_ANSGM_SP_1000: c_uint = 0x2;

pub const DW_VR_MII_EEE_MCTRL0: c_uint = 0x8006;

pub const DW_VR_MII_EEE_MCTRL1: c_uint = 0x800b;

pub const DW_VR_MII_DIG_CTRL2: c_uint = 0x80e1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_xpcs_clock {
    DW_XPCS_CORE_CLK,
    DW_XPCS_PAD_CLK,
    DW_XPCS_NUM_CLKS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_xpcs {
    pub info: dw_xpcs_info,
    pub desc: *const dw_xpcs_desc,
    pub mdiodev: *mut mdio_device,
    pub clks: [clk_bulk_data; DW_XPCS_NUM_CLKS],
    pub pcs: phylink_pcs,
    pub interface: phy_interface_t,
    pub need_reset: bool,
    pub eee_mult_fact: u8,
}

extern "C" {
    pub fn xpcs_read(xpcs: *mut dw_xpcs, dev: c_int, reg: u32) -> c_int;
}
extern "C" {
    pub fn xpcs_write(xpcs: *mut dw_xpcs, dev: c_int, reg: u32, val: u16) -> c_int;
}
extern "C" {
    pub fn xpcs_modify(xpcs: *mut dw_xpcs, dev: c_int, reg: u32, mask: u16, set: u16) -> c_int;
}
extern "C" {
    pub fn xpcs_read_vpcs(xpcs: *mut dw_xpcs, reg: c_int) -> c_int;
}
extern "C" {
    pub fn xpcs_write_vpcs(xpcs: *mut dw_xpcs, reg: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn nxp_sja1105_sgmii_pma_config(xpcs: *mut dw_xpcs) -> c_int;
}
extern "C" {
    pub fn nxp_sja1110_sgmii_pma_config(xpcs: *mut dw_xpcs) -> c_int;
}
extern "C" {
    pub fn nxp_sja1110_2500basex_pma_config(xpcs: *mut dw_xpcs) -> c_int;
}
extern "C" {
    pub fn txgbe_xpcs_switch_mode(xpcs: *mut dw_xpcs, interface: phy_interface_t) -> c_int;
}
