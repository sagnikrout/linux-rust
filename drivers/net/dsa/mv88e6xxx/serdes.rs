//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mv88e6xxx/serdes.h
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
// Marvell 88E6xxx SERDES manipulation, via SMI bus
//
// Copyright (c) 2008 Marvell Semiconductor
//
// Copyright (c) 2016 Andrew Lunn <andrew@lunn.ch>
//

pub const MV88E6321_PORT0_LANE: c_uint = 0x0c;
pub const MV88E6352_ADDR_SERDES: c_uint = 0x0f;
pub const MV88E6352_SERDES_PAGE_FIBER: c_uint = 0x01;
pub const MV88E6352_SERDES_IRQ: c_uint = 0x0b;
pub const MV88E6352_SERDES_INT_ENABLE: c_uint = 0x12;

pub const MV88E6352_SERDES_INT_STATUS: c_uint = 0x13;
pub const MV88E6341_PORT5_LANE: c_uint = 0x15;
pub const MV88E6390_PORT9_LANE0: c_uint = 0x09;
pub const MV88E6390_PORT9_LANE1: c_uint = 0x12;
pub const MV88E6390_PORT9_LANE2: c_uint = 0x13;
pub const MV88E6390_PORT9_LANE3: c_uint = 0x14;
pub const MV88E6390_PORT10_LANE0: c_uint = 0x0a;
pub const MV88E6390_PORT10_LANE1: c_uint = 0x15;
pub const MV88E6390_PORT10_LANE2: c_uint = 0x16;
pub const MV88E6390_PORT10_LANE3: c_uint = 0x17;
// 10GBASE-R and 10GBASE-X4/X2

pub const MV88E6390_10G_INT_ENABLE: c_uint = 0x9001;

pub const MV88E6390_10G_INT_STATUS: c_uint = 0x9003;
pub const MV88E6393X_10G_INT_ENABLE: c_uint = 0x9000;

pub const MV88E6393X_10G_INT_STATUS: c_uint = 0x9001;
// USXGMII
pub const MV88E6390_USXGMII_LP_STATUS: c_uint = 0xf0a2;
pub const MV88E6390_USXGMII_PHY_STATUS: c_uint = 0xf0a6;
// 1000BASE-X and SGMII

pub const MV88E6390_SGMII_INT_ENABLE: c_uint = 0xa001;

pub const MV88E6390_SGMII_INT_STATUS: c_uint = 0xa002;
pub const MV88E6390_SGMII_PHY_STATUS: c_uint = 0xa003;

pub const MV88E6390_SGMII_PHY_STATUS_SPEED_1000: c_uint = 0x8000;
pub const MV88E6390_SGMII_PHY_STATUS_SPEED_100: c_uint = 0x4000;
pub const MV88E6390_SGMII_PHY_STATUS_SPEED_10: c_uint = 0x0000;

// Packet generator pad packet checker
pub const MV88E6390_PG_CONTROL: c_uint = 0xf010;

pub const MV88E6393X_PORT0_LANE: c_uint = 0x00;
pub const MV88E6393X_PORT9_LANE: c_uint = 0x09;
pub const MV88E6393X_PORT10_LANE: c_uint = 0x0a;
// Port Operational Configuration
pub const MV88E6393X_SERDES_POC: c_uint = 0xf002;
pub const MV88E6393X_SERDES_POC_PCS_1000BASEX: c_uint = 0x0000;
pub const MV88E6393X_SERDES_POC_PCS_2500BASEX: c_uint = 0x0001;
pub const MV88E6393X_SERDES_POC_PCS_SGMII_PHY: c_uint = 0x0002;
pub const MV88E6393X_SERDES_POC_PCS_SGMII_MAC: c_uint = 0x0003;
pub const MV88E6393X_SERDES_POC_PCS_5GBASER: c_uint = 0x0004;
pub const MV88E6393X_SERDES_POC_PCS_10GBASER: c_uint = 0x0005;
pub const MV88E6393X_SERDES_POC_PCS_USXGMII_PHY: c_uint = 0x0006;
pub const MV88E6393X_SERDES_POC_PCS_USXGMII_MAC: c_uint = 0x0007;
pub const MV88E6393X_SERDES_POC_PCS_MASK: c_uint = 0x0007;

pub const MV88E6393X_SERDES_CTRL1: c_uint = 0xf003;

pub const MV88E6393X_ERRATA_4_8_REG: c_uint = 0xF074;

extern "C" {
    pub fn mv88e6321_serdes_get_lane(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6341_serdes_get_lane(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6352_serdes_get_lane(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6390_serdes_get_lane(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6390x_serdes_get_lane(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6393x_serdes_get_lane(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6352_serdes_get_sset_count(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6390_serdes_get_sset_count(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6352_serdes_get_regs_len(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6352_serdes_get_regs(chip: *mut mv88e6xxx_chip, port: c_int, _p: *mut c_void);
}
extern "C" {
    pub fn mv88e6390_serdes_get_regs_len(chip: *mut mv88e6xxx_chip, port: c_int) -> c_int;
}
extern "C" {
    pub fn mv88e6390_serdes_get_regs(chip: *mut mv88e6xxx_chip, port: c_int, _p: *mut c_void);
}
// Return the (first) SERDES lane address a port is using, -errno otherwise.
