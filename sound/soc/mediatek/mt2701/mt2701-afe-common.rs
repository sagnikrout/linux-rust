//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt2701/mt2701-afe-common.h
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
// mt2701-afe-common.h  --  Mediatek 2701 audio driver definitions
//
// Copyright (c) 2016 MediaTek Inc.
// Author: Garlic Tseng <garlic.tseng@mediatek.com>
//

pub const MT2701_PLL_DOMAIN_0_RATE: c_int = 98304000;
pub const MT2701_PLL_DOMAIN_1_RATE: c_int = 90316800;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audio_base_clock {
    MT2701_INFRA_SYS_AUDIO,
    MT2701_TOP_AUD_MCLK_SRC0,
    MT2701_TOP_AUD_MCLK_SRC1,
    MT2701_TOP_AUD_A1SYS,
    MT2701_TOP_AUD_A2SYS,
    MT2701_AUDSYS_AFE,
    MT2701_AUDSYS_AFE_CONN,
    MT2701_AUDSYS_A1SYS,
    MT2701_AUDSYS_A2SYS,
    MT2701_BASE_CLK_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt2701_i2s_data {
    pub i2s_ctrl_reg: c_int,
    pub i2s_asrc_fs_shift: c_int,
    pub i2s_asrc_fs_mask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt2701_i2s_path {
    pub mclk_rate: c_int,
    pub on: [c_int; MTK_STREAM_NUM],
    pub occupied: [c_int; MTK_STREAM_NUM],
    pub i2s_data: [*const mt2701_i2s_data; MTK_STREAM_NUM],
    pub hop_ck: [*mut clk; MTK_STREAM_NUM],
    pub sel_ck: *mut clk,
    pub div_ck: *mut clk,
    pub mclk_ck: *mut clk,
    pub asrco_ck: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt2701_soc_variants {
    pub has_one_heart_mode: bool,
    pub i2s_num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt2701_afe_private {
    pub base_ck: [*mut clk; MT2701_BASE_CLK_NUM],
    pub mrgif_ck: *mut clk,
    pub hadds2pll_ck: *mut clk,
    pub audio_hdmi_ck: *mut clk,
    pub audio_spdf_ck: *mut clk,
    pub audio_apll_ck: *mut clk,
    pub mrg_enable: [bool; MTK_STREAM_NUM],
    pub soc: *const mt2701_soc_variants,
    pub i2s_path: [mt2701_i2s_path; ],
}
