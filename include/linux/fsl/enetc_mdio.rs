//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/enetc_mdio.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2019 NXP

// PCS registers
pub const ENETC_PCS_LINK_TIMER1: c_uint = 0x12;
pub const ENETC_PCS_LINK_TIMER1_VAL: c_uint = 0x06a0;
pub const ENETC_PCS_LINK_TIMER2: c_uint = 0x13;
pub const ENETC_PCS_LINK_TIMER2_VAL: c_uint = 0x0003;
pub const ENETC_PCS_IF_MODE: c_uint = 0x14;

// Not a mistake, the SerDes PLL needs to be set at 3.125 GHz by Reset
// Configuration Word (RCW, outside Linux control) for 2.5G SGMII mode. The PCS
// still thinks it's at gigabit.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enetc_pcs_speed {
    ENETC_PCS_SPEED_10	= 0,
    ENETC_PCS_SPEED_100	= 1,
    ENETC_PCS_SPEED_1000	= 2,
    ENETC_PCS_SPEED_2500	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enetc_mdio_priv {
    pub hw: *mut enetc_hw,
    pub mdio_base: c_int,
}

extern "C" {
    pub fn enetc_mdio_read_c22(bus: *mut mii_bus, phy_id: c_int, regnum: c_int) -> c_int;
}
extern "C" {
    pub fn enetc_mdio_read_c45(bus: *mut mii_bus, phy_id: c_int, devad: c_int, regnum: c_int) -> c_int;
}

