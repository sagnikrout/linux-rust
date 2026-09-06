//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc4_rtr_regs.h
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
// TPC4_RTR (Prototype: TPC_RTR)
//
pub const mmTPC4_RTR_HBW_RD_RQ_E_ARB: c_uint = 0xF00100;
pub const mmTPC4_RTR_HBW_RD_RQ_W_ARB: c_uint = 0xF00104;
pub const mmTPC4_RTR_HBW_RD_RQ_N_ARB: c_uint = 0xF00108;
pub const mmTPC4_RTR_HBW_RD_RQ_S_ARB: c_uint = 0xF0010C;
pub const mmTPC4_RTR_HBW_RD_RQ_L_ARB: c_uint = 0xF00110;
pub const mmTPC4_RTR_HBW_E_ARB_MAX: c_uint = 0xF00120;
pub const mmTPC4_RTR_HBW_W_ARB_MAX: c_uint = 0xF00124;
pub const mmTPC4_RTR_HBW_N_ARB_MAX: c_uint = 0xF00128;
pub const mmTPC4_RTR_HBW_S_ARB_MAX: c_uint = 0xF0012C;
pub const mmTPC4_RTR_HBW_L_ARB_MAX: c_uint = 0xF00130;
pub const mmTPC4_RTR_HBW_RD_RS_E_ARB: c_uint = 0xF00140;
pub const mmTPC4_RTR_HBW_RD_RS_W_ARB: c_uint = 0xF00144;
pub const mmTPC4_RTR_HBW_RD_RS_N_ARB: c_uint = 0xF00148;
pub const mmTPC4_RTR_HBW_RD_RS_S_ARB: c_uint = 0xF0014C;
pub const mmTPC4_RTR_HBW_RD_RS_L_ARB: c_uint = 0xF00150;
pub const mmTPC4_RTR_HBW_WR_RQ_E_ARB: c_uint = 0xF00170;
pub const mmTPC4_RTR_HBW_WR_RQ_W_ARB: c_uint = 0xF00174;
pub const mmTPC4_RTR_HBW_WR_RQ_N_ARB: c_uint = 0xF00178;
pub const mmTPC4_RTR_HBW_WR_RQ_S_ARB: c_uint = 0xF0017C;
pub const mmTPC4_RTR_HBW_WR_RQ_L_ARB: c_uint = 0xF00180;
pub const mmTPC4_RTR_HBW_WR_RS_E_ARB: c_uint = 0xF00190;
pub const mmTPC4_RTR_HBW_WR_RS_W_ARB: c_uint = 0xF00194;
pub const mmTPC4_RTR_HBW_WR_RS_N_ARB: c_uint = 0xF00198;
pub const mmTPC4_RTR_HBW_WR_RS_S_ARB: c_uint = 0xF0019C;
pub const mmTPC4_RTR_HBW_WR_RS_L_ARB: c_uint = 0xF001A0;
pub const mmTPC4_RTR_LBW_RD_RQ_E_ARB: c_uint = 0xF00200;
pub const mmTPC4_RTR_LBW_RD_RQ_W_ARB: c_uint = 0xF00204;
pub const mmTPC4_RTR_LBW_RD_RQ_N_ARB: c_uint = 0xF00208;
pub const mmTPC4_RTR_LBW_RD_RQ_S_ARB: c_uint = 0xF0020C;
pub const mmTPC4_RTR_LBW_RD_RQ_L_ARB: c_uint = 0xF00210;
pub const mmTPC4_RTR_LBW_E_ARB_MAX: c_uint = 0xF00220;
pub const mmTPC4_RTR_LBW_W_ARB_MAX: c_uint = 0xF00224;
pub const mmTPC4_RTR_LBW_N_ARB_MAX: c_uint = 0xF00228;
pub const mmTPC4_RTR_LBW_S_ARB_MAX: c_uint = 0xF0022C;
pub const mmTPC4_RTR_LBW_L_ARB_MAX: c_uint = 0xF00230;
pub const mmTPC4_RTR_LBW_RD_RS_E_ARB: c_uint = 0xF00250;
pub const mmTPC4_RTR_LBW_RD_RS_W_ARB: c_uint = 0xF00254;
pub const mmTPC4_RTR_LBW_RD_RS_N_ARB: c_uint = 0xF00258;
pub const mmTPC4_RTR_LBW_RD_RS_S_ARB: c_uint = 0xF0025C;
pub const mmTPC4_RTR_LBW_RD_RS_L_ARB: c_uint = 0xF00260;
pub const mmTPC4_RTR_LBW_WR_RQ_E_ARB: c_uint = 0xF00270;
pub const mmTPC4_RTR_LBW_WR_RQ_W_ARB: c_uint = 0xF00274;
pub const mmTPC4_RTR_LBW_WR_RQ_N_ARB: c_uint = 0xF00278;
pub const mmTPC4_RTR_LBW_WR_RQ_S_ARB: c_uint = 0xF0027C;
pub const mmTPC4_RTR_LBW_WR_RQ_L_ARB: c_uint = 0xF00280;
pub const mmTPC4_RTR_LBW_WR_RS_E_ARB: c_uint = 0xF00290;
pub const mmTPC4_RTR_LBW_WR_RS_W_ARB: c_uint = 0xF00294;
pub const mmTPC4_RTR_LBW_WR_RS_N_ARB: c_uint = 0xF00298;
pub const mmTPC4_RTR_LBW_WR_RS_S_ARB: c_uint = 0xF0029C;
pub const mmTPC4_RTR_LBW_WR_RS_L_ARB: c_uint = 0xF002A0;
pub const mmTPC4_RTR_DBG_E_ARB: c_uint = 0xF00300;
pub const mmTPC4_RTR_DBG_W_ARB: c_uint = 0xF00304;
pub const mmTPC4_RTR_DBG_N_ARB: c_uint = 0xF00308;
pub const mmTPC4_RTR_DBG_S_ARB: c_uint = 0xF0030C;
pub const mmTPC4_RTR_DBG_L_ARB: c_uint = 0xF00310;
pub const mmTPC4_RTR_DBG_E_ARB_MAX: c_uint = 0xF00320;
pub const mmTPC4_RTR_DBG_W_ARB_MAX: c_uint = 0xF00324;
pub const mmTPC4_RTR_DBG_N_ARB_MAX: c_uint = 0xF00328;
pub const mmTPC4_RTR_DBG_S_ARB_MAX: c_uint = 0xF0032C;
pub const mmTPC4_RTR_DBG_L_ARB_MAX: c_uint = 0xF00330;
pub const mmTPC4_RTR_SPLIT_COEF_0: c_uint = 0xF00400;
pub const mmTPC4_RTR_SPLIT_COEF_1: c_uint = 0xF00404;
pub const mmTPC4_RTR_SPLIT_COEF_2: c_uint = 0xF00408;
pub const mmTPC4_RTR_SPLIT_COEF_3: c_uint = 0xF0040C;
pub const mmTPC4_RTR_SPLIT_COEF_4: c_uint = 0xF00410;
pub const mmTPC4_RTR_SPLIT_COEF_5: c_uint = 0xF00414;
pub const mmTPC4_RTR_SPLIT_COEF_6: c_uint = 0xF00418;
pub const mmTPC4_RTR_SPLIT_COEF_7: c_uint = 0xF0041C;
pub const mmTPC4_RTR_SPLIT_COEF_8: c_uint = 0xF00420;
pub const mmTPC4_RTR_SPLIT_COEF_9: c_uint = 0xF00424;
pub const mmTPC4_RTR_SPLIT_CFG: c_uint = 0xF00440;
pub const mmTPC4_RTR_SPLIT_RD_SAT: c_uint = 0xF00444;
pub const mmTPC4_RTR_SPLIT_RD_RST_TOKEN: c_uint = 0xF00448;
pub const mmTPC4_RTR_SPLIT_RD_TIMEOUT_0: c_uint = 0xF0044C;
pub const mmTPC4_RTR_SPLIT_RD_TIMEOUT_1: c_uint = 0xF00450;
pub const mmTPC4_RTR_SPLIT_WR_SAT: c_uint = 0xF00454;
pub const mmTPC4_RTR_WPLIT_WR_TST_TOLEN: c_uint = 0xF00458;
pub const mmTPC4_RTR_SPLIT_WR_TIMEOUT_0: c_uint = 0xF0045C;
pub const mmTPC4_RTR_SPLIT_WR_TIMEOUT_1: c_uint = 0xF00460;
pub const mmTPC4_RTR_HBW_RANGE_HIT: c_uint = 0xF00470;
pub const mmTPC4_RTR_HBW_RANGE_MASK_L_0: c_uint = 0xF00480;
pub const mmTPC4_RTR_HBW_RANGE_MASK_L_1: c_uint = 0xF00484;
pub const mmTPC4_RTR_HBW_RANGE_MASK_L_2: c_uint = 0xF00488;
pub const mmTPC4_RTR_HBW_RANGE_MASK_L_3: c_uint = 0xF0048C;
pub const mmTPC4_RTR_HBW_RANGE_MASK_L_4: c_uint = 0xF00490;
pub const mmTPC4_RTR_HBW_RANGE_MASK_L_5: c_uint = 0xF00494;
pub const mmTPC4_RTR_HBW_RANGE_MASK_L_6: c_uint = 0xF00498;
pub const mmTPC4_RTR_HBW_RANGE_MASK_L_7: c_uint = 0xF0049C;
pub const mmTPC4_RTR_HBW_RANGE_MASK_H_0: c_uint = 0xF004A0;
pub const mmTPC4_RTR_HBW_RANGE_MASK_H_1: c_uint = 0xF004A4;
pub const mmTPC4_RTR_HBW_RANGE_MASK_H_2: c_uint = 0xF004A8;
pub const mmTPC4_RTR_HBW_RANGE_MASK_H_3: c_uint = 0xF004AC;
pub const mmTPC4_RTR_HBW_RANGE_MASK_H_4: c_uint = 0xF004B0;
pub const mmTPC4_RTR_HBW_RANGE_MASK_H_5: c_uint = 0xF004B4;
pub const mmTPC4_RTR_HBW_RANGE_MASK_H_6: c_uint = 0xF004B8;
pub const mmTPC4_RTR_HBW_RANGE_MASK_H_7: c_uint = 0xF004BC;
pub const mmTPC4_RTR_HBW_RANGE_BASE_L_0: c_uint = 0xF004C0;
pub const mmTPC4_RTR_HBW_RANGE_BASE_L_1: c_uint = 0xF004C4;
pub const mmTPC4_RTR_HBW_RANGE_BASE_L_2: c_uint = 0xF004C8;
pub const mmTPC4_RTR_HBW_RANGE_BASE_L_3: c_uint = 0xF004CC;
pub const mmTPC4_RTR_HBW_RANGE_BASE_L_4: c_uint = 0xF004D0;
pub const mmTPC4_RTR_HBW_RANGE_BASE_L_5: c_uint = 0xF004D4;
pub const mmTPC4_RTR_HBW_RANGE_BASE_L_6: c_uint = 0xF004D8;
pub const mmTPC4_RTR_HBW_RANGE_BASE_L_7: c_uint = 0xF004DC;
pub const mmTPC4_RTR_HBW_RANGE_BASE_H_0: c_uint = 0xF004E0;
pub const mmTPC4_RTR_HBW_RANGE_BASE_H_1: c_uint = 0xF004E4;
pub const mmTPC4_RTR_HBW_RANGE_BASE_H_2: c_uint = 0xF004E8;
pub const mmTPC4_RTR_HBW_RANGE_BASE_H_3: c_uint = 0xF004EC;
pub const mmTPC4_RTR_HBW_RANGE_BASE_H_4: c_uint = 0xF004F0;
pub const mmTPC4_RTR_HBW_RANGE_BASE_H_5: c_uint = 0xF004F4;
pub const mmTPC4_RTR_HBW_RANGE_BASE_H_6: c_uint = 0xF004F8;
pub const mmTPC4_RTR_HBW_RANGE_BASE_H_7: c_uint = 0xF004FC;
pub const mmTPC4_RTR_LBW_RANGE_HIT: c_uint = 0xF00500;
pub const mmTPC4_RTR_LBW_RANGE_MASK_0: c_uint = 0xF00510;
pub const mmTPC4_RTR_LBW_RANGE_MASK_1: c_uint = 0xF00514;
pub const mmTPC4_RTR_LBW_RANGE_MASK_2: c_uint = 0xF00518;
pub const mmTPC4_RTR_LBW_RANGE_MASK_3: c_uint = 0xF0051C;
pub const mmTPC4_RTR_LBW_RANGE_MASK_4: c_uint = 0xF00520;
pub const mmTPC4_RTR_LBW_RANGE_MASK_5: c_uint = 0xF00524;
pub const mmTPC4_RTR_LBW_RANGE_MASK_6: c_uint = 0xF00528;
pub const mmTPC4_RTR_LBW_RANGE_MASK_7: c_uint = 0xF0052C;
pub const mmTPC4_RTR_LBW_RANGE_MASK_8: c_uint = 0xF00530;
pub const mmTPC4_RTR_LBW_RANGE_MASK_9: c_uint = 0xF00534;
pub const mmTPC4_RTR_LBW_RANGE_MASK_10: c_uint = 0xF00538;
pub const mmTPC4_RTR_LBW_RANGE_MASK_11: c_uint = 0xF0053C;
pub const mmTPC4_RTR_LBW_RANGE_MASK_12: c_uint = 0xF00540;
pub const mmTPC4_RTR_LBW_RANGE_MASK_13: c_uint = 0xF00544;
pub const mmTPC4_RTR_LBW_RANGE_MASK_14: c_uint = 0xF00548;
pub const mmTPC4_RTR_LBW_RANGE_MASK_15: c_uint = 0xF0054C;
pub const mmTPC4_RTR_LBW_RANGE_BASE_0: c_uint = 0xF00550;
pub const mmTPC4_RTR_LBW_RANGE_BASE_1: c_uint = 0xF00554;
pub const mmTPC4_RTR_LBW_RANGE_BASE_2: c_uint = 0xF00558;
pub const mmTPC4_RTR_LBW_RANGE_BASE_3: c_uint = 0xF0055C;
pub const mmTPC4_RTR_LBW_RANGE_BASE_4: c_uint = 0xF00560;
pub const mmTPC4_RTR_LBW_RANGE_BASE_5: c_uint = 0xF00564;
pub const mmTPC4_RTR_LBW_RANGE_BASE_6: c_uint = 0xF00568;
pub const mmTPC4_RTR_LBW_RANGE_BASE_7: c_uint = 0xF0056C;
pub const mmTPC4_RTR_LBW_RANGE_BASE_8: c_uint = 0xF00570;
pub const mmTPC4_RTR_LBW_RANGE_BASE_9: c_uint = 0xF00574;
pub const mmTPC4_RTR_LBW_RANGE_BASE_10: c_uint = 0xF00578;
pub const mmTPC4_RTR_LBW_RANGE_BASE_11: c_uint = 0xF0057C;
pub const mmTPC4_RTR_LBW_RANGE_BASE_12: c_uint = 0xF00580;
pub const mmTPC4_RTR_LBW_RANGE_BASE_13: c_uint = 0xF00584;
pub const mmTPC4_RTR_LBW_RANGE_BASE_14: c_uint = 0xF00588;
pub const mmTPC4_RTR_LBW_RANGE_BASE_15: c_uint = 0xF0058C;
pub const mmTPC4_RTR_RGLTR: c_uint = 0xF00590;
pub const mmTPC4_RTR_RGLTR_WR_RESULT: c_uint = 0xF00594;
pub const mmTPC4_RTR_RGLTR_RD_RESULT: c_uint = 0xF00598;
pub const mmTPC4_RTR_SCRAMB_EN: c_uint = 0xF00600;
pub const mmTPC4_RTR_NON_LIN_SCRAMB: c_uint = 0xF00604;
