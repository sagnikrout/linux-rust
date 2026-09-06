//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8186/mt8186-afe-common.h
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
// mt8186-afe-common.h  --  Mediatek 8186 audio driver definitions
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
//

// SA suggest apply -0.3db to audio/speech path

// MCLK
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8186_afe_private {
    pub clk: *mut clk,
    pub lookup: *mut clk_lookup,
    pub topckgen: *mut regmap,
    pub apmixedsys: *mut regmap,
    pub infracfg: *mut regmap,
    pub irq_cnt: [c_int; MT8186_MEMIF_NUM],
    pub stf_positive_gain_db: c_int,
    pub pm_runtime_bypass_reg_ctl: c_int,
    pub sgen_mode: c_int,
    pub sgen_rate: c_int,
    pub sgen_amplitude: c_int,
// xrun assert
    pub xrun_assert: [c_int; MT8186_MEMIF_NUM],
// dai
    pub dai_on: [bool; MT8186_DAI_NUM],
    pub dai_priv: [*mut c_void; MT8186_DAI_NUM],
// adda
    pub mtkaif_calibration_ok: bool,
    pub mtkaif_protocol: c_int,
    pub mtkaif_chosen_phase: [c_int; 4],
    pub mtkaif_phase_cycle: [c_int; 4],
    pub mtkaif_calibration_num_phase: c_int,
    pub mtkaif_dmic: c_int,
    pub mtkaif_looback0: c_int,
    pub mtkaif_looback1: c_int,
// mck
    pub mck_rate: [c_int; MT8186_MCK_NUM],
}

extern "C" {
    pub fn mt8186_dai_adda_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8186_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8186_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8186_dai_hw_gain_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8186_dai_src_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8186_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8186_dai_hostless_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8186_add_misc_control(component: *mut snd_soc_component) -> c_int;
}
