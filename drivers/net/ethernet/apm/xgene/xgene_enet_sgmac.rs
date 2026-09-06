//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene/xgene_enet_sgmac.h
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
// Applied Micro X-Gene SoC Ethernet Driver
//
// Copyright (c) 2014, Applied Micro Circuits Corporation
// Authors: Iyappan Subramanian <isubramanian@apm.com>
// Keyur Chudgar <kchudgar@apm.com>
//

pub const INT_PHY_ADDR: c_uint = 0x1e;
pub const SGMII_TBI_CONTROL_ADDR: c_uint = 0x44;
pub const SGMII_CONTROL_ADDR: c_uint = 0x00;
pub const SGMII_STATUS_ADDR: c_uint = 0x04;
pub const SGMII_BASE_PAGE_ABILITY_ADDR: c_uint = 0x14;

pub const SG_RX_DV_GATE_REG_0_ADDR: c_uint = 0x05fc;
pub const SGMII_EN: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_phy_speed {
    PHY_SPEED_10,
    PHY_SPEED_100,
    PHY_SPEED_1000
}
