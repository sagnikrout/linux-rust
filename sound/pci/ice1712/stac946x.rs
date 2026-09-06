//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/stac946x.h
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
pub const STAC946X_RESET: c_uint = 0x00;
pub const STAC946X_STATUS: c_uint = 0x01;
pub const STAC946X_MASTER_VOLUME: c_uint = 0x02;
pub const STAC946X_LF_VOLUME: c_uint = 0x03;
pub const STAC946X_RF_VOLUME: c_uint = 0x04;
pub const STAC946X_LR_VOLUME: c_uint = 0x05;
pub const STAC946X_RR_VOLUME: c_uint = 0x06;
pub const STAC946X_CENTER_VOLUME: c_uint = 0x07;
pub const STAC946X_LFE_VOLUME: c_uint = 0x08;
pub const STAC946X_MIC_L_VOLUME: c_uint = 0x09;
pub const STAC946X_MIC_R_VOLUME: c_uint = 0x0a;
pub const STAC946X_DEEMPHASIS: c_uint = 0x0c;
pub const STAC946X_GENERAL_PURPOSE: c_uint = 0x0d;
pub const STAC946X_AUDIO_PORT_CONTROL: c_uint = 0x0e;
pub const STAC946X_MASTER_CLOCKING: c_uint = 0x0f;
pub const STAC946X_POWERDOWN_CTRL1: c_uint = 0x10;
pub const STAC946X_POWERDOWN_CTRL2: c_uint = 0x11;
pub const STAC946X_REVISION_CODE: c_uint = 0x12;
pub const STAC946X_ADDRESS_CONTROL: c_uint = 0x13;
pub const STAC946X_ADDRESS: c_uint = 0x14;
