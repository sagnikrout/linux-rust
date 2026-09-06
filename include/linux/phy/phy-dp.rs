//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phy/phy-dp.h
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
// Copyright (C) 2019 Cadence Design Systems Inc.
//

pub const PHY_SUBMODE_DP: c_int = 0;
pub const PHY_SUBMODE_EDP: c_int = 1;
//
// struct phy_configure_opts_dp - DisplayPort PHY configuration set
//
// This structure is used to represent the configuration state of a
// DisplayPort phy.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_configure_opts_dp {
//
// @link_rate:
//
// Link Rate, in Mb/s, of the main link.
//
// Allowed values: 1620, 2160, 2430, 2700, 3240, 4320, 5400, 8100 Mb/s
//
    pub link_rate: c_uint,
//
// @lanes:
//
// Number of active, consecutive, data lanes, starting from
// lane 0, used for the transmissions on main link.
//
// Allowed values: 1, 2, 4
//
    pub lanes: c_uint,
//
// @voltage:
//
// Voltage swing levels, as specified by DisplayPort specification,
// to be used by particular lanes. One value per lane.
// voltage[0] is for lane 0, voltage[1] is for lane 1, etc.
//
// Maximum value: 3
//
    pub voltage: [c_uint; 4],
//
// @pre:
//
// Pre-emphasis levels, as specified by DisplayPort specification, to be
// used by particular lanes. One value per lane.
//
// Maximum value: 3
//
    pub pre: [c_uint; 4],
//
// @ssc:
//
// Flag indicating, whether or not to enable spread-spectrum clocking.
//
    pub 1: u8 ssc :,
//
// @set_rate:
//
// Flag indicating, whether or not reconfigure link rate and SSC to
// requested values.
//
    pub 1: u8 set_rate :,
//
// @set_lanes:
//
// Flag indicating, whether or not reconfigure lane count to
// requested value.
//
    pub 1: u8 set_lanes :,
//
// @set_voltages:
//
// Flag indicating, whether or not reconfigure voltage swing
// and pre-emphasis to requested values. Only lanes specified
// by "lanes" parameter will be affected.
//
    pub 1: u8 set_voltages :,
}
