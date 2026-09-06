//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/boards/sof_sdw_common.h
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
// Copyright (c) 2020 Intel Corporation
//
// sof_sdw_common.h - prototypes for common helpers
//

pub const MAX_HDMI_NUM: c_int = 4;
pub const SOC_SDW_MAX_CPU_DAIS: c_int = 16;
pub const SOC_SDW_INTEL_BIDIR_PDI_BASE: c_int = 2;
// 8 combinations with 4 links + unused group 0
pub const SDW_MAX_GROUPS: c_int = 9;
// Deprecated and no longer supported by the code

// Deprecated and no longer supported by the code

// BT audio offload: reserve 3 bits for future
pub const SOF_BT_OFFLOAD_SSP_SHIFT: c_int = 18;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_mc_ctx {
    pub hdmi: sof_hdmi_private,
// To store SDW Pin index for each SoundWire link
    pub sdw_pin_index: [c_uint; SDW_INTEL_MAX_LINKS],
}

// generic HDMI support
extern "C" {
    pub fn sof_sdw_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}
extern "C" {
    pub fn sof_sdw_hdmi_card_late_probe(card: *mut snd_soc_card) -> c_int;
}
