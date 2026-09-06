//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8188/mt8188-afe-common.h
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
// mt8188-afe-common.h  --  MediaTek 8188 audio driver definitions
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
// Trevor Wu <trevor.wu@mediatek.com>
// Chun-Chia Chiu <chun-chia.chiu@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_dai_memif_irq_priv {
    pub asys_timing_sel: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtkaif_param {
    pub mtkaif_calibration_ok: bool,
    pub mtkaif_chosen_phase: [c_int; MT8188_MTKAIF_MISO_NUM],
    pub mtkaif_phase_cycle: [c_int; MT8188_MTKAIF_MISO_NUM],
    pub mtkaif_dmic_on: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8188_afe_private {
    pub clk: *mut clk,
    pub lookup: *mut clk_lookup,
    pub topckgen: *mut regmap,
    pub pm_runtime_bypass_reg_ctl: c_int,
    pub /: *mut *mut spinlock_t afe_ctrl_lock; / Lock for afe control,
    pub irq_priv: [mtk_dai_memif_irq_priv; MT8188_AFE_IRQ_NUM],
    pub mtkaif_params: mtkaif_param,
// dai
    pub dai_priv: [*mut c_void; MT8188_DAI_NUM],
}

extern "C" {
    pub fn mt8188_afe_fs_timing(rate: c_uint) -> c_int;
}
// dai register
extern "C" {
    pub fn mt8188_dai_adda_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8188_dai_dmic_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8188_dai_etdm_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8188_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int;
}

