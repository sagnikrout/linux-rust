//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/lpass-hdmi.h
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
// Copyright (c) 2020 The Linux Foundation. All rights reserved.
//
// lpass_hdmi.h - Definitions for the QTi LPASS HDMI
//

pub const LPASS_HDMITX_LEGACY_DISABLE: c_uint = 0x0;
pub const LPASS_HDMITX_LEGACY_ENABLE: c_uint = 0x1;
pub const LPASS_DP_AUDIO_BITWIDTH16: c_uint = 0x0;
pub const LPASS_DP_AUDIO_BITWIDTH24: c_uint = 0xb;
pub const LPASS_DATA_FORMAT_SHIFT: c_uint = 0x1;
pub const LPASS_FREQ_BIT_SHIFT: c_int = 24;
pub const LPASS_DATA_FORMAT_LINEAR: c_uint = 0x0;
pub const LPASS_DATA_FORMAT_NON_LINEAR: c_uint = 0x1;
pub const LPASS_SAMPLING_FREQ32: c_uint = 0x3;
pub const LPASS_SAMPLING_FREQ44: c_uint = 0x0;
pub const LPASS_SAMPLING_FREQ48: c_uint = 0x2;
pub const LPASS_TX_CTL_RESET: c_uint = 0x1;
pub const LPASS_TX_CTL_CLEAR: c_uint = 0x0;
pub const LPASS_SSTREAM_ENABLE: c_int = 1;
pub const LPASS_SSTREAM_DISABLE: c_int = 0;
pub const LPASS_LAYOUT_SP_DEFAULT: c_uint = 0xf;
pub const LPASS_SSTREAM_DEFAULT_ENABLE: c_int = 1;
pub const LPASS_SSTREAM_DEFAULT_DISABLE: c_int = 0;
pub const LPASS_MUTE_ENABLE: c_int = 1;
pub const LPASS_MUTE_DISABLE: c_int = 0;
pub const LPASS_META_DEFAULT_VAL: c_int = 0;
pub const HW_MODE: c_int = 1;
pub const SW_MODE: c_int = 0;
pub const LEGACY_LPASS_LPAIF: c_int = 1;
pub const LEGACY_LPASS_HDMI: c_int = 0;
pub const REPLACE_VBIT: c_uint = 0x1;
pub const LINEAR_PCM_DATA: c_uint = 0x0;
pub const NON_LINEAR_PCM_DATA: c_uint = 0x1;
pub const HDMITX_PARITY_CALC_EN: c_uint = 0x1;
pub const HDMITX_PARITY_CALC_DIS: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpass_sstream_ctl {
    pub sstream_en: *mut regmap_field,
    pub dma_sel: *mut regmap_field,
    pub auto_bbit_en: *mut regmap_field,
    pub layout: *mut regmap_field,
    pub layout_sp: *mut regmap_field,
    pub set_sp_on_en: *mut regmap_field,
    pub dp_audio: *mut regmap_field,
    pub dp_staffing_en: *mut regmap_field,
    pub dp_sp_b_hw_en: *mut regmap_field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpass_dp_metadata_ctl {
    pub mute: *mut regmap_field,
    pub as_sdp_cc: *mut regmap_field,
    pub as_sdp_ct: *mut regmap_field,
    pub aif_db4: *mut regmap_field,
    pub frequency: *mut regmap_field,
    pub mst_index: *mut regmap_field,
    pub dptx_index: *mut regmap_field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpass_hdmi_tx_ctl {
    pub soft_reset: *mut regmap_field,
    pub force_reset: *mut regmap_field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpass_hdmitx_dmactl {
    pub use_hw_chs: *mut regmap_field,
    pub use_hw_usr: *mut regmap_field,
    pub hw_chs_sel: *mut regmap_field,
    pub hw_usr_sel: *mut regmap_field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpass_vbit_ctrl {
    pub replace_vbit: *mut regmap_field,
    pub vbit_stream: *mut regmap_field,
}
