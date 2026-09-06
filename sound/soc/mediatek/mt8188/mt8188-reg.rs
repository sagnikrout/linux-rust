//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8188/mt8188-reg.h
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
// mt8188-reg.h  --  MediaTek 8188 audio driver reg definition
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
// Trevor Wu <trevor.wu@mediatek.com>
// Chun-Chia Chiu <chun-chia.chiu@mediatek.com>
//

// PWR1_ASM_CON1

// PWR1_ASM_CON2

// PWR1_ASM_CON3

// AUDIO_TOP_CON0

// AUDIO_TOP_CON1

// AUDIO_TOP_CON3

// AUDIO_TOP_CON4

// AUDIO_TOP_CON6

// AFE_GAINx_CON0

// AFE_GAINx_CON1

// AFE_GAINx_CON2

// AFE_GAINx_CON3

// AFE_GAINx_CUR

// ASYS_TOP_CON

// PWR2_TOP_CON0

// PWR2_TOP_CON1

pub const PWR2_TOP_CON1_DMIC_CKDIV_ON_SHIFT: c_int = 1;
// PCM_INTF_CON1

// PCM_INTF_CON2

// AFE_MPHONE_MULTIx_CON0

// AFE_MPHONE_MULTIx_CON1

// AFE_MPHONE_MULTIx_CON2

// AFE_AUD_PAD_TOP

// AFE_ADDA_MTKAIF_CFG0

// AFE_ADDA_MTKAIF_RX_CFG2

// AFE_ADDA_MTKAIF_SYNCWORD_CFG

// AFE_DMICx_UL_SRC_CON0

// DMIC_BYPASS_HW_GAIN

// DMIC_GAINx_CON0

pub const DMIC_GAIN_CON0_SAMPLE_PER_STEP_SHIFT: c_int = 8;

// DMIC_GAINx_CON1

// DMIC_GAINx_CON2

// DMIC_GAINx_CON3

// DMIC_GAINx_CUR

// ETDM_INx_AFIFO_CON

// ETDM_COWORK_CON0

pub const ETDM_OUT1_SLAVE_SEL_SHIFT: c_int = 20;

pub const ETDM_OUT1_SYNC_SEL_SHIFT: c_int = 16;
// ETDM_COWORK_CON1

pub const ETDM_IN1_SDATA_SEL_SHIFT: c_int = 20;

pub const ETDM_IN1_SDATA0_SEL_SHIFT: c_int = 16;

pub const ETDM_IN1_SYNC_SEL_SHIFT: c_int = 12;

pub const ETDM_IN1_SLAVE_SEL_SHIFT: c_int = 8;
// ETDM_COWORK_CON2

pub const ETDM_IN2_SYNC_SEL_SHIFT: c_int = 28;

pub const ETDM_IN2_SLAVE_SEL_SHIFT: c_int = 24;

pub const ETDM_OUT3_SLAVE_SEL_SHIFT: c_int = 20;

pub const ETDM_OUT3_SYNC_SEL_SHIFT: c_int = 16;

pub const ETDM_OUT2_SLAVE_SEL_SHIFT: c_int = 8;

pub const ETDM_OUT2_SYNC_SEL_SHIFT: c_int = 4;
// ETDM_COWORK_CON3

pub const ETDM_IN2_SDATA_SEL_SHIFT: c_int = 4;

pub const ETDM_IN2_SDATA0_SEL_SHIFT: c_int = 0;
// ETDM_x_CONx

pub const ETDM_CON0_EN_SHIFT: c_int = 0;

pub const ETDM_IN_CON2_CLOCK_SHIFT: c_int = 10;

pub const ETDM_OUT_CON4_CLOCK_SHIFT: c_int = 6;

// GASRC_TIMING_CON0

// GASRC_TIMING_CON1

// GASRC_TIMING_CON2

// GASRC_TIMING_CON3

// GASRC_TIMING_CON4

// GASRC_TIMING_CON5

// AFE_DPTX_CON

pub const AFE_DPTX_CON_ON_SHIFT: c_int = 0;
// AFE_ADDA_DL_SRC2_CON0

pub const DL_2_GAIN_ON_CTL_PRE_SHIFT: c_int = 1;
pub const DL_2_SRC_ON_TMP_CTRL_PRE_SHIFT: c_int = 0;
// AFE_ADDA_DL_SRC2_CON1

pub const DL_2_GAIN_CTL_PRE_SHIFT: c_int = 16;
// AFE_ADDA_TOP_CON0

// AFE_ADDA_UL_DL_CON0
pub const ADDA_AFE_ON_SHIFT: c_int = 0;
// AFE_ADDA_DL_SDM_DCCOMP_CON

// AFE_ADDA_UL_SRC_CON0

pub const UL_SRC_ON_TMP_CTL_SHIFT: c_int = 0;
// AFE_GASRCx_NEW_CON0

// AFE_GASRCx_NEW_CON5

// AFE_GASRCx_NEW_CON6

// AFE_GASRCx_NEW_CON7

