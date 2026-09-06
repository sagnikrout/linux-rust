//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/amd-sbi/tsi-core.h
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
// AMD SBTSI core driver private definitions.
//
// Copyright (C) 2026 Advanced Micro Devices, Inc.
//

//
// struct sbtsi_i3c_priv - per-device state for I3C SBTSI (includes DMA-safe buffers)
// @data: public device state exposed via dev_set_drvdata()
// @tx:   outgoing I3C bytes (DMA_TO_DEVICE); [0] register address, [1] value
// @rx:   incoming I3C data byte (DMA_FROM_DEVICE)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbtsi_i3c_priv {
    pub data: sbtsi_data,
    pub tx: [u8; 2],
    pub __aligned(ARCH_DMA_MINALIGN): u8 rx,
}

extern "C" {
    pub fn create_misc_tsi_device(data: *mut sbtsi_data, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn sbtsi_data_release(kref: *mut kref);
}
