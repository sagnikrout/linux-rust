//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc0_nrtr_regs.h
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
// TPC0_NRTR (Prototype: IF_NRTR)
//
pub const mmTPC0_NRTR_HBW_MAX_CRED: c_uint = 0xE00100;
pub const mmTPC0_NRTR_LBW_MAX_CRED: c_uint = 0xE00120;
pub const mmTPC0_NRTR_DBG_E_ARB: c_uint = 0xE00300;
pub const mmTPC0_NRTR_DBG_W_ARB: c_uint = 0xE00304;
pub const mmTPC0_NRTR_DBG_N_ARB: c_uint = 0xE00308;
pub const mmTPC0_NRTR_DBG_S_ARB: c_uint = 0xE0030C;
pub const mmTPC0_NRTR_DBG_L_ARB: c_uint = 0xE00310;
pub const mmTPC0_NRTR_DBG_E_ARB_MAX: c_uint = 0xE00320;
pub const mmTPC0_NRTR_DBG_W_ARB_MAX: c_uint = 0xE00324;
pub const mmTPC0_NRTR_DBG_N_ARB_MAX: c_uint = 0xE00328;
pub const mmTPC0_NRTR_DBG_S_ARB_MAX: c_uint = 0xE0032C;
pub const mmTPC0_NRTR_DBG_L_ARB_MAX: c_uint = 0xE00330;
pub const mmTPC0_NRTR_SPLIT_COEF_0: c_uint = 0xE00400;
pub const mmTPC0_NRTR_SPLIT_COEF_1: c_uint = 0xE00404;
pub const mmTPC0_NRTR_SPLIT_COEF_2: c_uint = 0xE00408;
pub const mmTPC0_NRTR_SPLIT_COEF_3: c_uint = 0xE0040C;
pub const mmTPC0_NRTR_SPLIT_COEF_4: c_uint = 0xE00410;
pub const mmTPC0_NRTR_SPLIT_COEF_5: c_uint = 0xE00414;
pub const mmTPC0_NRTR_SPLIT_COEF_6: c_uint = 0xE00418;
pub const mmTPC0_NRTR_SPLIT_COEF_7: c_uint = 0xE0041C;
pub const mmTPC0_NRTR_SPLIT_COEF_8: c_uint = 0xE00420;
pub const mmTPC0_NRTR_SPLIT_COEF_9: c_uint = 0xE00424;
pub const mmTPC0_NRTR_SPLIT_CFG: c_uint = 0xE00440;
pub const mmTPC0_NRTR_SPLIT_RD_SAT: c_uint = 0xE00444;
pub const mmTPC0_NRTR_SPLIT_RD_RST_TOKEN: c_uint = 0xE00448;
pub const mmTPC0_NRTR_SPLIT_RD_TIMEOUT_0: c_uint = 0xE0044C;
pub const mmTPC0_NRTR_SPLIT_RD_TIMEOUT_1: c_uint = 0xE00450;
pub const mmTPC0_NRTR_SPLIT_WR_SAT: c_uint = 0xE00454;
pub const mmTPC0_NRTR_WPLIT_WR_TST_TOLEN: c_uint = 0xE00458;
pub const mmTPC0_NRTR_SPLIT_WR_TIMEOUT_0: c_uint = 0xE0045C;
pub const mmTPC0_NRTR_SPLIT_WR_TIMEOUT_1: c_uint = 0xE00460;
pub const mmTPC0_NRTR_HBW_RANGE_HIT: c_uint = 0xE00470;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_L_0: c_uint = 0xE00480;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_L_1: c_uint = 0xE00484;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_L_2: c_uint = 0xE00488;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_L_3: c_uint = 0xE0048C;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_L_4: c_uint = 0xE00490;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_L_5: c_uint = 0xE00494;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_L_6: c_uint = 0xE00498;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_L_7: c_uint = 0xE0049C;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_H_0: c_uint = 0xE004A0;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_H_1: c_uint = 0xE004A4;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_H_2: c_uint = 0xE004A8;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_H_3: c_uint = 0xE004AC;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_H_4: c_uint = 0xE004B0;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_H_5: c_uint = 0xE004B4;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_H_6: c_uint = 0xE004B8;
pub const mmTPC0_NRTR_HBW_RANGE_MASK_H_7: c_uint = 0xE004BC;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_L_0: c_uint = 0xE004C0;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_L_1: c_uint = 0xE004C4;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_L_2: c_uint = 0xE004C8;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_L_3: c_uint = 0xE004CC;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_L_4: c_uint = 0xE004D0;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_L_5: c_uint = 0xE004D4;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_L_6: c_uint = 0xE004D8;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_L_7: c_uint = 0xE004DC;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_H_0: c_uint = 0xE004E0;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_H_1: c_uint = 0xE004E4;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_H_2: c_uint = 0xE004E8;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_H_3: c_uint = 0xE004EC;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_H_4: c_uint = 0xE004F0;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_H_5: c_uint = 0xE004F4;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_H_6: c_uint = 0xE004F8;
pub const mmTPC0_NRTR_HBW_RANGE_BASE_H_7: c_uint = 0xE004FC;
pub const mmTPC0_NRTR_LBW_RANGE_HIT: c_uint = 0xE00500;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_0: c_uint = 0xE00510;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_1: c_uint = 0xE00514;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_2: c_uint = 0xE00518;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_3: c_uint = 0xE0051C;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_4: c_uint = 0xE00520;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_5: c_uint = 0xE00524;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_6: c_uint = 0xE00528;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_7: c_uint = 0xE0052C;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_8: c_uint = 0xE00530;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_9: c_uint = 0xE00534;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_10: c_uint = 0xE00538;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_11: c_uint = 0xE0053C;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_12: c_uint = 0xE00540;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_13: c_uint = 0xE00544;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_14: c_uint = 0xE00548;
pub const mmTPC0_NRTR_LBW_RANGE_MASK_15: c_uint = 0xE0054C;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_0: c_uint = 0xE00550;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_1: c_uint = 0xE00554;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_2: c_uint = 0xE00558;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_3: c_uint = 0xE0055C;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_4: c_uint = 0xE00560;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_5: c_uint = 0xE00564;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_6: c_uint = 0xE00568;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_7: c_uint = 0xE0056C;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_8: c_uint = 0xE00570;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_9: c_uint = 0xE00574;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_10: c_uint = 0xE00578;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_11: c_uint = 0xE0057C;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_12: c_uint = 0xE00580;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_13: c_uint = 0xE00584;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_14: c_uint = 0xE00588;
pub const mmTPC0_NRTR_LBW_RANGE_BASE_15: c_uint = 0xE0058C;
pub const mmTPC0_NRTR_RGLTR: c_uint = 0xE00590;
pub const mmTPC0_NRTR_RGLTR_WR_RESULT: c_uint = 0xE00594;
pub const mmTPC0_NRTR_RGLTR_RD_RESULT: c_uint = 0xE00598;
pub const mmTPC0_NRTR_SCRAMB_EN: c_uint = 0xE00600;
pub const mmTPC0_NRTR_NON_LIN_SCRAMB: c_uint = 0xE00604;
