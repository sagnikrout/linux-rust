//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc3_rtr_regs.h
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
// TPC3_RTR (Prototype: TPC_RTR)
//
pub const mmTPC3_RTR_HBW_RD_RQ_E_ARB: c_uint = 0xEC0100;
pub const mmTPC3_RTR_HBW_RD_RQ_W_ARB: c_uint = 0xEC0104;
pub const mmTPC3_RTR_HBW_RD_RQ_N_ARB: c_uint = 0xEC0108;
pub const mmTPC3_RTR_HBW_RD_RQ_S_ARB: c_uint = 0xEC010C;
pub const mmTPC3_RTR_HBW_RD_RQ_L_ARB: c_uint = 0xEC0110;
pub const mmTPC3_RTR_HBW_E_ARB_MAX: c_uint = 0xEC0120;
pub const mmTPC3_RTR_HBW_W_ARB_MAX: c_uint = 0xEC0124;
pub const mmTPC3_RTR_HBW_N_ARB_MAX: c_uint = 0xEC0128;
pub const mmTPC3_RTR_HBW_S_ARB_MAX: c_uint = 0xEC012C;
pub const mmTPC3_RTR_HBW_L_ARB_MAX: c_uint = 0xEC0130;
pub const mmTPC3_RTR_HBW_RD_RS_E_ARB: c_uint = 0xEC0140;
pub const mmTPC3_RTR_HBW_RD_RS_W_ARB: c_uint = 0xEC0144;
pub const mmTPC3_RTR_HBW_RD_RS_N_ARB: c_uint = 0xEC0148;
pub const mmTPC3_RTR_HBW_RD_RS_S_ARB: c_uint = 0xEC014C;
pub const mmTPC3_RTR_HBW_RD_RS_L_ARB: c_uint = 0xEC0150;
pub const mmTPC3_RTR_HBW_WR_RQ_E_ARB: c_uint = 0xEC0170;
pub const mmTPC3_RTR_HBW_WR_RQ_W_ARB: c_uint = 0xEC0174;
pub const mmTPC3_RTR_HBW_WR_RQ_N_ARB: c_uint = 0xEC0178;
pub const mmTPC3_RTR_HBW_WR_RQ_S_ARB: c_uint = 0xEC017C;
pub const mmTPC3_RTR_HBW_WR_RQ_L_ARB: c_uint = 0xEC0180;
pub const mmTPC3_RTR_HBW_WR_RS_E_ARB: c_uint = 0xEC0190;
pub const mmTPC3_RTR_HBW_WR_RS_W_ARB: c_uint = 0xEC0194;
pub const mmTPC3_RTR_HBW_WR_RS_N_ARB: c_uint = 0xEC0198;
pub const mmTPC3_RTR_HBW_WR_RS_S_ARB: c_uint = 0xEC019C;
pub const mmTPC3_RTR_HBW_WR_RS_L_ARB: c_uint = 0xEC01A0;
pub const mmTPC3_RTR_LBW_RD_RQ_E_ARB: c_uint = 0xEC0200;
pub const mmTPC3_RTR_LBW_RD_RQ_W_ARB: c_uint = 0xEC0204;
pub const mmTPC3_RTR_LBW_RD_RQ_N_ARB: c_uint = 0xEC0208;
pub const mmTPC3_RTR_LBW_RD_RQ_S_ARB: c_uint = 0xEC020C;
pub const mmTPC3_RTR_LBW_RD_RQ_L_ARB: c_uint = 0xEC0210;
pub const mmTPC3_RTR_LBW_E_ARB_MAX: c_uint = 0xEC0220;
pub const mmTPC3_RTR_LBW_W_ARB_MAX: c_uint = 0xEC0224;
pub const mmTPC3_RTR_LBW_N_ARB_MAX: c_uint = 0xEC0228;
pub const mmTPC3_RTR_LBW_S_ARB_MAX: c_uint = 0xEC022C;
pub const mmTPC3_RTR_LBW_L_ARB_MAX: c_uint = 0xEC0230;
pub const mmTPC3_RTR_LBW_RD_RS_E_ARB: c_uint = 0xEC0250;
pub const mmTPC3_RTR_LBW_RD_RS_W_ARB: c_uint = 0xEC0254;
pub const mmTPC3_RTR_LBW_RD_RS_N_ARB: c_uint = 0xEC0258;
pub const mmTPC3_RTR_LBW_RD_RS_S_ARB: c_uint = 0xEC025C;
pub const mmTPC3_RTR_LBW_RD_RS_L_ARB: c_uint = 0xEC0260;
pub const mmTPC3_RTR_LBW_WR_RQ_E_ARB: c_uint = 0xEC0270;
pub const mmTPC3_RTR_LBW_WR_RQ_W_ARB: c_uint = 0xEC0274;
pub const mmTPC3_RTR_LBW_WR_RQ_N_ARB: c_uint = 0xEC0278;
pub const mmTPC3_RTR_LBW_WR_RQ_S_ARB: c_uint = 0xEC027C;
pub const mmTPC3_RTR_LBW_WR_RQ_L_ARB: c_uint = 0xEC0280;
pub const mmTPC3_RTR_LBW_WR_RS_E_ARB: c_uint = 0xEC0290;
pub const mmTPC3_RTR_LBW_WR_RS_W_ARB: c_uint = 0xEC0294;
pub const mmTPC3_RTR_LBW_WR_RS_N_ARB: c_uint = 0xEC0298;
pub const mmTPC3_RTR_LBW_WR_RS_S_ARB: c_uint = 0xEC029C;
pub const mmTPC3_RTR_LBW_WR_RS_L_ARB: c_uint = 0xEC02A0;
pub const mmTPC3_RTR_DBG_E_ARB: c_uint = 0xEC0300;
pub const mmTPC3_RTR_DBG_W_ARB: c_uint = 0xEC0304;
pub const mmTPC3_RTR_DBG_N_ARB: c_uint = 0xEC0308;
pub const mmTPC3_RTR_DBG_S_ARB: c_uint = 0xEC030C;
pub const mmTPC3_RTR_DBG_L_ARB: c_uint = 0xEC0310;
pub const mmTPC3_RTR_DBG_E_ARB_MAX: c_uint = 0xEC0320;
pub const mmTPC3_RTR_DBG_W_ARB_MAX: c_uint = 0xEC0324;
pub const mmTPC3_RTR_DBG_N_ARB_MAX: c_uint = 0xEC0328;
pub const mmTPC3_RTR_DBG_S_ARB_MAX: c_uint = 0xEC032C;
pub const mmTPC3_RTR_DBG_L_ARB_MAX: c_uint = 0xEC0330;
pub const mmTPC3_RTR_SPLIT_COEF_0: c_uint = 0xEC0400;
pub const mmTPC3_RTR_SPLIT_COEF_1: c_uint = 0xEC0404;
pub const mmTPC3_RTR_SPLIT_COEF_2: c_uint = 0xEC0408;
pub const mmTPC3_RTR_SPLIT_COEF_3: c_uint = 0xEC040C;
pub const mmTPC3_RTR_SPLIT_COEF_4: c_uint = 0xEC0410;
pub const mmTPC3_RTR_SPLIT_COEF_5: c_uint = 0xEC0414;
pub const mmTPC3_RTR_SPLIT_COEF_6: c_uint = 0xEC0418;
pub const mmTPC3_RTR_SPLIT_COEF_7: c_uint = 0xEC041C;
pub const mmTPC3_RTR_SPLIT_COEF_8: c_uint = 0xEC0420;
pub const mmTPC3_RTR_SPLIT_COEF_9: c_uint = 0xEC0424;
pub const mmTPC3_RTR_SPLIT_CFG: c_uint = 0xEC0440;
pub const mmTPC3_RTR_SPLIT_RD_SAT: c_uint = 0xEC0444;
pub const mmTPC3_RTR_SPLIT_RD_RST_TOKEN: c_uint = 0xEC0448;
pub const mmTPC3_RTR_SPLIT_RD_TIMEOUT_0: c_uint = 0xEC044C;
pub const mmTPC3_RTR_SPLIT_RD_TIMEOUT_1: c_uint = 0xEC0450;
pub const mmTPC3_RTR_SPLIT_WR_SAT: c_uint = 0xEC0454;
pub const mmTPC3_RTR_WPLIT_WR_TST_TOLEN: c_uint = 0xEC0458;
pub const mmTPC3_RTR_SPLIT_WR_TIMEOUT_0: c_uint = 0xEC045C;
pub const mmTPC3_RTR_SPLIT_WR_TIMEOUT_1: c_uint = 0xEC0460;
pub const mmTPC3_RTR_HBW_RANGE_HIT: c_uint = 0xEC0470;
pub const mmTPC3_RTR_HBW_RANGE_MASK_L_0: c_uint = 0xEC0480;
pub const mmTPC3_RTR_HBW_RANGE_MASK_L_1: c_uint = 0xEC0484;
pub const mmTPC3_RTR_HBW_RANGE_MASK_L_2: c_uint = 0xEC0488;
pub const mmTPC3_RTR_HBW_RANGE_MASK_L_3: c_uint = 0xEC048C;
pub const mmTPC3_RTR_HBW_RANGE_MASK_L_4: c_uint = 0xEC0490;
pub const mmTPC3_RTR_HBW_RANGE_MASK_L_5: c_uint = 0xEC0494;
pub const mmTPC3_RTR_HBW_RANGE_MASK_L_6: c_uint = 0xEC0498;
pub const mmTPC3_RTR_HBW_RANGE_MASK_L_7: c_uint = 0xEC049C;
pub const mmTPC3_RTR_HBW_RANGE_MASK_H_0: c_uint = 0xEC04A0;
pub const mmTPC3_RTR_HBW_RANGE_MASK_H_1: c_uint = 0xEC04A4;
pub const mmTPC3_RTR_HBW_RANGE_MASK_H_2: c_uint = 0xEC04A8;
pub const mmTPC3_RTR_HBW_RANGE_MASK_H_3: c_uint = 0xEC04AC;
pub const mmTPC3_RTR_HBW_RANGE_MASK_H_4: c_uint = 0xEC04B0;
pub const mmTPC3_RTR_HBW_RANGE_MASK_H_5: c_uint = 0xEC04B4;
pub const mmTPC3_RTR_HBW_RANGE_MASK_H_6: c_uint = 0xEC04B8;
pub const mmTPC3_RTR_HBW_RANGE_MASK_H_7: c_uint = 0xEC04BC;
pub const mmTPC3_RTR_HBW_RANGE_BASE_L_0: c_uint = 0xEC04C0;
pub const mmTPC3_RTR_HBW_RANGE_BASE_L_1: c_uint = 0xEC04C4;
pub const mmTPC3_RTR_HBW_RANGE_BASE_L_2: c_uint = 0xEC04C8;
pub const mmTPC3_RTR_HBW_RANGE_BASE_L_3: c_uint = 0xEC04CC;
pub const mmTPC3_RTR_HBW_RANGE_BASE_L_4: c_uint = 0xEC04D0;
pub const mmTPC3_RTR_HBW_RANGE_BASE_L_5: c_uint = 0xEC04D4;
pub const mmTPC3_RTR_HBW_RANGE_BASE_L_6: c_uint = 0xEC04D8;
pub const mmTPC3_RTR_HBW_RANGE_BASE_L_7: c_uint = 0xEC04DC;
pub const mmTPC3_RTR_HBW_RANGE_BASE_H_0: c_uint = 0xEC04E0;
pub const mmTPC3_RTR_HBW_RANGE_BASE_H_1: c_uint = 0xEC04E4;
pub const mmTPC3_RTR_HBW_RANGE_BASE_H_2: c_uint = 0xEC04E8;
pub const mmTPC3_RTR_HBW_RANGE_BASE_H_3: c_uint = 0xEC04EC;
pub const mmTPC3_RTR_HBW_RANGE_BASE_H_4: c_uint = 0xEC04F0;
pub const mmTPC3_RTR_HBW_RANGE_BASE_H_5: c_uint = 0xEC04F4;
pub const mmTPC3_RTR_HBW_RANGE_BASE_H_6: c_uint = 0xEC04F8;
pub const mmTPC3_RTR_HBW_RANGE_BASE_H_7: c_uint = 0xEC04FC;
pub const mmTPC3_RTR_LBW_RANGE_HIT: c_uint = 0xEC0500;
pub const mmTPC3_RTR_LBW_RANGE_MASK_0: c_uint = 0xEC0510;
pub const mmTPC3_RTR_LBW_RANGE_MASK_1: c_uint = 0xEC0514;
pub const mmTPC3_RTR_LBW_RANGE_MASK_2: c_uint = 0xEC0518;
pub const mmTPC3_RTR_LBW_RANGE_MASK_3: c_uint = 0xEC051C;
pub const mmTPC3_RTR_LBW_RANGE_MASK_4: c_uint = 0xEC0520;
pub const mmTPC3_RTR_LBW_RANGE_MASK_5: c_uint = 0xEC0524;
pub const mmTPC3_RTR_LBW_RANGE_MASK_6: c_uint = 0xEC0528;
pub const mmTPC3_RTR_LBW_RANGE_MASK_7: c_uint = 0xEC052C;
pub const mmTPC3_RTR_LBW_RANGE_MASK_8: c_uint = 0xEC0530;
pub const mmTPC3_RTR_LBW_RANGE_MASK_9: c_uint = 0xEC0534;
pub const mmTPC3_RTR_LBW_RANGE_MASK_10: c_uint = 0xEC0538;
pub const mmTPC3_RTR_LBW_RANGE_MASK_11: c_uint = 0xEC053C;
pub const mmTPC3_RTR_LBW_RANGE_MASK_12: c_uint = 0xEC0540;
pub const mmTPC3_RTR_LBW_RANGE_MASK_13: c_uint = 0xEC0544;
pub const mmTPC3_RTR_LBW_RANGE_MASK_14: c_uint = 0xEC0548;
pub const mmTPC3_RTR_LBW_RANGE_MASK_15: c_uint = 0xEC054C;
pub const mmTPC3_RTR_LBW_RANGE_BASE_0: c_uint = 0xEC0550;
pub const mmTPC3_RTR_LBW_RANGE_BASE_1: c_uint = 0xEC0554;
pub const mmTPC3_RTR_LBW_RANGE_BASE_2: c_uint = 0xEC0558;
pub const mmTPC3_RTR_LBW_RANGE_BASE_3: c_uint = 0xEC055C;
pub const mmTPC3_RTR_LBW_RANGE_BASE_4: c_uint = 0xEC0560;
pub const mmTPC3_RTR_LBW_RANGE_BASE_5: c_uint = 0xEC0564;
pub const mmTPC3_RTR_LBW_RANGE_BASE_6: c_uint = 0xEC0568;
pub const mmTPC3_RTR_LBW_RANGE_BASE_7: c_uint = 0xEC056C;
pub const mmTPC3_RTR_LBW_RANGE_BASE_8: c_uint = 0xEC0570;
pub const mmTPC3_RTR_LBW_RANGE_BASE_9: c_uint = 0xEC0574;
pub const mmTPC3_RTR_LBW_RANGE_BASE_10: c_uint = 0xEC0578;
pub const mmTPC3_RTR_LBW_RANGE_BASE_11: c_uint = 0xEC057C;
pub const mmTPC3_RTR_LBW_RANGE_BASE_12: c_uint = 0xEC0580;
pub const mmTPC3_RTR_LBW_RANGE_BASE_13: c_uint = 0xEC0584;
pub const mmTPC3_RTR_LBW_RANGE_BASE_14: c_uint = 0xEC0588;
pub const mmTPC3_RTR_LBW_RANGE_BASE_15: c_uint = 0xEC058C;
pub const mmTPC3_RTR_RGLTR: c_uint = 0xEC0590;
pub const mmTPC3_RTR_RGLTR_WR_RESULT: c_uint = 0xEC0594;
pub const mmTPC3_RTR_RGLTR_RD_RESULT: c_uint = 0xEC0598;
pub const mmTPC3_RTR_SCRAMB_EN: c_uint = 0xEC0600;
pub const mmTPC3_RTR_NON_LIN_SCRAMB: c_uint = 0xEC0604;
