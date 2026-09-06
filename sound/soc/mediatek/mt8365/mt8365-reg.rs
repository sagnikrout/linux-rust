//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8365/mt8365-reg.h
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
// MediaTek 8365 audio driver reg definition
//
// Copyright (c) 2024 MediaTek Inc.
// Authors: Jia Zeng <jia.zeng@mediatek.com>
// Alexandre Mergnat <amergnat@baylibre.com>
//

pub const AFE_IRQ_STATUS_BITS: c_uint = 0x3ff;
// AUDIO_TOP_CON0 (0x0000)

// AUDIO_TOP_CON1 (0x0004)

// AUDIO_TOP_CON3 (0x000C)

// AFE_I2S_CON (0x0018)

// AFE_ASRC_2CH_CON0

// CON2

// CON5

// AFE_I2S_CON1 (0x0034)

// AFE_I2S_CON2 (0x0038)

// AFE_I2S_CON3 (0x004C)

// AFE_ADDA_DL_SRC2_CON0 (0x0108)

// AFE_ADDA_UL_SRC_CON0 (0x0114)

// AFE_ADDA_UL_DL_CON0

// AFE_APLL_TUNER_CFG (0x03f0)

// AFE_APLL_TUNER_CFG1 (0x03f4)

// PCM_INTF_CON1 (0x0550)

// AFE_DMIC0_UL_SRC_CON0 (0x05b4)
// AFE_DMIC1_UL_SRC_CON0 (0x0620)
// AFE_DMIC2_UL_SRC_CON0 (0x0780)
// AFE_DMIC3_UL_SRC_CON0 (0x07ec)
//

// AFE_CONN_24BIT (0x0AA4)

// AFE_HD_ENGEN_ENABLE

// AFE_GAIN1_CON0 (0x0410)

// AFE_GAIN1_CON1 (0x0414)

// AFE_GAIN1_CUR (0x0B78)

// AFE_CM1_CON0 (0x0e50)
// AFE_CM2_CON0 (0x0e60)

// AFE_CM2_CONN*

// AFE_CM1_CON*

