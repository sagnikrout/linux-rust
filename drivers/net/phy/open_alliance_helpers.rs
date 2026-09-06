//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/open_alliance_helpers.h
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
// These defines reflect the TDR (Time Delay Reflection) diagnostic feature
// for 1000BASE-T1 automotive Ethernet PHYs as specified by the OPEN Alliance.
//
// The register values are part of the HDD.TDR register, which provides
// information about the cable status and faults. The exact register offset
// is device-specific and should be provided by the driver.
//

pub const OA_1000BT1_HDD_TDR_ACTIVATION_OFF: c_int = 1;
pub const OA_1000BT1_HDD_TDR_ACTIVATION_ON: c_int = 2;

pub const OA_1000BT1_HDD_TDR_STATUS_SHORT: c_int = 3;
pub const OA_1000BT1_HDD_TDR_STATUS_OPEN: c_int = 6;
pub const OA_1000BT1_HDD_TDR_STATUS_NOISE: c_int = 5;
pub const OA_1000BT1_HDD_TDR_STATUS_CABLE_OK: c_int = 7;
pub const OA_1000BT1_HDD_TDR_STATUS_TEST_IN_PROGRESS: c_int = 8;
pub const OA_1000BT1_HDD_TDR_STATUS_TEST_NOT_POSSIBLE: c_int = 13;
//
// OA_1000BT1_HDD_TDR_DISTANCE_MASK:
// This mask is used to extract the distance to the first/main fault
// detected by the TDR feature. Each bit represents an approximate distance
// of 1 meter, ranging from 0 to 31 meters. The exact interpretation of the
// bits may vary, but generally:
// 000000 = no error
// 000001 = error about 0-1m away
// 000010 = error between 1-2m away
// ...
// 011111 = error about 30-31m away
// 111111 = resolution not possible / out of distance
//

pub const OA_1000BT1_HDD_TDR_DISTANCE_NO_ERROR: c_int = 0;
pub const OA_1000BT1_HDD_TDR_DISTANCE_RESOLUTION_NOT_POSSIBLE: c_uint = 0x3f;
extern "C" {
    pub fn oa_1000bt1_get_ethtool_cable_result_code(reg_value: u16) -> c_int;
}
extern "C" {
    pub fn oa_1000bt1_get_tdr_distance(reg_value: u16) -> c_int;
}
