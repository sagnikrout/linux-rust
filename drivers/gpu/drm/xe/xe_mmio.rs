//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_mmio.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2021-2023 Intel Corporation
//

extern "C" {
    pub fn xe_mmio_probe_early(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_mmio_probe_tiles(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_mmio_init(mmio: *mut xe_mmio, tile: *mut xe_tile, ptr: *mut void __iomem, size: u32);
}
extern "C" {
    pub fn xe_mmio_read8(mmio: *mut xe_mmio, reg: xe_reg) -> u8;
}
extern "C" {
    pub fn xe_mmio_write8(mmio: *mut xe_mmio, reg: xe_reg, val: u8);
}
extern "C" {
    pub fn xe_mmio_read16(mmio: *mut xe_mmio, reg: xe_reg) -> u16;
}
extern "C" {
    pub fn xe_mmio_write32(mmio: *mut xe_mmio, reg: xe_reg, val: u32);
}
extern "C" {
    pub fn xe_mmio_read32(mmio: *mut xe_mmio, reg: xe_reg) -> u32;
}
extern "C" {
    pub fn xe_mmio_rmw32(mmio: *mut xe_mmio, reg: xe_reg, clr: u32, set: u32) -> u32;
}
extern "C" {
    pub fn xe_mmio_write32_and_verify(mmio: *mut xe_mmio, reg: xe_reg, val: u32, mask: u32, eval: u32) -> c_int;
}
extern "C" {
    pub fn xe_mmio_in_range(mmio: *const xe_mmio, range: *const xe_mmio_range, reg: xe_reg) -> bool;
}
extern "C" {
    pub fn xe_mmio_read64_2x32(mmio: *mut xe_mmio, reg: xe_reg) -> u64;
}

extern "C" {
    pub fn xe_mmio_init_vf_view(mmio: *mut xe_mmio, base: *const xe_mmio, vfid: c_uint);
}

