//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/vpe/sc.h
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
// Copyright (c) 2013 Texas Instruments Inc.
//
// David Griego, <dagriego@biglakesoftware.com>
// Dale Farnsworth, <dale@farnsworth.org>
// Archit Taneja, <archit@ti.com>
//
// Scaler regs
pub const CFG_SC0: c_uint = 0x0;

pub const CFG_SC1: c_uint = 0x4;
pub const CFG_ROW_ACC_INC_MASK: c_uint = 0x07ffffff;
pub const CFG_ROW_ACC_INC_SHIFT: c_int = 0;
pub const CFG_SC2: c_uint = 0x08;
pub const CFG_ROW_ACC_OFFSET_MASK: c_uint = 0x0fffffff;
pub const CFG_ROW_ACC_OFFSET_SHIFT: c_int = 0;
pub const CFG_SC3: c_uint = 0x0c;
pub const CFG_ROW_ACC_OFFSET_B_MASK: c_uint = 0x0fffffff;
pub const CFG_ROW_ACC_OFFSET_B_SHIFT: c_int = 0;
pub const CFG_SC4: c_uint = 0x10;
pub const CFG_TAR_H_MASK: c_uint = 0x07ff;
pub const CFG_TAR_H_SHIFT: c_int = 0;
pub const CFG_TAR_W_MASK: c_uint = 0x07ff;
pub const CFG_TAR_W_SHIFT: c_int = 12;
pub const CFG_LIN_ACC_INC_U_MASK: c_uint = 0x07;
pub const CFG_LIN_ACC_INC_U_SHIFT: c_int = 24;
pub const CFG_NLIN_ACC_INIT_U_MASK: c_uint = 0x07;
pub const CFG_NLIN_ACC_INIT_U_SHIFT: c_int = 28;
pub const CFG_SC5: c_uint = 0x14;
pub const CFG_SRC_H_MASK: c_uint = 0x07ff;
pub const CFG_SRC_H_SHIFT: c_int = 0;
pub const CFG_SRC_W_MASK: c_uint = 0x07ff;
pub const CFG_SRC_W_SHIFT: c_int = 12;
pub const CFG_NLIN_ACC_INC_U_MASK: c_uint = 0x07;
pub const CFG_NLIN_ACC_INC_U_SHIFT: c_int = 24;
pub const CFG_SC6: c_uint = 0x18;
pub const CFG_ROW_ACC_INIT_RAV_MASK: c_uint = 0x03ff;
pub const CFG_ROW_ACC_INIT_RAV_SHIFT: c_int = 0;
pub const CFG_ROW_ACC_INIT_RAV_B_MASK: c_uint = 0x03ff;
pub const CFG_ROW_ACC_INIT_RAV_B_SHIFT: c_int = 10;
pub const CFG_SC8: c_uint = 0x20;
pub const CFG_NLIN_LEFT_MASK: c_uint = 0x07ff;
pub const CFG_NLIN_LEFT_SHIFT: c_int = 0;
pub const CFG_NLIN_RIGHT_MASK: c_uint = 0x07ff;
pub const CFG_NLIN_RIGHT_SHIFT: c_int = 12;
pub const CFG_SC9: c_uint = 0x24;

pub const CFG_SC10: c_uint = 0x28;

pub const CFG_SC11: c_uint = 0x2c;

pub const CFG_SC12: c_uint = 0x30;
pub const CFG_COL_ACC_OFFSET_MASK: c_uint = 0x01ffffff;
pub const CFG_COL_ACC_OFFSET_SHIFT: c_int = 0;
pub const CFG_SC13: c_uint = 0x34;
pub const CFG_SC_FACTOR_RAV_MASK: c_uint = 0xff;
pub const CFG_SC_FACTOR_RAV_SHIFT: c_int = 0;
pub const CFG_CHROMA_INTP_THR_MASK: c_uint = 0x03ff;
pub const CFG_CHROMA_INTP_THR_SHIFT: c_int = 12;
pub const CFG_DELTA_CHROMA_THR_MASK: c_uint = 0x0f;
pub const CFG_DELTA_CHROMA_THR_SHIFT: c_int = 24;
pub const CFG_SC17: c_uint = 0x44;
pub const CFG_EV_THR_MASK: c_uint = 0x03ff;
pub const CFG_EV_THR_SHIFT: c_int = 12;
pub const CFG_DELTA_LUMA_THR_MASK: c_uint = 0x0f;
pub const CFG_DELTA_LUMA_THR_SHIFT: c_int = 24;
pub const CFG_DELTA_EV_THR_MASK: c_uint = 0x0f;
pub const CFG_DELTA_EV_THR_SHIFT: c_int = 28;
pub const CFG_SC18: c_uint = 0x48;
pub const CFG_HS_FACTOR_MASK: c_uint = 0x03ff;
pub const CFG_HS_FACTOR_SHIFT: c_int = 0;
pub const CFG_CONF_DEFAULT_MASK: c_uint = 0x01ff;
pub const CFG_CONF_DEFAULT_SHIFT: c_int = 16;
pub const CFG_SC19: c_uint = 0x4c;
pub const CFG_HPF_COEFF0_MASK: c_uint = 0xff;
pub const CFG_HPF_COEFF0_SHIFT: c_int = 0;
pub const CFG_HPF_COEFF1_MASK: c_uint = 0xff;
pub const CFG_HPF_COEFF1_SHIFT: c_int = 8;
pub const CFG_HPF_COEFF2_MASK: c_uint = 0xff;
pub const CFG_HPF_COEFF2_SHIFT: c_int = 16;
pub const CFG_HPF_COEFF3_MASK: c_uint = 0xff;
pub const CFG_HPF_COEFF3_SHIFT: c_int = 23;
pub const CFG_SC20: c_uint = 0x50;
pub const CFG_HPF_COEFF4_MASK: c_uint = 0xff;
pub const CFG_HPF_COEFF4_SHIFT: c_int = 0;
pub const CFG_HPF_COEFF5_MASK: c_uint = 0xff;
pub const CFG_HPF_COEFF5_SHIFT: c_int = 8;
pub const CFG_HPF_NORM_SHIFT_MASK: c_uint = 0x07;
pub const CFG_HPF_NORM_SHIFT_SHIFT: c_int = 16;
pub const CFG_NL_LIMIT_MASK: c_uint = 0x1ff;
pub const CFG_NL_LIMIT_SHIFT: c_int = 20;
pub const CFG_SC21: c_uint = 0x54;
pub const CFG_NL_LO_THR_MASK: c_uint = 0x01ff;
pub const CFG_NL_LO_THR_SHIFT: c_int = 0;
pub const CFG_NL_LO_SLOPE_MASK: c_uint = 0xff;
pub const CFG_NL_LO_SLOPE_SHIFT: c_int = 16;
pub const CFG_SC22: c_uint = 0x58;
pub const CFG_NL_HI_THR_MASK: c_uint = 0x01ff;
pub const CFG_NL_HI_THR_SHIFT: c_int = 0;
pub const CFG_NL_HI_SLOPE_SH_MASK: c_uint = 0x07;
pub const CFG_NL_HI_SLOPE_SH_SHIFT: c_int = 16;
pub const CFG_SC23: c_uint = 0x5c;
pub const CFG_GRADIENT_THR_MASK: c_uint = 0x07ff;
pub const CFG_GRADIENT_THR_SHIFT: c_int = 0;
pub const CFG_GRADIENT_THR_RANGE_MASK: c_uint = 0x0f;
pub const CFG_GRADIENT_THR_RANGE_SHIFT: c_int = 12;
pub const CFG_MIN_GY_THR_MASK: c_uint = 0xff;
pub const CFG_MIN_GY_THR_SHIFT: c_int = 16;
pub const CFG_MIN_GY_THR_RANGE_MASK: c_uint = 0x0f;
pub const CFG_MIN_GY_THR_RANGE_SHIFT: c_int = 28;
pub const CFG_SC24: c_uint = 0x60;
pub const CFG_ORG_H_MASK: c_uint = 0x07ff;
pub const CFG_ORG_H_SHIFT: c_int = 0;
pub const CFG_ORG_W_MASK: c_uint = 0x07ff;
pub const CFG_ORG_W_SHIFT: c_int = 16;
pub const CFG_SC25: c_uint = 0x64;
pub const CFG_OFF_H_MASK: c_uint = 0x07ff;
pub const CFG_OFF_H_SHIFT: c_int = 0;
pub const CFG_OFF_W_MASK: c_uint = 0x07ff;
pub const CFG_OFF_W_SHIFT: c_int = 16;
// number of phases supported by the polyphase scalers
pub const SC_NUM_PHASES: c_int = 32;
// number of taps used by horizontal polyphase scaler
pub const SC_H_NUM_TAPS: c_int = 7;
// number of taps used by vertical polyphase scaler
pub const SC_V_NUM_TAPS: c_int = 5;
// number of taps expected by the scaler in it's coefficient memory
pub const SC_NUM_TAPS_MEM_ALIGN: c_int = 8;
// Maximum frame width the scaler can handle (in pixels)
pub const SC_MAX_PIXEL_WIDTH: c_int = 2047;
// Maximum frame height the scaler can handle (in lines)
pub const SC_MAX_PIXEL_HEIGHT: c_int = 2047;
//
// coefficient memory size in bytes:
// num phases x num sets(luma and chroma) x num taps(aligned) x coeff size
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc_data {
    pub base: *mut void __iomem,
    pub res: *mut resource,
    pub /: *mut *mut dma_addr_t loaded_coeff_h; / loaded h coeffs in SC,
    pub /: *mut *mut dma_addr_t loaded_coeff_v; / loaded v coeffs in SC,
    pub /: *mut *mut bool load_coeff_h; / have new h SC coeffs,
    pub /: *mut *mut bool load_coeff_v; / have new v SC coeffs,
    pub pdev: *mut platform_device,
}

extern "C" {
    pub fn sc_dump_regs(sc: *mut sc_data);
}
