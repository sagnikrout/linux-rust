//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/typec_tbt.h
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

pub const USB_TYPEC_VENDOR_INTEL: c_uint = 0x8087;
// Alias for convenience

// Connector state for Thunderbolt3

//
// struct typec_thunderbolt_data - Thundebolt3 Alt Mode specific data
// @device_mode: Device Discover Mode VDO
// @cable_mode: Cable Discover Mode VDO
// @enter_vdo: Enter Mode VDO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_thunderbolt_data {
    pub device_mode: u32,
    pub cable_mode: u32,
    pub enter_vdo: u32,
}

// TBT3 Device Discover Mode VDO bits

pub const TBT_ADAPTER_LEGACY: c_int = 0;
pub const TBT_ADAPTER_TBT3: c_int = 1;

// TBT3 Cable Discover Mode VDO bits

pub const TBT_CABLE_USB3_GEN1: c_int = 1;
pub const TBT_CABLE_USB3_PASSIVE: c_int = 2;
pub const TBT_CABLE_10_AND_20GBPS: c_int = 3;

pub const TBT_GEN3_NON_ROUNDED: c_int = 0;
pub const TBT_GEN3_GEN4_ROUNDED_NON_ROUNDED: c_int = 1;

// TBT3 Device Enter Mode VDO bits

