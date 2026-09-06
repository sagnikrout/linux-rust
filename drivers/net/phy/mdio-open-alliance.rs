//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/mdio-open-alliance.h
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
// mdio-open-alliance.h - definition of OPEN Alliance SIG standard registers
//

// NOTE: all OATC14 registers are located in MDIO_MMD_VEND2
// Open Alliance TC14 (10BASE-T1S) registers
pub const MDIO_OATC14_PLCA_IDVER: c_uint = 0xca00  /* PLCA ID and version */;
pub const MDIO_OATC14_PLCA_CTRL0: c_uint = 0xca01	/* PLCA Control register 0 */;
pub const MDIO_OATC14_PLCA_CTRL1: c_uint = 0xca02	/* PLCA Control register 1 */;
pub const MDIO_OATC14_PLCA_STATUS: c_uint = 0xca03	/* PLCA Status register */;
pub const MDIO_OATC14_PLCA_TOTMR: c_uint = 0xca04	/* PLCA TO Timer register */;
pub const MDIO_OATC14_PLCA_BURST: c_uint = 0xca05	/* PLCA BURST mode register */;
// Open Alliance TC14 PLCA IDVER register
pub const MDIO_OATC14_PLCA_IDM: c_uint = 0xff00	/* PLCA MAP ID */;
pub const MDIO_OATC14_PLCA_VER: c_uint = 0x00ff	/* PLCA MAP version */;
// Open Alliance TC14 PLCA CTRL0 register

// Open Alliance TC14 PLCA CTRL1 register
pub const MDIO_OATC14_PLCA_NCNT: c_uint = 0xff00	/* PLCA node count */;
pub const MDIO_OATC14_PLCA_ID: c_uint = 0x00ff	/* PLCA local node ID */;
// Open Alliance TC14 PLCA STATUS register

// Open Alliance TC14 PLCA TOTMR register
pub const MDIO_OATC14_PLCA_TOT: c_uint = 0x00ff;
// Open Alliance TC14 PLCA BURST register
pub const MDIO_OATC14_PLCA_MAXBC: c_uint = 0xff00;
pub const MDIO_OATC14_PLCA_BTMR: c_uint = 0x00ff;
// Version Identifiers
pub const OATC14_IDM: c_uint = 0x0a00;
//
// Open Alliance TC14 (10BASE-T1S) - Advanced Diagnostic Features Registers
//
// Refer to the OPEN Alliance documentation:
// https://opensig.org/automotive-ethernet-specifications
//
// Specification:
// "10BASE-T1S Advanced Diagnostic PHY Features"
// https://opensig.org/wp-content/uploads/2025/06/OPEN_Alliance_10BASE-T1S_Advanced_PHY_features_for-automotive_Ethernet_V2.1b.pdf
//
// Advanced Diagnostic Features Capability Register
pub const MDIO_OATC14_ADFCAP: c_uint = 0xcc00;

// Harness Defect Detection Register
pub const MDIO_OATC14_HDD: c_uint = 0xcc01;

// Dynamic Channel Quality SQI Register
pub const MDIO_OATC14_DCQ_SQI: c_uint = 0xcc03;

// Dynamic Channel Quality SQI Plus Register
pub const MDIO_OATC14_DCQ_SQIPLUS: c_uint = 0xcc04;

// SQI is supported using 3 bits means 8 levels (0-7)
pub const OATC14_SQI_MAX_LEVEL: c_int = 7;
// Bus Short/Open Status:
// 0 0 - no fault; everything is ok. (Default)
// 0 1 - detected as an open or missing termination(s)
// 1 0 - detected as a short or extra termination(s)
// 1 1 - fault but fault type not detectable. More details can be available by
// vender specific register if supported.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum oatc14_hdd_status {
    OATC14_HDD_STATUS_CABLE_OK = 0,
    OATC14_HDD_STATUS_OPEN,
    OATC14_HDD_STATUS_SHORT,
    OATC14_HDD_STATUS_NOT_DETECTABLE,
}
