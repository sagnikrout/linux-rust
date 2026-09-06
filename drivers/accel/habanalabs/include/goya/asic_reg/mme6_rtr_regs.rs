//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/mme6_rtr_regs.h
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
// MME6_RTR (Prototype: MME_RTR)
//
pub const mmMME6_RTR_HBW_RD_RQ_E_ARB: c_uint = 0x180100;
pub const mmMME6_RTR_HBW_RD_RQ_W_ARB: c_uint = 0x180104;
pub const mmMME6_RTR_HBW_RD_RQ_N_ARB: c_uint = 0x180108;
pub const mmMME6_RTR_HBW_RD_RQ_S_ARB: c_uint = 0x18010C;
pub const mmMME6_RTR_HBW_RD_RQ_L_ARB: c_uint = 0x180110;
pub const mmMME6_RTR_HBW_E_ARB_MAX: c_uint = 0x180120;
pub const mmMME6_RTR_HBW_W_ARB_MAX: c_uint = 0x180124;
pub const mmMME6_RTR_HBW_N_ARB_MAX: c_uint = 0x180128;
pub const mmMME6_RTR_HBW_S_ARB_MAX: c_uint = 0x18012C;
pub const mmMME6_RTR_HBW_L_ARB_MAX: c_uint = 0x180130;
pub const mmMME6_RTR_HBW_RD_RS_MAX_CREDIT: c_uint = 0x180140;
pub const mmMME6_RTR_HBW_WR_RQ_MAX_CREDIT: c_uint = 0x180144;
pub const mmMME6_RTR_HBW_RD_RQ_MAX_CREDIT: c_uint = 0x180148;
pub const mmMME6_RTR_HBW_RD_RS_E_ARB: c_uint = 0x180150;
pub const mmMME6_RTR_HBW_RD_RS_W_ARB: c_uint = 0x180154;
pub const mmMME6_RTR_HBW_RD_RS_N_ARB: c_uint = 0x180158;
pub const mmMME6_RTR_HBW_RD_RS_S_ARB: c_uint = 0x18015C;
pub const mmMME6_RTR_HBW_RD_RS_L_ARB: c_uint = 0x180160;
pub const mmMME6_RTR_HBW_WR_RQ_E_ARB: c_uint = 0x180170;
pub const mmMME6_RTR_HBW_WR_RQ_W_ARB: c_uint = 0x180174;
pub const mmMME6_RTR_HBW_WR_RQ_N_ARB: c_uint = 0x180178;
pub const mmMME6_RTR_HBW_WR_RQ_S_ARB: c_uint = 0x18017C;
pub const mmMME6_RTR_HBW_WR_RQ_L_ARB: c_uint = 0x180180;
pub const mmMME6_RTR_HBW_WR_RS_E_ARB: c_uint = 0x180190;
pub const mmMME6_RTR_HBW_WR_RS_W_ARB: c_uint = 0x180194;
pub const mmMME6_RTR_HBW_WR_RS_N_ARB: c_uint = 0x180198;
pub const mmMME6_RTR_HBW_WR_RS_S_ARB: c_uint = 0x18019C;
pub const mmMME6_RTR_HBW_WR_RS_L_ARB: c_uint = 0x1801A0;
pub const mmMME6_RTR_LBW_RD_RQ_E_ARB: c_uint = 0x180200;
pub const mmMME6_RTR_LBW_RD_RQ_W_ARB: c_uint = 0x180204;
pub const mmMME6_RTR_LBW_RD_RQ_N_ARB: c_uint = 0x180208;
pub const mmMME6_RTR_LBW_RD_RQ_S_ARB: c_uint = 0x18020C;
pub const mmMME6_RTR_LBW_RD_RQ_L_ARB: c_uint = 0x180210;
pub const mmMME6_RTR_LBW_E_ARB_MAX: c_uint = 0x180220;
pub const mmMME6_RTR_LBW_W_ARB_MAX: c_uint = 0x180224;
pub const mmMME6_RTR_LBW_N_ARB_MAX: c_uint = 0x180228;
pub const mmMME6_RTR_LBW_S_ARB_MAX: c_uint = 0x18022C;
pub const mmMME6_RTR_LBW_L_ARB_MAX: c_uint = 0x180230;
pub const mmMME6_RTR_LBW_SRAM_MAX_CREDIT: c_uint = 0x180240;
pub const mmMME6_RTR_LBW_RD_RS_E_ARB: c_uint = 0x180250;
pub const mmMME6_RTR_LBW_RD_RS_W_ARB: c_uint = 0x180254;
pub const mmMME6_RTR_LBW_RD_RS_N_ARB: c_uint = 0x180258;
pub const mmMME6_RTR_LBW_RD_RS_S_ARB: c_uint = 0x18025C;
pub const mmMME6_RTR_LBW_RD_RS_L_ARB: c_uint = 0x180260;
pub const mmMME6_RTR_LBW_WR_RQ_E_ARB: c_uint = 0x180270;
pub const mmMME6_RTR_LBW_WR_RQ_W_ARB: c_uint = 0x180274;
pub const mmMME6_RTR_LBW_WR_RQ_N_ARB: c_uint = 0x180278;
pub const mmMME6_RTR_LBW_WR_RQ_S_ARB: c_uint = 0x18027C;
pub const mmMME6_RTR_LBW_WR_RQ_L_ARB: c_uint = 0x180280;
pub const mmMME6_RTR_LBW_WR_RS_E_ARB: c_uint = 0x180290;
pub const mmMME6_RTR_LBW_WR_RS_W_ARB: c_uint = 0x180294;
pub const mmMME6_RTR_LBW_WR_RS_N_ARB: c_uint = 0x180298;
pub const mmMME6_RTR_LBW_WR_RS_S_ARB: c_uint = 0x18029C;
pub const mmMME6_RTR_LBW_WR_RS_L_ARB: c_uint = 0x1802A0;
pub const mmMME6_RTR_DBG_E_ARB: c_uint = 0x180300;
pub const mmMME6_RTR_DBG_W_ARB: c_uint = 0x180304;
pub const mmMME6_RTR_DBG_N_ARB: c_uint = 0x180308;
pub const mmMME6_RTR_DBG_S_ARB: c_uint = 0x18030C;
pub const mmMME6_RTR_DBG_L_ARB: c_uint = 0x180310;
pub const mmMME6_RTR_DBG_E_ARB_MAX: c_uint = 0x180320;
pub const mmMME6_RTR_DBG_W_ARB_MAX: c_uint = 0x180324;
pub const mmMME6_RTR_DBG_N_ARB_MAX: c_uint = 0x180328;
pub const mmMME6_RTR_DBG_S_ARB_MAX: c_uint = 0x18032C;
pub const mmMME6_RTR_DBG_L_ARB_MAX: c_uint = 0x180330;
pub const mmMME6_RTR_SPLIT_COEF_0: c_uint = 0x180400;
pub const mmMME6_RTR_SPLIT_COEF_1: c_uint = 0x180404;
pub const mmMME6_RTR_SPLIT_COEF_2: c_uint = 0x180408;
pub const mmMME6_RTR_SPLIT_COEF_3: c_uint = 0x18040C;
pub const mmMME6_RTR_SPLIT_COEF_4: c_uint = 0x180410;
pub const mmMME6_RTR_SPLIT_COEF_5: c_uint = 0x180414;
pub const mmMME6_RTR_SPLIT_COEF_6: c_uint = 0x180418;
pub const mmMME6_RTR_SPLIT_COEF_7: c_uint = 0x18041C;
pub const mmMME6_RTR_SPLIT_COEF_8: c_uint = 0x180420;
pub const mmMME6_RTR_SPLIT_COEF_9: c_uint = 0x180424;
pub const mmMME6_RTR_SPLIT_CFG: c_uint = 0x180440;
pub const mmMME6_RTR_SPLIT_RD_SAT: c_uint = 0x180444;
pub const mmMME6_RTR_SPLIT_RD_RST_TOKEN: c_uint = 0x180448;
pub const mmMME6_RTR_SPLIT_RD_TIMEOUT_0: c_uint = 0x18044C;
pub const mmMME6_RTR_SPLIT_RD_TIMEOUT_1: c_uint = 0x180450;
pub const mmMME6_RTR_SPLIT_WR_SAT: c_uint = 0x180454;
pub const mmMME6_RTR_WPLIT_WR_TST_TOLEN: c_uint = 0x180458;
pub const mmMME6_RTR_SPLIT_WR_TIMEOUT_0: c_uint = 0x18045C;
pub const mmMME6_RTR_SPLIT_WR_TIMEOUT_1: c_uint = 0x180460;
pub const mmMME6_RTR_HBW_RANGE_HIT: c_uint = 0x180470;
pub const mmMME6_RTR_HBW_RANGE_MASK_L_0: c_uint = 0x180480;
pub const mmMME6_RTR_HBW_RANGE_MASK_L_1: c_uint = 0x180484;
pub const mmMME6_RTR_HBW_RANGE_MASK_L_2: c_uint = 0x180488;
pub const mmMME6_RTR_HBW_RANGE_MASK_L_3: c_uint = 0x18048C;
pub const mmMME6_RTR_HBW_RANGE_MASK_L_4: c_uint = 0x180490;
pub const mmMME6_RTR_HBW_RANGE_MASK_L_5: c_uint = 0x180494;
pub const mmMME6_RTR_HBW_RANGE_MASK_L_6: c_uint = 0x180498;
pub const mmMME6_RTR_HBW_RANGE_MASK_L_7: c_uint = 0x18049C;
pub const mmMME6_RTR_HBW_RANGE_MASK_H_0: c_uint = 0x1804A0;
pub const mmMME6_RTR_HBW_RANGE_MASK_H_1: c_uint = 0x1804A4;
pub const mmMME6_RTR_HBW_RANGE_MASK_H_2: c_uint = 0x1804A8;
pub const mmMME6_RTR_HBW_RANGE_MASK_H_3: c_uint = 0x1804AC;
pub const mmMME6_RTR_HBW_RANGE_MASK_H_4: c_uint = 0x1804B0;
pub const mmMME6_RTR_HBW_RANGE_MASK_H_5: c_uint = 0x1804B4;
pub const mmMME6_RTR_HBW_RANGE_MASK_H_6: c_uint = 0x1804B8;
pub const mmMME6_RTR_HBW_RANGE_MASK_H_7: c_uint = 0x1804BC;
pub const mmMME6_RTR_HBW_RANGE_BASE_L_0: c_uint = 0x1804C0;
pub const mmMME6_RTR_HBW_RANGE_BASE_L_1: c_uint = 0x1804C4;
pub const mmMME6_RTR_HBW_RANGE_BASE_L_2: c_uint = 0x1804C8;
pub const mmMME6_RTR_HBW_RANGE_BASE_L_3: c_uint = 0x1804CC;
pub const mmMME6_RTR_HBW_RANGE_BASE_L_4: c_uint = 0x1804D0;
pub const mmMME6_RTR_HBW_RANGE_BASE_L_5: c_uint = 0x1804D4;
pub const mmMME6_RTR_HBW_RANGE_BASE_L_6: c_uint = 0x1804D8;
pub const mmMME6_RTR_HBW_RANGE_BASE_L_7: c_uint = 0x1804DC;
pub const mmMME6_RTR_HBW_RANGE_BASE_H_0: c_uint = 0x1804E0;
pub const mmMME6_RTR_HBW_RANGE_BASE_H_1: c_uint = 0x1804E4;
pub const mmMME6_RTR_HBW_RANGE_BASE_H_2: c_uint = 0x1804E8;
pub const mmMME6_RTR_HBW_RANGE_BASE_H_3: c_uint = 0x1804EC;
pub const mmMME6_RTR_HBW_RANGE_BASE_H_4: c_uint = 0x1804F0;
pub const mmMME6_RTR_HBW_RANGE_BASE_H_5: c_uint = 0x1804F4;
pub const mmMME6_RTR_HBW_RANGE_BASE_H_6: c_uint = 0x1804F8;
pub const mmMME6_RTR_HBW_RANGE_BASE_H_7: c_uint = 0x1804FC;
pub const mmMME6_RTR_LBW_RANGE_HIT: c_uint = 0x180500;
pub const mmMME6_RTR_LBW_RANGE_MASK_0: c_uint = 0x180510;
pub const mmMME6_RTR_LBW_RANGE_MASK_1: c_uint = 0x180514;
pub const mmMME6_RTR_LBW_RANGE_MASK_2: c_uint = 0x180518;
pub const mmMME6_RTR_LBW_RANGE_MASK_3: c_uint = 0x18051C;
pub const mmMME6_RTR_LBW_RANGE_MASK_4: c_uint = 0x180520;
pub const mmMME6_RTR_LBW_RANGE_MASK_5: c_uint = 0x180524;
pub const mmMME6_RTR_LBW_RANGE_MASK_6: c_uint = 0x180528;
pub const mmMME6_RTR_LBW_RANGE_MASK_7: c_uint = 0x18052C;
pub const mmMME6_RTR_LBW_RANGE_MASK_8: c_uint = 0x180530;
pub const mmMME6_RTR_LBW_RANGE_MASK_9: c_uint = 0x180534;
pub const mmMME6_RTR_LBW_RANGE_MASK_10: c_uint = 0x180538;
pub const mmMME6_RTR_LBW_RANGE_MASK_11: c_uint = 0x18053C;
pub const mmMME6_RTR_LBW_RANGE_MASK_12: c_uint = 0x180540;
pub const mmMME6_RTR_LBW_RANGE_MASK_13: c_uint = 0x180544;
pub const mmMME6_RTR_LBW_RANGE_MASK_14: c_uint = 0x180548;
pub const mmMME6_RTR_LBW_RANGE_MASK_15: c_uint = 0x18054C;
pub const mmMME6_RTR_LBW_RANGE_BASE_0: c_uint = 0x180550;
pub const mmMME6_RTR_LBW_RANGE_BASE_1: c_uint = 0x180554;
pub const mmMME6_RTR_LBW_RANGE_BASE_2: c_uint = 0x180558;
pub const mmMME6_RTR_LBW_RANGE_BASE_3: c_uint = 0x18055C;
pub const mmMME6_RTR_LBW_RANGE_BASE_4: c_uint = 0x180560;
pub const mmMME6_RTR_LBW_RANGE_BASE_5: c_uint = 0x180564;
pub const mmMME6_RTR_LBW_RANGE_BASE_6: c_uint = 0x180568;
pub const mmMME6_RTR_LBW_RANGE_BASE_7: c_uint = 0x18056C;
pub const mmMME6_RTR_LBW_RANGE_BASE_8: c_uint = 0x180570;
pub const mmMME6_RTR_LBW_RANGE_BASE_9: c_uint = 0x180574;
pub const mmMME6_RTR_LBW_RANGE_BASE_10: c_uint = 0x180578;
pub const mmMME6_RTR_LBW_RANGE_BASE_11: c_uint = 0x18057C;
pub const mmMME6_RTR_LBW_RANGE_BASE_12: c_uint = 0x180580;
pub const mmMME6_RTR_LBW_RANGE_BASE_13: c_uint = 0x180584;
pub const mmMME6_RTR_LBW_RANGE_BASE_14: c_uint = 0x180588;
pub const mmMME6_RTR_LBW_RANGE_BASE_15: c_uint = 0x18058C;
pub const mmMME6_RTR_RGLTR: c_uint = 0x180590;
pub const mmMME6_RTR_RGLTR_WR_RESULT: c_uint = 0x180594;
pub const mmMME6_RTR_RGLTR_RD_RESULT: c_uint = 0x180598;
pub const mmMME6_RTR_SCRAMB_EN: c_uint = 0x180600;
pub const mmMME6_RTR_NON_LIN_SCRAMB: c_uint = 0x180604;
