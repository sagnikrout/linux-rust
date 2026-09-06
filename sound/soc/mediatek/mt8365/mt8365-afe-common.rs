//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8365/mt8365-afe-common.h
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
// MediaTek 8365 audio driver common definitions
//
// Copyright (c) 2024 MediaTek Inc.
// Authors: Jia Zeng <jia.zeng@mediatek.com>
// Alexandre Mergnat <amergnat@baylibre.com>
//

//
// MT8365_AFE_MEMIF_SPDIF_OUT,
//
// MT8365_AFE_MEMIF_SPDIF_IN,
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt8365_afe_tdm_ch_start {
    AFE_TDM_CH_START_O28_O29 = 0,
    AFE_TDM_CH_START_O30_O31,
    AFE_TDM_CH_START_O32_O33,
    AFE_TDM_CH_START_O34_O35,
    AFE_TDM_CH_ZERO,
}

// MCLK
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8365_fe_dai_data {
    pub use_sram: bool,
    pub sram_phy_addr: c_uint,
    pub sram_vir_addr: *mut void __iomem,
    pub sram_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8365_be_dai_data {
    pub 1]: bool prepared[SNDRV_PCM_STREAM_LAST +,
    pub fmt_mode: c_uint,
}

pub const MT8365_CLK_26M: c_int = 26000000;
pub const MT8365_CLK_24M: c_int = 24000000;
pub const MT8365_CLK_22M: c_int = 22000000;
pub const MT8365_CM_UPDATA_CNT_SET: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt8365_cm_num {
    MT8365_CM1 = 0,
    MT8365_CM2,
    MT8365_CM_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt8365_cm2_mux_in {
    MT8365_FROM_GASRC1 = 1,
    MT8365_FROM_GASRC2,
    MT8365_FROM_TDM_ASRC,
    MT8365_CM_MUX_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm2_mux_conn_in {
    GENERAL2_ASRC_OUT_LCH = 0,
    GENERAL2_ASRC_OUT_RCH = 1,
    TDM_IN_CH0 = 2,
    TDM_IN_CH1 = 3,
    TDM_IN_CH2 = 4,
    TDM_IN_CH3 = 5,
    TDM_IN_CH4 = 6,
    TDM_IN_CH5 = 7,
    TDM_IN_CH6 = 8,
    TDM_IN_CH7 = 9,
    GENERAL1_ASRC_OUT_LCH = 10,
    GENERAL1_ASRC_OUT_RCH = 11,
    TDM_OUT_ASRC_CH0 = 12,
    TDM_OUT_ASRC_CH1 = 13,
    TDM_OUT_ASRC_CH2 = 14,
    TDM_OUT_ASRC_CH3 = 15,
    TDM_OUT_ASRC_CH4 = 16,
    TDM_OUT_ASRC_CH5 = 17,
    TDM_OUT_ASRC_CH6 = 18,
    TDM_OUT_ASRC_CH7 = 19
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8365_cm_ctrl_reg {
    pub con0: c_uint,
    pub con1: c_uint,
    pub con2: c_uint,
    pub con3: c_uint,
    pub con4: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8365_control_data {
    pub bypass_cm1: bool,
    pub bypass_cm2: bool,
    pub loopback_type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmic_input_mode {
    DMIC_MODE_3P25M = 0,
    DMIC_MODE_1P625M,
    DMIC_MODE_812P5K,
    DMIC_MODE_406P25K,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iir_mode {
    IIR_MODE0 = 0,
    IIR_MODE1,
    IIR_MODE2,
    IIR_MODE3,
    IIR_MODE4,
    IIR_MODE5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8365_gasrc_ctrl_reg {
    pub con0: c_uint,
    pub con2: c_uint,
    pub con3: c_uint,
    pub con4: c_uint,
    pub con5: c_uint,
    pub con6: c_uint,
    pub con9: c_uint,
    pub con10: c_uint,
    pub con12: c_uint,
    pub con13: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8365_gasrc_data {
    pub duplex: bool,
    pub tx_mode: bool,
    pub cali_on: bool,
    pub tdm_asrc_out_cm2: bool,
    pub iir_on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8365_afe_private {
    pub clocks: [*mut clk; MT8365_CLK_NUM],
    pub topckgen: *mut regmap,
    pub fe_data: [mt8365_fe_dai_data; MT8365_AFE_MEMIF_NUM],
    pub be_data: [mt8365_be_dai_data; MT8365_AFE_BACKEND_NUM],
    pub ctrl_data: mt8365_control_data,
    pub gasrc_data: [mt8365_gasrc_data; MT8365_TDM_ASRC_NUM],
    pub afe_on_ref_cnt: c_int,
    pub top_cg_ref_cnt: [c_int; MT8365_TOP_CG_NUM],
    pub afe_sram_vir_addr: *mut void __iomem,
    pub afe_sram_phy_addr: c_uint,
    pub afe_sram_size: c_uint,
// locks
    pub afe_ctrl_lock: spinlock_t,
    pub access*/: *mut *mut mutex afe_clk_mutex; / Protect & sync APLL TUNER registers,
    pub debugfs_dentry: [*mut dentry; MT8365_AFE_DEBUGFS_NUM],    pub apll_tuner_ref_cnt: [c_int; MT8365_AFE_APLL_NUM],
    pub tdm_out_mode: c_uint,
    pub cm2_mux_input: c_uint,
// dai
    pub dai_on: [bool; MT8365_AFE_BACKEND_END],
    pub dai_priv: [*mut c_void; MT8365_AFE_BACKEND_END],
}

//
// A = (26M / fs) * 64
// B = 8125 / A
// return = DEC2HEX(B * 2^23)
//
extern "C" {
    pub fn mt8365_afe_rate_supported(rate: c_uint, id: c_uint) -> bool;
}
extern "C" {
    pub fn mt8365_afe_channel_supported(channel: c_uint, id: c_uint) -> bool;
}
extern "C" {
    pub fn mt8365_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_fs_timing(rate: c_uint) -> c_int;
}
extern "C" {
    pub fn mt8365_afe_set_i2s_out_enable(afe: *mut mtk_base_afe, enable: bool);
}
extern "C" {
    pub fn mt8365_afe_set_i2s_out(afe: *mut mtk_base_afe, rate: c_uint, bit_width: c_int) -> c_int;
}
extern "C" {
    pub fn mt8365_dai_adda_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_dai_enable_adda_on(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_dai_disable_adda_on(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_dai_dmic_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int;
}
extern "C" {
    pub fn mt8365_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int;
}
