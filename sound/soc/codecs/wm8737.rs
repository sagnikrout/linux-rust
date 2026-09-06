//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8737.h
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
// wm8737.c  --  WM8523 ALSA SoC Audio driver
//
// Copyright 2010 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// Register values.
//
pub const WM8737_LEFT_PGA_VOLUME: c_uint = 0x00;
pub const WM8737_RIGHT_PGA_VOLUME: c_uint = 0x01;
pub const WM8737_AUDIO_PATH_L: c_uint = 0x02;
pub const WM8737_AUDIO_PATH_R: c_uint = 0x03;
pub const WM8737_3D_ENHANCE: c_uint = 0x04;
pub const WM8737_ADC_CONTROL: c_uint = 0x05;
pub const WM8737_POWER_MANAGEMENT: c_uint = 0x06;
pub const WM8737_AUDIO_FORMAT: c_uint = 0x07;
pub const WM8737_CLOCKING: c_uint = 0x08;
pub const WM8737_MIC_PREAMP_CONTROL: c_uint = 0x09;
pub const WM8737_MISC_BIAS_CONTROL: c_uint = 0x0A;
pub const WM8737_NOISE_GATE: c_uint = 0x0B;
pub const WM8737_ALC1: c_uint = 0x0C;
pub const WM8737_ALC2: c_uint = 0x0D;
pub const WM8737_ALC3: c_uint = 0x0E;
pub const WM8737_RESET: c_uint = 0x0F;
pub const WM8737_REGISTER_COUNT: c_int = 16;
pub const WM8737_MAX_REGISTER: c_uint = 0x0F;
//
// Field Definitions.
//
// R0 (0x00) - Left PGA volume
//
pub const WM8737_LVU: c_uint = 0x0100  /* LVU */;
pub const WM8737_LVU_MASK: c_uint = 0x0100  /* LVU */;

pub const WM8737_LINVOL_MASK: c_uint = 0x00FF  /* LINVOL - [7:0] */;

//
// R1 (0x01) - Right PGA volume
//
pub const WM8737_RVU: c_uint = 0x0100  /* RVU */;
pub const WM8737_RVU_MASK: c_uint = 0x0100  /* RVU */;

pub const WM8737_RINVOL_MASK: c_uint = 0x00FF  /* RINVOL - [7:0] */;

//
// R2 (0x02) - AUDIO path L
//
pub const WM8737_LINSEL_MASK: c_uint = 0x0180  /* LINSEL - [8:7] */;

pub const WM8737_LMICBOOST_MASK: c_uint = 0x0060  /* LMICBOOST - [6:5] */;

pub const WM8737_LMBE: c_uint = 0x0010  /* LMBE */;
pub const WM8737_LMBE_MASK: c_uint = 0x0010  /* LMBE */;

pub const WM8737_LMZC: c_uint = 0x0008  /* LMZC */;
pub const WM8737_LMZC_MASK: c_uint = 0x0008  /* LMZC */;

pub const WM8737_LPZC: c_uint = 0x0004  /* LPZC */;
pub const WM8737_LPZC_MASK: c_uint = 0x0004  /* LPZC */;

pub const WM8737_LZCTO_MASK: c_uint = 0x0003  /* LZCTO - [1:0] */;

//
// R3 (0x03) - AUDIO path R
//
pub const WM8737_RINSEL_MASK: c_uint = 0x0180  /* RINSEL - [8:7] */;

pub const WM8737_RMICBOOST_MASK: c_uint = 0x0060  /* RMICBOOST - [6:5] */;

pub const WM8737_RMBE: c_uint = 0x0010  /* RMBE */;
pub const WM8737_RMBE_MASK: c_uint = 0x0010  /* RMBE */;

pub const WM8737_RMZC: c_uint = 0x0008  /* RMZC */;
pub const WM8737_RMZC_MASK: c_uint = 0x0008  /* RMZC */;

pub const WM8737_RPZC: c_uint = 0x0004  /* RPZC */;
pub const WM8737_RPZC_MASK: c_uint = 0x0004  /* RPZC */;

pub const WM8737_RZCTO_MASK: c_uint = 0x0003  /* RZCTO - [1:0] */;

//
// R4 (0x04) - 3D Enhance
//
pub const WM8737_DIV2: c_uint = 0x0080  /* DIV2 */;
pub const WM8737_DIV2_MASK: c_uint = 0x0080  /* DIV2 */;

pub const WM8737_3DLC: c_uint = 0x0040  /* 3DLC */;
pub const WM8737_3DLC_MASK: c_uint = 0x0040  /* 3DLC */;

pub const WM8737_3DUC: c_uint = 0x0020  /* 3DUC */;
pub const WM8737_3DUC_MASK: c_uint = 0x0020  /* 3DUC */;

pub const WM8737_3DDEPTH_MASK: c_uint = 0x001E  /* 3DDEPTH - [4:1] */;

pub const WM8737_3DE: c_uint = 0x0001  /* 3DE */;
pub const WM8737_3DE_MASK: c_uint = 0x0001  /* 3DE */;

//
// R5 (0x05) - ADC Control
//
pub const WM8737_MONOMIX_MASK: c_uint = 0x0180  /* MONOMIX - [8:7] */;

pub const WM8737_POLARITY_MASK: c_uint = 0x0060  /* POLARITY - [6:5] */;

pub const WM8737_HPOR: c_uint = 0x0010  /* HPOR */;
pub const WM8737_HPOR_MASK: c_uint = 0x0010  /* HPOR */;

pub const WM8737_LP: c_uint = 0x0004  /* LP */;
pub const WM8737_LP_MASK: c_uint = 0x0004  /* LP */;

pub const WM8737_MONOUT: c_uint = 0x0002  /* MONOUT */;
pub const WM8737_MONOUT_MASK: c_uint = 0x0002  /* MONOUT */;

pub const WM8737_ADCHPD: c_uint = 0x0001  /* ADCHPD */;
pub const WM8737_ADCHPD_MASK: c_uint = 0x0001  /* ADCHPD */;

//
// R6 (0x06) - Power Management
//
pub const WM8737_VMID: c_uint = 0x0100  /* VMID */;
pub const WM8737_VMID_MASK: c_uint = 0x0100  /* VMID */;

pub const WM8737_VREF: c_uint = 0x0080  /* VREF */;
pub const WM8737_VREF_MASK: c_uint = 0x0080  /* VREF */;

pub const WM8737_AI: c_uint = 0x0040  /* AI */;
pub const WM8737_AI_MASK: c_uint = 0x0040  /* AI */;

pub const WM8737_PGL: c_uint = 0x0020  /* PGL */;
pub const WM8737_PGL_MASK: c_uint = 0x0020  /* PGL */;

pub const WM8737_PGR: c_uint = 0x0010  /* PGR */;
pub const WM8737_PGR_MASK: c_uint = 0x0010  /* PGR */;

pub const WM8737_ADL: c_uint = 0x0008  /* ADL */;
pub const WM8737_ADL_MASK: c_uint = 0x0008  /* ADL */;

pub const WM8737_ADR: c_uint = 0x0004  /* ADR */;
pub const WM8737_ADR_MASK: c_uint = 0x0004  /* ADR */;

pub const WM8737_MICBIAS_MASK: c_uint = 0x0003  /* MICBIAS - [1:0] */;

//
// R7 (0x07) - Audio Format
//
pub const WM8737_SDODIS: c_uint = 0x0080  /* SDODIS */;
pub const WM8737_SDODIS_MASK: c_uint = 0x0080  /* SDODIS */;

pub const WM8737_MS: c_uint = 0x0040  /* MS */;
pub const WM8737_MS_MASK: c_uint = 0x0040  /* MS */;

pub const WM8737_LRP: c_uint = 0x0010  /* LRP */;
pub const WM8737_LRP_MASK: c_uint = 0x0010  /* LRP */;

pub const WM8737_WL_MASK: c_uint = 0x000C  /* WL - [3:2] */;

pub const WM8737_FORMAT_MASK: c_uint = 0x0003  /* FORMAT - [1:0] */;

//
// R8 (0x08) - Clocking
//
pub const WM8737_AUTODETECT: c_uint = 0x0080  /* AUTODETECT */;
pub const WM8737_AUTODETECT_MASK: c_uint = 0x0080  /* AUTODETECT */;

pub const WM8737_CLKDIV2: c_uint = 0x0040  /* CLKDIV2 */;
pub const WM8737_CLKDIV2_MASK: c_uint = 0x0040  /* CLKDIV2 */;

pub const WM8737_SR_MASK: c_uint = 0x003E  /* SR - [5:1] */;

pub const WM8737_USB_MODE: c_uint = 0x0001  /* USB MODE */;
pub const WM8737_USB_MODE_MASK: c_uint = 0x0001  /* USB MODE */;

//
// R9 (0x09) - MIC Preamp Control
//
pub const WM8737_RBYPEN: c_uint = 0x0008  /* RBYPEN */;
pub const WM8737_RBYPEN_MASK: c_uint = 0x0008  /* RBYPEN */;

pub const WM8737_LBYPEN: c_uint = 0x0004  /* LBYPEN */;
pub const WM8737_LBYPEN_MASK: c_uint = 0x0004  /* LBYPEN */;

pub const WM8737_MBCTRL_MASK: c_uint = 0x0003  /* MBCTRL - [1:0] */;

//
// R10 (0x0A) - Misc Bias Control
//
pub const WM8737_VMIDSEL_MASK: c_uint = 0x000C  /* VMIDSEL - [3:2] */;

pub const WM8737_LINPUT1_DC_BIAS_ENABLE: c_uint = 0x0002  /* LINPUT1 DC BIAS ENABLE */;
pub const WM8737_LINPUT1_DC_BIAS_ENABLE_MASK: c_uint = 0x0002  /* LINPUT1 DC BIAS ENABLE */;

pub const WM8737_RINPUT1_DC_BIAS_ENABLE: c_uint = 0x0001  /* RINPUT1 DC BIAS ENABLE */;
pub const WM8737_RINPUT1_DC_BIAS_ENABLE_MASK: c_uint = 0x0001  /* RINPUT1 DC BIAS ENABLE */;

//
// R11 (0x0B) - Noise Gate
//
pub const WM8737_NGTH_MASK: c_uint = 0x001C  /* NGTH - [4:2] */;

pub const WM8737_NGAT: c_uint = 0x0001  /* NGAT */;
pub const WM8737_NGAT_MASK: c_uint = 0x0001  /* NGAT */;

//
// R12 (0x0C) - ALC1
//
pub const WM8737_ALCSEL_MASK: c_uint = 0x0180  /* ALCSEL - [8:7] */;

pub const WM8737_MAX_GAIN_MASK: c_uint = 0x0070  /* MAX GAIN - [6:4] */;

pub const WM8737_ALCL_MASK: c_uint = 0x000F  /* ALCL - [3:0] */;

//
// R13 (0x0D) - ALC2
//
pub const WM8737_ALCZCE: c_uint = 0x0010  /* ALCZCE */;
pub const WM8737_ALCZCE_MASK: c_uint = 0x0010  /* ALCZCE */;

pub const WM8737_HLD_MASK: c_uint = 0x000F  /* HLD - [3:0] */;

//
// R14 (0x0E) - ALC3
//
pub const WM8737_DCY_MASK: c_uint = 0x00F0  /* DCY - [7:4] */;

pub const WM8737_ATK_MASK: c_uint = 0x000F  /* ATK - [3:0] */;

//
// R15 (0x0F) - Reset
//
pub const WM8737_RESET_MASK: c_uint = 0x01FF  /* RESET - [8:0] */;

