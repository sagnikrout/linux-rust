//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/marvell_phy.h
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
// Mask used for ID comparisons
pub const MARVELL_PHY_ID_MASK: c_uint = 0xfffffff0;
// Known PHY IDs
pub const MARVELL_PHY_ID_88E1101: c_uint = 0x01410c60;
pub const MARVELL_PHY_ID_88E3082: c_uint = 0x01410c80;
pub const MARVELL_PHY_ID_88E1112: c_uint = 0x01410c90;
pub const MARVELL_PHY_ID_88E1111: c_uint = 0x01410cc0;
pub const MARVELL_PHY_ID_88E1118: c_uint = 0x01410e10;
pub const MARVELL_PHY_ID_88E1121R: c_uint = 0x01410cb0;
pub const MARVELL_PHY_ID_88E1145: c_uint = 0x01410cd0;
pub const MARVELL_PHY_ID_88E1149R: c_uint = 0x01410e50;
pub const MARVELL_PHY_ID_88E1240: c_uint = 0x01410e30;
pub const MARVELL_PHY_ID_88E1318S: c_uint = 0x01410e90;
pub const MARVELL_PHY_ID_88E1340S: c_uint = 0x01410dc0;
pub const MARVELL_PHY_ID_88E1116R: c_uint = 0x01410e40;
pub const MARVELL_PHY_ID_88E1510: c_uint = 0x01410dd0;
pub const MARVELL_PHY_ID_88E1540: c_uint = 0x01410eb0;
pub const MARVELL_PHY_ID_88E1545: c_uint = 0x01410ea0;
pub const MARVELL_PHY_ID_88E1548P: c_uint = 0x01410ec0;
pub const MARVELL_PHY_ID_88E3016: c_uint = 0x01410e60;
pub const MARVELL_PHY_ID_88X3310: c_uint = 0x002b09a0;
pub const MARVELL_PHY_ID_88E2110: c_uint = 0x002b09b0;
pub const MARVELL_PHY_ID_88X2222: c_uint = 0x01410f10;
pub const MARVELL_PHY_ID_88Q2110: c_uint = 0x002b0980;
pub const MARVELL_PHY_ID_88Q2220: c_uint = 0x002b0b20;
// Marvel 88E1111 in Finisar SFP module with modified PHY ID
pub const MARVELL_PHY_ID_88E1111_FINISAR: c_uint = 0x01ff0cc0;
// ID from 88E6020, assumed to be the same for the whole 6250 family
pub const MARVELL_PHY_ID_88E6250_FAMILY: c_uint = 0x01410db0;
// These Ethernet switch families contain embedded PHYs, but they do
// not have a model ID. So the switch driver traps reads to the ID2
// register and returns the switch family ID
//
pub const MARVELL_PHY_ID_88E6341_FAMILY: c_uint = 0x01410f41;
pub const MARVELL_PHY_ID_88E6390_FAMILY: c_uint = 0x01410f90;
pub const MARVELL_PHY_ID_88E6393_FAMILY: c_uint = 0x002b0b9b;

// struct phy_device dev_flags definitions
pub const MARVELL_PHY_M1145_FLAGS_RESISTANCE: c_uint = 0x00000001;
pub const MARVELL_PHY_M1118_DNS323_LEDS: c_uint = 0x00000002;
pub const MARVELL_PHY_LED0_LINK_LED1_ACTIVE: c_uint = 0x00000004;
