//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8189/mt8189-afe-common.h
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
// mt8189-afe-common.h  --  Mediatek 8189 audio driver definitions
//
// Copyright (c) 2025 MediaTek Inc.
// Author: Darren Ye <darren.ye@mediatek.com>
//

// HW IPM 2.0
// update irq ID (= enum) from AFE_IRQ_MCU_STATUS
// AUDIO_ENGEN_CON0
// AUDIO_TOP_CON4
// MCLK
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8189_afe_private {
    pub clk: *mut clk,
    pub pmic_regmap: *mut regmap,
// dai
    pub dai_priv: [*mut c_void; MT8189_DAI_NUM],
// adda
    pub mtkaif_protocol: c_int,
    pub mtkaif_chosen_phase: [c_int; 4],
    pub mtkaif_phase_cycle: [c_int; 4],
    pub mtkaif_calibration_num_phase: c_int,
    pub mtkaif_dmic: c_int,
    pub mtkaif_dmic_ch34: c_int,
// add for vs1 voter
    pub is_adda_dl_on: bool,
    pub is_adda_ul_on: bool,
// adda dl vol idx is at maximum
    pub is_adda_dl_max_vol: bool,
// current vote status of vs1
    pub is_mt6363_vote: bool,
// mck
    pub mck_rate: [c_int; MT8189_MCK_NUM],
// channel merge
    pub cm_rate: [c_uint; CM_NUM],
    pub cm_channels: c_uint,
}

extern "C" {
    pub fn mt8189_dai_adda_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8189_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8189_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8189_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int;
}
