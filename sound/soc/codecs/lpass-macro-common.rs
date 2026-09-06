//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/lpass-macro-common.h
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
// Copyright (c) 2022, The Linux Foundation. All rights reserved.
//
// NPL clock is expected

// The soundwire block should be internally reset at probe

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpass_version {
    LPASS_VER_9_0_0,
    LPASS_VER_9_2_0,
    LPASS_VER_10_0_0,
    LPASS_VER_11_0_0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpass_codec_version {
    LPASS_CODEC_VERSION_UNKNOWN,
    LPASS_CODEC_VERSION_1_0,
    LPASS_CODEC_VERSION_1_1,
    LPASS_CODEC_VERSION_1_2,
    LPASS_CODEC_VERSION_2_0,
    LPASS_CODEC_VERSION_2_1,
    LPASS_CODEC_VERSION_2_5,
    LPASS_CODEC_VERSION_2_6,
    LPASS_CODEC_VERSION_2_7,
    LPASS_CODEC_VERSION_2_8,
    LPASS_CODEC_VERSION_2_9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpass_macro {
    pub macro_pd: *mut device,
    pub dcodec_pd: *mut device,
}

extern "C" {
    pub fn lpass_macro_pds_exit(pds: *mut lpass_macro);
}
extern "C" {
    pub fn lpass_macro_set_codec_version(version: lpass_codec_version);
}
extern "C" {
    pub fn lpass_macro_get_codec_version() -> lpass_codec_version;
}
