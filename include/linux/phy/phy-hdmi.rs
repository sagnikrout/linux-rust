//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phy/phy-hdmi.h
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
// Copyright 2022,2024 NXP
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_hdmi_mode {
    PHY_HDMI_MODE_TMDS,
    PHY_HDMI_MODE_FRL,
}

//
// struct phy_configure_opts_hdmi - HDMI configuration set
// @bpc: Bits per color channel.
// @tmds_char_rate: HDMI TMDS Character Rate in Hertz.
// @frl.rate_per_lane: HDMI FRL Rate per Lane in Gbps.
// @frl.lanes: HDMI FRL lanes count.
//
// This structure is used to represent the configuration state of a HDMI phy.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_configure_opts_hdmi {
    pub bpc: c_uint,
    pub tmds_char_rate: c_ulonglong,
    pub rate_per_lane: u8,
    pub lanes: u8,
    pub frl: },
}
