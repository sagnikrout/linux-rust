//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/verisilicon/hantro_h1_regs.h
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
// Hantro VPU codec driver
//
// Copyright 2018 Google LLC.
// Tomasz Figa <tfiga@chromium.org>
//
// Encoder registers.
pub const H1_REG_INTERRUPT: c_uint = 0x004;

pub const H1_REG_AXI_CTRL: c_uint = 0x008;

pub const H1_REG_ADDR_OUTPUT_STREAM: c_uint = 0x014;
pub const H1_REG_ADDR_OUTPUT_CTRL: c_uint = 0x018;
pub const H1_REG_ADDR_REF_LUMA: c_uint = 0x01c;
pub const H1_REG_ADDR_REF_CHROMA: c_uint = 0x020;
pub const H1_REG_ADDR_REC_LUMA: c_uint = 0x024;
pub const H1_REG_ADDR_REC_CHROMA: c_uint = 0x028;
pub const H1_REG_ADDR_IN_PLANE_0: c_uint = 0x02c;
pub const H1_REG_ADDR_IN_PLANE_1: c_uint = 0x030;
pub const H1_REG_ADDR_IN_PLANE_2: c_uint = 0x034;
pub const H1_REG_ENC_CTRL: c_uint = 0x038;

pub const H1_REG_IN_IMG_CTRL: c_uint = 0x03c;

pub const H1_REG_ENC_CTRL0: c_uint = 0x040;

pub const H1_REG_ENC_CTRL1: c_uint = 0x044;

pub const H1_REG_ENC_CTRL2: c_uint = 0x048;

pub const H1_REG_ENC_CTRL3: c_uint = 0x04c;

pub const H1_REG_ENC_CTRL4: c_uint = 0x050;

pub const H1_REG_ENC_CTRL5: c_uint = 0x054;

pub const H1_REG_STR_HDR_REM_MSB: c_uint = 0x058;
pub const H1_REG_STR_HDR_REM_LSB: c_uint = 0x05c;
pub const H1_REG_STR_BUF_LIMIT: c_uint = 0x060;
pub const H1_REG_MAD_CTRL: c_uint = 0x064;

pub const H1_REG_ADDR_VP8_PROB_CNT: c_uint = 0x068;
pub const H1_REG_QP_VAL: c_uint = 0x06c;

// (i & 1))) & 0xffff) \
// 32)

pub const H1_REG_VP8_BOOL_ENC: c_uint = 0x08c;
pub const H1_REG_CHKPT_DELTA_QP: c_uint = 0x090;

pub const H1_REG_VP8_CTRL0: c_uint = 0x090;
pub const H1_REG_RLC_CTRL: c_uint = 0x094;
pub const H1_REG_RLC_CTRL_STR_OFFS_SHIFT: c_int = 23;

pub const H1_REG_MB_CTRL: c_uint = 0x098;

pub const H1_REG_ADDR_NEXT_PIC: c_uint = 0x09c;

pub const H1_REG_STABILIZATION_OUTPUT: c_uint = 0x0A0;
pub const H1_REG_ADDR_CABAC_TBL: c_uint = 0x0cc;
pub const H1_REG_ADDR_MV_OUT: c_uint = 0x0d0;

pub const H1_REG_RGB_MASK_MSB: c_uint = 0x0dc;
pub const H1_REG_INTRA_AREA_CTRL: c_uint = 0x0e0;
pub const H1_REG_CIR_INTRA_CTRL: c_uint = 0x0e4;

pub const H1_REG_FIRST_ROI_AREA: c_uint = 0x0f0;
pub const H1_REG_SECOND_ROI_AREA: c_uint = 0x0f4;
pub const H1_REG_MVC_CTRL: c_uint = 0x0f8;

pub const H1_REG_ADDR_VP8_SEG_MAP: c_uint = 0x11c;

pub const H1_REG_VP8_CTRL1: c_uint = 0x280;
pub const H1_REG_VP8_BIT_COST_GOLDEN: c_uint = 0x284;

