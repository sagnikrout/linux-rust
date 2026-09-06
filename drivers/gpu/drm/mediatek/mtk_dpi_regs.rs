//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mediatek/mtk_dpi_regs.h
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
pub const DPI_EN: c_uint = 0x00;

pub const DPI_RET: c_uint = 0x04;

pub const DPI_INTEN: c_uint = 0x08;

pub const DPI_INTSTA: c_uint = 0x0C;

pub const DPI_CON: c_uint = 0x10;

// DPI_CON: DPI instances

// DPI_CON: DPINTF instances

pub const DPI_OUTPUT_SETTING: c_uint = 0x14;
pub const CH_SWAP: c_int = 0;
pub const DPINTF_CH_SWAP: c_int = 1;

pub const SWAP_RGB: c_uint = 0x00;
pub const SWAP_GBR: c_uint = 0x01;
pub const SWAP_BRG: c_uint = 0x02;
pub const SWAP_RBG: c_uint = 0x03;
pub const SWAP_GRB: c_uint = 0x04;
pub const SWAP_BGR: c_uint = 0x05;

pub const OUT_BIT: c_int = 18;

pub const OUT_BIT_8: c_uint = 0x00;
pub const OUT_BIT_10: c_uint = 0x01;
pub const OUT_BIT_12: c_uint = 0x02;
pub const OUT_BIT_16: c_uint = 0x03;
pub const YC_MAP: c_int = 20;

pub const YC_MAP_RGB: c_uint = 0x00;
pub const YC_MAP_CYCY: c_uint = 0x04;
pub const YC_MAP_YCYC: c_uint = 0x05;
pub const YC_MAP_CY: c_uint = 0x06;
pub const YC_MAP_YC: c_uint = 0x07;
pub const DPI_SIZE: c_uint = 0x18;
pub const HSIZE: c_int = 0;

pub const VSIZE: c_int = 16;

pub const DPI_DDR_SETTING: c_uint = 0x1C;

pub const DPI_TGEN_HWIDTH: c_uint = 0x20;
pub const HPW: c_int = 0;

pub const DPI_TGEN_HPORCH: c_uint = 0x24;
pub const HBP: c_int = 0;

pub const HFP: c_int = 16;

pub const DPI_TGEN_VWIDTH: c_uint = 0x28;
pub const DPI_TGEN_VPORCH: c_uint = 0x2C;
pub const VSYNC_WIDTH_SHIFT: c_int = 0;

pub const VSYNC_HALF_LINE_SHIFT: c_int = 16;

pub const VSYNC_BACK_PORCH_SHIFT: c_int = 0;

pub const VSYNC_FRONT_PORCH_SHIFT: c_int = 16;

pub const DPI_BG_HCNTL: c_uint = 0x30;

pub const DPI_BG_VCNTL: c_uint = 0x34;

pub const DPI_BG_COLOR: c_uint = 0x38;

pub const DPI_FIFO_CTL: c_uint = 0x3C;

pub const DPI_STATUS: c_uint = 0x40;

pub const DPI_TMODE: c_uint = 0x44;

pub const DPI_CHECKSUM: c_uint = 0x48;

pub const DPI_DUMMY: c_uint = 0x50;

pub const DPI_TGEN_VWIDTH_LEVEN: c_uint = 0x68;
pub const DPI_TGEN_VPORCH_LEVEN: c_uint = 0x6C;
pub const DPI_TGEN_VWIDTH_RODD: c_uint = 0x70;
pub const DPI_TGEN_VPORCH_RODD: c_uint = 0x74;
pub const DPI_TGEN_VWIDTH_REVEN: c_uint = 0x78;
pub const DPI_TGEN_VPORCH_REVEN: c_uint = 0x7C;
pub const DPI_ESAV_VTIMING_LODD: c_uint = 0x80;

pub const DPI_ESAV_VTIMING_LEVEN: c_uint = 0x84;

pub const DPI_ESAV_VTIMING_RODD: c_uint = 0x88;

pub const DPI_ESAV_VTIMING_REVEN: c_uint = 0x8C;

pub const DPI_ESAV_FTIMING: c_uint = 0x90;

pub const DPI_CLPF_SETTING: c_uint = 0x94;

pub const DPI_Y_LIMIT: c_uint = 0x98;
pub const Y_LIMINT_BOT: c_int = 0;

pub const Y_LIMINT_TOP: c_int = 16;

pub const DPI_C_LIMIT: c_uint = 0x9C;
pub const C_LIMIT_BOT: c_int = 0;

pub const C_LIMIT_TOP: c_int = 16;

pub const DPI_YUV422_SETTING: c_uint = 0xA0;

pub const DPI_EMBSYNC_SETTING: c_uint = 0xA4;

pub const DPI_ESAV_CODE_SET0: c_uint = 0xA8;

pub const DPI_ESAV_CODE_SET1: c_uint = 0xAC;

pub const DPI_MATRIX_SET: c_uint = 0xB4;

pub const MATRIX_SEL_RGB_TO_JPEG: c_int = 0;
pub const MATRIX_SEL_RGB_TO_BT601: c_int = 2;
pub const DPI_PATTERN0: c_uint = 0xf00;

