//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/jpeg/mtk_jpeg_dec_reg.h
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
// Copyright (c) 2016 MediaTek Inc.
// Author: Ming Hsiu Tsai <minghsiu.tsai@mediatek.com>
// Rick Chang <rick.chang@mediatek.com>
//
pub const MTK_JPEG_BLOCK_MAX: c_int = 10;
pub const MTK_JPEG_DCTSIZE: c_int = 8;
pub const BIT_INQST_MASK_ERROR_BS: c_uint = 0x20;
pub const BIT_INQST_MASK_PAUSE: c_uint = 0x10;
pub const BIT_INQST_MASK_OVERFLOW: c_uint = 0x04;
pub const BIT_INQST_MASK_UNDERFLOW: c_uint = 0x02;
pub const BIT_INQST_MASK_EOF: c_uint = 0x01;
pub const BIT_INQST_MASK_ALLIRQ: c_uint = 0x37;
pub const JPGDEC_REG_RESET: c_uint = 0x0090;
pub const JPGDEC_REG_BRZ_FACTOR: c_uint = 0x00f8;
pub const JPGDEC_REG_DU_NUM: c_uint = 0x00fc;
pub const JPGDEC_REG_DEST_ADDR0_Y: c_uint = 0x0140;
pub const JPGDEC_REG_DEST_ADDR0_U: c_uint = 0x0144;
pub const JPGDEC_REG_DEST_ADDR0_V: c_uint = 0x0148;
pub const JPGDEC_REG_DEST_ADDR1_Y: c_uint = 0x014c;
pub const JPGDEC_REG_DEST_ADDR1_U: c_uint = 0x0150;
pub const JPGDEC_REG_DEST_ADDR1_V: c_uint = 0x0154;
pub const JPGDEC_REG_STRIDE_Y: c_uint = 0x0158;
pub const JPGDEC_REG_STRIDE_UV: c_uint = 0x015c;
pub const JPGDEC_REG_IMG_STRIDE_Y: c_uint = 0x0160;
pub const JPGDEC_REG_IMG_STRIDE_UV: c_uint = 0x0164;
pub const JPGDEC_REG_WDMA_CTRL: c_uint = 0x016c;
pub const JPGDEC_REG_PAUSE_MCU_NUM: c_uint = 0x0170;
pub const JPGDEC_REG_OPERATION_MODE: c_uint = 0x017c;
pub const JPGDEC_REG_FILE_ADDR: c_uint = 0x0200;
pub const JPGDEC_REG_COMP_ID: c_uint = 0x020c;
pub const JPGDEC_REG_TOTAL_MCU_NUM: c_uint = 0x0210;
pub const JPGDEC_REG_COMP0_DATA_UNIT_NUM: c_uint = 0x0224;
pub const JPGDEC_REG_DU_CTRL: c_uint = 0x023c;
pub const JPGDEC_REG_TRIG: c_uint = 0x0240;
pub const JPGDEC_REG_FILE_BRP: c_uint = 0x0248;
pub const JPGDEC_REG_FILE_TOTAL_SIZE: c_uint = 0x024c;
pub const JPGDEC_REG_QT_ID: c_uint = 0x0270;
pub const JPGDEC_REG_INTERRUPT_STATUS: c_uint = 0x0274;
pub const JPGDEC_REG_STATUS: c_uint = 0x0278;
pub const JPGDEC_REG_BIT_STREAM_SIZE: c_uint = 0x0344;
pub const JPGDEC_REG_DEST_ADDR0_Y_EXT: c_uint = 0x0360;
pub const JPGDEC_REG_DEST_ADDR0_U_EXT: c_uint = 0x0364;
pub const JPGDEC_REG_DEST_ADDR0_V_EXT: c_uint = 0x0368;
pub const JPGDEC_REG_DEST_ADDR1_Y_EXT: c_uint = 0x036c;
pub const JPGDEC_REG_DEST_ADDR1_U_EXT: c_uint = 0x0370;
pub const JPGDEC_REG_DEST_ADDR1_V_EXT: c_uint = 0x0374;
pub const JPGDEC_REG_FILE_ADDR_EXT: c_uint = 0x0378;
pub const JPGDEC_REG_FILE_BRP_EXT: c_uint = 0x037c;
