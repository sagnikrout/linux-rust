//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc5_rtr_regs.h
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
// TPC5_RTR (Prototype: TPC_RTR)
//
pub const mmTPC5_RTR_HBW_RD_RQ_E_ARB: c_uint = 0xF40100;
pub const mmTPC5_RTR_HBW_RD_RQ_W_ARB: c_uint = 0xF40104;
pub const mmTPC5_RTR_HBW_RD_RQ_N_ARB: c_uint = 0xF40108;
pub const mmTPC5_RTR_HBW_RD_RQ_S_ARB: c_uint = 0xF4010C;
pub const mmTPC5_RTR_HBW_RD_RQ_L_ARB: c_uint = 0xF40110;
pub const mmTPC5_RTR_HBW_E_ARB_MAX: c_uint = 0xF40120;
pub const mmTPC5_RTR_HBW_W_ARB_MAX: c_uint = 0xF40124;
pub const mmTPC5_RTR_HBW_N_ARB_MAX: c_uint = 0xF40128;
pub const mmTPC5_RTR_HBW_S_ARB_MAX: c_uint = 0xF4012C;
pub const mmTPC5_RTR_HBW_L_ARB_MAX: c_uint = 0xF40130;
pub const mmTPC5_RTR_HBW_RD_RS_E_ARB: c_uint = 0xF40140;
pub const mmTPC5_RTR_HBW_RD_RS_W_ARB: c_uint = 0xF40144;
pub const mmTPC5_RTR_HBW_RD_RS_N_ARB: c_uint = 0xF40148;
pub const mmTPC5_RTR_HBW_RD_RS_S_ARB: c_uint = 0xF4014C;
pub const mmTPC5_RTR_HBW_RD_RS_L_ARB: c_uint = 0xF40150;
pub const mmTPC5_RTR_HBW_WR_RQ_E_ARB: c_uint = 0xF40170;
pub const mmTPC5_RTR_HBW_WR_RQ_W_ARB: c_uint = 0xF40174;
pub const mmTPC5_RTR_HBW_WR_RQ_N_ARB: c_uint = 0xF40178;
pub const mmTPC5_RTR_HBW_WR_RQ_S_ARB: c_uint = 0xF4017C;
pub const mmTPC5_RTR_HBW_WR_RQ_L_ARB: c_uint = 0xF40180;
pub const mmTPC5_RTR_HBW_WR_RS_E_ARB: c_uint = 0xF40190;
pub const mmTPC5_RTR_HBW_WR_RS_W_ARB: c_uint = 0xF40194;
pub const mmTPC5_RTR_HBW_WR_RS_N_ARB: c_uint = 0xF40198;
pub const mmTPC5_RTR_HBW_WR_RS_S_ARB: c_uint = 0xF4019C;
pub const mmTPC5_RTR_HBW_WR_RS_L_ARB: c_uint = 0xF401A0;
pub const mmTPC5_RTR_LBW_RD_RQ_E_ARB: c_uint = 0xF40200;
pub const mmTPC5_RTR_LBW_RD_RQ_W_ARB: c_uint = 0xF40204;
pub const mmTPC5_RTR_LBW_RD_RQ_N_ARB: c_uint = 0xF40208;
pub const mmTPC5_RTR_LBW_RD_RQ_S_ARB: c_uint = 0xF4020C;
pub const mmTPC5_RTR_LBW_RD_RQ_L_ARB: c_uint = 0xF40210;
pub const mmTPC5_RTR_LBW_E_ARB_MAX: c_uint = 0xF40220;
pub const mmTPC5_RTR_LBW_W_ARB_MAX: c_uint = 0xF40224;
pub const mmTPC5_RTR_LBW_N_ARB_MAX: c_uint = 0xF40228;
pub const mmTPC5_RTR_LBW_S_ARB_MAX: c_uint = 0xF4022C;
pub const mmTPC5_RTR_LBW_L_ARB_MAX: c_uint = 0xF40230;
pub const mmTPC5_RTR_LBW_RD_RS_E_ARB: c_uint = 0xF40250;
pub const mmTPC5_RTR_LBW_RD_RS_W_ARB: c_uint = 0xF40254;
pub const mmTPC5_RTR_LBW_RD_RS_N_ARB: c_uint = 0xF40258;
pub const mmTPC5_RTR_LBW_RD_RS_S_ARB: c_uint = 0xF4025C;
pub const mmTPC5_RTR_LBW_RD_RS_L_ARB: c_uint = 0xF40260;
pub const mmTPC5_RTR_LBW_WR_RQ_E_ARB: c_uint = 0xF40270;
pub const mmTPC5_RTR_LBW_WR_RQ_W_ARB: c_uint = 0xF40274;
pub const mmTPC5_RTR_LBW_WR_RQ_N_ARB: c_uint = 0xF40278;
pub const mmTPC5_RTR_LBW_WR_RQ_S_ARB: c_uint = 0xF4027C;
pub const mmTPC5_RTR_LBW_WR_RQ_L_ARB: c_uint = 0xF40280;
pub const mmTPC5_RTR_LBW_WR_RS_E_ARB: c_uint = 0xF40290;
pub const mmTPC5_RTR_LBW_WR_RS_W_ARB: c_uint = 0xF40294;
pub const mmTPC5_RTR_LBW_WR_RS_N_ARB: c_uint = 0xF40298;
pub const mmTPC5_RTR_LBW_WR_RS_S_ARB: c_uint = 0xF4029C;
pub const mmTPC5_RTR_LBW_WR_RS_L_ARB: c_uint = 0xF402A0;
pub const mmTPC5_RTR_DBG_E_ARB: c_uint = 0xF40300;
pub const mmTPC5_RTR_DBG_W_ARB: c_uint = 0xF40304;
pub const mmTPC5_RTR_DBG_N_ARB: c_uint = 0xF40308;
pub const mmTPC5_RTR_DBG_S_ARB: c_uint = 0xF4030C;
pub const mmTPC5_RTR_DBG_L_ARB: c_uint = 0xF40310;
pub const mmTPC5_RTR_DBG_E_ARB_MAX: c_uint = 0xF40320;
pub const mmTPC5_RTR_DBG_W_ARB_MAX: c_uint = 0xF40324;
pub const mmTPC5_RTR_DBG_N_ARB_MAX: c_uint = 0xF40328;
pub const mmTPC5_RTR_DBG_S_ARB_MAX: c_uint = 0xF4032C;
pub const mmTPC5_RTR_DBG_L_ARB_MAX: c_uint = 0xF40330;
pub const mmTPC5_RTR_SPLIT_COEF_0: c_uint = 0xF40400;
pub const mmTPC5_RTR_SPLIT_COEF_1: c_uint = 0xF40404;
pub const mmTPC5_RTR_SPLIT_COEF_2: c_uint = 0xF40408;
pub const mmTPC5_RTR_SPLIT_COEF_3: c_uint = 0xF4040C;
pub const mmTPC5_RTR_SPLIT_COEF_4: c_uint = 0xF40410;
pub const mmTPC5_RTR_SPLIT_COEF_5: c_uint = 0xF40414;
pub const mmTPC5_RTR_SPLIT_COEF_6: c_uint = 0xF40418;
pub const mmTPC5_RTR_SPLIT_COEF_7: c_uint = 0xF4041C;
pub const mmTPC5_RTR_SPLIT_COEF_8: c_uint = 0xF40420;
pub const mmTPC5_RTR_SPLIT_COEF_9: c_uint = 0xF40424;
pub const mmTPC5_RTR_SPLIT_CFG: c_uint = 0xF40440;
pub const mmTPC5_RTR_SPLIT_RD_SAT: c_uint = 0xF40444;
pub const mmTPC5_RTR_SPLIT_RD_RST_TOKEN: c_uint = 0xF40448;
pub const mmTPC5_RTR_SPLIT_RD_TIMEOUT_0: c_uint = 0xF4044C;
pub const mmTPC5_RTR_SPLIT_RD_TIMEOUT_1: c_uint = 0xF40450;
pub const mmTPC5_RTR_SPLIT_WR_SAT: c_uint = 0xF40454;
pub const mmTPC5_RTR_WPLIT_WR_TST_TOLEN: c_uint = 0xF40458;
pub const mmTPC5_RTR_SPLIT_WR_TIMEOUT_0: c_uint = 0xF4045C;
pub const mmTPC5_RTR_SPLIT_WR_TIMEOUT_1: c_uint = 0xF40460;
pub const mmTPC5_RTR_HBW_RANGE_HIT: c_uint = 0xF40470;
pub const mmTPC5_RTR_HBW_RANGE_MASK_L_0: c_uint = 0xF40480;
pub const mmTPC5_RTR_HBW_RANGE_MASK_L_1: c_uint = 0xF40484;
pub const mmTPC5_RTR_HBW_RANGE_MASK_L_2: c_uint = 0xF40488;
pub const mmTPC5_RTR_HBW_RANGE_MASK_L_3: c_uint = 0xF4048C;
pub const mmTPC5_RTR_HBW_RANGE_MASK_L_4: c_uint = 0xF40490;
pub const mmTPC5_RTR_HBW_RANGE_MASK_L_5: c_uint = 0xF40494;
pub const mmTPC5_RTR_HBW_RANGE_MASK_L_6: c_uint = 0xF40498;
pub const mmTPC5_RTR_HBW_RANGE_MASK_L_7: c_uint = 0xF4049C;
pub const mmTPC5_RTR_HBW_RANGE_MASK_H_0: c_uint = 0xF404A0;
pub const mmTPC5_RTR_HBW_RANGE_MASK_H_1: c_uint = 0xF404A4;
pub const mmTPC5_RTR_HBW_RANGE_MASK_H_2: c_uint = 0xF404A8;
pub const mmTPC5_RTR_HBW_RANGE_MASK_H_3: c_uint = 0xF404AC;
pub const mmTPC5_RTR_HBW_RANGE_MASK_H_4: c_uint = 0xF404B0;
pub const mmTPC5_RTR_HBW_RANGE_MASK_H_5: c_uint = 0xF404B4;
pub const mmTPC5_RTR_HBW_RANGE_MASK_H_6: c_uint = 0xF404B8;
pub const mmTPC5_RTR_HBW_RANGE_MASK_H_7: c_uint = 0xF404BC;
pub const mmTPC5_RTR_HBW_RANGE_BASE_L_0: c_uint = 0xF404C0;
pub const mmTPC5_RTR_HBW_RANGE_BASE_L_1: c_uint = 0xF404C4;
pub const mmTPC5_RTR_HBW_RANGE_BASE_L_2: c_uint = 0xF404C8;
pub const mmTPC5_RTR_HBW_RANGE_BASE_L_3: c_uint = 0xF404CC;
pub const mmTPC5_RTR_HBW_RANGE_BASE_L_4: c_uint = 0xF404D0;
pub const mmTPC5_RTR_HBW_RANGE_BASE_L_5: c_uint = 0xF404D4;
pub const mmTPC5_RTR_HBW_RANGE_BASE_L_6: c_uint = 0xF404D8;
pub const mmTPC5_RTR_HBW_RANGE_BASE_L_7: c_uint = 0xF404DC;
pub const mmTPC5_RTR_HBW_RANGE_BASE_H_0: c_uint = 0xF404E0;
pub const mmTPC5_RTR_HBW_RANGE_BASE_H_1: c_uint = 0xF404E4;
pub const mmTPC5_RTR_HBW_RANGE_BASE_H_2: c_uint = 0xF404E8;
pub const mmTPC5_RTR_HBW_RANGE_BASE_H_3: c_uint = 0xF404EC;
pub const mmTPC5_RTR_HBW_RANGE_BASE_H_4: c_uint = 0xF404F0;
pub const mmTPC5_RTR_HBW_RANGE_BASE_H_5: c_uint = 0xF404F4;
pub const mmTPC5_RTR_HBW_RANGE_BASE_H_6: c_uint = 0xF404F8;
pub const mmTPC5_RTR_HBW_RANGE_BASE_H_7: c_uint = 0xF404FC;
pub const mmTPC5_RTR_LBW_RANGE_HIT: c_uint = 0xF40500;
pub const mmTPC5_RTR_LBW_RANGE_MASK_0: c_uint = 0xF40510;
pub const mmTPC5_RTR_LBW_RANGE_MASK_1: c_uint = 0xF40514;
pub const mmTPC5_RTR_LBW_RANGE_MASK_2: c_uint = 0xF40518;
pub const mmTPC5_RTR_LBW_RANGE_MASK_3: c_uint = 0xF4051C;
pub const mmTPC5_RTR_LBW_RANGE_MASK_4: c_uint = 0xF40520;
pub const mmTPC5_RTR_LBW_RANGE_MASK_5: c_uint = 0xF40524;
pub const mmTPC5_RTR_LBW_RANGE_MASK_6: c_uint = 0xF40528;
pub const mmTPC5_RTR_LBW_RANGE_MASK_7: c_uint = 0xF4052C;
pub const mmTPC5_RTR_LBW_RANGE_MASK_8: c_uint = 0xF40530;
pub const mmTPC5_RTR_LBW_RANGE_MASK_9: c_uint = 0xF40534;
pub const mmTPC5_RTR_LBW_RANGE_MASK_10: c_uint = 0xF40538;
pub const mmTPC5_RTR_LBW_RANGE_MASK_11: c_uint = 0xF4053C;
pub const mmTPC5_RTR_LBW_RANGE_MASK_12: c_uint = 0xF40540;
pub const mmTPC5_RTR_LBW_RANGE_MASK_13: c_uint = 0xF40544;
pub const mmTPC5_RTR_LBW_RANGE_MASK_14: c_uint = 0xF40548;
pub const mmTPC5_RTR_LBW_RANGE_MASK_15: c_uint = 0xF4054C;
pub const mmTPC5_RTR_LBW_RANGE_BASE_0: c_uint = 0xF40550;
pub const mmTPC5_RTR_LBW_RANGE_BASE_1: c_uint = 0xF40554;
pub const mmTPC5_RTR_LBW_RANGE_BASE_2: c_uint = 0xF40558;
pub const mmTPC5_RTR_LBW_RANGE_BASE_3: c_uint = 0xF4055C;
pub const mmTPC5_RTR_LBW_RANGE_BASE_4: c_uint = 0xF40560;
pub const mmTPC5_RTR_LBW_RANGE_BASE_5: c_uint = 0xF40564;
pub const mmTPC5_RTR_LBW_RANGE_BASE_6: c_uint = 0xF40568;
pub const mmTPC5_RTR_LBW_RANGE_BASE_7: c_uint = 0xF4056C;
pub const mmTPC5_RTR_LBW_RANGE_BASE_8: c_uint = 0xF40570;
pub const mmTPC5_RTR_LBW_RANGE_BASE_9: c_uint = 0xF40574;
pub const mmTPC5_RTR_LBW_RANGE_BASE_10: c_uint = 0xF40578;
pub const mmTPC5_RTR_LBW_RANGE_BASE_11: c_uint = 0xF4057C;
pub const mmTPC5_RTR_LBW_RANGE_BASE_12: c_uint = 0xF40580;
pub const mmTPC5_RTR_LBW_RANGE_BASE_13: c_uint = 0xF40584;
pub const mmTPC5_RTR_LBW_RANGE_BASE_14: c_uint = 0xF40588;
pub const mmTPC5_RTR_LBW_RANGE_BASE_15: c_uint = 0xF4058C;
pub const mmTPC5_RTR_RGLTR: c_uint = 0xF40590;
pub const mmTPC5_RTR_RGLTR_WR_RESULT: c_uint = 0xF40594;
pub const mmTPC5_RTR_RGLTR_RD_RESULT: c_uint = 0xF40598;
pub const mmTPC5_RTR_SCRAMB_EN: c_uint = 0xF40600;
pub const mmTPC5_RTR_NON_LIN_SCRAMB: c_uint = 0xF40604;
