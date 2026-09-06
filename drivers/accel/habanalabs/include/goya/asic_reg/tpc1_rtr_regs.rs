//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc1_rtr_regs.h
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
// TPC1_RTR (Prototype: TPC_RTR)
//
pub const mmTPC1_RTR_HBW_RD_RQ_E_ARB: c_uint = 0xE40100;
pub const mmTPC1_RTR_HBW_RD_RQ_W_ARB: c_uint = 0xE40104;
pub const mmTPC1_RTR_HBW_RD_RQ_N_ARB: c_uint = 0xE40108;
pub const mmTPC1_RTR_HBW_RD_RQ_S_ARB: c_uint = 0xE4010C;
pub const mmTPC1_RTR_HBW_RD_RQ_L_ARB: c_uint = 0xE40110;
pub const mmTPC1_RTR_HBW_E_ARB_MAX: c_uint = 0xE40120;
pub const mmTPC1_RTR_HBW_W_ARB_MAX: c_uint = 0xE40124;
pub const mmTPC1_RTR_HBW_N_ARB_MAX: c_uint = 0xE40128;
pub const mmTPC1_RTR_HBW_S_ARB_MAX: c_uint = 0xE4012C;
pub const mmTPC1_RTR_HBW_L_ARB_MAX: c_uint = 0xE40130;
pub const mmTPC1_RTR_HBW_RD_RS_E_ARB: c_uint = 0xE40140;
pub const mmTPC1_RTR_HBW_RD_RS_W_ARB: c_uint = 0xE40144;
pub const mmTPC1_RTR_HBW_RD_RS_N_ARB: c_uint = 0xE40148;
pub const mmTPC1_RTR_HBW_RD_RS_S_ARB: c_uint = 0xE4014C;
pub const mmTPC1_RTR_HBW_RD_RS_L_ARB: c_uint = 0xE40150;
pub const mmTPC1_RTR_HBW_WR_RQ_E_ARB: c_uint = 0xE40170;
pub const mmTPC1_RTR_HBW_WR_RQ_W_ARB: c_uint = 0xE40174;
pub const mmTPC1_RTR_HBW_WR_RQ_N_ARB: c_uint = 0xE40178;
pub const mmTPC1_RTR_HBW_WR_RQ_S_ARB: c_uint = 0xE4017C;
pub const mmTPC1_RTR_HBW_WR_RQ_L_ARB: c_uint = 0xE40180;
pub const mmTPC1_RTR_HBW_WR_RS_E_ARB: c_uint = 0xE40190;
pub const mmTPC1_RTR_HBW_WR_RS_W_ARB: c_uint = 0xE40194;
pub const mmTPC1_RTR_HBW_WR_RS_N_ARB: c_uint = 0xE40198;
pub const mmTPC1_RTR_HBW_WR_RS_S_ARB: c_uint = 0xE4019C;
pub const mmTPC1_RTR_HBW_WR_RS_L_ARB: c_uint = 0xE401A0;
pub const mmTPC1_RTR_LBW_RD_RQ_E_ARB: c_uint = 0xE40200;
pub const mmTPC1_RTR_LBW_RD_RQ_W_ARB: c_uint = 0xE40204;
pub const mmTPC1_RTR_LBW_RD_RQ_N_ARB: c_uint = 0xE40208;
pub const mmTPC1_RTR_LBW_RD_RQ_S_ARB: c_uint = 0xE4020C;
pub const mmTPC1_RTR_LBW_RD_RQ_L_ARB: c_uint = 0xE40210;
pub const mmTPC1_RTR_LBW_E_ARB_MAX: c_uint = 0xE40220;
pub const mmTPC1_RTR_LBW_W_ARB_MAX: c_uint = 0xE40224;
pub const mmTPC1_RTR_LBW_N_ARB_MAX: c_uint = 0xE40228;
pub const mmTPC1_RTR_LBW_S_ARB_MAX: c_uint = 0xE4022C;
pub const mmTPC1_RTR_LBW_L_ARB_MAX: c_uint = 0xE40230;
pub const mmTPC1_RTR_LBW_RD_RS_E_ARB: c_uint = 0xE40250;
pub const mmTPC1_RTR_LBW_RD_RS_W_ARB: c_uint = 0xE40254;
pub const mmTPC1_RTR_LBW_RD_RS_N_ARB: c_uint = 0xE40258;
pub const mmTPC1_RTR_LBW_RD_RS_S_ARB: c_uint = 0xE4025C;
pub const mmTPC1_RTR_LBW_RD_RS_L_ARB: c_uint = 0xE40260;
pub const mmTPC1_RTR_LBW_WR_RQ_E_ARB: c_uint = 0xE40270;
pub const mmTPC1_RTR_LBW_WR_RQ_W_ARB: c_uint = 0xE40274;
pub const mmTPC1_RTR_LBW_WR_RQ_N_ARB: c_uint = 0xE40278;
pub const mmTPC1_RTR_LBW_WR_RQ_S_ARB: c_uint = 0xE4027C;
pub const mmTPC1_RTR_LBW_WR_RQ_L_ARB: c_uint = 0xE40280;
pub const mmTPC1_RTR_LBW_WR_RS_E_ARB: c_uint = 0xE40290;
pub const mmTPC1_RTR_LBW_WR_RS_W_ARB: c_uint = 0xE40294;
pub const mmTPC1_RTR_LBW_WR_RS_N_ARB: c_uint = 0xE40298;
pub const mmTPC1_RTR_LBW_WR_RS_S_ARB: c_uint = 0xE4029C;
pub const mmTPC1_RTR_LBW_WR_RS_L_ARB: c_uint = 0xE402A0;
pub const mmTPC1_RTR_DBG_E_ARB: c_uint = 0xE40300;
pub const mmTPC1_RTR_DBG_W_ARB: c_uint = 0xE40304;
pub const mmTPC1_RTR_DBG_N_ARB: c_uint = 0xE40308;
pub const mmTPC1_RTR_DBG_S_ARB: c_uint = 0xE4030C;
pub const mmTPC1_RTR_DBG_L_ARB: c_uint = 0xE40310;
pub const mmTPC1_RTR_DBG_E_ARB_MAX: c_uint = 0xE40320;
pub const mmTPC1_RTR_DBG_W_ARB_MAX: c_uint = 0xE40324;
pub const mmTPC1_RTR_DBG_N_ARB_MAX: c_uint = 0xE40328;
pub const mmTPC1_RTR_DBG_S_ARB_MAX: c_uint = 0xE4032C;
pub const mmTPC1_RTR_DBG_L_ARB_MAX: c_uint = 0xE40330;
pub const mmTPC1_RTR_SPLIT_COEF_0: c_uint = 0xE40400;
pub const mmTPC1_RTR_SPLIT_COEF_1: c_uint = 0xE40404;
pub const mmTPC1_RTR_SPLIT_COEF_2: c_uint = 0xE40408;
pub const mmTPC1_RTR_SPLIT_COEF_3: c_uint = 0xE4040C;
pub const mmTPC1_RTR_SPLIT_COEF_4: c_uint = 0xE40410;
pub const mmTPC1_RTR_SPLIT_COEF_5: c_uint = 0xE40414;
pub const mmTPC1_RTR_SPLIT_COEF_6: c_uint = 0xE40418;
pub const mmTPC1_RTR_SPLIT_COEF_7: c_uint = 0xE4041C;
pub const mmTPC1_RTR_SPLIT_COEF_8: c_uint = 0xE40420;
pub const mmTPC1_RTR_SPLIT_COEF_9: c_uint = 0xE40424;
pub const mmTPC1_RTR_SPLIT_CFG: c_uint = 0xE40440;
pub const mmTPC1_RTR_SPLIT_RD_SAT: c_uint = 0xE40444;
pub const mmTPC1_RTR_SPLIT_RD_RST_TOKEN: c_uint = 0xE40448;
pub const mmTPC1_RTR_SPLIT_RD_TIMEOUT_0: c_uint = 0xE4044C;
pub const mmTPC1_RTR_SPLIT_RD_TIMEOUT_1: c_uint = 0xE40450;
pub const mmTPC1_RTR_SPLIT_WR_SAT: c_uint = 0xE40454;
pub const mmTPC1_RTR_WPLIT_WR_TST_TOLEN: c_uint = 0xE40458;
pub const mmTPC1_RTR_SPLIT_WR_TIMEOUT_0: c_uint = 0xE4045C;
pub const mmTPC1_RTR_SPLIT_WR_TIMEOUT_1: c_uint = 0xE40460;
pub const mmTPC1_RTR_HBW_RANGE_HIT: c_uint = 0xE40470;
pub const mmTPC1_RTR_HBW_RANGE_MASK_L_0: c_uint = 0xE40480;
pub const mmTPC1_RTR_HBW_RANGE_MASK_L_1: c_uint = 0xE40484;
pub const mmTPC1_RTR_HBW_RANGE_MASK_L_2: c_uint = 0xE40488;
pub const mmTPC1_RTR_HBW_RANGE_MASK_L_3: c_uint = 0xE4048C;
pub const mmTPC1_RTR_HBW_RANGE_MASK_L_4: c_uint = 0xE40490;
pub const mmTPC1_RTR_HBW_RANGE_MASK_L_5: c_uint = 0xE40494;
pub const mmTPC1_RTR_HBW_RANGE_MASK_L_6: c_uint = 0xE40498;
pub const mmTPC1_RTR_HBW_RANGE_MASK_L_7: c_uint = 0xE4049C;
pub const mmTPC1_RTR_HBW_RANGE_MASK_H_0: c_uint = 0xE404A0;
pub const mmTPC1_RTR_HBW_RANGE_MASK_H_1: c_uint = 0xE404A4;
pub const mmTPC1_RTR_HBW_RANGE_MASK_H_2: c_uint = 0xE404A8;
pub const mmTPC1_RTR_HBW_RANGE_MASK_H_3: c_uint = 0xE404AC;
pub const mmTPC1_RTR_HBW_RANGE_MASK_H_4: c_uint = 0xE404B0;
pub const mmTPC1_RTR_HBW_RANGE_MASK_H_5: c_uint = 0xE404B4;
pub const mmTPC1_RTR_HBW_RANGE_MASK_H_6: c_uint = 0xE404B8;
pub const mmTPC1_RTR_HBW_RANGE_MASK_H_7: c_uint = 0xE404BC;
pub const mmTPC1_RTR_HBW_RANGE_BASE_L_0: c_uint = 0xE404C0;
pub const mmTPC1_RTR_HBW_RANGE_BASE_L_1: c_uint = 0xE404C4;
pub const mmTPC1_RTR_HBW_RANGE_BASE_L_2: c_uint = 0xE404C8;
pub const mmTPC1_RTR_HBW_RANGE_BASE_L_3: c_uint = 0xE404CC;
pub const mmTPC1_RTR_HBW_RANGE_BASE_L_4: c_uint = 0xE404D0;
pub const mmTPC1_RTR_HBW_RANGE_BASE_L_5: c_uint = 0xE404D4;
pub const mmTPC1_RTR_HBW_RANGE_BASE_L_6: c_uint = 0xE404D8;
pub const mmTPC1_RTR_HBW_RANGE_BASE_L_7: c_uint = 0xE404DC;
pub const mmTPC1_RTR_HBW_RANGE_BASE_H_0: c_uint = 0xE404E0;
pub const mmTPC1_RTR_HBW_RANGE_BASE_H_1: c_uint = 0xE404E4;
pub const mmTPC1_RTR_HBW_RANGE_BASE_H_2: c_uint = 0xE404E8;
pub const mmTPC1_RTR_HBW_RANGE_BASE_H_3: c_uint = 0xE404EC;
pub const mmTPC1_RTR_HBW_RANGE_BASE_H_4: c_uint = 0xE404F0;
pub const mmTPC1_RTR_HBW_RANGE_BASE_H_5: c_uint = 0xE404F4;
pub const mmTPC1_RTR_HBW_RANGE_BASE_H_6: c_uint = 0xE404F8;
pub const mmTPC1_RTR_HBW_RANGE_BASE_H_7: c_uint = 0xE404FC;
pub const mmTPC1_RTR_LBW_RANGE_HIT: c_uint = 0xE40500;
pub const mmTPC1_RTR_LBW_RANGE_MASK_0: c_uint = 0xE40510;
pub const mmTPC1_RTR_LBW_RANGE_MASK_1: c_uint = 0xE40514;
pub const mmTPC1_RTR_LBW_RANGE_MASK_2: c_uint = 0xE40518;
pub const mmTPC1_RTR_LBW_RANGE_MASK_3: c_uint = 0xE4051C;
pub const mmTPC1_RTR_LBW_RANGE_MASK_4: c_uint = 0xE40520;
pub const mmTPC1_RTR_LBW_RANGE_MASK_5: c_uint = 0xE40524;
pub const mmTPC1_RTR_LBW_RANGE_MASK_6: c_uint = 0xE40528;
pub const mmTPC1_RTR_LBW_RANGE_MASK_7: c_uint = 0xE4052C;
pub const mmTPC1_RTR_LBW_RANGE_MASK_8: c_uint = 0xE40530;
pub const mmTPC1_RTR_LBW_RANGE_MASK_9: c_uint = 0xE40534;
pub const mmTPC1_RTR_LBW_RANGE_MASK_10: c_uint = 0xE40538;
pub const mmTPC1_RTR_LBW_RANGE_MASK_11: c_uint = 0xE4053C;
pub const mmTPC1_RTR_LBW_RANGE_MASK_12: c_uint = 0xE40540;
pub const mmTPC1_RTR_LBW_RANGE_MASK_13: c_uint = 0xE40544;
pub const mmTPC1_RTR_LBW_RANGE_MASK_14: c_uint = 0xE40548;
pub const mmTPC1_RTR_LBW_RANGE_MASK_15: c_uint = 0xE4054C;
pub const mmTPC1_RTR_LBW_RANGE_BASE_0: c_uint = 0xE40550;
pub const mmTPC1_RTR_LBW_RANGE_BASE_1: c_uint = 0xE40554;
pub const mmTPC1_RTR_LBW_RANGE_BASE_2: c_uint = 0xE40558;
pub const mmTPC1_RTR_LBW_RANGE_BASE_3: c_uint = 0xE4055C;
pub const mmTPC1_RTR_LBW_RANGE_BASE_4: c_uint = 0xE40560;
pub const mmTPC1_RTR_LBW_RANGE_BASE_5: c_uint = 0xE40564;
pub const mmTPC1_RTR_LBW_RANGE_BASE_6: c_uint = 0xE40568;
pub const mmTPC1_RTR_LBW_RANGE_BASE_7: c_uint = 0xE4056C;
pub const mmTPC1_RTR_LBW_RANGE_BASE_8: c_uint = 0xE40570;
pub const mmTPC1_RTR_LBW_RANGE_BASE_9: c_uint = 0xE40574;
pub const mmTPC1_RTR_LBW_RANGE_BASE_10: c_uint = 0xE40578;
pub const mmTPC1_RTR_LBW_RANGE_BASE_11: c_uint = 0xE4057C;
pub const mmTPC1_RTR_LBW_RANGE_BASE_12: c_uint = 0xE40580;
pub const mmTPC1_RTR_LBW_RANGE_BASE_13: c_uint = 0xE40584;
pub const mmTPC1_RTR_LBW_RANGE_BASE_14: c_uint = 0xE40588;
pub const mmTPC1_RTR_LBW_RANGE_BASE_15: c_uint = 0xE4058C;
pub const mmTPC1_RTR_RGLTR: c_uint = 0xE40590;
pub const mmTPC1_RTR_RGLTR_WR_RESULT: c_uint = 0xE40594;
pub const mmTPC1_RTR_RGLTR_RD_RESULT: c_uint = 0xE40598;
pub const mmTPC1_RTR_SCRAMB_EN: c_uint = 0xE40600;
pub const mmTPC1_RTR_NON_LIN_SCRAMB: c_uint = 0xE40604;
