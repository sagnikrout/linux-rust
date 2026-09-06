//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/busses/i2c-piix4.h
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
// PIIX4/SB800 SMBus Interfaces
//
// Copyright (c) 2024, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Authors: Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
// Sanket Goswami <Sanket.Goswami@amd.com>
//

// PIIX4 SMBus address offsets

// PIIX4 constants
pub const PIIX4_BLOCK_DATA: c_uint = 0x14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sb800_mmio_cfg {
    pub addr: *mut void __iomem,
    pub use_mmio: bool,
}

extern "C" {
    pub fn piix4_sb800_port_sel(port: u8, mmio_cfg: *mut sb800_mmio_cfg) -> c_int;
}
extern "C" {
    pub fn piix4_transaction(piix4_adapter: *mut i2c_adapter, piix4_smba: c_ushort) -> c_int;
}
extern "C" {
    pub fn piix4_sb800_region_request(dev: *mut device, mmio_cfg: *mut sb800_mmio_cfg) -> c_int;
}
extern "C" {
    pub fn piix4_sb800_region_release(dev: *mut device, mmio_cfg: *mut sb800_mmio_cfg);
}
