//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/tvp5150_reg.h
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
// tvp5150 - Texas Instruments TVP5150A/AM1 video decoder registers
//
// Copyright (c) 2005,2006 Mauro Carvalho Chehab <mchehab@kernel.org>
//
pub const TVP5150_VD_IN_SRC_SEL_1: c_uint = 0x00 /* Video input source selection #1 */;
pub const TVP5150_ANAL_CHL_CTL: c_uint = 0x01 /* Analog channel controls */;
pub const TVP5150_OP_MODE_CTL: c_uint = 0x02 /* Operation mode controls */;
pub const TVP5150_MISC_CTL: c_uint = 0x03 /* Miscellaneous controls */;

pub const TVP5150_AUTOSW_MSK: c_uint = 0x04 /* Autoswitch mask: TVP5150A / TVP5150AM */;
// Reserved 05h
pub const TVP5150_COLOR_KIL_THSH_CTL: c_uint = 0x06 /* Color killer threshold control */;
pub const TVP5150_LUMA_PROC_CTL_1: c_uint = 0x07 /* Luminance processing control #1 */;
pub const TVP5150_LUMA_PROC_CTL_2: c_uint = 0x08 /* Luminance processing control #2 */;
pub const TVP5150_BRIGHT_CTL: c_uint = 0x09 /* Brightness control */;
pub const TVP5150_SATURATION_CTL: c_uint = 0x0a /* Color saturation control */;
pub const TVP5150_HUE_CTL: c_uint = 0x0b /* Hue control */;
pub const TVP5150_CONTRAST_CTL: c_uint = 0x0c /* Contrast control */;
pub const TVP5150_DATA_RATE_SEL: c_uint = 0x0d /* Outputs and data rates select */;
pub const TVP5150_LUMA_PROC_CTL_3: c_uint = 0x0e /* Luminance processing control #3 */;
pub const TVP5150_CONF_SHARED_PIN: c_uint = 0x0f /* Configuration shared pins */;
// Reserved 10h
pub const TVP5150_ACT_VD_CROP_ST_MSB: c_uint = 0x11 /* Active video cropping start MSB */;
pub const TVP5150_ACT_VD_CROP_ST_LSB: c_uint = 0x12 /* Active video cropping start LSB */;
pub const TVP5150_ACT_VD_CROP_STP_MSB: c_uint = 0x13 /* Active video cropping stop MSB */;
pub const TVP5150_ACT_VD_CROP_STP_LSB: c_uint = 0x14 /* Active video cropping stop LSB */;
pub const TVP5150_GENLOCK: c_uint = 0x15 /* Genlock/RTC */;
pub const TVP5150_HORIZ_SYNC_START: c_uint = 0x16 /* Horizontal sync start */;
// Reserved 17h
pub const TVP5150_VERT_BLANKING_START: c_uint = 0x18 /* Vertical blanking start */;
pub const TVP5150_VERT_BLANKING_STOP: c_uint = 0x19 /* Vertical blanking stop */;
pub const TVP5150_CHROMA_PROC_CTL_1: c_uint = 0x1a /* Chrominance processing control #1 */;
pub const TVP5150_CHROMA_PROC_CTL_2: c_uint = 0x1b /* Chrominance processing control #2 */;
pub const TVP5150_INT_RESET_REG_B: c_uint = 0x1c /* Interrupt reset register B */;
pub const TVP5150_INT_ENABLE_REG_B: c_uint = 0x1d /* Interrupt enable register B */;
pub const TVP5150_INTT_CONFIG_REG_B: c_uint = 0x1e /* Interrupt configuration register B */;
// Reserved 1Fh-27h

pub const TVP5150_VIDEO_STD: c_uint = 0x28 /* Video standard */;
pub const VIDEO_STD_AUTO_SWITCH_BIT: c_uint = 0x00;
pub const VIDEO_STD_NTSC_MJ_BIT: c_uint = 0x02;
pub const VIDEO_STD_PAL_BDGHIN_BIT: c_uint = 0x04;
pub const VIDEO_STD_PAL_M_BIT: c_uint = 0x06;
pub const VIDEO_STD_PAL_COMBINATION_N_BIT: c_uint = 0x08;
pub const VIDEO_STD_NTSC_4_43_BIT: c_uint = 0x0a;
pub const VIDEO_STD_SECAM_BIT: c_uint = 0x0c;
pub const VIDEO_STD_NTSC_MJ_BIT_AS: c_uint = 0x01;
pub const VIDEO_STD_PAL_BDGHIN_BIT_AS: c_uint = 0x03;
pub const VIDEO_STD_PAL_M_BIT_AS: c_uint = 0x05;
pub const VIDEO_STD_PAL_COMBINATION_N_BIT_AS: c_uint = 0x07;
pub const VIDEO_STD_NTSC_4_43_BIT_AS: c_uint = 0x09;
pub const VIDEO_STD_SECAM_BIT_AS: c_uint = 0x0b;
// Reserved 29h-2bh
pub const TVP5150_CB_GAIN_FACT: c_uint = 0x2c /* Cb gain factor */;
pub const TVP5150_CR_GAIN_FACTOR: c_uint = 0x2d /* Cr gain factor */;
pub const TVP5150_MACROVISION_ON_CTR: c_uint = 0x2e /* Macrovision on counter */;
pub const TVP5150_MACROVISION_OFF_CTR: c_uint = 0x2f /* Macrovision off counter */;
pub const TVP5150_REV_SELECT: c_uint = 0x30 /* revision select (TVP5150AM1 only) */;
// Reserved	31h-7Fh
pub const TVP5150_MSB_DEV_ID: c_uint = 0x80 /* MSB of device ID */;
pub const TVP5150_LSB_DEV_ID: c_uint = 0x81 /* LSB of device ID */;
pub const TVP5150_ROM_MAJOR_VER: c_uint = 0x82 /* ROM major version */;
pub const TVP5150_ROM_MINOR_VER: c_uint = 0x83 /* ROM minor version */;
pub const TVP5150_VERT_LN_COUNT_MSB: c_uint = 0x84 /* Vertical line count MSB */;
pub const TVP5150_VERT_LN_COUNT_LSB: c_uint = 0x85 /* Vertical line count LSB */;
pub const TVP5150_INT_STATUS_REG_B: c_uint = 0x86 /* Interrupt status register B */;
pub const TVP5150_INT_ACTIVE_REG_B: c_uint = 0x87 /* Interrupt active register B */;
pub const TVP5150_STATUS_REG_1: c_uint = 0x88 /* Status register #1 */;
pub const TVP5150_STATUS_REG_2: c_uint = 0x89 /* Status register #2 */;
pub const TVP5150_STATUS_REG_3: c_uint = 0x8a /* Status register #3 */;
pub const TVP5150_STATUS_REG_4: c_uint = 0x8b /* Status register #4 */;
pub const TVP5150_STATUS_REG_5: c_uint = 0x8c /* Status register #5 */;
// Reserved	8Dh-8Fh
// Closed caption data registers
pub const TVP5150_CC_DATA_INI: c_uint = 0x90;
pub const TVP5150_CC_DATA_END: c_uint = 0x93;
// WSS data registers
pub const TVP5150_WSS_DATA_INI: c_uint = 0x94;
pub const TVP5150_WSS_DATA_END: c_uint = 0x99;
// VPS data registers
pub const TVP5150_VPS_DATA_INI: c_uint = 0x9a;
pub const TVP5150_VPS_DATA_END: c_uint = 0xa6;
// VITC data registers
pub const TVP5150_VITC_DATA_INI: c_uint = 0xa7;
pub const TVP5150_VITC_DATA_END: c_uint = 0xaf;
pub const TVP5150_VBI_FIFO_READ_DATA: c_uint = 0xb0 /* VBI FIFO read data */;
// Teletext filter 1
pub const TVP5150_TELETEXT_FIL1_INI: c_uint = 0xb1;
pub const TVP5150_TELETEXT_FIL1_END: c_uint = 0xb5;
// Teletext filter 2
pub const TVP5150_TELETEXT_FIL2_INI: c_uint = 0xb6;
pub const TVP5150_TELETEXT_FIL2_END: c_uint = 0xba;
pub const TVP5150_TELETEXT_FIL_ENA: c_uint = 0xbb /* Teletext filter enable */;
// Reserved	BCh-BFh
pub const TVP5150_INT_STATUS_REG_A: c_uint = 0xc0 /* Interrupt status register A */;

pub const TVP5150_INT_ENABLE_REG_A: c_uint = 0xc1 /* Interrupt enable register A */;
pub const TVP5150_INT_CONF: c_uint = 0xc2 /* Interrupt configuration */;

pub const TVP5150_VDP_CONF_RAM_DATA: c_uint = 0xc3 /* VDP configuration RAM data */;
pub const TVP5150_CONF_RAM_ADDR_LOW: c_uint = 0xc4 /* Configuration RAM address low byte */;
pub const TVP5150_CONF_RAM_ADDR_HIGH: c_uint = 0xc5 /* Configuration RAM address high byte */;
pub const TVP5150_VDP_STATUS_REG: c_uint = 0xc6 /* VDP status register */;
pub const TVP5150_FIFO_WORD_COUNT: c_uint = 0xc7 /* FIFO word count */;
pub const TVP5150_FIFO_INT_THRESHOLD: c_uint = 0xc8 /* FIFO interrupt threshold */;
pub const TVP5150_FIFO_RESET: c_uint = 0xc9 /* FIFO reset */;
pub const TVP5150_LINE_NUMBER_INT: c_uint = 0xca /* Line number interrupt */;
pub const TVP5150_PIX_ALIGN_REG_LOW: c_uint = 0xcb /* Pixel alignment register low byte */;
pub const TVP5150_PIX_ALIGN_REG_HIGH: c_uint = 0xcc /* Pixel alignment register high byte */;
pub const TVP5150_FIFO_OUT_CTRL: c_uint = 0xcd /* FIFO output control */;
// Reserved	CEh
pub const TVP5150_FULL_FIELD_ENA: c_uint = 0xcf /* Full field enable 1 */;
// Line mode registers
pub const TVP5150_LINE_MODE_INI: c_uint = 0xd0;
pub const TVP5150_LINE_MODE_END: c_uint = 0xfb;
pub const TVP5150_FULL_FIELD_MODE_REG: c_uint = 0xfc /* Full field mode register */;
// Reserved	FDh-FFh
