//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/net/mscc-phy-vsc8531.h
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


//
// Device Tree constants for Microsemi VSC8531 PHY
//
// Author: Nagaraju Lakkaraju
//
// License: Dual MIT/GPL
// Copyright (c) 2017 Microsemi Corporation
//
// PHY LED Modes
pub const VSC8531_LINK_ACTIVITY: c_int = 0;
pub const VSC8531_LINK_1000_ACTIVITY: c_int = 1;
pub const VSC8531_LINK_100_ACTIVITY: c_int = 2;
pub const VSC8531_LINK_10_ACTIVITY: c_int = 3;
pub const VSC8531_LINK_100_1000_ACTIVITY: c_int = 4;
pub const VSC8531_LINK_10_1000_ACTIVITY: c_int = 5;
pub const VSC8531_LINK_10_100_ACTIVITY: c_int = 6;
pub const VSC8584_LINK_100FX_1000X_ACTIVITY: c_int = 7;
pub const VSC8531_DUPLEX_COLLISION: c_int = 8;
pub const VSC8531_COLLISION: c_int = 9;
pub const VSC8531_ACTIVITY: c_int = 10;
pub const VSC8584_100FX_1000X_ACTIVITY: c_int = 11;
pub const VSC8531_AUTONEG_FAULT: c_int = 12;
pub const VSC8531_SERIAL_MODE: c_int = 13;
pub const VSC8531_FORCE_LED_OFF: c_int = 14;
pub const VSC8531_FORCE_LED_ON: c_int = 15;
