//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/efx_devlink.h
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
// Driver for AMD network controllers and boards
// Copyright (C) 2023, Advanced Micro Devices, Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//

// Custom devlink-info version object names for details that do not map to the
// generic standardized names.
//

pub const EFX_MAX_VERSION_INFO_LEN: c_int = 64;
extern "C" {
    pub fn efx_probe_devlink_and_lock(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_probe_devlink_unlock(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_fini_devlink_lock(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_fini_devlink_and_unlock(efx: *mut efx_nic);
}

extern "C" {
    pub fn ef100_pf_set_devlink_port(efx: *mut efx_nic);
}
extern "C" {
    pub fn ef100_rep_set_devlink_port(efv: *mut efx_rep);
}
extern "C" {
    pub fn ef100_pf_unset_devlink_port(efx: *mut efx_nic);
}
extern "C" {
    pub fn ef100_rep_unset_devlink_port(efv: *mut efx_rep);
}

