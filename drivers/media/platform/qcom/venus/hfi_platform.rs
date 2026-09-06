//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/hfi_platform.h
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
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
//

pub const MAX_PLANES: c_int = 4;
pub const MAX_FMT_ENTRIES: c_int = 32;
pub const MAX_CAP_ENTRIES: c_int = 32;
pub const MAX_ALLOC_MODE_ENTRIES: c_int = 16;
pub const MAX_CODEC_NUM: c_int = 32;
pub const MAX_SESSIONS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_formats {
    pub buftype: u32,
    pub fmt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_plat_caps {
    pub codec: u32,
    pub domain: u32,
    pub cap_bufs_mode_dynamic: bool,
    pub num_caps: c_uint,
    pub caps: [hfi_capability; MAX_CAP_ENTRIES],
    pub num_pl: c_uint,
    pub pl: [hfi_profile_level; HFI_MAX_PROFILE_COUNT],
    pub num_fmts: c_uint,
    pub fmts: [raw_formats; MAX_FMT_ENTRIES],
    pub /: *mut *mut bool valid; / used only for Venus v1xx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_platform_codec_freq_data {
    pub pixfmt: u32,
    pub session_type: u32,
    pub vpp_freq: c_ulong,
    pub vsp_freq: c_ulong,
    pub low_power_freq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_platform {
    pub codec): u32 session_type, u32,
    pub codec): u32 session_type, u32,
    pub codec): u32 session_type, u32,
    pub count): *mut *mut u32 dec_codecs, u32,
    pub entries): *mut c_uint,
    pub bufreq): *mut u32 buftype, struct hfi_buffer_requirements,
}
