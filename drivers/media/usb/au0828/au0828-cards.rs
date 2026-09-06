//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/au0828/au0828-cards.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for the Auvitek USB bridge
//
// Copyright (c) 2008 Steven Toth <stoth@linuxtv.org>
//
pub const AU0828_BOARD_UNKNOWN: c_int = 0;
pub const AU0828_BOARD_HAUPPAUGE_HVR950Q: c_int = 1;
pub const AU0828_BOARD_HAUPPAUGE_HVR850: c_int = 2;
pub const AU0828_BOARD_DVICO_FUSIONHDTV7: c_int = 3;
pub const AU0828_BOARD_HAUPPAUGE_HVR950Q_MXL: c_int = 4;
pub const AU0828_BOARD_HAUPPAUGE_WOODBURY: c_int = 5;
pub const AU0828_BOARD_HAUPPAUGE_IMPACTVCBE: c_int = 6;
pub const AU0828_BOARD_HAUPPAUGE_HVR1265: c_int = 7;
pub const AU0828_BOARD_MONOPRICE_106456: c_int = 8;
