//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phy/phy-lvds.h
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
// Copyright 2020,2022 NXP
//
// struct phy_configure_opts_lvds - LVDS configuration set
// @bits_per_lane_and_dclk_cycle:	Number of bits per lane per differential
// clock cycle.
// @differential_clk_rate:		Clock rate, in Hertz, of the LVDS
// differential clock.
// @lanes:				Number of active, consecutive,
// data lanes, starting from lane 0,
// used for the transmissions.
// @is_slave:				Boolean, true if the phy is a slave
// which works together with a master
// phy to support dual link transmission,
// otherwise a regular phy or a master phy.
//
// This structure is used to represent the configuration state of a LVDS phy.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_configure_opts_lvds {
    pub bits_per_lane_and_dclk_cycle: c_uint,
    pub differential_clk_rate: c_ulong,
    pub lanes: c_uint,
    pub is_slave: bool,
}
