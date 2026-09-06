//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra_cif.h
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
// SPDX-FileCopyrightText: Copyright (c) 2020-2025 NVIDIA CORPORATION. All rights reserved.
//
// tegra_cif.h - TEGRA Audio CIF Programming
//

pub const TEGRA_ACIF_CTRL_FIFO_TH_SHIFT: c_int = 24;
pub const TEGRA_ACIF_CTRL_AUDIO_CH_SHIFT: c_int = 20;
pub const TEGRA_ACIF_CTRL_CLIENT_CH_SHIFT: c_int = 16;
pub const TEGRA_ACIF_CTRL_AUDIO_BITS_SHIFT: c_int = 12;
pub const TEGRA_ACIF_CTRL_CLIENT_BITS_SHIFT: c_int = 8;
pub const TEGRA_ACIF_CTRL_EXPAND_SHIFT: c_int = 6;
pub const TEGRA_ACIF_CTRL_STEREO_CONV_SHIFT: c_int = 4;
pub const TEGRA_ACIF_CTRL_REPLICATE_SHIFT: c_int = 3;
pub const TEGRA_ACIF_CTRL_TRUNCATE_SHIFT: c_int = 1;
pub const TEGRA_ACIF_CTRL_MONO_CONV_SHIFT: c_int = 0;
pub const TEGRA264_ACIF_CTRL_AUDIO_BITS_SHIFT: c_int = 11;
pub const TEGRA264_ACIF_CTRL_CLIENT_CH_SHIFT: c_int = 14;
pub const TEGRA264_ACIF_CTRL_AUDIO_CH_SHIFT: c_int = 19;
// AUDIO/CLIENT_BITS values
pub const TEGRA_ACIF_BITS_8: c_int = 1;
pub const TEGRA_ACIF_BITS_16: c_int = 3;
pub const TEGRA_ACIF_BITS_24: c_int = 5;
pub const TEGRA_ACIF_BITS_32: c_int = 7;
pub const TEGRA_ACIF_UPDATE_MASK: c_uint = 0x3ffffffb;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_cif_conf {
    pub threshold: c_uint,
    pub audio_ch: c_uint,
    pub client_ch: c_uint,
    pub audio_bits: c_uint,
    pub client_bits: c_uint,
    pub expand: c_uint,
    pub stereo_conv: c_uint,
    pub replicate: c_uint,
    pub truncate: c_uint,
    pub mono_conv: c_uint,
}
