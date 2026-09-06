//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/mme5_rtr_regs.h
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
// MME5_RTR (Prototype: MME_RTR)
//
pub const mmMME5_RTR_HBW_RD_RQ_E_ARB: c_uint = 0x140100;
pub const mmMME5_RTR_HBW_RD_RQ_W_ARB: c_uint = 0x140104;
pub const mmMME5_RTR_HBW_RD_RQ_N_ARB: c_uint = 0x140108;
pub const mmMME5_RTR_HBW_RD_RQ_S_ARB: c_uint = 0x14010C;
pub const mmMME5_RTR_HBW_RD_RQ_L_ARB: c_uint = 0x140110;
pub const mmMME5_RTR_HBW_E_ARB_MAX: c_uint = 0x140120;
pub const mmMME5_RTR_HBW_W_ARB_MAX: c_uint = 0x140124;
pub const mmMME5_RTR_HBW_N_ARB_MAX: c_uint = 0x140128;
pub const mmMME5_RTR_HBW_S_ARB_MAX: c_uint = 0x14012C;
pub const mmMME5_RTR_HBW_L_ARB_MAX: c_uint = 0x140130;
pub const mmMME5_RTR_HBW_RD_RS_MAX_CREDIT: c_uint = 0x140140;
pub const mmMME5_RTR_HBW_WR_RQ_MAX_CREDIT: c_uint = 0x140144;
pub const mmMME5_RTR_HBW_RD_RQ_MAX_CREDIT: c_uint = 0x140148;
pub const mmMME5_RTR_HBW_RD_RS_E_ARB: c_uint = 0x140150;
pub const mmMME5_RTR_HBW_RD_RS_W_ARB: c_uint = 0x140154;
pub const mmMME5_RTR_HBW_RD_RS_N_ARB: c_uint = 0x140158;
pub const mmMME5_RTR_HBW_RD_RS_S_ARB: c_uint = 0x14015C;
pub const mmMME5_RTR_HBW_RD_RS_L_ARB: c_uint = 0x140160;
pub const mmMME5_RTR_HBW_WR_RQ_E_ARB: c_uint = 0x140170;
pub const mmMME5_RTR_HBW_WR_RQ_W_ARB: c_uint = 0x140174;
pub const mmMME5_RTR_HBW_WR_RQ_N_ARB: c_uint = 0x140178;
pub const mmMME5_RTR_HBW_WR_RQ_S_ARB: c_uint = 0x14017C;
pub const mmMME5_RTR_HBW_WR_RQ_L_ARB: c_uint = 0x140180;
pub const mmMME5_RTR_HBW_WR_RS_E_ARB: c_uint = 0x140190;
pub const mmMME5_RTR_HBW_WR_RS_W_ARB: c_uint = 0x140194;
pub const mmMME5_RTR_HBW_WR_RS_N_ARB: c_uint = 0x140198;
pub const mmMME5_RTR_HBW_WR_RS_S_ARB: c_uint = 0x14019C;
pub const mmMME5_RTR_HBW_WR_RS_L_ARB: c_uint = 0x1401A0;
pub const mmMME5_RTR_LBW_RD_RQ_E_ARB: c_uint = 0x140200;
pub const mmMME5_RTR_LBW_RD_RQ_W_ARB: c_uint = 0x140204;
pub const mmMME5_RTR_LBW_RD_RQ_N_ARB: c_uint = 0x140208;
pub const mmMME5_RTR_LBW_RD_RQ_S_ARB: c_uint = 0x14020C;
pub const mmMME5_RTR_LBW_RD_RQ_L_ARB: c_uint = 0x140210;
pub const mmMME5_RTR_LBW_E_ARB_MAX: c_uint = 0x140220;
pub const mmMME5_RTR_LBW_W_ARB_MAX: c_uint = 0x140224;
pub const mmMME5_RTR_LBW_N_ARB_MAX: c_uint = 0x140228;
pub const mmMME5_RTR_LBW_S_ARB_MAX: c_uint = 0x14022C;
pub const mmMME5_RTR_LBW_L_ARB_MAX: c_uint = 0x140230;
pub const mmMME5_RTR_LBW_SRAM_MAX_CREDIT: c_uint = 0x140240;
pub const mmMME5_RTR_LBW_RD_RS_E_ARB: c_uint = 0x140250;
pub const mmMME5_RTR_LBW_RD_RS_W_ARB: c_uint = 0x140254;
pub const mmMME5_RTR_LBW_RD_RS_N_ARB: c_uint = 0x140258;
pub const mmMME5_RTR_LBW_RD_RS_S_ARB: c_uint = 0x14025C;
pub const mmMME5_RTR_LBW_RD_RS_L_ARB: c_uint = 0x140260;
pub const mmMME5_RTR_LBW_WR_RQ_E_ARB: c_uint = 0x140270;
pub const mmMME5_RTR_LBW_WR_RQ_W_ARB: c_uint = 0x140274;
pub const mmMME5_RTR_LBW_WR_RQ_N_ARB: c_uint = 0x140278;
pub const mmMME5_RTR_LBW_WR_RQ_S_ARB: c_uint = 0x14027C;
pub const mmMME5_RTR_LBW_WR_RQ_L_ARB: c_uint = 0x140280;
pub const mmMME5_RTR_LBW_WR_RS_E_ARB: c_uint = 0x140290;
pub const mmMME5_RTR_LBW_WR_RS_W_ARB: c_uint = 0x140294;
pub const mmMME5_RTR_LBW_WR_RS_N_ARB: c_uint = 0x140298;
pub const mmMME5_RTR_LBW_WR_RS_S_ARB: c_uint = 0x14029C;
pub const mmMME5_RTR_LBW_WR_RS_L_ARB: c_uint = 0x1402A0;
pub const mmMME5_RTR_DBG_E_ARB: c_uint = 0x140300;
pub const mmMME5_RTR_DBG_W_ARB: c_uint = 0x140304;
pub const mmMME5_RTR_DBG_N_ARB: c_uint = 0x140308;
pub const mmMME5_RTR_DBG_S_ARB: c_uint = 0x14030C;
pub const mmMME5_RTR_DBG_L_ARB: c_uint = 0x140310;
pub const mmMME5_RTR_DBG_E_ARB_MAX: c_uint = 0x140320;
pub const mmMME5_RTR_DBG_W_ARB_MAX: c_uint = 0x140324;
pub const mmMME5_RTR_DBG_N_ARB_MAX: c_uint = 0x140328;
pub const mmMME5_RTR_DBG_S_ARB_MAX: c_uint = 0x14032C;
pub const mmMME5_RTR_DBG_L_ARB_MAX: c_uint = 0x140330;
pub const mmMME5_RTR_SPLIT_COEF_0: c_uint = 0x140400;
pub const mmMME5_RTR_SPLIT_COEF_1: c_uint = 0x140404;
pub const mmMME5_RTR_SPLIT_COEF_2: c_uint = 0x140408;
pub const mmMME5_RTR_SPLIT_COEF_3: c_uint = 0x14040C;
pub const mmMME5_RTR_SPLIT_COEF_4: c_uint = 0x140410;
pub const mmMME5_RTR_SPLIT_COEF_5: c_uint = 0x140414;
pub const mmMME5_RTR_SPLIT_COEF_6: c_uint = 0x140418;
pub const mmMME5_RTR_SPLIT_COEF_7: c_uint = 0x14041C;
pub const mmMME5_RTR_SPLIT_COEF_8: c_uint = 0x140420;
pub const mmMME5_RTR_SPLIT_COEF_9: c_uint = 0x140424;
pub const mmMME5_RTR_SPLIT_CFG: c_uint = 0x140440;
pub const mmMME5_RTR_SPLIT_RD_SAT: c_uint = 0x140444;
pub const mmMME5_RTR_SPLIT_RD_RST_TOKEN: c_uint = 0x140448;
pub const mmMME5_RTR_SPLIT_RD_TIMEOUT_0: c_uint = 0x14044C;
pub const mmMME5_RTR_SPLIT_RD_TIMEOUT_1: c_uint = 0x140450;
pub const mmMME5_RTR_SPLIT_WR_SAT: c_uint = 0x140454;
pub const mmMME5_RTR_WPLIT_WR_TST_TOLEN: c_uint = 0x140458;
pub const mmMME5_RTR_SPLIT_WR_TIMEOUT_0: c_uint = 0x14045C;
pub const mmMME5_RTR_SPLIT_WR_TIMEOUT_1: c_uint = 0x140460;
pub const mmMME5_RTR_HBW_RANGE_HIT: c_uint = 0x140470;
pub const mmMME5_RTR_HBW_RANGE_MASK_L_0: c_uint = 0x140480;
pub const mmMME5_RTR_HBW_RANGE_MASK_L_1: c_uint = 0x140484;
pub const mmMME5_RTR_HBW_RANGE_MASK_L_2: c_uint = 0x140488;
pub const mmMME5_RTR_HBW_RANGE_MASK_L_3: c_uint = 0x14048C;
pub const mmMME5_RTR_HBW_RANGE_MASK_L_4: c_uint = 0x140490;
pub const mmMME5_RTR_HBW_RANGE_MASK_L_5: c_uint = 0x140494;
pub const mmMME5_RTR_HBW_RANGE_MASK_L_6: c_uint = 0x140498;
pub const mmMME5_RTR_HBW_RANGE_MASK_L_7: c_uint = 0x14049C;
pub const mmMME5_RTR_HBW_RANGE_MASK_H_0: c_uint = 0x1404A0;
pub const mmMME5_RTR_HBW_RANGE_MASK_H_1: c_uint = 0x1404A4;
pub const mmMME5_RTR_HBW_RANGE_MASK_H_2: c_uint = 0x1404A8;
pub const mmMME5_RTR_HBW_RANGE_MASK_H_3: c_uint = 0x1404AC;
pub const mmMME5_RTR_HBW_RANGE_MASK_H_4: c_uint = 0x1404B0;
pub const mmMME5_RTR_HBW_RANGE_MASK_H_5: c_uint = 0x1404B4;
pub const mmMME5_RTR_HBW_RANGE_MASK_H_6: c_uint = 0x1404B8;
pub const mmMME5_RTR_HBW_RANGE_MASK_H_7: c_uint = 0x1404BC;
pub const mmMME5_RTR_HBW_RANGE_BASE_L_0: c_uint = 0x1404C0;
pub const mmMME5_RTR_HBW_RANGE_BASE_L_1: c_uint = 0x1404C4;
pub const mmMME5_RTR_HBW_RANGE_BASE_L_2: c_uint = 0x1404C8;
pub const mmMME5_RTR_HBW_RANGE_BASE_L_3: c_uint = 0x1404CC;
pub const mmMME5_RTR_HBW_RANGE_BASE_L_4: c_uint = 0x1404D0;
pub const mmMME5_RTR_HBW_RANGE_BASE_L_5: c_uint = 0x1404D4;
pub const mmMME5_RTR_HBW_RANGE_BASE_L_6: c_uint = 0x1404D8;
pub const mmMME5_RTR_HBW_RANGE_BASE_L_7: c_uint = 0x1404DC;
pub const mmMME5_RTR_HBW_RANGE_BASE_H_0: c_uint = 0x1404E0;
pub const mmMME5_RTR_HBW_RANGE_BASE_H_1: c_uint = 0x1404E4;
pub const mmMME5_RTR_HBW_RANGE_BASE_H_2: c_uint = 0x1404E8;
pub const mmMME5_RTR_HBW_RANGE_BASE_H_3: c_uint = 0x1404EC;
pub const mmMME5_RTR_HBW_RANGE_BASE_H_4: c_uint = 0x1404F0;
pub const mmMME5_RTR_HBW_RANGE_BASE_H_5: c_uint = 0x1404F4;
pub const mmMME5_RTR_HBW_RANGE_BASE_H_6: c_uint = 0x1404F8;
pub const mmMME5_RTR_HBW_RANGE_BASE_H_7: c_uint = 0x1404FC;
pub const mmMME5_RTR_LBW_RANGE_HIT: c_uint = 0x140500;
pub const mmMME5_RTR_LBW_RANGE_MASK_0: c_uint = 0x140510;
pub const mmMME5_RTR_LBW_RANGE_MASK_1: c_uint = 0x140514;
pub const mmMME5_RTR_LBW_RANGE_MASK_2: c_uint = 0x140518;
pub const mmMME5_RTR_LBW_RANGE_MASK_3: c_uint = 0x14051C;
pub const mmMME5_RTR_LBW_RANGE_MASK_4: c_uint = 0x140520;
pub const mmMME5_RTR_LBW_RANGE_MASK_5: c_uint = 0x140524;
pub const mmMME5_RTR_LBW_RANGE_MASK_6: c_uint = 0x140528;
pub const mmMME5_RTR_LBW_RANGE_MASK_7: c_uint = 0x14052C;
pub const mmMME5_RTR_LBW_RANGE_MASK_8: c_uint = 0x140530;
pub const mmMME5_RTR_LBW_RANGE_MASK_9: c_uint = 0x140534;
pub const mmMME5_RTR_LBW_RANGE_MASK_10: c_uint = 0x140538;
pub const mmMME5_RTR_LBW_RANGE_MASK_11: c_uint = 0x14053C;
pub const mmMME5_RTR_LBW_RANGE_MASK_12: c_uint = 0x140540;
pub const mmMME5_RTR_LBW_RANGE_MASK_13: c_uint = 0x140544;
pub const mmMME5_RTR_LBW_RANGE_MASK_14: c_uint = 0x140548;
pub const mmMME5_RTR_LBW_RANGE_MASK_15: c_uint = 0x14054C;
pub const mmMME5_RTR_LBW_RANGE_BASE_0: c_uint = 0x140550;
pub const mmMME5_RTR_LBW_RANGE_BASE_1: c_uint = 0x140554;
pub const mmMME5_RTR_LBW_RANGE_BASE_2: c_uint = 0x140558;
pub const mmMME5_RTR_LBW_RANGE_BASE_3: c_uint = 0x14055C;
pub const mmMME5_RTR_LBW_RANGE_BASE_4: c_uint = 0x140560;
pub const mmMME5_RTR_LBW_RANGE_BASE_5: c_uint = 0x140564;
pub const mmMME5_RTR_LBW_RANGE_BASE_6: c_uint = 0x140568;
pub const mmMME5_RTR_LBW_RANGE_BASE_7: c_uint = 0x14056C;
pub const mmMME5_RTR_LBW_RANGE_BASE_8: c_uint = 0x140570;
pub const mmMME5_RTR_LBW_RANGE_BASE_9: c_uint = 0x140574;
pub const mmMME5_RTR_LBW_RANGE_BASE_10: c_uint = 0x140578;
pub const mmMME5_RTR_LBW_RANGE_BASE_11: c_uint = 0x14057C;
pub const mmMME5_RTR_LBW_RANGE_BASE_12: c_uint = 0x140580;
pub const mmMME5_RTR_LBW_RANGE_BASE_13: c_uint = 0x140584;
pub const mmMME5_RTR_LBW_RANGE_BASE_14: c_uint = 0x140588;
pub const mmMME5_RTR_LBW_RANGE_BASE_15: c_uint = 0x14058C;
pub const mmMME5_RTR_RGLTR: c_uint = 0x140590;
pub const mmMME5_RTR_RGLTR_WR_RESULT: c_uint = 0x140594;
pub const mmMME5_RTR_RGLTR_RD_RESULT: c_uint = 0x140598;
pub const mmMME5_RTR_SCRAMB_EN: c_uint = 0x140600;
pub const mmMME5_RTR_NON_LIN_SCRAMB: c_uint = 0x140604;
