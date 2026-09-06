//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra_isomgr_bw.h
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
// SPDX-FileCopyrightText: Copyright (c) 2025 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra_isomgr_bw.h - Definitions for ADMA bandwidth calculation
//
// Playback and Capture streams
pub const STREAM_TYPE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_adma_isomgr {
// Protect pcm devices bandwidth
    pub mutex: mutex,
// interconnect path handle
    pub icc_path_handle: *mut icc_path,
    pub bw_per_dev: [*mut u32; STREAM_TYPE],
    pub current_bandwidth: u32,
    pub max_pcm_device: u32,
    pub max_bw: u32,
}

extern "C" {
    pub fn tegra_isomgr_adma_register(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn tegra_isomgr_adma_unregister(dev: *mut device);
}
