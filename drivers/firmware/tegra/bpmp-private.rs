//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/tegra/bpmp-private.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018, NVIDIA CORPORATION.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_bpmp_ops {
    pub bpmp): *mut *mut int (init)(struct tegra_bpmp,
    pub bpmp): *mut *mut void (deinit)(struct tegra_bpmp,
    pub channel): *mut *mut bool (is_response_ready)(struct tegra_bpmp_channel,
    pub channel): *mut *mut bool (is_request_ready)(struct tegra_bpmp_channel,
    pub channel): *mut *mut int (ack_response)(struct tegra_bpmp_channel,
    pub channel): *mut *mut int (ack_request)(struct tegra_bpmp_channel,
    pub channel): *mut *mut bool (is_response_channel_free)(struct tegra_bpmp_channel,
    pub channel): *mut *mut bool (is_request_channel_free)(struct tegra_bpmp_channel,
    pub channel): *mut *mut int (post_response)(struct tegra_bpmp_channel,
    pub channel): *mut *mut int (post_request)(struct tegra_bpmp_channel,
    pub bpmp): *mut *mut int (ring_doorbell)(struct tegra_bpmp,
    pub bpmp): *mut *mut int (resume)(struct tegra_bpmp,
}
