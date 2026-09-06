//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/zoran/zoran_card.h
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
// Zoran zr36057/zr36067 PCI controller driver, for the
// Pinnacle/Miro DC10/DC10+/DC30/DC30+, Iomega Buz, Linux
// Media Labs LML33/LML33R10.
//
// This part handles card-specific data and detection
//
// Copyright (C) 2000 Serguei Miridonov <mirsev@cicese.mx>
//
// Anybody who uses more than four?
pub const BUZ_MAX: c_int = 4;
extern "C" {
    pub fn zoran_open_init_params(zr: *mut zoran);
}
extern "C" {
    pub fn zoran_vdev_release(vdev: *mut video_device);
}
extern "C" {
    pub fn zr36016_write(codec: *mut videocodec, reg: u16, val: u32);
}
