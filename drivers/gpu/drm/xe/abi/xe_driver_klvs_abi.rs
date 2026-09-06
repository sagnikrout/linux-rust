//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/xe_driver_klvs_abi.h
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
// Copyright © 2026 Intel Corporation
//

//
// DOC: Xe Driver KLVs
//
// The Xe driver uses the following keys from the `GuC Reserved KLVs`_ range:
//
// _`MIGRATION_KLV_DEVICE_DEVID_KEY` :
// PCI device ID of the migrated VF.
// _`MIGRATION_KLV_DEVICE_REVID_KEY` :
// PCI device revision ID of the migrated VF.
//
pub const MIGRATION_KLV_DEVICE_DEVID_KEY: c_uint = 0xf001u;

pub const MIGRATION_KLV_DEVICE_REVID_KEY: c_uint = 0xf002u;

