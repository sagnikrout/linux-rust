//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/ingenic/ingenic-ipu.h
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
// Ingenic JZ47xx IPU - Register definitions and private API
//
// Copyright (C) 2020, Paul Cercueil <paul@crapouillou.net>

pub const JZ_REG_IPU_CTRL: c_uint = 0x00;
pub const JZ_REG_IPU_STATUS: c_uint = 0x04;
pub const JZ_REG_IPU_D_FMT: c_uint = 0x08;
pub const JZ_REG_IPU_Y_ADDR: c_uint = 0x0c;
pub const JZ_REG_IPU_U_ADDR: c_uint = 0x10;
pub const JZ_REG_IPU_V_ADDR: c_uint = 0x14;
pub const JZ_REG_IPU_IN_GS: c_uint = 0x18;
pub const JZ_REG_IPU_Y_STRIDE: c_uint = 0x1c;
pub const JZ_REG_IPU_UV_STRIDE: c_uint = 0x20;
pub const JZ_REG_IPU_OUT_ADDR: c_uint = 0x24;
pub const JZ_REG_IPU_OUT_GS: c_uint = 0x28;
pub const JZ_REG_IPU_OUT_STRIDE: c_uint = 0x2c;
pub const JZ_REG_IPU_RSZ_COEF_INDEX: c_uint = 0x30;
pub const JZ_REG_IPU_CSC_C0_COEF: c_uint = 0x34;
pub const JZ_REG_IPU_CSC_C1_COEF: c_uint = 0x38;
pub const JZ_REG_IPU_CSC_C2_COEF: c_uint = 0x3c;
pub const JZ_REG_IPU_CSC_C3_COEF: c_uint = 0x40;
pub const JZ_REG_IPU_CSC_C4_COEF: c_uint = 0x44;
pub const JZ_REG_IPU_HRSZ_COEF_LUT: c_uint = 0x48;
pub const JZ_REG_IPU_VRSZ_COEF_LUT: c_uint = 0x4c;
pub const JZ_REG_IPU_CSC_OFFSET: c_uint = 0x50;
pub const JZ_REG_IPU_Y_PHY_T_ADDR: c_uint = 0x54;
pub const JZ_REG_IPU_U_PHY_T_ADDR: c_uint = 0x58;
pub const JZ_REG_IPU_V_PHY_T_ADDR: c_uint = 0x5c;
pub const JZ_REG_IPU_OUT_PHY_T_ADDR: c_uint = 0x60;

pub const JZ_IPU_IN_GS_H_LSB: c_uint = 0x0;
pub const JZ_IPU_IN_GS_W_LSB: c_uint = 0x10;
pub const JZ_IPU_OUT_GS_H_LSB: c_uint = 0x0;
pub const JZ_IPU_OUT_GS_W_LSB: c_uint = 0x10;
pub const JZ_IPU_Y_STRIDE_Y_LSB: c_int = 0;
pub const JZ_IPU_UV_STRIDE_U_LSB: c_int = 16;
pub const JZ_IPU_UV_STRIDE_V_LSB: c_int = 0;
pub const JZ_IPU_D_FMT_IN_FMT_LSB: c_int = 0;

pub const JZ_IPU_D_FMT_YUV_FMT_LSB: c_int = 2;

pub const JZ_IPU_D_FMT_OUT_FMT_LSB: c_int = 19;

pub const JZ_IPU_D_FMT_RGB_OUT_OFT_LSB: c_int = 22;

pub const JZ4725B_IPU_RSZ_LUT_COEF_LSB: c_int = 2;
pub const JZ4725B_IPU_RSZ_LUT_COEF_MASK: c_uint = 0x7ff;

pub const JZ4760_IPU_RSZ_COEF20_LSB: c_int = 6;
pub const JZ4760_IPU_RSZ_COEF31_LSB: c_int = 17;
pub const JZ4760_IPU_RSZ_COEF_MASK: c_uint = 0x7ff;
pub const JZ4760_IPU_RSZ_OFFSET_LSB: c_int = 1;
pub const JZ4760_IPU_RSZ_OFFSET_MASK: c_uint = 0x1f;
pub const JZ_IPU_CSC_OFFSET_CHROMA_LSB: c_int = 16;
pub const JZ_IPU_CSC_OFFSET_LUMA_LSB: c_int = 16;
