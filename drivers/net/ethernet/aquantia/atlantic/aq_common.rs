//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_common.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Atlantic Network Driver
//
// Copyright (C) 2014-2019 aQuantia Corporation
// Copyright (C) 2019-2020 Marvell International Ltd.
//
// File aq_common.h: Basic includes for all files in project.

pub const PCI_VENDOR_ID_AQUANTIA: c_uint = 0x1D6A;
pub const AQ_DEVICE_ID_0001: c_uint = 0x0001;
pub const AQ_DEVICE_ID_D100: c_uint = 0xD100;
pub const AQ_DEVICE_ID_D107: c_uint = 0xD107;
pub const AQ_DEVICE_ID_D108: c_uint = 0xD108;
pub const AQ_DEVICE_ID_D109: c_uint = 0xD109;
pub const AQ_DEVICE_ID_AQC100: c_uint = 0x00B1;
pub const AQ_DEVICE_ID_AQC107: c_uint = 0x07B1;
pub const AQ_DEVICE_ID_AQC108: c_uint = 0x08B1;
pub const AQ_DEVICE_ID_AQC109: c_uint = 0x09B1;
pub const AQ_DEVICE_ID_AQC111: c_uint = 0x11B1;
pub const AQ_DEVICE_ID_AQC112: c_uint = 0x12B1;
pub const AQ_DEVICE_ID_AQC100S: c_uint = 0x80B1;
pub const AQ_DEVICE_ID_AQC107S: c_uint = 0x87B1;
pub const AQ_DEVICE_ID_AQC108S: c_uint = 0x88B1;
pub const AQ_DEVICE_ID_AQC109S: c_uint = 0x89B1;
pub const AQ_DEVICE_ID_AQC111S: c_uint = 0x91B1;
pub const AQ_DEVICE_ID_AQC112S: c_uint = 0x92B1;
pub const AQ_DEVICE_ID_AQC113DEV: c_uint = 0x00C0;
pub const AQ_DEVICE_ID_AQC113CS: c_uint = 0x94C0;
pub const AQ_DEVICE_ID_AQC113CA: c_uint = 0x34C0;
pub const AQ_DEVICE_ID_AQC114CS: c_uint = 0x93C0;
pub const AQ_DEVICE_ID_AQC113: c_uint = 0x04C0;
pub const AQ_DEVICE_ID_AQC113C: c_uint = 0x14C0;
pub const AQ_DEVICE_ID_AQC115C: c_uint = 0x12C0;
pub const AQ_DEVICE_ID_AQC116C: c_uint = 0x11C0;

pub const AQ_HWREV_ANY: c_int = 0;
pub const AQ_HWREV_1: c_int = 1;
pub const AQ_HWREV_2: c_int = 2;

