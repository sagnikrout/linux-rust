//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/adv7183_regs.h
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
// adv7183 - Analog Devices ADV7183 video decoder registers
//
// Copyright (c) 2011 Analog Devices Inc.
//
pub const ADV7183_IN_CTRL: c_uint = 0x00 /* Input control */;
pub const ADV7183_VD_SEL: c_uint = 0x01 /* Video selection */;
pub const ADV7183_OUT_CTRL: c_uint = 0x03 /* Output control */;
pub const ADV7183_EXT_OUT_CTRL: c_uint = 0x04 /* Extended output control */;
pub const ADV7183_AUTO_DET_EN: c_uint = 0x07 /* Autodetect enable */;
pub const ADV7183_CONTRAST: c_uint = 0x08 /* Contrast */;
pub const ADV7183_BRIGHTNESS: c_uint = 0x0A /* Brightness */;
pub const ADV7183_HUE: c_uint = 0x0B /* Hue */;
pub const ADV7183_DEF_Y: c_uint = 0x0C /* Default value Y */;
pub const ADV7183_DEF_C: c_uint = 0x0D /* Default value C */;
pub const ADV7183_ADI_CTRL: c_uint = 0x0E /* ADI control */;
pub const ADV7183_POW_MANAGE: c_uint = 0x0F /* Power Management */;
pub const ADV7183_STATUS_1: c_uint = 0x10 /* Status 1 */;
pub const ADV7183_IDENT: c_uint = 0x11 /* Ident */;
pub const ADV7183_STATUS_2: c_uint = 0x12 /* Status 2 */;
pub const ADV7183_STATUS_3: c_uint = 0x13 /* Status 3 */;
pub const ADV7183_ANAL_CLAMP_CTRL: c_uint = 0x14 /* Analog clamp control */;
pub const ADV7183_DIGI_CLAMP_CTRL_1: c_uint = 0x15 /* Digital clamp control 1 */;
pub const ADV7183_SHAP_FILT_CTRL: c_uint = 0x17 /* Shaping filter control */;
pub const ADV7183_SHAP_FILT_CTRL_2: c_uint = 0x18 /* Shaping filter control 2 */;
pub const ADV7183_COMB_FILT_CTRL: c_uint = 0x19 /* Comb filter control */;
pub const ADV7183_ADI_CTRL_2: c_uint = 0x1D /* ADI control 2 */;
pub const ADV7183_PIX_DELAY_CTRL: c_uint = 0x27 /* Pixel delay control */;
pub const ADV7183_MISC_GAIN_CTRL: c_uint = 0x2B /* Misc gain control */;
pub const ADV7183_AGC_MODE_CTRL: c_uint = 0x2C /* AGC mode control */;
pub const ADV7183_CHRO_GAIN_CTRL_1: c_uint = 0x2D /* Chroma gain control 1 */;
pub const ADV7183_CHRO_GAIN_CTRL_2: c_uint = 0x2E /* Chroma gain control 2 */;
pub const ADV7183_LUMA_GAIN_CTRL_1: c_uint = 0x2F /* Luma gain control 1 */;
pub const ADV7183_LUMA_GAIN_CTRL_2: c_uint = 0x30 /* Luma gain control 2 */;
pub const ADV7183_VS_FIELD_CTRL_1: c_uint = 0x31 /* Vsync field control 1 */;
pub const ADV7183_VS_FIELD_CTRL_2: c_uint = 0x32 /* Vsync field control 2 */;
pub const ADV7183_VS_FIELD_CTRL_3: c_uint = 0x33 /* Vsync field control 3 */;
pub const ADV7183_HS_POS_CTRL_1: c_uint = 0x34 /* Hsync position control 1 */;
pub const ADV7183_HS_POS_CTRL_2: c_uint = 0x35 /* Hsync position control 2 */;
pub const ADV7183_HS_POS_CTRL_3: c_uint = 0x36 /* Hsync position control 3 */;
pub const ADV7183_POLARITY: c_uint = 0x37 /* Polarity */;
pub const ADV7183_NTSC_COMB_CTRL: c_uint = 0x38 /* NTSC comb control */;
pub const ADV7183_PAL_COMB_CTRL: c_uint = 0x39 /* PAL comb control */;
pub const ADV7183_ADC_CTRL: c_uint = 0x3A /* ADC control */;
pub const ADV7183_MAN_WIN_CTRL: c_uint = 0x3D /* Manual window control */;
pub const ADV7183_RESAMPLE_CTRL: c_uint = 0x41 /* Resample control */;
pub const ADV7183_GEMSTAR_CTRL_1: c_uint = 0x48 /* Gemstar ctrl 1 */;
pub const ADV7183_GEMSTAR_CTRL_2: c_uint = 0x49 /* Gemstar ctrl 2 */;
pub const ADV7183_GEMSTAR_CTRL_3: c_uint = 0x4A /* Gemstar ctrl 3 */;
pub const ADV7183_GEMSTAR_CTRL_4: c_uint = 0x4B /* Gemstar ctrl 4 */;
pub const ADV7183_GEMSTAR_CTRL_5: c_uint = 0x4C /* Gemstar ctrl 5 */;
pub const ADV7183_CTI_DNR_CTRL_1: c_uint = 0x4D /* CTI DNR ctrl 1 */;
pub const ADV7183_CTI_DNR_CTRL_2: c_uint = 0x4E /* CTI DNR ctrl 2 */;
pub const ADV7183_CTI_DNR_CTRL_4: c_uint = 0x50 /* CTI DNR ctrl 4 */;
pub const ADV7183_LOCK_CNT: c_uint = 0x51 /* Lock count */;
pub const ADV7183_FREE_LINE_LEN: c_uint = 0x8F /* Free-Run line length 1 */;
pub const ADV7183_VBI_INFO: c_uint = 0x90 /* VBI info */;
pub const ADV7183_WSS_1: c_uint = 0x91 /* WSS 1 */;
pub const ADV7183_WSS_2: c_uint = 0x92 /* WSS 2 */;
pub const ADV7183_EDTV_1: c_uint = 0x93 /* EDTV 1 */;
pub const ADV7183_EDTV_2: c_uint = 0x94 /* EDTV 2 */;
pub const ADV7183_EDTV_3: c_uint = 0x95 /* EDTV 3 */;
pub const ADV7183_CGMS_1: c_uint = 0x96 /* CGMS 1 */;
pub const ADV7183_CGMS_2: c_uint = 0x97 /* CGMS 2 */;
pub const ADV7183_CGMS_3: c_uint = 0x98 /* CGMS 3 */;
pub const ADV7183_CCAP_1: c_uint = 0x99 /* CCAP 1 */;
pub const ADV7183_CCAP_2: c_uint = 0x9A /* CCAP 2 */;
pub const ADV7183_LETTERBOX_1: c_uint = 0x9B /* Letterbox 1 */;
pub const ADV7183_LETTERBOX_2: c_uint = 0x9C /* Letterbox 2 */;
pub const ADV7183_LETTERBOX_3: c_uint = 0x9D /* Letterbox 3 */;
pub const ADV7183_CRC_EN: c_uint = 0xB2 /* CRC enable */;
pub const ADV7183_ADC_SWITCH_1: c_uint = 0xC3 /* ADC switch 1 */;
pub const ADV7183_ADC_SWITCH_2: c_uint = 0xC4 /* ADC switch 2 */;
pub const ADV7183_LETTERBOX_CTRL_1: c_uint = 0xDC /* Letterbox control 1 */;
pub const ADV7183_LETTERBOX_CTRL_2: c_uint = 0xDD /* Letterbox control 2 */;
pub const ADV7183_SD_OFFSET_CB: c_uint = 0xE1 /* SD offset Cb */;
pub const ADV7183_SD_OFFSET_CR: c_uint = 0xE2 /* SD offset Cr */;
pub const ADV7183_SD_SATURATION_CB: c_uint = 0xE3 /* SD saturation Cb */;
pub const ADV7183_SD_SATURATION_CR: c_uint = 0xE4 /* SD saturation Cr */;
pub const ADV7183_NTSC_V_BEGIN: c_uint = 0xE5 /* NTSC V bit begin */;
pub const ADV7183_NTSC_V_END: c_uint = 0xE6 /* NTSC V bit end */;
pub const ADV7183_NTSC_F_TOGGLE: c_uint = 0xE7 /* NTSC F bit toggle */;
pub const ADV7183_PAL_V_BEGIN: c_uint = 0xE8 /* PAL V bit begin */;
pub const ADV7183_PAL_V_END: c_uint = 0xE9 /* PAL V bit end */;
pub const ADV7183_PAL_F_TOGGLE: c_uint = 0xEA /* PAL F bit toggle */;
pub const ADV7183_DRIVE_STR: c_uint = 0xF4 /* Drive strength */;
pub const ADV7183_IF_COMP_CTRL: c_uint = 0xF8 /* IF comp control */;
pub const ADV7183_VS_MODE_CTRL: c_uint = 0xF9 /* VS mode control */;
