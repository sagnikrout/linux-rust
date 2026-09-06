//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssg/icssg_mii_rt.h
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
// PRU-ICSS MII_RT register definitions
//
// Copyright (C) 2015-2022 Texas Instruments Incorporated - https://www.ti.com
//

// PRUSS_MII_RT Registers
pub const PRUSS_MII_RT_RXCFG0: c_uint = 0x0;
pub const PRUSS_MII_RT_RXCFG1: c_uint = 0x4;
pub const PRUSS_MII_RT_TXCFG0: c_uint = 0x10;
pub const PRUSS_MII_RT_TXCFG1: c_uint = 0x14;
pub const PRUSS_MII_RT_TX_CRC0: c_uint = 0x20;
pub const PRUSS_MII_RT_TX_CRC1: c_uint = 0x24;
pub const PRUSS_MII_RT_TX_IPG0: c_uint = 0x30;
pub const PRUSS_MII_RT_TX_IPG1: c_uint = 0x34;
pub const PRUSS_MII_RT_PRS0: c_uint = 0x38;
pub const PRUSS_MII_RT_PRS1: c_uint = 0x3c;
pub const PRUSS_MII_RT_RX_FRMS0: c_uint = 0x40;
pub const PRUSS_MII_RT_RX_FRMS1: c_uint = 0x44;
pub const PRUSS_MII_RT_RX_PCNT0: c_uint = 0x48;
pub const PRUSS_MII_RT_RX_PCNT1: c_uint = 0x4c;
pub const PRUSS_MII_RT_RX_ERR0: c_uint = 0x50;
pub const PRUSS_MII_RT_RX_ERR1: c_uint = 0x54;
// PRUSS_MII_RT_RXCFG0/1 bits

// PRUSS_MII_RT_TXCFG0/1 bits

pub const PRUSS_MII_RT_TXCFG_TX_START_DELAY_SHIFT: c_int = 16;

pub const PRUSS_MII_RT_TXCFG_TX_CLK_DELAY_SHIFT: c_int = 28;

// PRUSS_MII_RT_TX_IPG0/1 bits
pub const PRUSS_MII_RT_TX_IPG_IPG_SHIFT: c_int = 0;

// PRUSS_MII_RT_PRS0/1 bits

// PRUSS_MII_RT_RX_FRMS0/1 bits
pub const PRUSS_MII_RT_RX_FRMS_MIN_FRM_SHIFT: c_int = 0;

pub const PRUSS_MII_RT_RX_FRMS_MAX_FRM_SHIFT: c_int = 16;

// Min/Max in MII_RT_RX_FRMS
// For EMAC and Switch

// for HSR and PRP

// PRUSS_MII_RT_RX_PCNT0/1 bits
pub const PRUSS_MII_RT_RX_PCNT_MIN_PCNT_SHIFT: c_int = 0;

pub const PRUSS_MII_RT_RX_PCNT_MAX_PCNT_SHIFT: c_int = 4;

// PRUSS_MII_RT_RX_ERR0/1 bits

pub const ICSSG_CFG_OFFSET: c_int = 0;
pub const RGMII_CFG_OFFSET: c_int = 4;
// Constant to choose between MII0 and MII1
pub const ICSS_MII0: c_int = 0;
pub const ICSS_MII1: c_int = 1;
// ICSSG_CFG Register bits

pub const ICSSG_CFG_MII1_MODE_SHIFT: c_int = 5;

pub const ICSSG_CFG_MII0_MODE_SHIFT: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mii_mode {
    MII_MODE_MII = 0,
    MII_MODE_RGMII
}

// RGMII CFG Register bits

pub const RGMII_CFG_SPEED_MII0_SHIFT: c_int = 1;
pub const RGMII_CFG_SPEED_MII1_SHIFT: c_int = 5;

pub const RGMII_CFG_FULLDUPLEX_MII0_SHIFT: c_int = 3;
pub const RGMII_CFG_FULLDUPLEX_MII1_SHIFT: c_int = 7;
pub const RGMII_CFG_SPEED_10M: c_int = 0;
pub const RGMII_CFG_SPEED_100M: c_int = 1;
pub const RGMII_CFG_SPEED_1G: c_int = 2;
extern "C" {
    pub fn icssg_mii_update_ipg(mii_rt: *mut regmap, mii: c_int, ipg: u32);
}
extern "C" {
    pub fn icssg_mii_update_mtu(mii_rt: *mut regmap, mii: c_int, mtu: c_int);
}
extern "C" {
    pub fn icssg_update_rgmii_cfg(miig_rt: *mut regmap, emac: *mut prueth_emac);
}
extern "C" {
    pub fn icssg_rgmii_cfg_get_bitfield(miig_rt: *mut regmap, mask: u32, shift: u32) -> u32;
}
extern "C" {
    pub fn icssg_rgmii_get_speed(miig_rt: *mut regmap, mii: c_int) -> u32;
}
extern "C" {
    pub fn icssg_rgmii_get_fullduplex(miig_rt: *mut regmap, mii: c_int) -> u32;
}
extern "C" {
    pub fn icssg_miig_set_interface_mode(miig_rt: *mut regmap, mii: c_int, phy_if: phy_interface_t);
}
