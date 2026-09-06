//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc7_nrtr_regs.h
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
// TPC7_NRTR (Prototype: IF_NRTR)
//
pub const mmTPC7_NRTR_HBW_MAX_CRED: c_uint = 0xFC0100;
pub const mmTPC7_NRTR_LBW_MAX_CRED: c_uint = 0xFC0120;
pub const mmTPC7_NRTR_DBG_E_ARB: c_uint = 0xFC0300;
pub const mmTPC7_NRTR_DBG_W_ARB: c_uint = 0xFC0304;
pub const mmTPC7_NRTR_DBG_N_ARB: c_uint = 0xFC0308;
pub const mmTPC7_NRTR_DBG_S_ARB: c_uint = 0xFC030C;
pub const mmTPC7_NRTR_DBG_L_ARB: c_uint = 0xFC0310;
pub const mmTPC7_NRTR_DBG_E_ARB_MAX: c_uint = 0xFC0320;
pub const mmTPC7_NRTR_DBG_W_ARB_MAX: c_uint = 0xFC0324;
pub const mmTPC7_NRTR_DBG_N_ARB_MAX: c_uint = 0xFC0328;
pub const mmTPC7_NRTR_DBG_S_ARB_MAX: c_uint = 0xFC032C;
pub const mmTPC7_NRTR_DBG_L_ARB_MAX: c_uint = 0xFC0330;
pub const mmTPC7_NRTR_SPLIT_COEF_0: c_uint = 0xFC0400;
pub const mmTPC7_NRTR_SPLIT_COEF_1: c_uint = 0xFC0404;
pub const mmTPC7_NRTR_SPLIT_COEF_2: c_uint = 0xFC0408;
pub const mmTPC7_NRTR_SPLIT_COEF_3: c_uint = 0xFC040C;
pub const mmTPC7_NRTR_SPLIT_COEF_4: c_uint = 0xFC0410;
pub const mmTPC7_NRTR_SPLIT_COEF_5: c_uint = 0xFC0414;
pub const mmTPC7_NRTR_SPLIT_COEF_6: c_uint = 0xFC0418;
pub const mmTPC7_NRTR_SPLIT_COEF_7: c_uint = 0xFC041C;
pub const mmTPC7_NRTR_SPLIT_COEF_8: c_uint = 0xFC0420;
pub const mmTPC7_NRTR_SPLIT_COEF_9: c_uint = 0xFC0424;
pub const mmTPC7_NRTR_SPLIT_CFG: c_uint = 0xFC0440;
pub const mmTPC7_NRTR_SPLIT_RD_SAT: c_uint = 0xFC0444;
pub const mmTPC7_NRTR_SPLIT_RD_RST_TOKEN: c_uint = 0xFC0448;
pub const mmTPC7_NRTR_SPLIT_RD_TIMEOUT_0: c_uint = 0xFC044C;
pub const mmTPC7_NRTR_SPLIT_RD_TIMEOUT_1: c_uint = 0xFC0450;
pub const mmTPC7_NRTR_SPLIT_WR_SAT: c_uint = 0xFC0454;
pub const mmTPC7_NRTR_WPLIT_WR_TST_TOLEN: c_uint = 0xFC0458;
pub const mmTPC7_NRTR_SPLIT_WR_TIMEOUT_0: c_uint = 0xFC045C;
pub const mmTPC7_NRTR_SPLIT_WR_TIMEOUT_1: c_uint = 0xFC0460;
pub const mmTPC7_NRTR_HBW_RANGE_HIT: c_uint = 0xFC0470;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_L_0: c_uint = 0xFC0480;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_L_1: c_uint = 0xFC0484;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_L_2: c_uint = 0xFC0488;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_L_3: c_uint = 0xFC048C;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_L_4: c_uint = 0xFC0490;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_L_5: c_uint = 0xFC0494;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_L_6: c_uint = 0xFC0498;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_L_7: c_uint = 0xFC049C;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_H_0: c_uint = 0xFC04A0;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_H_1: c_uint = 0xFC04A4;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_H_2: c_uint = 0xFC04A8;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_H_3: c_uint = 0xFC04AC;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_H_4: c_uint = 0xFC04B0;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_H_5: c_uint = 0xFC04B4;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_H_6: c_uint = 0xFC04B8;
pub const mmTPC7_NRTR_HBW_RANGE_MASK_H_7: c_uint = 0xFC04BC;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_L_0: c_uint = 0xFC04C0;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_L_1: c_uint = 0xFC04C4;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_L_2: c_uint = 0xFC04C8;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_L_3: c_uint = 0xFC04CC;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_L_4: c_uint = 0xFC04D0;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_L_5: c_uint = 0xFC04D4;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_L_6: c_uint = 0xFC04D8;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_L_7: c_uint = 0xFC04DC;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_H_0: c_uint = 0xFC04E0;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_H_1: c_uint = 0xFC04E4;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_H_2: c_uint = 0xFC04E8;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_H_3: c_uint = 0xFC04EC;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_H_4: c_uint = 0xFC04F0;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_H_5: c_uint = 0xFC04F4;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_H_6: c_uint = 0xFC04F8;
pub const mmTPC7_NRTR_HBW_RANGE_BASE_H_7: c_uint = 0xFC04FC;
pub const mmTPC7_NRTR_LBW_RANGE_HIT: c_uint = 0xFC0500;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_0: c_uint = 0xFC0510;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_1: c_uint = 0xFC0514;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_2: c_uint = 0xFC0518;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_3: c_uint = 0xFC051C;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_4: c_uint = 0xFC0520;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_5: c_uint = 0xFC0524;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_6: c_uint = 0xFC0528;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_7: c_uint = 0xFC052C;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_8: c_uint = 0xFC0530;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_9: c_uint = 0xFC0534;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_10: c_uint = 0xFC0538;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_11: c_uint = 0xFC053C;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_12: c_uint = 0xFC0540;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_13: c_uint = 0xFC0544;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_14: c_uint = 0xFC0548;
pub const mmTPC7_NRTR_LBW_RANGE_MASK_15: c_uint = 0xFC054C;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_0: c_uint = 0xFC0550;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_1: c_uint = 0xFC0554;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_2: c_uint = 0xFC0558;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_3: c_uint = 0xFC055C;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_4: c_uint = 0xFC0560;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_5: c_uint = 0xFC0564;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_6: c_uint = 0xFC0568;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_7: c_uint = 0xFC056C;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_8: c_uint = 0xFC0570;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_9: c_uint = 0xFC0574;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_10: c_uint = 0xFC0578;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_11: c_uint = 0xFC057C;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_12: c_uint = 0xFC0580;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_13: c_uint = 0xFC0584;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_14: c_uint = 0xFC0588;
pub const mmTPC7_NRTR_LBW_RANGE_BASE_15: c_uint = 0xFC058C;
pub const mmTPC7_NRTR_RGLTR: c_uint = 0xFC0590;
pub const mmTPC7_NRTR_RGLTR_WR_RESULT: c_uint = 0xFC0594;
pub const mmTPC7_NRTR_RGLTR_RD_RESULT: c_uint = 0xFC0598;
pub const mmTPC7_NRTR_SCRAMB_EN: c_uint = 0xFC0600;
pub const mmTPC7_NRTR_NON_LIN_SCRAMB: c_uint = 0xFC0604;
