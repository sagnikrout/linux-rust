//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt7986/mt7986-reg.h
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
// mt7986-reg.h  --  MediaTek 7986 audio driver reg definition
//
// Copyright (c) 2023 MediaTek Inc.
// Authors: Vic Wu <vic.wu@mediatek.com>
// Maso Huang <maso.huang@mediatek.com>
//
pub const AUDIO_TOP_CON2: c_uint = 0x0008;
pub const AUDIO_TOP_CON4: c_uint = 0x0010;
pub const AUDIO_ENGEN_CON0: c_uint = 0x0014;
pub const AFE_IRQ_MCU_EN: c_uint = 0x0100;
pub const AFE_IRQ_MCU_STATUS: c_uint = 0x0120;
pub const AFE_IRQ_MCU_CLR: c_uint = 0x0128;
pub const AFE_IRQ0_MCU_CFG0: c_uint = 0x0140;
pub const AFE_IRQ0_MCU_CFG1: c_uint = 0x0144;
pub const AFE_IRQ1_MCU_CFG0: c_uint = 0x0148;
pub const AFE_IRQ1_MCU_CFG1: c_uint = 0x014c;
pub const AFE_IRQ2_MCU_CFG0: c_uint = 0x0150;
pub const AFE_IRQ2_MCU_CFG1: c_uint = 0x0154;
pub const ETDM_IN5_CON0: c_uint = 0x13f0;
pub const ETDM_IN5_CON1: c_uint = 0x13f4;
pub const ETDM_IN5_CON2: c_uint = 0x13f8;
pub const ETDM_IN5_CON3: c_uint = 0x13fc;
pub const ETDM_IN5_CON4: c_uint = 0x1400;
pub const ETDM_OUT5_CON0: c_uint = 0x1570;
pub const ETDM_OUT5_CON4: c_uint = 0x1580;
pub const ETDM_OUT5_CON5: c_uint = 0x1584;
pub const ETDM_4_7_COWORK_CON0: c_uint = 0x15e0;
pub const ETDM_4_7_COWORK_CON1: c_uint = 0x15e4;
pub const AFE_CONN018_1: c_uint = 0x1b44;
pub const AFE_CONN018_4: c_uint = 0x1b50;
pub const AFE_CONN019_1: c_uint = 0x1b64;
pub const AFE_CONN019_4: c_uint = 0x1b70;
pub const AFE_CONN124_1: c_uint = 0x2884;
pub const AFE_CONN124_4: c_uint = 0x2890;
pub const AFE_CONN125_1: c_uint = 0x28a4;
pub const AFE_CONN125_4: c_uint = 0x28b0;
pub const AFE_CONN_RS_0: c_uint = 0x3920;
pub const AFE_CONN_RS_3: c_uint = 0x392c;
pub const AFE_CONN_16BIT_0: c_uint = 0x3960;
pub const AFE_CONN_16BIT_3: c_uint = 0x396c;
pub const AFE_CONN_24BIT_0: c_uint = 0x3980;
pub const AFE_CONN_24BIT_3: c_uint = 0x398c;
pub const AFE_MEMIF_CON0: c_uint = 0x3d98;
pub const AFE_MEMIF_RD_MON: c_uint = 0x3da0;
pub const AFE_MEMIF_WR_MON: c_uint = 0x3da4;
pub const AFE_DL0_BASE_MSB: c_uint = 0x3e40;
pub const AFE_DL0_BASE: c_uint = 0x3e44;
pub const AFE_DL0_CUR_MSB: c_uint = 0x3e48;
pub const AFE_DL0_CUR: c_uint = 0x3e4c;
pub const AFE_DL0_END_MSB: c_uint = 0x3e50;
pub const AFE_DL0_END: c_uint = 0x3e54;
pub const AFE_DL0_RCH_MON: c_uint = 0x3e58;
pub const AFE_DL0_LCH_MON: c_uint = 0x3e5c;
pub const AFE_DL0_CON0: c_uint = 0x3e60;
pub const AFE_VUL0_BASE_MSB: c_uint = 0x4220;
pub const AFE_VUL0_BASE: c_uint = 0x4224;
pub const AFE_VUL0_CUR_MSB: c_uint = 0x4228;
pub const AFE_VUL0_CUR: c_uint = 0x422c;
pub const AFE_VUL0_END_MSB: c_uint = 0x4230;
pub const AFE_VUL0_END: c_uint = 0x4234;
pub const AFE_VUL0_CON0: c_uint = 0x4238;

pub const AFE_IRQ_STATUS_BITS: c_uint = 0x7;
pub const AFE_IRQ_CNT_SHIFT: c_int = 0;
pub const AFE_IRQ_CNT_MASK: c_uint = 0xffffff;
// AUDIO_TOP_CON2

// AUDIO_TOP_CON4

// AUDIO_ENGEN_CON0

// AFE_DL0_CON0
pub const DL0_ON_SFT: c_int = 28;
pub const DL0_ON_MASK: c_uint = 0x1;

pub const DL0_MINLEN_SFT: c_int = 20;
pub const DL0_MINLEN_MASK: c_uint = 0xf;

pub const DL0_MODE_SFT: c_int = 8;
pub const DL0_MODE_MASK: c_uint = 0x1f;

pub const DL0_PBUF_SIZE_SFT: c_int = 5;
pub const DL0_PBUF_SIZE_MASK: c_uint = 0x3;

pub const DL0_MONO_SFT: c_int = 4;
pub const DL0_MONO_MASK: c_uint = 0x1;

pub const DL0_HALIGN_SFT: c_int = 2;
pub const DL0_HALIGN_MASK: c_uint = 0x1;

pub const DL0_HD_MODE_SFT: c_int = 0;
pub const DL0_HD_MODE_MASK: c_uint = 0x3;

// AFE_VUL0_CON0
pub const VUL0_ON_SFT: c_int = 28;
pub const VUL0_ON_MASK: c_uint = 0x1;

pub const VUL0_MODE_SFT: c_int = 8;
pub const VUL0_MODE_MASK: c_uint = 0x1f;

pub const VUL0_MONO_SFT: c_int = 4;
pub const VUL0_MONO_MASK: c_uint = 0x1;

pub const VUL0_HALIGN_SFT: c_int = 2;
pub const VUL0_HALIGN_MASK: c_uint = 0x1;

pub const VUL0_HD_MODE_SFT: c_int = 0;
pub const VUL0_HD_MODE_MASK: c_uint = 0x3;

// AFE_IRQ_MCU_CON
pub const IRQ_MCU_MODE_SFT: c_int = 4;
pub const IRQ_MCU_MODE_MASK: c_uint = 0x1f;

pub const IRQ_MCU_ON_SFT: c_int = 0;
pub const IRQ_MCU_ON_MASK: c_uint = 0x1;

pub const IRQ0_MCU_CLR_SFT: c_int = 0;
pub const IRQ0_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ1_MCU_CLR_SFT: c_int = 1;
pub const IRQ1_MCU_CLR_MASK: c_uint = 0x1;

pub const IRQ2_MCU_CLR_SFT: c_int = 2;
pub const IRQ2_MCU_CLR_MASK: c_uint = 0x1;

// ETDM_IN5_CON2

pub const IN_CLK_SRC_SFT: c_int = 10;

// ETDM_IN5_CON3

pub const IN_SEL_FS_SFT: c_int = 26;

// ETDM_IN5_CON4

pub const IN_RELATCH_SFT: c_int = 20;

// ETDM_IN5_CON0 & ETDM_OUT5_CON0

// ETDM_OUT5_CON4

pub const OUT_RELATCH_SFT: c_int = 24;

pub const OUT_CLK_SRC_SFT: c_int = 6;

pub const OUT_SEL_FS_SFT: c_int = 0;

// ETDM_OUT5_CON5

// ETDM_4_7_COWORK_CON0

pub const OUT_SEL_SFT: c_int = 12;

