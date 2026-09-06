//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/mme2_rtr_regs.h
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
// MME2_RTR (Prototype: MME_RTR)
//
pub const mmMME2_RTR_HBW_RD_RQ_E_ARB: c_uint = 0x80100;
pub const mmMME2_RTR_HBW_RD_RQ_W_ARB: c_uint = 0x80104;
pub const mmMME2_RTR_HBW_RD_RQ_N_ARB: c_uint = 0x80108;
pub const mmMME2_RTR_HBW_RD_RQ_S_ARB: c_uint = 0x8010C;
pub const mmMME2_RTR_HBW_RD_RQ_L_ARB: c_uint = 0x80110;
pub const mmMME2_RTR_HBW_E_ARB_MAX: c_uint = 0x80120;
pub const mmMME2_RTR_HBW_W_ARB_MAX: c_uint = 0x80124;
pub const mmMME2_RTR_HBW_N_ARB_MAX: c_uint = 0x80128;
pub const mmMME2_RTR_HBW_S_ARB_MAX: c_uint = 0x8012C;
pub const mmMME2_RTR_HBW_L_ARB_MAX: c_uint = 0x80130;
pub const mmMME2_RTR_HBW_RD_RS_MAX_CREDIT: c_uint = 0x80140;
pub const mmMME2_RTR_HBW_WR_RQ_MAX_CREDIT: c_uint = 0x80144;
pub const mmMME2_RTR_HBW_RD_RQ_MAX_CREDIT: c_uint = 0x80148;
pub const mmMME2_RTR_HBW_RD_RS_E_ARB: c_uint = 0x80150;
pub const mmMME2_RTR_HBW_RD_RS_W_ARB: c_uint = 0x80154;
pub const mmMME2_RTR_HBW_RD_RS_N_ARB: c_uint = 0x80158;
pub const mmMME2_RTR_HBW_RD_RS_S_ARB: c_uint = 0x8015C;
pub const mmMME2_RTR_HBW_RD_RS_L_ARB: c_uint = 0x80160;
pub const mmMME2_RTR_HBW_WR_RQ_E_ARB: c_uint = 0x80170;
pub const mmMME2_RTR_HBW_WR_RQ_W_ARB: c_uint = 0x80174;
pub const mmMME2_RTR_HBW_WR_RQ_N_ARB: c_uint = 0x80178;
pub const mmMME2_RTR_HBW_WR_RQ_S_ARB: c_uint = 0x8017C;
pub const mmMME2_RTR_HBW_WR_RQ_L_ARB: c_uint = 0x80180;
pub const mmMME2_RTR_HBW_WR_RS_E_ARB: c_uint = 0x80190;
pub const mmMME2_RTR_HBW_WR_RS_W_ARB: c_uint = 0x80194;
pub const mmMME2_RTR_HBW_WR_RS_N_ARB: c_uint = 0x80198;
pub const mmMME2_RTR_HBW_WR_RS_S_ARB: c_uint = 0x8019C;
pub const mmMME2_RTR_HBW_WR_RS_L_ARB: c_uint = 0x801A0;
pub const mmMME2_RTR_LBW_RD_RQ_E_ARB: c_uint = 0x80200;
pub const mmMME2_RTR_LBW_RD_RQ_W_ARB: c_uint = 0x80204;
pub const mmMME2_RTR_LBW_RD_RQ_N_ARB: c_uint = 0x80208;
pub const mmMME2_RTR_LBW_RD_RQ_S_ARB: c_uint = 0x8020C;
pub const mmMME2_RTR_LBW_RD_RQ_L_ARB: c_uint = 0x80210;
pub const mmMME2_RTR_LBW_E_ARB_MAX: c_uint = 0x80220;
pub const mmMME2_RTR_LBW_W_ARB_MAX: c_uint = 0x80224;
pub const mmMME2_RTR_LBW_N_ARB_MAX: c_uint = 0x80228;
pub const mmMME2_RTR_LBW_S_ARB_MAX: c_uint = 0x8022C;
pub const mmMME2_RTR_LBW_L_ARB_MAX: c_uint = 0x80230;
pub const mmMME2_RTR_LBW_SRAM_MAX_CREDIT: c_uint = 0x80240;
pub const mmMME2_RTR_LBW_RD_RS_E_ARB: c_uint = 0x80250;
pub const mmMME2_RTR_LBW_RD_RS_W_ARB: c_uint = 0x80254;
pub const mmMME2_RTR_LBW_RD_RS_N_ARB: c_uint = 0x80258;
pub const mmMME2_RTR_LBW_RD_RS_S_ARB: c_uint = 0x8025C;
pub const mmMME2_RTR_LBW_RD_RS_L_ARB: c_uint = 0x80260;
pub const mmMME2_RTR_LBW_WR_RQ_E_ARB: c_uint = 0x80270;
pub const mmMME2_RTR_LBW_WR_RQ_W_ARB: c_uint = 0x80274;
pub const mmMME2_RTR_LBW_WR_RQ_N_ARB: c_uint = 0x80278;
pub const mmMME2_RTR_LBW_WR_RQ_S_ARB: c_uint = 0x8027C;
pub const mmMME2_RTR_LBW_WR_RQ_L_ARB: c_uint = 0x80280;
pub const mmMME2_RTR_LBW_WR_RS_E_ARB: c_uint = 0x80290;
pub const mmMME2_RTR_LBW_WR_RS_W_ARB: c_uint = 0x80294;
pub const mmMME2_RTR_LBW_WR_RS_N_ARB: c_uint = 0x80298;
pub const mmMME2_RTR_LBW_WR_RS_S_ARB: c_uint = 0x8029C;
pub const mmMME2_RTR_LBW_WR_RS_L_ARB: c_uint = 0x802A0;
pub const mmMME2_RTR_DBG_E_ARB: c_uint = 0x80300;
pub const mmMME2_RTR_DBG_W_ARB: c_uint = 0x80304;
pub const mmMME2_RTR_DBG_N_ARB: c_uint = 0x80308;
pub const mmMME2_RTR_DBG_S_ARB: c_uint = 0x8030C;
pub const mmMME2_RTR_DBG_L_ARB: c_uint = 0x80310;
pub const mmMME2_RTR_DBG_E_ARB_MAX: c_uint = 0x80320;
pub const mmMME2_RTR_DBG_W_ARB_MAX: c_uint = 0x80324;
pub const mmMME2_RTR_DBG_N_ARB_MAX: c_uint = 0x80328;
pub const mmMME2_RTR_DBG_S_ARB_MAX: c_uint = 0x8032C;
pub const mmMME2_RTR_DBG_L_ARB_MAX: c_uint = 0x80330;
pub const mmMME2_RTR_SPLIT_COEF_0: c_uint = 0x80400;
pub const mmMME2_RTR_SPLIT_COEF_1: c_uint = 0x80404;
pub const mmMME2_RTR_SPLIT_COEF_2: c_uint = 0x80408;
pub const mmMME2_RTR_SPLIT_COEF_3: c_uint = 0x8040C;
pub const mmMME2_RTR_SPLIT_COEF_4: c_uint = 0x80410;
pub const mmMME2_RTR_SPLIT_COEF_5: c_uint = 0x80414;
pub const mmMME2_RTR_SPLIT_COEF_6: c_uint = 0x80418;
pub const mmMME2_RTR_SPLIT_COEF_7: c_uint = 0x8041C;
pub const mmMME2_RTR_SPLIT_COEF_8: c_uint = 0x80420;
pub const mmMME2_RTR_SPLIT_COEF_9: c_uint = 0x80424;
pub const mmMME2_RTR_SPLIT_CFG: c_uint = 0x80440;
pub const mmMME2_RTR_SPLIT_RD_SAT: c_uint = 0x80444;
pub const mmMME2_RTR_SPLIT_RD_RST_TOKEN: c_uint = 0x80448;
pub const mmMME2_RTR_SPLIT_RD_TIMEOUT_0: c_uint = 0x8044C;
pub const mmMME2_RTR_SPLIT_RD_TIMEOUT_1: c_uint = 0x80450;
pub const mmMME2_RTR_SPLIT_WR_SAT: c_uint = 0x80454;
pub const mmMME2_RTR_WPLIT_WR_TST_TOLEN: c_uint = 0x80458;
pub const mmMME2_RTR_SPLIT_WR_TIMEOUT_0: c_uint = 0x8045C;
pub const mmMME2_RTR_SPLIT_WR_TIMEOUT_1: c_uint = 0x80460;
pub const mmMME2_RTR_HBW_RANGE_HIT: c_uint = 0x80470;
pub const mmMME2_RTR_HBW_RANGE_MASK_L_0: c_uint = 0x80480;
pub const mmMME2_RTR_HBW_RANGE_MASK_L_1: c_uint = 0x80484;
pub const mmMME2_RTR_HBW_RANGE_MASK_L_2: c_uint = 0x80488;
pub const mmMME2_RTR_HBW_RANGE_MASK_L_3: c_uint = 0x8048C;
pub const mmMME2_RTR_HBW_RANGE_MASK_L_4: c_uint = 0x80490;
pub const mmMME2_RTR_HBW_RANGE_MASK_L_5: c_uint = 0x80494;
pub const mmMME2_RTR_HBW_RANGE_MASK_L_6: c_uint = 0x80498;
pub const mmMME2_RTR_HBW_RANGE_MASK_L_7: c_uint = 0x8049C;
pub const mmMME2_RTR_HBW_RANGE_MASK_H_0: c_uint = 0x804A0;
pub const mmMME2_RTR_HBW_RANGE_MASK_H_1: c_uint = 0x804A4;
pub const mmMME2_RTR_HBW_RANGE_MASK_H_2: c_uint = 0x804A8;
pub const mmMME2_RTR_HBW_RANGE_MASK_H_3: c_uint = 0x804AC;
pub const mmMME2_RTR_HBW_RANGE_MASK_H_4: c_uint = 0x804B0;
pub const mmMME2_RTR_HBW_RANGE_MASK_H_5: c_uint = 0x804B4;
pub const mmMME2_RTR_HBW_RANGE_MASK_H_6: c_uint = 0x804B8;
pub const mmMME2_RTR_HBW_RANGE_MASK_H_7: c_uint = 0x804BC;
pub const mmMME2_RTR_HBW_RANGE_BASE_L_0: c_uint = 0x804C0;
pub const mmMME2_RTR_HBW_RANGE_BASE_L_1: c_uint = 0x804C4;
pub const mmMME2_RTR_HBW_RANGE_BASE_L_2: c_uint = 0x804C8;
pub const mmMME2_RTR_HBW_RANGE_BASE_L_3: c_uint = 0x804CC;
pub const mmMME2_RTR_HBW_RANGE_BASE_L_4: c_uint = 0x804D0;
pub const mmMME2_RTR_HBW_RANGE_BASE_L_5: c_uint = 0x804D4;
pub const mmMME2_RTR_HBW_RANGE_BASE_L_6: c_uint = 0x804D8;
pub const mmMME2_RTR_HBW_RANGE_BASE_L_7: c_uint = 0x804DC;
pub const mmMME2_RTR_HBW_RANGE_BASE_H_0: c_uint = 0x804E0;
pub const mmMME2_RTR_HBW_RANGE_BASE_H_1: c_uint = 0x804E4;
pub const mmMME2_RTR_HBW_RANGE_BASE_H_2: c_uint = 0x804E8;
pub const mmMME2_RTR_HBW_RANGE_BASE_H_3: c_uint = 0x804EC;
pub const mmMME2_RTR_HBW_RANGE_BASE_H_4: c_uint = 0x804F0;
pub const mmMME2_RTR_HBW_RANGE_BASE_H_5: c_uint = 0x804F4;
pub const mmMME2_RTR_HBW_RANGE_BASE_H_6: c_uint = 0x804F8;
pub const mmMME2_RTR_HBW_RANGE_BASE_H_7: c_uint = 0x804FC;
pub const mmMME2_RTR_LBW_RANGE_HIT: c_uint = 0x80500;
pub const mmMME2_RTR_LBW_RANGE_MASK_0: c_uint = 0x80510;
pub const mmMME2_RTR_LBW_RANGE_MASK_1: c_uint = 0x80514;
pub const mmMME2_RTR_LBW_RANGE_MASK_2: c_uint = 0x80518;
pub const mmMME2_RTR_LBW_RANGE_MASK_3: c_uint = 0x8051C;
pub const mmMME2_RTR_LBW_RANGE_MASK_4: c_uint = 0x80520;
pub const mmMME2_RTR_LBW_RANGE_MASK_5: c_uint = 0x80524;
pub const mmMME2_RTR_LBW_RANGE_MASK_6: c_uint = 0x80528;
pub const mmMME2_RTR_LBW_RANGE_MASK_7: c_uint = 0x8052C;
pub const mmMME2_RTR_LBW_RANGE_MASK_8: c_uint = 0x80530;
pub const mmMME2_RTR_LBW_RANGE_MASK_9: c_uint = 0x80534;
pub const mmMME2_RTR_LBW_RANGE_MASK_10: c_uint = 0x80538;
pub const mmMME2_RTR_LBW_RANGE_MASK_11: c_uint = 0x8053C;
pub const mmMME2_RTR_LBW_RANGE_MASK_12: c_uint = 0x80540;
pub const mmMME2_RTR_LBW_RANGE_MASK_13: c_uint = 0x80544;
pub const mmMME2_RTR_LBW_RANGE_MASK_14: c_uint = 0x80548;
pub const mmMME2_RTR_LBW_RANGE_MASK_15: c_uint = 0x8054C;
pub const mmMME2_RTR_LBW_RANGE_BASE_0: c_uint = 0x80550;
pub const mmMME2_RTR_LBW_RANGE_BASE_1: c_uint = 0x80554;
pub const mmMME2_RTR_LBW_RANGE_BASE_2: c_uint = 0x80558;
pub const mmMME2_RTR_LBW_RANGE_BASE_3: c_uint = 0x8055C;
pub const mmMME2_RTR_LBW_RANGE_BASE_4: c_uint = 0x80560;
pub const mmMME2_RTR_LBW_RANGE_BASE_5: c_uint = 0x80564;
pub const mmMME2_RTR_LBW_RANGE_BASE_6: c_uint = 0x80568;
pub const mmMME2_RTR_LBW_RANGE_BASE_7: c_uint = 0x8056C;
pub const mmMME2_RTR_LBW_RANGE_BASE_8: c_uint = 0x80570;
pub const mmMME2_RTR_LBW_RANGE_BASE_9: c_uint = 0x80574;
pub const mmMME2_RTR_LBW_RANGE_BASE_10: c_uint = 0x80578;
pub const mmMME2_RTR_LBW_RANGE_BASE_11: c_uint = 0x8057C;
pub const mmMME2_RTR_LBW_RANGE_BASE_12: c_uint = 0x80580;
pub const mmMME2_RTR_LBW_RANGE_BASE_13: c_uint = 0x80584;
pub const mmMME2_RTR_LBW_RANGE_BASE_14: c_uint = 0x80588;
pub const mmMME2_RTR_LBW_RANGE_BASE_15: c_uint = 0x8058C;
pub const mmMME2_RTR_RGLTR: c_uint = 0x80590;
pub const mmMME2_RTR_RGLTR_WR_RESULT: c_uint = 0x80594;
pub const mmMME2_RTR_RGLTR_RD_RESULT: c_uint = 0x80598;
pub const mmMME2_RTR_SCRAMB_EN: c_uint = 0x80600;
pub const mmMME2_RTR_NON_LIN_SCRAMB: c_uint = 0x80604;
