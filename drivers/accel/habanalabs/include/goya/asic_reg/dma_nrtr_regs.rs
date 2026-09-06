//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/dma_nrtr_regs.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// DMA_NRTR (Prototype: IF_NRTR)
//
pub const mmDMA_NRTR_HBW_MAX_CRED: c_uint = 0x1C0100;
pub const mmDMA_NRTR_LBW_MAX_CRED: c_uint = 0x1C0120;
pub const mmDMA_NRTR_DBG_E_ARB: c_uint = 0x1C0300;
pub const mmDMA_NRTR_DBG_W_ARB: c_uint = 0x1C0304;
pub const mmDMA_NRTR_DBG_N_ARB: c_uint = 0x1C0308;
pub const mmDMA_NRTR_DBG_S_ARB: c_uint = 0x1C030C;
pub const mmDMA_NRTR_DBG_L_ARB: c_uint = 0x1C0310;
pub const mmDMA_NRTR_DBG_E_ARB_MAX: c_uint = 0x1C0320;
pub const mmDMA_NRTR_DBG_W_ARB_MAX: c_uint = 0x1C0324;
pub const mmDMA_NRTR_DBG_N_ARB_MAX: c_uint = 0x1C0328;
pub const mmDMA_NRTR_DBG_S_ARB_MAX: c_uint = 0x1C032C;
pub const mmDMA_NRTR_DBG_L_ARB_MAX: c_uint = 0x1C0330;
pub const mmDMA_NRTR_SPLIT_COEF_0: c_uint = 0x1C0400;
pub const mmDMA_NRTR_SPLIT_COEF_1: c_uint = 0x1C0404;
pub const mmDMA_NRTR_SPLIT_COEF_2: c_uint = 0x1C0408;
pub const mmDMA_NRTR_SPLIT_COEF_3: c_uint = 0x1C040C;
pub const mmDMA_NRTR_SPLIT_COEF_4: c_uint = 0x1C0410;
pub const mmDMA_NRTR_SPLIT_COEF_5: c_uint = 0x1C0414;
pub const mmDMA_NRTR_SPLIT_COEF_6: c_uint = 0x1C0418;
pub const mmDMA_NRTR_SPLIT_COEF_7: c_uint = 0x1C041C;
pub const mmDMA_NRTR_SPLIT_COEF_8: c_uint = 0x1C0420;
pub const mmDMA_NRTR_SPLIT_COEF_9: c_uint = 0x1C0424;
pub const mmDMA_NRTR_SPLIT_CFG: c_uint = 0x1C0440;
pub const mmDMA_NRTR_SPLIT_RD_SAT: c_uint = 0x1C0444;
pub const mmDMA_NRTR_SPLIT_RD_RST_TOKEN: c_uint = 0x1C0448;
pub const mmDMA_NRTR_SPLIT_RD_TIMEOUT_0: c_uint = 0x1C044C;
pub const mmDMA_NRTR_SPLIT_RD_TIMEOUT_1: c_uint = 0x1C0450;
pub const mmDMA_NRTR_SPLIT_WR_SAT: c_uint = 0x1C0454;
pub const mmDMA_NRTR_WPLIT_WR_TST_TOLEN: c_uint = 0x1C0458;
pub const mmDMA_NRTR_SPLIT_WR_TIMEOUT_0: c_uint = 0x1C045C;
pub const mmDMA_NRTR_SPLIT_WR_TIMEOUT_1: c_uint = 0x1C0460;
pub const mmDMA_NRTR_HBW_RANGE_HIT: c_uint = 0x1C0470;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_0: c_uint = 0x1C0480;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_1: c_uint = 0x1C0484;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_2: c_uint = 0x1C0488;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_3: c_uint = 0x1C048C;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_4: c_uint = 0x1C0490;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_5: c_uint = 0x1C0494;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_6: c_uint = 0x1C0498;
pub const mmDMA_NRTR_HBW_RANGE_MASK_L_7: c_uint = 0x1C049C;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_0: c_uint = 0x1C04A0;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_1: c_uint = 0x1C04A4;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_2: c_uint = 0x1C04A8;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_3: c_uint = 0x1C04AC;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_4: c_uint = 0x1C04B0;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_5: c_uint = 0x1C04B4;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_6: c_uint = 0x1C04B8;
pub const mmDMA_NRTR_HBW_RANGE_MASK_H_7: c_uint = 0x1C04BC;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_0: c_uint = 0x1C04C0;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_1: c_uint = 0x1C04C4;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_2: c_uint = 0x1C04C8;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_3: c_uint = 0x1C04CC;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_4: c_uint = 0x1C04D0;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_5: c_uint = 0x1C04D4;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_6: c_uint = 0x1C04D8;
pub const mmDMA_NRTR_HBW_RANGE_BASE_L_7: c_uint = 0x1C04DC;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_0: c_uint = 0x1C04E0;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_1: c_uint = 0x1C04E4;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_2: c_uint = 0x1C04E8;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_3: c_uint = 0x1C04EC;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_4: c_uint = 0x1C04F0;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_5: c_uint = 0x1C04F4;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_6: c_uint = 0x1C04F8;
pub const mmDMA_NRTR_HBW_RANGE_BASE_H_7: c_uint = 0x1C04FC;
pub const mmDMA_NRTR_LBW_RANGE_HIT: c_uint = 0x1C0500;
pub const mmDMA_NRTR_LBW_RANGE_MASK_0: c_uint = 0x1C0510;
pub const mmDMA_NRTR_LBW_RANGE_MASK_1: c_uint = 0x1C0514;
pub const mmDMA_NRTR_LBW_RANGE_MASK_2: c_uint = 0x1C0518;
pub const mmDMA_NRTR_LBW_RANGE_MASK_3: c_uint = 0x1C051C;
pub const mmDMA_NRTR_LBW_RANGE_MASK_4: c_uint = 0x1C0520;
pub const mmDMA_NRTR_LBW_RANGE_MASK_5: c_uint = 0x1C0524;
pub const mmDMA_NRTR_LBW_RANGE_MASK_6: c_uint = 0x1C0528;
pub const mmDMA_NRTR_LBW_RANGE_MASK_7: c_uint = 0x1C052C;
pub const mmDMA_NRTR_LBW_RANGE_MASK_8: c_uint = 0x1C0530;
pub const mmDMA_NRTR_LBW_RANGE_MASK_9: c_uint = 0x1C0534;
pub const mmDMA_NRTR_LBW_RANGE_MASK_10: c_uint = 0x1C0538;
pub const mmDMA_NRTR_LBW_RANGE_MASK_11: c_uint = 0x1C053C;
pub const mmDMA_NRTR_LBW_RANGE_MASK_12: c_uint = 0x1C0540;
pub const mmDMA_NRTR_LBW_RANGE_MASK_13: c_uint = 0x1C0544;
pub const mmDMA_NRTR_LBW_RANGE_MASK_14: c_uint = 0x1C0548;
pub const mmDMA_NRTR_LBW_RANGE_MASK_15: c_uint = 0x1C054C;
pub const mmDMA_NRTR_LBW_RANGE_BASE_0: c_uint = 0x1C0550;
pub const mmDMA_NRTR_LBW_RANGE_BASE_1: c_uint = 0x1C0554;
pub const mmDMA_NRTR_LBW_RANGE_BASE_2: c_uint = 0x1C0558;
pub const mmDMA_NRTR_LBW_RANGE_BASE_3: c_uint = 0x1C055C;
pub const mmDMA_NRTR_LBW_RANGE_BASE_4: c_uint = 0x1C0560;
pub const mmDMA_NRTR_LBW_RANGE_BASE_5: c_uint = 0x1C0564;
pub const mmDMA_NRTR_LBW_RANGE_BASE_6: c_uint = 0x1C0568;
pub const mmDMA_NRTR_LBW_RANGE_BASE_7: c_uint = 0x1C056C;
pub const mmDMA_NRTR_LBW_RANGE_BASE_8: c_uint = 0x1C0570;
pub const mmDMA_NRTR_LBW_RANGE_BASE_9: c_uint = 0x1C0574;
pub const mmDMA_NRTR_LBW_RANGE_BASE_10: c_uint = 0x1C0578;
pub const mmDMA_NRTR_LBW_RANGE_BASE_11: c_uint = 0x1C057C;
pub const mmDMA_NRTR_LBW_RANGE_BASE_12: c_uint = 0x1C0580;
pub const mmDMA_NRTR_LBW_RANGE_BASE_13: c_uint = 0x1C0584;
pub const mmDMA_NRTR_LBW_RANGE_BASE_14: c_uint = 0x1C0588;
pub const mmDMA_NRTR_LBW_RANGE_BASE_15: c_uint = 0x1C058C;
pub const mmDMA_NRTR_RGLTR: c_uint = 0x1C0590;
pub const mmDMA_NRTR_RGLTR_WR_RESULT: c_uint = 0x1C0594;
pub const mmDMA_NRTR_RGLTR_RD_RESULT: c_uint = 0x1C0598;
pub const mmDMA_NRTR_SCRAMB_EN: c_uint = 0x1C0600;
pub const mmDMA_NRTR_NON_LIN_SCRAMB: c_uint = 0x1C0604;
