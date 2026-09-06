//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/mme4_rtr_regs.h
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
// MME4_RTR (Prototype: MME_RTR)
//
pub const mmMME4_RTR_HBW_RD_RQ_E_ARB: c_uint = 0x100100;
pub const mmMME4_RTR_HBW_RD_RQ_W_ARB: c_uint = 0x100104;
pub const mmMME4_RTR_HBW_RD_RQ_N_ARB: c_uint = 0x100108;
pub const mmMME4_RTR_HBW_RD_RQ_S_ARB: c_uint = 0x10010C;
pub const mmMME4_RTR_HBW_RD_RQ_L_ARB: c_uint = 0x100110;
pub const mmMME4_RTR_HBW_E_ARB_MAX: c_uint = 0x100120;
pub const mmMME4_RTR_HBW_W_ARB_MAX: c_uint = 0x100124;
pub const mmMME4_RTR_HBW_N_ARB_MAX: c_uint = 0x100128;
pub const mmMME4_RTR_HBW_S_ARB_MAX: c_uint = 0x10012C;
pub const mmMME4_RTR_HBW_L_ARB_MAX: c_uint = 0x100130;
pub const mmMME4_RTR_HBW_RD_RS_MAX_CREDIT: c_uint = 0x100140;
pub const mmMME4_RTR_HBW_WR_RQ_MAX_CREDIT: c_uint = 0x100144;
pub const mmMME4_RTR_HBW_RD_RQ_MAX_CREDIT: c_uint = 0x100148;
pub const mmMME4_RTR_HBW_RD_RS_E_ARB: c_uint = 0x100150;
pub const mmMME4_RTR_HBW_RD_RS_W_ARB: c_uint = 0x100154;
pub const mmMME4_RTR_HBW_RD_RS_N_ARB: c_uint = 0x100158;
pub const mmMME4_RTR_HBW_RD_RS_S_ARB: c_uint = 0x10015C;
pub const mmMME4_RTR_HBW_RD_RS_L_ARB: c_uint = 0x100160;
pub const mmMME4_RTR_HBW_WR_RQ_E_ARB: c_uint = 0x100170;
pub const mmMME4_RTR_HBW_WR_RQ_W_ARB: c_uint = 0x100174;
pub const mmMME4_RTR_HBW_WR_RQ_N_ARB: c_uint = 0x100178;
pub const mmMME4_RTR_HBW_WR_RQ_S_ARB: c_uint = 0x10017C;
pub const mmMME4_RTR_HBW_WR_RQ_L_ARB: c_uint = 0x100180;
pub const mmMME4_RTR_HBW_WR_RS_E_ARB: c_uint = 0x100190;
pub const mmMME4_RTR_HBW_WR_RS_W_ARB: c_uint = 0x100194;
pub const mmMME4_RTR_HBW_WR_RS_N_ARB: c_uint = 0x100198;
pub const mmMME4_RTR_HBW_WR_RS_S_ARB: c_uint = 0x10019C;
pub const mmMME4_RTR_HBW_WR_RS_L_ARB: c_uint = 0x1001A0;
pub const mmMME4_RTR_LBW_RD_RQ_E_ARB: c_uint = 0x100200;
pub const mmMME4_RTR_LBW_RD_RQ_W_ARB: c_uint = 0x100204;
pub const mmMME4_RTR_LBW_RD_RQ_N_ARB: c_uint = 0x100208;
pub const mmMME4_RTR_LBW_RD_RQ_S_ARB: c_uint = 0x10020C;
pub const mmMME4_RTR_LBW_RD_RQ_L_ARB: c_uint = 0x100210;
pub const mmMME4_RTR_LBW_E_ARB_MAX: c_uint = 0x100220;
pub const mmMME4_RTR_LBW_W_ARB_MAX: c_uint = 0x100224;
pub const mmMME4_RTR_LBW_N_ARB_MAX: c_uint = 0x100228;
pub const mmMME4_RTR_LBW_S_ARB_MAX: c_uint = 0x10022C;
pub const mmMME4_RTR_LBW_L_ARB_MAX: c_uint = 0x100230;
pub const mmMME4_RTR_LBW_SRAM_MAX_CREDIT: c_uint = 0x100240;
pub const mmMME4_RTR_LBW_RD_RS_E_ARB: c_uint = 0x100250;
pub const mmMME4_RTR_LBW_RD_RS_W_ARB: c_uint = 0x100254;
pub const mmMME4_RTR_LBW_RD_RS_N_ARB: c_uint = 0x100258;
pub const mmMME4_RTR_LBW_RD_RS_S_ARB: c_uint = 0x10025C;
pub const mmMME4_RTR_LBW_RD_RS_L_ARB: c_uint = 0x100260;
pub const mmMME4_RTR_LBW_WR_RQ_E_ARB: c_uint = 0x100270;
pub const mmMME4_RTR_LBW_WR_RQ_W_ARB: c_uint = 0x100274;
pub const mmMME4_RTR_LBW_WR_RQ_N_ARB: c_uint = 0x100278;
pub const mmMME4_RTR_LBW_WR_RQ_S_ARB: c_uint = 0x10027C;
pub const mmMME4_RTR_LBW_WR_RQ_L_ARB: c_uint = 0x100280;
pub const mmMME4_RTR_LBW_WR_RS_E_ARB: c_uint = 0x100290;
pub const mmMME4_RTR_LBW_WR_RS_W_ARB: c_uint = 0x100294;
pub const mmMME4_RTR_LBW_WR_RS_N_ARB: c_uint = 0x100298;
pub const mmMME4_RTR_LBW_WR_RS_S_ARB: c_uint = 0x10029C;
pub const mmMME4_RTR_LBW_WR_RS_L_ARB: c_uint = 0x1002A0;
pub const mmMME4_RTR_DBG_E_ARB: c_uint = 0x100300;
pub const mmMME4_RTR_DBG_W_ARB: c_uint = 0x100304;
pub const mmMME4_RTR_DBG_N_ARB: c_uint = 0x100308;
pub const mmMME4_RTR_DBG_S_ARB: c_uint = 0x10030C;
pub const mmMME4_RTR_DBG_L_ARB: c_uint = 0x100310;
pub const mmMME4_RTR_DBG_E_ARB_MAX: c_uint = 0x100320;
pub const mmMME4_RTR_DBG_W_ARB_MAX: c_uint = 0x100324;
pub const mmMME4_RTR_DBG_N_ARB_MAX: c_uint = 0x100328;
pub const mmMME4_RTR_DBG_S_ARB_MAX: c_uint = 0x10032C;
pub const mmMME4_RTR_DBG_L_ARB_MAX: c_uint = 0x100330;
pub const mmMME4_RTR_SPLIT_COEF_0: c_uint = 0x100400;
pub const mmMME4_RTR_SPLIT_COEF_1: c_uint = 0x100404;
pub const mmMME4_RTR_SPLIT_COEF_2: c_uint = 0x100408;
pub const mmMME4_RTR_SPLIT_COEF_3: c_uint = 0x10040C;
pub const mmMME4_RTR_SPLIT_COEF_4: c_uint = 0x100410;
pub const mmMME4_RTR_SPLIT_COEF_5: c_uint = 0x100414;
pub const mmMME4_RTR_SPLIT_COEF_6: c_uint = 0x100418;
pub const mmMME4_RTR_SPLIT_COEF_7: c_uint = 0x10041C;
pub const mmMME4_RTR_SPLIT_COEF_8: c_uint = 0x100420;
pub const mmMME4_RTR_SPLIT_COEF_9: c_uint = 0x100424;
pub const mmMME4_RTR_SPLIT_CFG: c_uint = 0x100440;
pub const mmMME4_RTR_SPLIT_RD_SAT: c_uint = 0x100444;
pub const mmMME4_RTR_SPLIT_RD_RST_TOKEN: c_uint = 0x100448;
pub const mmMME4_RTR_SPLIT_RD_TIMEOUT_0: c_uint = 0x10044C;
pub const mmMME4_RTR_SPLIT_RD_TIMEOUT_1: c_uint = 0x100450;
pub const mmMME4_RTR_SPLIT_WR_SAT: c_uint = 0x100454;
pub const mmMME4_RTR_WPLIT_WR_TST_TOLEN: c_uint = 0x100458;
pub const mmMME4_RTR_SPLIT_WR_TIMEOUT_0: c_uint = 0x10045C;
pub const mmMME4_RTR_SPLIT_WR_TIMEOUT_1: c_uint = 0x100460;
pub const mmMME4_RTR_HBW_RANGE_HIT: c_uint = 0x100470;
pub const mmMME4_RTR_HBW_RANGE_MASK_L_0: c_uint = 0x100480;
pub const mmMME4_RTR_HBW_RANGE_MASK_L_1: c_uint = 0x100484;
pub const mmMME4_RTR_HBW_RANGE_MASK_L_2: c_uint = 0x100488;
pub const mmMME4_RTR_HBW_RANGE_MASK_L_3: c_uint = 0x10048C;
pub const mmMME4_RTR_HBW_RANGE_MASK_L_4: c_uint = 0x100490;
pub const mmMME4_RTR_HBW_RANGE_MASK_L_5: c_uint = 0x100494;
pub const mmMME4_RTR_HBW_RANGE_MASK_L_6: c_uint = 0x100498;
pub const mmMME4_RTR_HBW_RANGE_MASK_L_7: c_uint = 0x10049C;
pub const mmMME4_RTR_HBW_RANGE_MASK_H_0: c_uint = 0x1004A0;
pub const mmMME4_RTR_HBW_RANGE_MASK_H_1: c_uint = 0x1004A4;
pub const mmMME4_RTR_HBW_RANGE_MASK_H_2: c_uint = 0x1004A8;
pub const mmMME4_RTR_HBW_RANGE_MASK_H_3: c_uint = 0x1004AC;
pub const mmMME4_RTR_HBW_RANGE_MASK_H_4: c_uint = 0x1004B0;
pub const mmMME4_RTR_HBW_RANGE_MASK_H_5: c_uint = 0x1004B4;
pub const mmMME4_RTR_HBW_RANGE_MASK_H_6: c_uint = 0x1004B8;
pub const mmMME4_RTR_HBW_RANGE_MASK_H_7: c_uint = 0x1004BC;
pub const mmMME4_RTR_HBW_RANGE_BASE_L_0: c_uint = 0x1004C0;
pub const mmMME4_RTR_HBW_RANGE_BASE_L_1: c_uint = 0x1004C4;
pub const mmMME4_RTR_HBW_RANGE_BASE_L_2: c_uint = 0x1004C8;
pub const mmMME4_RTR_HBW_RANGE_BASE_L_3: c_uint = 0x1004CC;
pub const mmMME4_RTR_HBW_RANGE_BASE_L_4: c_uint = 0x1004D0;
pub const mmMME4_RTR_HBW_RANGE_BASE_L_5: c_uint = 0x1004D4;
pub const mmMME4_RTR_HBW_RANGE_BASE_L_6: c_uint = 0x1004D8;
pub const mmMME4_RTR_HBW_RANGE_BASE_L_7: c_uint = 0x1004DC;
pub const mmMME4_RTR_HBW_RANGE_BASE_H_0: c_uint = 0x1004E0;
pub const mmMME4_RTR_HBW_RANGE_BASE_H_1: c_uint = 0x1004E4;
pub const mmMME4_RTR_HBW_RANGE_BASE_H_2: c_uint = 0x1004E8;
pub const mmMME4_RTR_HBW_RANGE_BASE_H_3: c_uint = 0x1004EC;
pub const mmMME4_RTR_HBW_RANGE_BASE_H_4: c_uint = 0x1004F0;
pub const mmMME4_RTR_HBW_RANGE_BASE_H_5: c_uint = 0x1004F4;
pub const mmMME4_RTR_HBW_RANGE_BASE_H_6: c_uint = 0x1004F8;
pub const mmMME4_RTR_HBW_RANGE_BASE_H_7: c_uint = 0x1004FC;
pub const mmMME4_RTR_LBW_RANGE_HIT: c_uint = 0x100500;
pub const mmMME4_RTR_LBW_RANGE_MASK_0: c_uint = 0x100510;
pub const mmMME4_RTR_LBW_RANGE_MASK_1: c_uint = 0x100514;
pub const mmMME4_RTR_LBW_RANGE_MASK_2: c_uint = 0x100518;
pub const mmMME4_RTR_LBW_RANGE_MASK_3: c_uint = 0x10051C;
pub const mmMME4_RTR_LBW_RANGE_MASK_4: c_uint = 0x100520;
pub const mmMME4_RTR_LBW_RANGE_MASK_5: c_uint = 0x100524;
pub const mmMME4_RTR_LBW_RANGE_MASK_6: c_uint = 0x100528;
pub const mmMME4_RTR_LBW_RANGE_MASK_7: c_uint = 0x10052C;
pub const mmMME4_RTR_LBW_RANGE_MASK_8: c_uint = 0x100530;
pub const mmMME4_RTR_LBW_RANGE_MASK_9: c_uint = 0x100534;
pub const mmMME4_RTR_LBW_RANGE_MASK_10: c_uint = 0x100538;
pub const mmMME4_RTR_LBW_RANGE_MASK_11: c_uint = 0x10053C;
pub const mmMME4_RTR_LBW_RANGE_MASK_12: c_uint = 0x100540;
pub const mmMME4_RTR_LBW_RANGE_MASK_13: c_uint = 0x100544;
pub const mmMME4_RTR_LBW_RANGE_MASK_14: c_uint = 0x100548;
pub const mmMME4_RTR_LBW_RANGE_MASK_15: c_uint = 0x10054C;
pub const mmMME4_RTR_LBW_RANGE_BASE_0: c_uint = 0x100550;
pub const mmMME4_RTR_LBW_RANGE_BASE_1: c_uint = 0x100554;
pub const mmMME4_RTR_LBW_RANGE_BASE_2: c_uint = 0x100558;
pub const mmMME4_RTR_LBW_RANGE_BASE_3: c_uint = 0x10055C;
pub const mmMME4_RTR_LBW_RANGE_BASE_4: c_uint = 0x100560;
pub const mmMME4_RTR_LBW_RANGE_BASE_5: c_uint = 0x100564;
pub const mmMME4_RTR_LBW_RANGE_BASE_6: c_uint = 0x100568;
pub const mmMME4_RTR_LBW_RANGE_BASE_7: c_uint = 0x10056C;
pub const mmMME4_RTR_LBW_RANGE_BASE_8: c_uint = 0x100570;
pub const mmMME4_RTR_LBW_RANGE_BASE_9: c_uint = 0x100574;
pub const mmMME4_RTR_LBW_RANGE_BASE_10: c_uint = 0x100578;
pub const mmMME4_RTR_LBW_RANGE_BASE_11: c_uint = 0x10057C;
pub const mmMME4_RTR_LBW_RANGE_BASE_12: c_uint = 0x100580;
pub const mmMME4_RTR_LBW_RANGE_BASE_13: c_uint = 0x100584;
pub const mmMME4_RTR_LBW_RANGE_BASE_14: c_uint = 0x100588;
pub const mmMME4_RTR_LBW_RANGE_BASE_15: c_uint = 0x10058C;
pub const mmMME4_RTR_RGLTR: c_uint = 0x100590;
pub const mmMME4_RTR_RGLTR_WR_RESULT: c_uint = 0x100594;
pub const mmMME4_RTR_RGLTR_RD_RESULT: c_uint = 0x100598;
pub const mmMME4_RTR_SCRAMB_EN: c_uint = 0x100600;
pub const mmMME4_RTR_NON_LIN_SCRAMB: c_uint = 0x100604;
