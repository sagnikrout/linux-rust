//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_mhccif.h
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
// Copyright (c) 2021, MediaTek Inc.
// Copyright (c) 2021-2022, Intel Corporation.
//
// Authors:
// Haijun Liu <haijun.liu@mediatek.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//
// Contributors:
// Amir Hanania <amir.hanania@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
//

extern "C" {
    pub fn t7xx_mhccif_mask_set(t7xx_dev: *mut t7xx_pci_dev, val: u32);
}
extern "C" {
    pub fn t7xx_mhccif_mask_clr(t7xx_dev: *mut t7xx_pci_dev, val: u32);
}
extern "C" {
    pub fn t7xx_mhccif_mask_get(t7xx_dev: *mut t7xx_pci_dev) -> u32;
}
extern "C" {
    pub fn t7xx_mhccif_init(t7xx_dev: *mut t7xx_pci_dev);
}
extern "C" {
    pub fn t7xx_mhccif_read_sw_int_sts(t7xx_dev: *mut t7xx_pci_dev) -> u32;
}
extern "C" {
    pub fn t7xx_mhccif_h2d_swint_trigger(t7xx_dev: *mut t7xx_pci_dev, channel: u32);
}
