//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/net/microchip-lan78xx.h
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
// LED modes for LAN7800/LAN7850 embedded PHY
pub const LAN78XX_LINK_ACTIVITY: c_int = 0;
pub const LAN78XX_LINK_1000_ACTIVITY: c_int = 1;
pub const LAN78XX_LINK_100_ACTIVITY: c_int = 2;
pub const LAN78XX_LINK_10_ACTIVITY: c_int = 3;
pub const LAN78XX_LINK_100_1000_ACTIVITY: c_int = 4;
pub const LAN78XX_LINK_10_1000_ACTIVITY: c_int = 5;
pub const LAN78XX_LINK_10_100_ACTIVITY: c_int = 6;
pub const LAN78XX_DUPLEX_COLLISION: c_int = 8;
pub const LAN78XX_COLLISION: c_int = 9;
pub const LAN78XX_ACTIVITY: c_int = 10;
pub const LAN78XX_AUTONEG_FAULT: c_int = 12;
pub const LAN78XX_FORCE_LED_OFF: c_int = 14;
pub const LAN78XX_FORCE_LED_ON: c_int = 15;
