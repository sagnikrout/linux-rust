//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkisp1/rkisp1-regs.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Rockchip ISP1 Driver - Registers header
//
// Copyright (C) 2017 Rockchip Electronics Co., Ltd.
//
// ISP_CTRL

// ISP_ACQ_PROP

// VI_DPCL

// ISP_IMSC - ISP_MIS - ISP_RIS - ISP_ICR - ISP_ISR

// ISP_ERR

// MI_CTRL

// MI_INIT

// MI_CTRL_SHD

// RSZ_CTRL

// MI_IMSC - MI_MIS - MI_RIS - MI_ICR - MI_ISR

// MI_STATUS

// MI_DMA_CTRL

// MI_DMA_START

// MI_XTD_FORMAT_CTRL

// MI_OUTPUT_ALIGN_FORMAT

// MI_MP_OUTPUT_FIFO_SIZE

// VI_CCL

// VI_ISP_CLK_CTRL

// VI_ICCL

// VI_IRCL

// C_PROC_CTR

pub const RKISP1_CIF_C_PROC_CTRL_RESERVED: c_uint = 0xfffffffe;
pub const RKISP1_CIF_C_PROC_CONTRAST_RESERVED: c_uint = 0xffffff00;
pub const RKISP1_CIF_C_PROC_BRIGHTNESS_RESERVED: c_uint = 0xffffff00;
pub const RKISP1_CIF_C_PROC_HUE_RESERVED: c_uint = 0xffffff00;
pub const RKISP1_CIF_C_PROC_SATURATION_RESERVED: c_uint = 0xffffff00;
pub const RKISP1_CIF_C_PROC_MACC_RESERVED: c_uint = 0xe000e000;
pub const RKISP1_CIF_C_PROC_TONE_RESERVED: c_uint = 0xf000;
// DUAL_CROP_CTRL

// IMG_EFF_CTRL

pub const RKISP1_CIF_IMG_EFF_CTRL_MODE_MASK: c_uint = 0xe;
// IMG_EFF_COLOR_SEL
pub const RKISP1_CIF_IMG_EFF_COLOR_RGB: c_int = 0;

// MIPI_CTRL

// MIPI_DATA_SEL

// MIPI_IMSC, MIPI_RIS, MIPI_MIS, MIPI_ICR, MIPI_ISR

// SUPER_IMPOSE

// ISP HISTOGRAM CALCULATION : ISP_HIST_PROP

pub const RKISP1_CIF_ISP_HIST_PROP_MODE_MASK_V10: c_uint = 0x7;

pub const RKISP1_CIF_ISP_HIST_WINDOW_OFFSET_RESERVED_V10: c_uint = 0xfffff000;
pub const RKISP1_CIF_ISP_HIST_WINDOW_SIZE_RESERVED_V10: c_uint = 0xfffff800;
pub const RKISP1_CIF_ISP_HIST_WEIGHT_RESERVED_V10: c_uint = 0xe0e0e0e0;
pub const RKISP1_CIF_ISP_MAX_HIST_PREDIVIDER_V10: c_uint = 0x0000007f;
pub const RKISP1_CIF_ISP_HIST_ROW_NUM_V10: c_int = 5;
pub const RKISP1_CIF_ISP_HIST_COLUMN_NUM_V10: c_int = 5;

// ISP HISTOGRAM CALCULATION : CIF_ISP_HIST

pub const RKISP1_CIF_ISP_HIST_ROW_NUM_V12: c_int = 15;
pub const RKISP1_CIF_ISP_HIST_COLUMN_NUM_V12: c_int = 15;

// AUTO FOCUS MEASUREMENT:  ISP_AFM_CTRL

// SHUTTER CONTROL

// FLASH MODULE
// ISP_FLASH_CMD

// ISP_FLASH_CONFIG

// Demosaic:  ISP_DEMOSAIC

// ISP_FLAGS_SHD

pub const RKISP1_CIF_ISP_FLAGS_SHD_S_DATA_SHIFT: c_int = 16;

// AWB
// ISP_AWB_PROP

pub const RKISP1_CIF_ISP_AWB_MODE_MASK_NONE: c_uint = 0xfffffffc;

// ISP_AWB_GAIN_RB, ISP_AWB_GAIN_G

// ISP_AWB_REF

// ISP_AWB_THRESH

// ISP_AWB_MEAN

// ISP_AWB_WHITE_CNT

pub const RKISP1_CIF_ISP_AWB_GAINS_MAX_VAL: c_uint = 0x000003ff;
pub const RKISP1_CIF_ISP_AWB_WINDOW_OFFSET_MAX: c_uint = 0x00000fff;
pub const RKISP1_CIF_ISP_AWB_WINDOW_MAX_SIZE: c_uint = 0x00001fff;
pub const RKISP1_CIF_ISP_AWB_CBCR_MAX_REF: c_uint = 0x000000ff;
pub const RKISP1_CIF_ISP_AWB_THRES_MAX_YC: c_uint = 0x000000ff;
// AE
// ISP_EXP_CTRL

//
// '1' luminance calculation according to  Y=(R+G+B) x 0.332 (85/256)
// '0' luminance calculation according to Y=16+0.25R+0.5G+0.1094B
//

// ISP_EXP_H_SIZE

pub const RKISP1_CIF_ISP_EXP_HEIGHT_MASK_V10: c_uint = 0x000007ff;

pub const RKISP1_CIF_ISP_EXP_HEIGHT_MASK_V12: c_uint = 0x000007ff;
// ISP_EXP_V_SIZE : vertical size must be a multiple of 2).

// ISP_EXP_H_OFFSET

pub const RKISP1_CIF_ISP_EXP_MAX_HOFFS_V10: c_int = 2424;

pub const RKISP1_CIF_ISP_EXP_MAX_HOFFS_V12: c_uint = 0x1fff;
// ISP_EXP_V_OFFSET

pub const RKISP1_CIF_ISP_EXP_MAX_VOFFS_V10: c_int = 1806;

pub const RKISP1_CIF_ISP_EXP_MAX_VOFFS_V12: c_uint = 0x1fff;
pub const RKISP1_CIF_ISP_EXP_ROW_NUM_V10: c_int = 5;
pub const RKISP1_CIF_ISP_EXP_COLUMN_NUM_V10: c_int = 5;

pub const RKISP1_CIF_ISP_EXP_BLOCK_MAX_HSIZE_V10: c_int = 516;
pub const RKISP1_CIF_ISP_EXP_BLOCK_MIN_HSIZE_V10: c_int = 35;
pub const RKISP1_CIF_ISP_EXP_BLOCK_MAX_VSIZE_V10: c_int = 390;
pub const RKISP1_CIF_ISP_EXP_BLOCK_MIN_VSIZE_V10: c_int = 28;

pub const RKISP1_CIF_ISP_EXP_ROW_NUM_V12: c_int = 15;
pub const RKISP1_CIF_ISP_EXP_COLUMN_NUM_V12: c_int = 15;

pub const RKISP1_CIF_ISP_EXP_BLOCK_MAX_HSIZE_V12: c_uint = 0x7ff;
pub const RKISP1_CIF_ISP_EXP_BLOCK_MIN_HSIZE_V12: c_uint = 0xe;
pub const RKISP1_CIF_ISP_EXP_BLOCK_MAX_VSIZE_V12: c_uint = 0x7fe;
pub const RKISP1_CIF_ISP_EXP_BLOCK_MIN_VSIZE_V12: c_uint = 0xe;

// LSC: ISP_LSC_CTRL

pub const RKISP1_CIF_ISP_LSC_SECT_SIZE_RESERVED: c_uint = 0xfc00fc00;
pub const RKISP1_CIF_ISP_LSC_GRAD_RESERVED_V10: c_uint = 0xf000f000;
pub const RKISP1_CIF_ISP_LSC_SAMPLE_RESERVED_V10: c_uint = 0xf000f000;
pub const RKISP1_CIF_ISP_LSC_GRAD_RESERVED_V12: c_uint = 0xe000e000;
pub const RKISP1_CIF_ISP_LSC_SAMPLE_RESERVED_V12: c_uint = 0xe000e000;

// LSC: ISP_LSC_TABLE_SEL
pub const RKISP1_CIF_ISP_LSC_TABLE_0: c_int = 0;
pub const RKISP1_CIF_ISP_LSC_TABLE_1: c_int = 1;
// LSC: ISP_LSC_STATUS

pub const RKISP1_CIF_ISP_LSC_TABLE_ADDRESS_0: c_int = 0;
pub const RKISP1_CIF_ISP_LSC_TABLE_ADDRESS_153: c_int = 153;
// FLT
// ISP_FILT_MODE

//
// 0: green filter static mode (active filter factor = FILT_FAC_MID)
// 1: dynamic noise reduction/sharpen Default
//

pub const RKISP1_CIF_ISP_FLT_MODE_MAX: c_int = 1;

pub const RKISP1_CIF_ISP_FLT_CHROMA_MODE_MAX: c_int = 3;

pub const RKISP1_CIF_ISP_FLT_GREEN_STAGE1_MAX: c_int = 8;
pub const RKISP1_CIF_ISP_FLT_THREAD_RESERVED: c_uint = 0xfffffc00;
pub const RKISP1_CIF_ISP_FLT_FAC_RESERVED: c_uint = 0xffffffc0;
pub const RKISP1_CIF_ISP_FLT_LUM_WEIGHT_RESERVED: c_uint = 0xfff80000;
pub const RKISP1_CIF_ISP_CTK_COEFF_RESERVED: c_uint = 0xfffff800;
pub const RKISP1_CIF_ISP_XTALK_OFFSET_RESERVED: c_uint = 0xfffff000;
// GOC

pub const RKISP1_CIF_ISP_GOC_MODE_MAX: c_int = 1;
pub const RKISP1_CIF_ISP_GOC_RESERVED: c_uint = 0xfffff800;
// ISP_CTRL BIT 11

// DPCC

pub const RKISP1_CIF_ISP_DPCC_METHODS_SET_MASK: c_uint = 0x00001f1f;
pub const RKISP1_CIF_ISP_DPCC_LINE_THRESH_MASK: c_uint = 0x0000ffff;
pub const RKISP1_CIF_ISP_DPCC_LINE_MAD_FAC_MASK: c_uint = 0x00003f3f;
pub const RKISP1_CIF_ISP_DPCC_PG_FAC_MASK: c_uint = 0x00003f3f;
pub const RKISP1_CIF_ISP_DPCC_RND_THRESH_MASK: c_uint = 0x0000ffff;
pub const RKISP1_CIF_ISP_DPCC_RG_FAC_MASK: c_uint = 0x00003f3f;
pub const RKISP1_CIF_ISP_DPCC_RO_LIMIT_MASK: c_uint = 0x00000fff;
pub const RKISP1_CIF_ISP_DPCC_RND_OFFS_MASK: c_uint = 0x00000fff;
// BLS
// ISP_BLS_CTRL

pub const RKISP1_CIF_ISP_BLS_MODE_FIXED: c_int = 0;

// GAMMA-IN

pub const RKISP1_CIFISP_DEGAMMA_Y_RESERVED: c_uint = 0xfffff000;
// GAMMA-OUT

// AFM

pub const RKISP1_CIF_ISP_AFM_THRES_RESERVED: c_uint = 0xffff0000;
pub const RKISP1_CIF_ISP_AFM_VAR_SHIFT_RESERVED: c_uint = 0xfff8fff8;
pub const RKISP1_CIF_ISP_AFM_WINDOW_X_RESERVED: c_uint = 0xe000;
pub const RKISP1_CIF_ISP_AFM_WINDOW_Y_RESERVED: c_uint = 0xf000;
pub const RKISP1_CIF_ISP_AFM_WINDOW_X_MIN: c_uint = 0x5;
pub const RKISP1_CIF_ISP_AFM_WINDOW_Y_MIN: c_uint = 0x2;

// DPF

pub const RKISP1_CIF_ISP_DPF_NF_GAIN_RESERVED: c_uint = 0xfffff000;
pub const RKISP1_CIF_ISP_DPF_SPATIAL_COEFF_MAX: c_uint = 0x1f;
pub const RKISP1_CIF_ISP_DPF_NLL_COEFF_N_MAX: c_uint = 0x3ff;
// COMPAND

// WDR
// ISP_WDR_CTRL

// ISP_WDR_TONE_CURVE_YM

// ISP_WDR_OFFSET

// ISP_WDR_DELTAMIN

// CAC

// ===================================================================
// CIF Registers
// ===================================================================
pub const RKISP1_CIF_CTRL_BASE: c_uint = 0x00000000;

pub const RKISP1_CIF_IMG_EFF_BASE: c_uint = 0x00000200;

pub const RKISP1_CIF_SUPER_IMP_BASE: c_uint = 0x00000300;

pub const RKISP1_CIF_ISP_BASE: c_uint = 0x00000400;

pub const RKISP1_CIF_ISP_FLASH_BASE: c_uint = 0x00000660;

pub const RKISP1_CIF_ISP_SH_BASE: c_uint = 0x00000680;

pub const RKISP1_CIF_C_PROC_BASE: c_uint = 0x00000800;

pub const RKISP1_CIF_DUAL_CROP_BASE: c_uint = 0x00000880;

pub const RKISP1_CIF_MRSZ_BASE: c_uint = 0x00000c00;
pub const RKISP1_CIF_SRSZ_BASE: c_uint = 0x00001000;
pub const RKISP1_CIF_RSZ_CTRL: c_uint = 0x0000;
pub const RKISP1_CIF_RSZ_SCALE_HY: c_uint = 0x0004;
pub const RKISP1_CIF_RSZ_SCALE_HCB: c_uint = 0x0008;
pub const RKISP1_CIF_RSZ_SCALE_HCR: c_uint = 0x000c;
pub const RKISP1_CIF_RSZ_SCALE_VY: c_uint = 0x0010;
pub const RKISP1_CIF_RSZ_SCALE_VC: c_uint = 0x0014;
pub const RKISP1_CIF_RSZ_PHASE_HY: c_uint = 0x0018;
pub const RKISP1_CIF_RSZ_PHASE_HC: c_uint = 0x001c;
pub const RKISP1_CIF_RSZ_PHASE_VY: c_uint = 0x0020;
pub const RKISP1_CIF_RSZ_PHASE_VC: c_uint = 0x0024;
pub const RKISP1_CIF_RSZ_SCALE_LUT_ADDR: c_uint = 0x0028;
pub const RKISP1_CIF_RSZ_SCALE_LUT: c_uint = 0x002c;
pub const RKISP1_CIF_RSZ_CTRL_SHD: c_uint = 0x0030;
pub const RKISP1_CIF_RSZ_SCALE_HY_SHD: c_uint = 0x0034;
pub const RKISP1_CIF_RSZ_SCALE_HCB_SHD: c_uint = 0x0038;
pub const RKISP1_CIF_RSZ_SCALE_HCR_SHD: c_uint = 0x003c;
pub const RKISP1_CIF_RSZ_SCALE_VY_SHD: c_uint = 0x0040;
pub const RKISP1_CIF_RSZ_SCALE_VC_SHD: c_uint = 0x0044;
pub const RKISP1_CIF_RSZ_PHASE_HY_SHD: c_uint = 0x0048;
pub const RKISP1_CIF_RSZ_PHASE_HC_SHD: c_uint = 0x004c;
pub const RKISP1_CIF_RSZ_PHASE_VY_SHD: c_uint = 0x0050;
pub const RKISP1_CIF_RSZ_PHASE_VC_SHD: c_uint = 0x0054;
pub const RKISP1_CIF_MI_BASE: c_uint = 0x00001400;

pub const RKISP1_CIF_SMIA_BASE: c_uint = 0x00001a00;

pub const RKISP1_CIF_MIPI_BASE: c_uint = 0x00001c00;

pub const RKISP1_CIF_ISP_AFM_BASE: c_uint = 0x00002000;

pub const RKISP1_CIF_ISP_LSC_BASE: c_uint = 0x00002200;

pub const RKISP1_CIF_ISP_IS_BASE: c_uint = 0x00002300;

pub const RKISP1_CIF_ISP_HIST_BASE_V10: c_uint = 0x00002400;

pub const RKISP1_CIF_ISP_FILT_BASE: c_uint = 0x00002500;

pub const RKISP1_CIF_ISP_CAC_BASE: c_uint = 0x00002580;

pub const RKISP1_CIF_ISP_EXP_BASE: c_uint = 0x00002600;

pub const RKISP1_CIF_ISP_BLS_BASE: c_uint = 0x00002700;

pub const RKISP1_CIF_ISP_DPF_BASE: c_uint = 0x00002800;

pub const RKISP1_CIF_ISP_DPCC_BASE: c_uint = 0x00002900;

pub const RKISP1_CIF_ISP_WDR_BASE: c_uint = 0x00002a00;

pub const RKISP1_CIF_ISP_HIST_BASE_V12: c_uint = 0x00002c00;

pub const RKISP1_CIF_ISP_VSM_BASE: c_uint = 0x00002f00;

pub const RKISP1_CIF_ISP_COMPAND_BASE: c_uint = 0x00003200;

pub const RKISP1_CIF_ISP_CSI0_BASE: c_uint = 0x00007000;

