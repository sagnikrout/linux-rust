//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/common/mtk-dai-adda-common.h
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
// Copyright (c) 2021 MediaTek Inc.
// Copyright (c) 2024 Collabora Ltd.
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adda_input_mode_rate {
    MTK_AFE_ADDA_DL_RATE_8K = 0,
    MTK_AFE_ADDA_DL_RATE_11K = 1,
    MTK_AFE_ADDA_DL_RATE_12K = 2,
    MTK_AFE_ADDA_DL_RATE_16K = 3,
    MTK_AFE_ADDA_DL_RATE_22K = 4,
    MTK_AFE_ADDA_DL_RATE_24K = 5,
    MTK_AFE_ADDA_DL_RATE_32K = 6,
    MTK_AFE_ADDA_DL_RATE_44K = 7,
    MTK_AFE_ADDA_DL_RATE_48K = 8,
    MTK_AFE_ADDA_DL_RATE_96K = 9,
    MTK_AFE_ADDA_DL_RATE_192K = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adda_voice_mode_rate {
    MTK_AFE_ADDA_UL_RATE_8K = 0,
    MTK_AFE_ADDA_UL_RATE_16K = 1,
    MTK_AFE_ADDA_UL_RATE_32K = 2,
    MTK_AFE_ADDA_UL_RATE_48K = 3,
    MTK_AFE_ADDA_UL_RATE_96K = 4,
    MTK_AFE_ADDA_UL_RATE_192K = 5,
    MTK_AFE_ADDA_UL_RATE_48K_HD = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adda_rxif_delay_data {
    DELAY_DATA_MISO1 = 0,
    DELAY_DATA_MISO0 = 1,
    DELAY_DATA_MISO2 = 1,
}

extern "C" {
    pub fn mtk_adda_dl_rate_transform(afe: *mut mtk_base_afe, rate: u32) -> c_uint;
}
extern "C" {
    pub fn mtk_adda_ul_rate_transform(afe: *mut mtk_base_afe, rate: u32) -> c_uint;
}
