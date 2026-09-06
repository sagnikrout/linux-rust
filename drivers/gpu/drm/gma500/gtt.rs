//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/gtt.h
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
//
// Copyright (c) 2007-2008, Intel Corporation.
// All Rights Reserved.
//

// This wants cleaning up with respect to the psb_dev and un-needed stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_gtt {
    pub gatt_start: u32,
    pub mmu_gatt_start: u32,
    pub gtt_start: u32,
    pub gtt_phys_start: u32,
    pub gtt_pages: unsigned,
    pub gatt_pages: unsigned,
    pub stolen_size: c_ulong,
    pub vram_stolen_size: c_ulong,
}

// Exported functions
extern "C" {
    pub fn psb_gtt_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn psb_gtt_fini(dev: *mut drm_device);
}
extern "C" {
    pub fn psb_gtt_resume(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn psb_gtt_mask_pte(pfn: u32, type: c_int) -> u32;
}
extern "C" {
    pub fn psb_gtt_remove_pages(pdev: *mut drm_psb_private, res: *const resource);
}
