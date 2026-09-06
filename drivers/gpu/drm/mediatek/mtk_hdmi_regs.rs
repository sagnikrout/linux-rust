//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mediatek/mtk_hdmi_regs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2014 MediaTek Inc.
// Author: Jie Qiu <jie.qiu@mediatek.com>
//
pub const GRL_INT_MASK: c_uint = 0x18;
pub const GRL_IFM_PORT: c_uint = 0x188;
pub const GRL_CH_SWAP: c_uint = 0x198;

pub const GRL_I2S_C_STA0: c_uint = 0x140;
pub const GRL_I2S_C_STA1: c_uint = 0x144;
pub const GRL_I2S_C_STA2: c_uint = 0x148;
pub const GRL_I2S_C_STA3: c_uint = 0x14C;
pub const GRL_I2S_C_STA4: c_uint = 0x150;
pub const GRL_I2S_UV: c_uint = 0x154;

pub const I2S_UV_CH_EN_MASK: c_uint = 0x3c;

pub const GRL_ACP_ISRC_CTRL: c_uint = 0x158;

pub const GRL_CTS_CTRL: c_uint = 0x160;

pub const GRL_INT: c_uint = 0x14;

pub const GRL_INT_MASK: c_uint = 0x18;
pub const GRL_CTRL: c_uint = 0x1C;

pub const GRL_STATUS: c_uint = 0x20;

pub const GRL_DIVN: c_uint = 0x170;

pub const GRL_AUDIO_CFG: c_uint = 0x17C;

pub const GRL_NCTS: c_uint = 0x184;
pub const GRL_CH_SW0: c_uint = 0x18C;
pub const GRL_CH_SW1: c_uint = 0x190;
pub const GRL_CH_SW2: c_uint = 0x194;

pub const GRL_INFOFRM_VER: c_uint = 0x19C;
pub const GRL_INFOFRM_TYPE: c_uint = 0x1A0;
pub const GRL_INFOFRM_LNG: c_uint = 0x1A4;
pub const GRL_MIX_CTRL: c_uint = 0x1B4;

pub const GRL_AOUT_CFG: c_uint = 0x1C4;
pub const AOUT_BNUM_SEL_MASK: c_uint = 0x03;
pub const AOUT_24BIT: c_uint = 0x00;
pub const AOUT_20BIT: c_uint = 0x02;
pub const AOUT_16BIT: c_uint = 0x03;

pub const GRL_SHIFT_L1: c_uint = 0x1C0;
pub const GRL_SHIFT_R2: c_uint = 0x1B0;

pub const GRL_CFG0: c_uint = 0x24;
pub const CFG0_I2S_MODE_MASK: c_uint = 0x3;
pub const CFG0_I2S_MODE_RTJ: c_uint = 0x1;
pub const CFG0_I2S_MODE_LTJ: c_uint = 0x0;
pub const CFG0_I2S_MODE_I2S: c_uint = 0x2;
pub const CFG0_W_LENGTH_MASK: c_uint = 0x30;
pub const CFG0_W_LENGTH_24BIT: c_uint = 0x00;
pub const CFG0_W_LENGTH_16BIT: c_uint = 0x10;
pub const GRL_CFG1: c_uint = 0x28;

pub const GRL_CFG2: c_uint = 0x2c;

pub const GRL_CFG3: c_uint = 0x30;
pub const CFG3_AES_KEY_INDEX_MASK: c_uint = 0x3f;

pub const GRL_CFG4: c_uint = 0x34;

pub const GRL_CFG5: c_uint = 0x38;
pub const CFG5_CD_RATIO_MASK: c_uint = 0x8F;

pub const DUMMY_304: c_uint = 0x304;

pub const GRL_L_STATUS_0: c_uint = 0x200;
pub const GRL_L_STATUS_1: c_uint = 0x204;
pub const GRL_L_STATUS_2: c_uint = 0x208;
pub const GRL_L_STATUS_3: c_uint = 0x20c;
pub const GRL_L_STATUS_4: c_uint = 0x210;
pub const GRL_L_STATUS_5: c_uint = 0x214;
pub const GRL_L_STATUS_6: c_uint = 0x218;
pub const GRL_L_STATUS_7: c_uint = 0x21c;
pub const GRL_L_STATUS_8: c_uint = 0x220;
pub const GRL_L_STATUS_9: c_uint = 0x224;
pub const GRL_L_STATUS_10: c_uint = 0x228;
pub const GRL_L_STATUS_11: c_uint = 0x22c;
pub const GRL_L_STATUS_12: c_uint = 0x230;
pub const GRL_L_STATUS_13: c_uint = 0x234;
pub const GRL_L_STATUS_14: c_uint = 0x238;
pub const GRL_L_STATUS_15: c_uint = 0x23c;
pub const GRL_L_STATUS_16: c_uint = 0x240;
pub const GRL_L_STATUS_17: c_uint = 0x244;
pub const GRL_L_STATUS_18: c_uint = 0x248;
pub const GRL_L_STATUS_19: c_uint = 0x24c;
pub const GRL_L_STATUS_20: c_uint = 0x250;
pub const GRL_L_STATUS_21: c_uint = 0x254;
pub const GRL_L_STATUS_22: c_uint = 0x258;
pub const GRL_L_STATUS_23: c_uint = 0x25c;
pub const GRL_R_STATUS_0: c_uint = 0x260;
pub const GRL_R_STATUS_1: c_uint = 0x264;
pub const GRL_R_STATUS_2: c_uint = 0x268;
pub const GRL_R_STATUS_3: c_uint = 0x26c;
pub const GRL_R_STATUS_4: c_uint = 0x270;
pub const GRL_R_STATUS_5: c_uint = 0x274;
pub const GRL_R_STATUS_6: c_uint = 0x278;
pub const GRL_R_STATUS_7: c_uint = 0x27c;
pub const GRL_R_STATUS_8: c_uint = 0x280;
pub const GRL_R_STATUS_9: c_uint = 0x284;
pub const GRL_R_STATUS_10: c_uint = 0x288;
pub const GRL_R_STATUS_11: c_uint = 0x28c;
pub const GRL_R_STATUS_12: c_uint = 0x290;
pub const GRL_R_STATUS_13: c_uint = 0x294;
pub const GRL_R_STATUS_14: c_uint = 0x298;
pub const GRL_R_STATUS_15: c_uint = 0x29c;
pub const GRL_R_STATUS_16: c_uint = 0x2a0;
pub const GRL_R_STATUS_17: c_uint = 0x2a4;
pub const GRL_R_STATUS_18: c_uint = 0x2a8;
pub const GRL_R_STATUS_19: c_uint = 0x2ac;
pub const GRL_R_STATUS_20: c_uint = 0x2b0;
pub const GRL_R_STATUS_21: c_uint = 0x2b4;
pub const GRL_R_STATUS_22: c_uint = 0x2b8;
pub const GRL_R_STATUS_23: c_uint = 0x2bc;
pub const GRL_ABIST_CTRL0: c_uint = 0x2D4;
pub const GRL_ABIST_CTRL1: c_uint = 0x2D8;

pub const VIDEO_CFG_0: c_uint = 0x380;
pub const VIDEO_CFG_1: c_uint = 0x384;
pub const VIDEO_CFG_2: c_uint = 0x388;
pub const VIDEO_CFG_3: c_uint = 0x38c;
pub const VIDEO_CFG_4: c_uint = 0x390;

pub const HDMI_SYS_CFG1C: c_uint = 0x000;

pub const HDMI_SYS_CFG20: c_uint = 0x004;

pub const MTK_SIP_SET_AUTHORIZED_SECURE_REG: c_uint = 0x82000001;
