//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amlogic/c3/isp/c3-isp-regs.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2024 Amlogic, Inc. All rights reserved
//
pub const ISP_TOP_INPUT_SIZE: c_uint = 0x0000;

pub const ISP_TOP_FRM_SIZE: c_uint = 0x0004;

pub const ISP_TOP_HOLD_SIZE: c_uint = 0x0008;

pub const ISP_TOP_PATH_EN: c_uint = 0x0010;

pub const ISP_TOP_PATH_SEL: c_uint = 0x0014;

pub const ISP_TOP_DISPIN_SEL: c_uint = 0x0018;

pub const ISP_TOP_IRQ_EN: c_uint = 0x0080;

pub const ISP_TOP_IRQ_CLR: c_uint = 0x0084;
pub const ISP_TOP_RO_IRQ_STAT: c_uint = 0x01c4;

pub const ISP_TOP_MODE_CTRL: c_uint = 0x0400;
pub const ISP_TOP_FEO_CTRL0: c_uint = 0x040c;

pub const ISP_TOP_FEO_CTRL1_0: c_uint = 0x0410;

pub const ISP_TOP_FED_CTRL: c_uint = 0x0418;

pub const ISP_TOP_BEO_CTRL: c_uint = 0x041c;

pub const ISP_TOP_BED_CTRL: c_uint = 0x0420;

pub const ISP_TOP_3A_STAT_CRTL: c_uint = 0x0424;

pub const ISP_LSWB_BLC_OFST0: c_uint = 0x4028;

pub const ISP_LSWB_BLC_OFST1: c_uint = 0x402c;

pub const ISP_LSWB_BLC_PHSOFST: c_uint = 0x4034;

pub const ISP_LSWB_WB_GAIN0: c_uint = 0x4038;

pub const ISP_LSWB_WB_GAIN1: c_uint = 0x403c;

pub const ISP_LSWB_WB_GAIN2: c_uint = 0x4040;

pub const ISP_LSWB_WB_LIMIT0: c_uint = 0x4044;

pub const ISP_LSWB_WB_LIMIT1: c_uint = 0x4048;

pub const ISP_LSWB_WB_PHSOFST: c_uint = 0x4050;

pub const ISP_LSWB_LNS_PHSOFST: c_uint = 0x4054;

pub const ISP_DMS_COMMON_PARAM0: c_uint = 0x5000;

pub const ISP_CM0_COEF00_01: c_uint = 0x6048;

pub const ISP_CM0_COEF02_10: c_uint = 0x604c;

pub const ISP_CM0_COEF11_12: c_uint = 0x6050;

pub const ISP_CM0_COEF20_21: c_uint = 0x6054;

pub const ISP_CM0_COEF22_OUP_OFST0: c_uint = 0x6058;

pub const ISP_CCM_MTX_00_01: c_uint = 0x6098;

pub const ISP_CCM_MTX_02_03: c_uint = 0x609c;

pub const ISP_CCM_MTX_10_11: c_uint = 0x60A0;

pub const ISP_CCM_MTX_12_13: c_uint = 0x60A4;

pub const ISP_CCM_MTX_20_21: c_uint = 0x60A8;

pub const ISP_CCM_MTX_22_23_RS: c_uint = 0x60Ac;

pub const ISP_PST_GAMMA_LUT_ADDR: c_uint = 0x60cc;

pub const ISP_PST_GAMMA_LUT_DATA: c_uint = 0x60d0;

pub const DISP0_TOP_TOP_CTRL: c_uint = 0x8000;

pub const DISP0_TOP_CRP2_START: c_uint = 0x8004;

pub const DISP0_TOP_CRP2_SIZE: c_uint = 0x8008;

pub const DISP0_TOP_OUT_SIZE: c_uint = 0x800c;

pub const ISP_DISP0_TOP_IN_SIZE: c_uint = 0x804c;

pub const DISP0_PPS_SCALE_EN: c_uint = 0x8200;

pub const DISP0_PPS_VSC_START_PHASE_STEP: c_uint = 0x8224;

pub const DISP0_PPS_HSC_START_PHASE_STEP: c_uint = 0x8230;

pub const DISP0_PPS_444TO422: c_uint = 0x823c;

pub const ISP_SCALE0_COEF_IDX_LUMA: c_uint = 0x8240;

pub const ISP_SCALE0_COEF_LUMA: c_uint = 0x8244;

pub const ISP_SCALE0_COEF_IDX_CHRO: c_uint = 0x8248;

pub const ISP_SCALE0_COEF_CHRO: c_uint = 0x824c;

pub const ISP_AF_CTRL: c_uint = 0xa044;

pub const ISP_AF_HV_SIZE: c_uint = 0xa04c;

pub const ISP_AF_HV_BLKNUM: c_uint = 0xa050;

pub const ISP_AF_EN_CTRL: c_uint = 0xa054;

pub const ISP_AF_IDX_ADDR: c_uint = 0xa1c0;
pub const ISP_AF_IDX_DATA: c_uint = 0xa1c4;

pub const ISP_AE_CTRL: c_uint = 0xa448;

pub const ISP_AE_HV_SIZE: c_uint = 0xa464;

pub const ISP_AE_HV_BLKNUM: c_uint = 0xa468;

pub const ISP_AE_IDX_ADDR: c_uint = 0xa600;
pub const ISP_AE_IDX_DATA: c_uint = 0xa604;

pub const ISP_AE_BLK_WT_ADDR: c_uint = 0xa608;
pub const ISP_AE_BLK_WT_DATA: c_uint = 0xa60c;

pub const ISP_AWB_CTRL: c_uint = 0xa834;

pub const ISP_AWB_HV_SIZE: c_uint = 0xa83c;

pub const ISP_AWB_HV_BLKNUM: c_uint = 0xa840;

pub const ISP_AWB_STAT_RG: c_uint = 0xa848;

pub const ISP_AWB_STAT_BG: c_uint = 0xa84c;

pub const ISP_AWB_STAT_RG_HL: c_uint = 0xa850;

pub const ISP_AWB_STAT_BG_HL: c_uint = 0xa854;

pub const ISP_AWB_STAT_CTRL2: c_uint = 0xa858;

pub const ISP_AWB_IDX_ADDR: c_uint = 0xaa00;
pub const ISP_AWB_IDX_DATA: c_uint = 0xaa04;

pub const ISP_AWB_BLK_WT_ADDR: c_uint = 0xaa08;
pub const ISP_AWB_BLK_WT_DATA: c_uint = 0xaa0c;

pub const ISP_WRMIFX3_0_CH0_CTRL0: c_uint = 0xc400;

pub const ISP_WRMIFX3_0_CH0_CTRL1: c_uint = 0xc404;

pub const ISP_WRMIFX3_0_CH1_CTRL0: c_uint = 0xc408;

pub const ISP_WRMIFX3_0_CH1_CTRL1: c_uint = 0xc40c;

pub const ISP_WRMIFX3_0_WIN_LUMA_H: c_uint = 0xc420;

pub const ISP_WRMIFX3_0_WIN_LUMA_V: c_uint = 0xc424;

pub const ISP_WRMIFX3_0_WIN_CHROM_H: c_uint = 0xc428;

pub const ISP_WRMIFX3_0_WIN_CHROM_V: c_uint = 0xc42c;

pub const ISP_WRMIFX3_0_CH0_BADDR: c_uint = 0xc440;

pub const ISP_WRMIFX3_0_CH1_BADDR: c_uint = 0xc444;

pub const ISP_WRMIFX3_0_FMT_SIZE: c_uint = 0xc464;

pub const ISP_WRMIFX3_0_FMT_CTRL: c_uint = 0xc468;

pub const VIU_DMAWR_BADDR0: c_uint = 0xc840;

pub const VIU_DMAWR_BADDR1: c_uint = 0xc844;

pub const VIU_DMAWR_BADDR2: c_uint = 0xc848;

pub const VIU_DMAWR_SIZE0: c_uint = 0xc854;

pub const VIU_DMAWR_SIZE1: c_uint = 0xc858;

