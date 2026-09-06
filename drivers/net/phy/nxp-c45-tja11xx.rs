//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/nxp-c45-tja11xx.h
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
// NXP C45 PHY driver header file
// Copyright 2023 NXP
// Author: Radu Pirea <radu-nicolae.pirea@oss.nxp.com>
//

pub const VEND1_PORT_FUNC_ENABLES: c_uint = 0x8048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_c45_phy {
    pub phy_data: *const nxp_c45_phy_data,
    pub phydev: *mut phy_device,
    pub mii_ts: mii_timestamper,
    pub ptp_clock: *mut ptp_clock,
    pub caps: ptp_clock_info,
    pub tx_queue: sk_buff_head,
    pub rx_queue: sk_buff_head,
// used to access the PTP registers atomic
    pub ptp_lock: mutex,
    pub hwts_tx: c_int,
    pub hwts_rx: c_int,
    pub tx_delay: u32,
    pub rx_delay: u32,
    pub extts_ts: timespec64,
    pub extts_index: c_int,
    pub extts: bool,
    pub macsec: *mut nxp_c45_macsec,
    pub flags: u32,
}

extern "C" {
    pub fn nxp_c45_macsec_config_init(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn nxp_c45_macsec_probe(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn nxp_c45_macsec_remove(phydev: *mut phy_device);
}

