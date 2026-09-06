//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/mme1_rtr_regs.h
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
// MME1_RTR (Prototype: MME_RTR)
//
pub const mmMME1_RTR_HBW_RD_RQ_E_ARB: c_uint = 0x40100;
pub const mmMME1_RTR_HBW_RD_RQ_W_ARB: c_uint = 0x40104;
pub const mmMME1_RTR_HBW_RD_RQ_N_ARB: c_uint = 0x40108;
pub const mmMME1_RTR_HBW_RD_RQ_S_ARB: c_uint = 0x4010C;
pub const mmMME1_RTR_HBW_RD_RQ_L_ARB: c_uint = 0x40110;
pub const mmMME1_RTR_HBW_E_ARB_MAX: c_uint = 0x40120;
pub const mmMME1_RTR_HBW_W_ARB_MAX: c_uint = 0x40124;
pub const mmMME1_RTR_HBW_N_ARB_MAX: c_uint = 0x40128;
pub const mmMME1_RTR_HBW_S_ARB_MAX: c_uint = 0x4012C;
pub const mmMME1_RTR_HBW_L_ARB_MAX: c_uint = 0x40130;
pub const mmMME1_RTR_HBW_RD_RS_MAX_CREDIT: c_uint = 0x40140;
pub const mmMME1_RTR_HBW_WR_RQ_MAX_CREDIT: c_uint = 0x40144;
pub const mmMME1_RTR_HBW_RD_RQ_MAX_CREDIT: c_uint = 0x40148;
pub const mmMME1_RTR_HBW_RD_RS_E_ARB: c_uint = 0x40150;
pub const mmMME1_RTR_HBW_RD_RS_W_ARB: c_uint = 0x40154;
pub const mmMME1_RTR_HBW_RD_RS_N_ARB: c_uint = 0x40158;
pub const mmMME1_RTR_HBW_RD_RS_S_ARB: c_uint = 0x4015C;
pub const mmMME1_RTR_HBW_RD_RS_L_ARB: c_uint = 0x40160;
pub const mmMME1_RTR_HBW_WR_RQ_E_ARB: c_uint = 0x40170;
pub const mmMME1_RTR_HBW_WR_RQ_W_ARB: c_uint = 0x40174;
pub const mmMME1_RTR_HBW_WR_RQ_N_ARB: c_uint = 0x40178;
pub const mmMME1_RTR_HBW_WR_RQ_S_ARB: c_uint = 0x4017C;
pub const mmMME1_RTR_HBW_WR_RQ_L_ARB: c_uint = 0x40180;
pub const mmMME1_RTR_HBW_WR_RS_E_ARB: c_uint = 0x40190;
pub const mmMME1_RTR_HBW_WR_RS_W_ARB: c_uint = 0x40194;
pub const mmMME1_RTR_HBW_WR_RS_N_ARB: c_uint = 0x40198;
pub const mmMME1_RTR_HBW_WR_RS_S_ARB: c_uint = 0x4019C;
pub const mmMME1_RTR_HBW_WR_RS_L_ARB: c_uint = 0x401A0;
pub const mmMME1_RTR_LBW_RD_RQ_E_ARB: c_uint = 0x40200;
pub const mmMME1_RTR_LBW_RD_RQ_W_ARB: c_uint = 0x40204;
pub const mmMME1_RTR_LBW_RD_RQ_N_ARB: c_uint = 0x40208;
pub const mmMME1_RTR_LBW_RD_RQ_S_ARB: c_uint = 0x4020C;
pub const mmMME1_RTR_LBW_RD_RQ_L_ARB: c_uint = 0x40210;
pub const mmMME1_RTR_LBW_E_ARB_MAX: c_uint = 0x40220;
pub const mmMME1_RTR_LBW_W_ARB_MAX: c_uint = 0x40224;
pub const mmMME1_RTR_LBW_N_ARB_MAX: c_uint = 0x40228;
pub const mmMME1_RTR_LBW_S_ARB_MAX: c_uint = 0x4022C;
pub const mmMME1_RTR_LBW_L_ARB_MAX: c_uint = 0x40230;
pub const mmMME1_RTR_LBW_SRAM_MAX_CREDIT: c_uint = 0x40240;
pub const mmMME1_RTR_LBW_RD_RS_E_ARB: c_uint = 0x40250;
pub const mmMME1_RTR_LBW_RD_RS_W_ARB: c_uint = 0x40254;
pub const mmMME1_RTR_LBW_RD_RS_N_ARB: c_uint = 0x40258;
pub const mmMME1_RTR_LBW_RD_RS_S_ARB: c_uint = 0x4025C;
pub const mmMME1_RTR_LBW_RD_RS_L_ARB: c_uint = 0x40260;
pub const mmMME1_RTR_LBW_WR_RQ_E_ARB: c_uint = 0x40270;
pub const mmMME1_RTR_LBW_WR_RQ_W_ARB: c_uint = 0x40274;
pub const mmMME1_RTR_LBW_WR_RQ_N_ARB: c_uint = 0x40278;
pub const mmMME1_RTR_LBW_WR_RQ_S_ARB: c_uint = 0x4027C;
pub const mmMME1_RTR_LBW_WR_RQ_L_ARB: c_uint = 0x40280;
pub const mmMME1_RTR_LBW_WR_RS_E_ARB: c_uint = 0x40290;
pub const mmMME1_RTR_LBW_WR_RS_W_ARB: c_uint = 0x40294;
pub const mmMME1_RTR_LBW_WR_RS_N_ARB: c_uint = 0x40298;
pub const mmMME1_RTR_LBW_WR_RS_S_ARB: c_uint = 0x4029C;
pub const mmMME1_RTR_LBW_WR_RS_L_ARB: c_uint = 0x402A0;
pub const mmMME1_RTR_DBG_E_ARB: c_uint = 0x40300;
pub const mmMME1_RTR_DBG_W_ARB: c_uint = 0x40304;
pub const mmMME1_RTR_DBG_N_ARB: c_uint = 0x40308;
pub const mmMME1_RTR_DBG_S_ARB: c_uint = 0x4030C;
pub const mmMME1_RTR_DBG_L_ARB: c_uint = 0x40310;
pub const mmMME1_RTR_DBG_E_ARB_MAX: c_uint = 0x40320;
pub const mmMME1_RTR_DBG_W_ARB_MAX: c_uint = 0x40324;
pub const mmMME1_RTR_DBG_N_ARB_MAX: c_uint = 0x40328;
pub const mmMME1_RTR_DBG_S_ARB_MAX: c_uint = 0x4032C;
pub const mmMME1_RTR_DBG_L_ARB_MAX: c_uint = 0x40330;
pub const mmMME1_RTR_SPLIT_COEF_0: c_uint = 0x40400;
pub const mmMME1_RTR_SPLIT_COEF_1: c_uint = 0x40404;
pub const mmMME1_RTR_SPLIT_COEF_2: c_uint = 0x40408;
pub const mmMME1_RTR_SPLIT_COEF_3: c_uint = 0x4040C;
pub const mmMME1_RTR_SPLIT_COEF_4: c_uint = 0x40410;
pub const mmMME1_RTR_SPLIT_COEF_5: c_uint = 0x40414;
pub const mmMME1_RTR_SPLIT_COEF_6: c_uint = 0x40418;
pub const mmMME1_RTR_SPLIT_COEF_7: c_uint = 0x4041C;
pub const mmMME1_RTR_SPLIT_COEF_8: c_uint = 0x40420;
pub const mmMME1_RTR_SPLIT_COEF_9: c_uint = 0x40424;
pub const mmMME1_RTR_SPLIT_CFG: c_uint = 0x40440;
pub const mmMME1_RTR_SPLIT_RD_SAT: c_uint = 0x40444;
pub const mmMME1_RTR_SPLIT_RD_RST_TOKEN: c_uint = 0x40448;
pub const mmMME1_RTR_SPLIT_RD_TIMEOUT_0: c_uint = 0x4044C;
pub const mmMME1_RTR_SPLIT_RD_TIMEOUT_1: c_uint = 0x40450;
pub const mmMME1_RTR_SPLIT_WR_SAT: c_uint = 0x40454;
pub const mmMME1_RTR_WPLIT_WR_TST_TOLEN: c_uint = 0x40458;
pub const mmMME1_RTR_SPLIT_WR_TIMEOUT_0: c_uint = 0x4045C;
pub const mmMME1_RTR_SPLIT_WR_TIMEOUT_1: c_uint = 0x40460;
pub const mmMME1_RTR_HBW_RANGE_HIT: c_uint = 0x40470;
pub const mmMME1_RTR_HBW_RANGE_MASK_L_0: c_uint = 0x40480;
pub const mmMME1_RTR_HBW_RANGE_MASK_L_1: c_uint = 0x40484;
pub const mmMME1_RTR_HBW_RANGE_MASK_L_2: c_uint = 0x40488;
pub const mmMME1_RTR_HBW_RANGE_MASK_L_3: c_uint = 0x4048C;
pub const mmMME1_RTR_HBW_RANGE_MASK_L_4: c_uint = 0x40490;
pub const mmMME1_RTR_HBW_RANGE_MASK_L_5: c_uint = 0x40494;
pub const mmMME1_RTR_HBW_RANGE_MASK_L_6: c_uint = 0x40498;
pub const mmMME1_RTR_HBW_RANGE_MASK_L_7: c_uint = 0x4049C;
pub const mmMME1_RTR_HBW_RANGE_MASK_H_0: c_uint = 0x404A0;
pub const mmMME1_RTR_HBW_RANGE_MASK_H_1: c_uint = 0x404A4;
pub const mmMME1_RTR_HBW_RANGE_MASK_H_2: c_uint = 0x404A8;
pub const mmMME1_RTR_HBW_RANGE_MASK_H_3: c_uint = 0x404AC;
pub const mmMME1_RTR_HBW_RANGE_MASK_H_4: c_uint = 0x404B0;
pub const mmMME1_RTR_HBW_RANGE_MASK_H_5: c_uint = 0x404B4;
pub const mmMME1_RTR_HBW_RANGE_MASK_H_6: c_uint = 0x404B8;
pub const mmMME1_RTR_HBW_RANGE_MASK_H_7: c_uint = 0x404BC;
pub const mmMME1_RTR_HBW_RANGE_BASE_L_0: c_uint = 0x404C0;
pub const mmMME1_RTR_HBW_RANGE_BASE_L_1: c_uint = 0x404C4;
pub const mmMME1_RTR_HBW_RANGE_BASE_L_2: c_uint = 0x404C8;
pub const mmMME1_RTR_HBW_RANGE_BASE_L_3: c_uint = 0x404CC;
pub const mmMME1_RTR_HBW_RANGE_BASE_L_4: c_uint = 0x404D0;
pub const mmMME1_RTR_HBW_RANGE_BASE_L_5: c_uint = 0x404D4;
pub const mmMME1_RTR_HBW_RANGE_BASE_L_6: c_uint = 0x404D8;
pub const mmMME1_RTR_HBW_RANGE_BASE_L_7: c_uint = 0x404DC;
pub const mmMME1_RTR_HBW_RANGE_BASE_H_0: c_uint = 0x404E0;
pub const mmMME1_RTR_HBW_RANGE_BASE_H_1: c_uint = 0x404E4;
pub const mmMME1_RTR_HBW_RANGE_BASE_H_2: c_uint = 0x404E8;
pub const mmMME1_RTR_HBW_RANGE_BASE_H_3: c_uint = 0x404EC;
pub const mmMME1_RTR_HBW_RANGE_BASE_H_4: c_uint = 0x404F0;
pub const mmMME1_RTR_HBW_RANGE_BASE_H_5: c_uint = 0x404F4;
pub const mmMME1_RTR_HBW_RANGE_BASE_H_6: c_uint = 0x404F8;
pub const mmMME1_RTR_HBW_RANGE_BASE_H_7: c_uint = 0x404FC;
pub const mmMME1_RTR_LBW_RANGE_HIT: c_uint = 0x40500;
pub const mmMME1_RTR_LBW_RANGE_MASK_0: c_uint = 0x40510;
pub const mmMME1_RTR_LBW_RANGE_MASK_1: c_uint = 0x40514;
pub const mmMME1_RTR_LBW_RANGE_MASK_2: c_uint = 0x40518;
pub const mmMME1_RTR_LBW_RANGE_MASK_3: c_uint = 0x4051C;
pub const mmMME1_RTR_LBW_RANGE_MASK_4: c_uint = 0x40520;
pub const mmMME1_RTR_LBW_RANGE_MASK_5: c_uint = 0x40524;
pub const mmMME1_RTR_LBW_RANGE_MASK_6: c_uint = 0x40528;
pub const mmMME1_RTR_LBW_RANGE_MASK_7: c_uint = 0x4052C;
pub const mmMME1_RTR_LBW_RANGE_MASK_8: c_uint = 0x40530;
pub const mmMME1_RTR_LBW_RANGE_MASK_9: c_uint = 0x40534;
pub const mmMME1_RTR_LBW_RANGE_MASK_10: c_uint = 0x40538;
pub const mmMME1_RTR_LBW_RANGE_MASK_11: c_uint = 0x4053C;
pub const mmMME1_RTR_LBW_RANGE_MASK_12: c_uint = 0x40540;
pub const mmMME1_RTR_LBW_RANGE_MASK_13: c_uint = 0x40544;
pub const mmMME1_RTR_LBW_RANGE_MASK_14: c_uint = 0x40548;
pub const mmMME1_RTR_LBW_RANGE_MASK_15: c_uint = 0x4054C;
pub const mmMME1_RTR_LBW_RANGE_BASE_0: c_uint = 0x40550;
pub const mmMME1_RTR_LBW_RANGE_BASE_1: c_uint = 0x40554;
pub const mmMME1_RTR_LBW_RANGE_BASE_2: c_uint = 0x40558;
pub const mmMME1_RTR_LBW_RANGE_BASE_3: c_uint = 0x4055C;
pub const mmMME1_RTR_LBW_RANGE_BASE_4: c_uint = 0x40560;
pub const mmMME1_RTR_LBW_RANGE_BASE_5: c_uint = 0x40564;
pub const mmMME1_RTR_LBW_RANGE_BASE_6: c_uint = 0x40568;
pub const mmMME1_RTR_LBW_RANGE_BASE_7: c_uint = 0x4056C;
pub const mmMME1_RTR_LBW_RANGE_BASE_8: c_uint = 0x40570;
pub const mmMME1_RTR_LBW_RANGE_BASE_9: c_uint = 0x40574;
pub const mmMME1_RTR_LBW_RANGE_BASE_10: c_uint = 0x40578;
pub const mmMME1_RTR_LBW_RANGE_BASE_11: c_uint = 0x4057C;
pub const mmMME1_RTR_LBW_RANGE_BASE_12: c_uint = 0x40580;
pub const mmMME1_RTR_LBW_RANGE_BASE_13: c_uint = 0x40584;
pub const mmMME1_RTR_LBW_RANGE_BASE_14: c_uint = 0x40588;
pub const mmMME1_RTR_LBW_RANGE_BASE_15: c_uint = 0x4058C;
pub const mmMME1_RTR_RGLTR: c_uint = 0x40590;
pub const mmMME1_RTR_RGLTR_WR_RESULT: c_uint = 0x40594;
pub const mmMME1_RTR_RGLTR_RD_RESULT: c_uint = 0x40598;
pub const mmMME1_RTR_SCRAMB_EN: c_uint = 0x40600;
pub const mmMME1_RTR_NON_LIN_SCRAMB: c_uint = 0x40604;
