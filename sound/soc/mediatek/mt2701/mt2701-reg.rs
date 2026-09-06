//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt2701/mt2701-reg.h
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
// mt2701-reg.h  --  Mediatek 2701 audio driver reg definition
//
// Copyright (c) 2016 MediaTek Inc.
// Author: Garlic Tseng <garlic.tseng@mediatek.com>
//
pub const AUDIO_TOP_CON0: c_uint = 0x0000;
pub const AUDIO_TOP_CON3: c_uint = 0x000c;
pub const AUDIO_TOP_CON4: c_uint = 0x0010;
pub const AUDIO_TOP_CON5: c_uint = 0x0014;
pub const AFE_DAIBT_CON0: c_uint = 0x001c;
pub const AFE_MRGIF_CON: c_uint = 0x003c;
pub const AFE_HDMI_OUT_CON0: c_uint = 0x0370;
pub const AFE_HDMI_OUT_BASE: c_uint = 0x0374;
pub const AFE_HDMI_OUT_CUR: c_uint = 0x0378;
pub const AFE_HDMI_OUT_END: c_uint = 0x037c;
pub const AFE_HDMI_CONN0: c_uint = 0x0390;
pub const AFE_8CH_I2S_OUT_CON: c_uint = 0x0394;
pub const ASMI_TIMING_CON1: c_uint = 0x0100;
pub const ASMO_TIMING_CON1: c_uint = 0x0104;
pub const PWR1_ASM_CON1: c_uint = 0x0108;
pub const ASYS_TOP_CON: c_uint = 0x0600;
pub const ASYS_I2SIN1_CON: c_uint = 0x0604;
pub const ASYS_I2SIN2_CON: c_uint = 0x0608;
pub const ASYS_I2SIN3_CON: c_uint = 0x060c;
pub const ASYS_I2SIN4_CON: c_uint = 0x0610;
pub const ASYS_I2SIN5_CON: c_uint = 0x0614;
pub const ASYS_I2SO1_CON: c_uint = 0x061C;
pub const ASYS_I2SO2_CON: c_uint = 0x0620;
pub const ASYS_I2SO3_CON: c_uint = 0x0624;
pub const ASYS_I2SO4_CON: c_uint = 0x0628;
pub const ASYS_I2SO5_CON: c_uint = 0x062c;
pub const PWR2_TOP_CON: c_uint = 0x0634;
pub const AFE_CONN0: c_uint = 0x06c0;
pub const AFE_CONN1: c_uint = 0x06c4;
pub const AFE_CONN2: c_uint = 0x06c8;
pub const AFE_CONN3: c_uint = 0x06cc;
pub const AFE_CONN14: c_uint = 0x06f8;
pub const AFE_CONN15: c_uint = 0x06fc;
pub const AFE_CONN16: c_uint = 0x0700;
pub const AFE_CONN17: c_uint = 0x0704;
pub const AFE_CONN18: c_uint = 0x0708;
pub const AFE_CONN19: c_uint = 0x070c;
pub const AFE_CONN20: c_uint = 0x0710;
pub const AFE_CONN21: c_uint = 0x0714;
pub const AFE_CONN22: c_uint = 0x0718;
pub const AFE_CONN23: c_uint = 0x071c;
pub const AFE_CONN24: c_uint = 0x0720;
pub const AFE_CONN41: c_uint = 0x0764;
pub const ASYS_IRQ1_CON: c_uint = 0x0780;
pub const ASYS_IRQ2_CON: c_uint = 0x0784;
pub const ASYS_IRQ3_CON: c_uint = 0x0788;
pub const ASYS_IRQ_CLR: c_uint = 0x07c0;
pub const ASYS_IRQ_STATUS: c_uint = 0x07c4;
pub const PWR2_ASM_CON1: c_uint = 0x1070;
pub const AFE_DAC_CON0: c_uint = 0x1200;
pub const AFE_DAC_CON1: c_uint = 0x1204;
pub const AFE_DAC_CON2: c_uint = 0x1208;
pub const AFE_DAC_CON3: c_uint = 0x120c;
pub const AFE_DAC_CON4: c_uint = 0x1210;
pub const AFE_MEMIF_HD_CON1: c_uint = 0x121c;
pub const AFE_MEMIF_PBUF_SIZE: c_uint = 0x1238;
pub const AFE_MEMIF_HD_CON0: c_uint = 0x123c;
pub const AFE_DL1_BASE: c_uint = 0x1240;
pub const AFE_DL1_CUR: c_uint = 0x1244;
pub const AFE_DL2_BASE: c_uint = 0x1250;
pub const AFE_DL2_CUR: c_uint = 0x1254;
pub const AFE_DL3_BASE: c_uint = 0x1260;
pub const AFE_DL3_CUR: c_uint = 0x1264;
pub const AFE_DL4_BASE: c_uint = 0x1270;
pub const AFE_DL4_CUR: c_uint = 0x1274;
pub const AFE_DL5_BASE: c_uint = 0x1280;
pub const AFE_DL5_CUR: c_uint = 0x1284;
pub const AFE_DLMCH_BASE: c_uint = 0x12a0;
pub const AFE_DLMCH_CUR: c_uint = 0x12a4;
pub const AFE_ARB1_BASE: c_uint = 0x12b0;
pub const AFE_ARB1_CUR: c_uint = 0x12b4;
pub const AFE_VUL_BASE: c_uint = 0x1300;
pub const AFE_VUL_CUR: c_uint = 0x130c;
pub const AFE_UL2_BASE: c_uint = 0x1310;
pub const AFE_UL2_END: c_uint = 0x1318;
pub const AFE_UL2_CUR: c_uint = 0x131c;
pub const AFE_UL3_BASE: c_uint = 0x1320;
pub const AFE_UL3_END: c_uint = 0x1328;
pub const AFE_UL3_CUR: c_uint = 0x132c;
pub const AFE_UL4_BASE: c_uint = 0x1330;
pub const AFE_UL4_END: c_uint = 0x1338;
pub const AFE_UL4_CUR: c_uint = 0x133c;
pub const AFE_UL5_BASE: c_uint = 0x1340;
pub const AFE_UL5_END: c_uint = 0x1348;
pub const AFE_UL5_CUR: c_uint = 0x134c;
pub const AFE_DAI_BASE: c_uint = 0x1370;
pub const AFE_DAI_CUR: c_uint = 0x137c;
// AFE_DAIBT_CON0 (0x001c)

// PWR1_ASM_CON1 (0x0108)

// AFE_MRGIF_CON (0x003c)

// ASYS_TOP_CON (0x0600)

// PWR2_ASM_CON1 (0x1070)

// AFE_DAC_CON0 (0x1200)

// AFE_MEMIF_PBUF_SIZE (0x1238)

// AUDIO_TOP_CON3 (0x000c) -- HDMI BCK divider

// AFE_HDMI_OUT_CON0 (0x0370)

// AFE_8CH_I2S_OUT_CON (0x0394) -- on-SoC 8-channel I2S that feeds HDMI TX

// I2S in/out register bit control

// 0:EIAJ 1:I2S

