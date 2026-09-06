//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene-v2/mac.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Applied Micro X-Gene SoC Ethernet v2 Driver
//
// Copyright (c) 2017, Applied Micro Circuits Corporation
// Author(s): Iyappan Subramanian <isubramanian@apm.com>
// Keyur Chudgar <kchudgar@apm.com>
//
// Register offsets
pub const MAC_CONFIG_1: c_uint = 0xa000;
pub const MAC_CONFIG_2: c_uint = 0xa004;
pub const MII_MGMT_CONFIG: c_uint = 0xa020;
pub const MII_MGMT_COMMAND: c_uint = 0xa024;
pub const MII_MGMT_ADDRESS: c_uint = 0xa028;
pub const MII_MGMT_CONTROL: c_uint = 0xa02c;
pub const MII_MGMT_STATUS: c_uint = 0xa030;
pub const MII_MGMT_INDICATORS: c_uint = 0xa034;
pub const INTERFACE_CONTROL: c_uint = 0xa038;
pub const STATION_ADDR0: c_uint = 0xa040;
pub const STATION_ADDR1: c_uint = 0xa044;
pub const RGMII_REG_0: c_uint = 0x27e0;
pub const ICM_CONFIG0_REG_0: c_uint = 0x2c00;
pub const ICM_CONFIG2_REG_0: c_uint = 0x2c08;
pub const ECM_CONFIG0_REG_0: c_uint = 0x2d00;
// Register fields

pub const INTF_MODE_POS: c_int = 8;
pub const INTF_MODE_LEN: c_int = 2;
pub const HD_MODE_POS: c_int = 25;
pub const HD_MODE_LEN: c_int = 2;
pub const CFG_MACMODE_POS: c_int = 18;
pub const CFG_MACMODE_LEN: c_int = 2;
pub const CFG_WAITASYNCRD_POS: c_int = 0;
pub const CFG_WAITASYNCRD_LEN: c_int = 16;
pub const CFG_SPEED_125_POS: c_int = 24;
pub const CFG_WFIFOFULLTHR_POS: c_int = 0;
pub const CFG_WFIFOFULLTHR_LEN: c_int = 7;
pub const MGMT_CLOCK_SEL_POS: c_int = 0;
pub const MGMT_CLOCK_SEL_LEN: c_int = 3;
pub const PHY_ADDR_POS: c_int = 8;
pub const PHY_ADDR_LEN: c_int = 5;
pub const REG_ADDR_POS: c_int = 0;
pub const REG_ADDR_LEN: c_int = 5;

// var &= ~mask;
// var |= ((val << pos) & mask);

extern "C" {
    pub fn xge_mac_reset(pdata: *mut xge_pdata);
}
extern "C" {
    pub fn xge_mac_set_speed(pdata: *mut xge_pdata);
}
extern "C" {
    pub fn xge_mac_enable(pdata: *mut xge_pdata);
}
extern "C" {
    pub fn xge_mac_disable(pdata: *mut xge_pdata);
}
extern "C" {
    pub fn xge_mac_init(pdata: *mut xge_pdata);
}
extern "C" {
    pub fn xge_mac_set_station_addr(pdata: *mut xge_pdata);
}
