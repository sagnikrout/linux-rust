//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc2_rtr_regs.h
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
// TPC2_RTR (Prototype: TPC_RTR)
//
pub const mmTPC2_RTR_HBW_RD_RQ_E_ARB: c_uint = 0xE80100;
pub const mmTPC2_RTR_HBW_RD_RQ_W_ARB: c_uint = 0xE80104;
pub const mmTPC2_RTR_HBW_RD_RQ_N_ARB: c_uint = 0xE80108;
pub const mmTPC2_RTR_HBW_RD_RQ_S_ARB: c_uint = 0xE8010C;
pub const mmTPC2_RTR_HBW_RD_RQ_L_ARB: c_uint = 0xE80110;
pub const mmTPC2_RTR_HBW_E_ARB_MAX: c_uint = 0xE80120;
pub const mmTPC2_RTR_HBW_W_ARB_MAX: c_uint = 0xE80124;
pub const mmTPC2_RTR_HBW_N_ARB_MAX: c_uint = 0xE80128;
pub const mmTPC2_RTR_HBW_S_ARB_MAX: c_uint = 0xE8012C;
pub const mmTPC2_RTR_HBW_L_ARB_MAX: c_uint = 0xE80130;
pub const mmTPC2_RTR_HBW_RD_RS_E_ARB: c_uint = 0xE80140;
pub const mmTPC2_RTR_HBW_RD_RS_W_ARB: c_uint = 0xE80144;
pub const mmTPC2_RTR_HBW_RD_RS_N_ARB: c_uint = 0xE80148;
pub const mmTPC2_RTR_HBW_RD_RS_S_ARB: c_uint = 0xE8014C;
pub const mmTPC2_RTR_HBW_RD_RS_L_ARB: c_uint = 0xE80150;
pub const mmTPC2_RTR_HBW_WR_RQ_E_ARB: c_uint = 0xE80170;
pub const mmTPC2_RTR_HBW_WR_RQ_W_ARB: c_uint = 0xE80174;
pub const mmTPC2_RTR_HBW_WR_RQ_N_ARB: c_uint = 0xE80178;
pub const mmTPC2_RTR_HBW_WR_RQ_S_ARB: c_uint = 0xE8017C;
pub const mmTPC2_RTR_HBW_WR_RQ_L_ARB: c_uint = 0xE80180;
pub const mmTPC2_RTR_HBW_WR_RS_E_ARB: c_uint = 0xE80190;
pub const mmTPC2_RTR_HBW_WR_RS_W_ARB: c_uint = 0xE80194;
pub const mmTPC2_RTR_HBW_WR_RS_N_ARB: c_uint = 0xE80198;
pub const mmTPC2_RTR_HBW_WR_RS_S_ARB: c_uint = 0xE8019C;
pub const mmTPC2_RTR_HBW_WR_RS_L_ARB: c_uint = 0xE801A0;
pub const mmTPC2_RTR_LBW_RD_RQ_E_ARB: c_uint = 0xE80200;
pub const mmTPC2_RTR_LBW_RD_RQ_W_ARB: c_uint = 0xE80204;
pub const mmTPC2_RTR_LBW_RD_RQ_N_ARB: c_uint = 0xE80208;
pub const mmTPC2_RTR_LBW_RD_RQ_S_ARB: c_uint = 0xE8020C;
pub const mmTPC2_RTR_LBW_RD_RQ_L_ARB: c_uint = 0xE80210;
pub const mmTPC2_RTR_LBW_E_ARB_MAX: c_uint = 0xE80220;
pub const mmTPC2_RTR_LBW_W_ARB_MAX: c_uint = 0xE80224;
pub const mmTPC2_RTR_LBW_N_ARB_MAX: c_uint = 0xE80228;
pub const mmTPC2_RTR_LBW_S_ARB_MAX: c_uint = 0xE8022C;
pub const mmTPC2_RTR_LBW_L_ARB_MAX: c_uint = 0xE80230;
pub const mmTPC2_RTR_LBW_RD_RS_E_ARB: c_uint = 0xE80250;
pub const mmTPC2_RTR_LBW_RD_RS_W_ARB: c_uint = 0xE80254;
pub const mmTPC2_RTR_LBW_RD_RS_N_ARB: c_uint = 0xE80258;
pub const mmTPC2_RTR_LBW_RD_RS_S_ARB: c_uint = 0xE8025C;
pub const mmTPC2_RTR_LBW_RD_RS_L_ARB: c_uint = 0xE80260;
pub const mmTPC2_RTR_LBW_WR_RQ_E_ARB: c_uint = 0xE80270;
pub const mmTPC2_RTR_LBW_WR_RQ_W_ARB: c_uint = 0xE80274;
pub const mmTPC2_RTR_LBW_WR_RQ_N_ARB: c_uint = 0xE80278;
pub const mmTPC2_RTR_LBW_WR_RQ_S_ARB: c_uint = 0xE8027C;
pub const mmTPC2_RTR_LBW_WR_RQ_L_ARB: c_uint = 0xE80280;
pub const mmTPC2_RTR_LBW_WR_RS_E_ARB: c_uint = 0xE80290;
pub const mmTPC2_RTR_LBW_WR_RS_W_ARB: c_uint = 0xE80294;
pub const mmTPC2_RTR_LBW_WR_RS_N_ARB: c_uint = 0xE80298;
pub const mmTPC2_RTR_LBW_WR_RS_S_ARB: c_uint = 0xE8029C;
pub const mmTPC2_RTR_LBW_WR_RS_L_ARB: c_uint = 0xE802A0;
pub const mmTPC2_RTR_DBG_E_ARB: c_uint = 0xE80300;
pub const mmTPC2_RTR_DBG_W_ARB: c_uint = 0xE80304;
pub const mmTPC2_RTR_DBG_N_ARB: c_uint = 0xE80308;
pub const mmTPC2_RTR_DBG_S_ARB: c_uint = 0xE8030C;
pub const mmTPC2_RTR_DBG_L_ARB: c_uint = 0xE80310;
pub const mmTPC2_RTR_DBG_E_ARB_MAX: c_uint = 0xE80320;
pub const mmTPC2_RTR_DBG_W_ARB_MAX: c_uint = 0xE80324;
pub const mmTPC2_RTR_DBG_N_ARB_MAX: c_uint = 0xE80328;
pub const mmTPC2_RTR_DBG_S_ARB_MAX: c_uint = 0xE8032C;
pub const mmTPC2_RTR_DBG_L_ARB_MAX: c_uint = 0xE80330;
pub const mmTPC2_RTR_SPLIT_COEF_0: c_uint = 0xE80400;
pub const mmTPC2_RTR_SPLIT_COEF_1: c_uint = 0xE80404;
pub const mmTPC2_RTR_SPLIT_COEF_2: c_uint = 0xE80408;
pub const mmTPC2_RTR_SPLIT_COEF_3: c_uint = 0xE8040C;
pub const mmTPC2_RTR_SPLIT_COEF_4: c_uint = 0xE80410;
pub const mmTPC2_RTR_SPLIT_COEF_5: c_uint = 0xE80414;
pub const mmTPC2_RTR_SPLIT_COEF_6: c_uint = 0xE80418;
pub const mmTPC2_RTR_SPLIT_COEF_7: c_uint = 0xE8041C;
pub const mmTPC2_RTR_SPLIT_COEF_8: c_uint = 0xE80420;
pub const mmTPC2_RTR_SPLIT_COEF_9: c_uint = 0xE80424;
pub const mmTPC2_RTR_SPLIT_CFG: c_uint = 0xE80440;
pub const mmTPC2_RTR_SPLIT_RD_SAT: c_uint = 0xE80444;
pub const mmTPC2_RTR_SPLIT_RD_RST_TOKEN: c_uint = 0xE80448;
pub const mmTPC2_RTR_SPLIT_RD_TIMEOUT_0: c_uint = 0xE8044C;
pub const mmTPC2_RTR_SPLIT_RD_TIMEOUT_1: c_uint = 0xE80450;
pub const mmTPC2_RTR_SPLIT_WR_SAT: c_uint = 0xE80454;
pub const mmTPC2_RTR_WPLIT_WR_TST_TOLEN: c_uint = 0xE80458;
pub const mmTPC2_RTR_SPLIT_WR_TIMEOUT_0: c_uint = 0xE8045C;
pub const mmTPC2_RTR_SPLIT_WR_TIMEOUT_1: c_uint = 0xE80460;
pub const mmTPC2_RTR_HBW_RANGE_HIT: c_uint = 0xE80470;
pub const mmTPC2_RTR_HBW_RANGE_MASK_L_0: c_uint = 0xE80480;
pub const mmTPC2_RTR_HBW_RANGE_MASK_L_1: c_uint = 0xE80484;
pub const mmTPC2_RTR_HBW_RANGE_MASK_L_2: c_uint = 0xE80488;
pub const mmTPC2_RTR_HBW_RANGE_MASK_L_3: c_uint = 0xE8048C;
pub const mmTPC2_RTR_HBW_RANGE_MASK_L_4: c_uint = 0xE80490;
pub const mmTPC2_RTR_HBW_RANGE_MASK_L_5: c_uint = 0xE80494;
pub const mmTPC2_RTR_HBW_RANGE_MASK_L_6: c_uint = 0xE80498;
pub const mmTPC2_RTR_HBW_RANGE_MASK_L_7: c_uint = 0xE8049C;
pub const mmTPC2_RTR_HBW_RANGE_MASK_H_0: c_uint = 0xE804A0;
pub const mmTPC2_RTR_HBW_RANGE_MASK_H_1: c_uint = 0xE804A4;
pub const mmTPC2_RTR_HBW_RANGE_MASK_H_2: c_uint = 0xE804A8;
pub const mmTPC2_RTR_HBW_RANGE_MASK_H_3: c_uint = 0xE804AC;
pub const mmTPC2_RTR_HBW_RANGE_MASK_H_4: c_uint = 0xE804B0;
pub const mmTPC2_RTR_HBW_RANGE_MASK_H_5: c_uint = 0xE804B4;
pub const mmTPC2_RTR_HBW_RANGE_MASK_H_6: c_uint = 0xE804B8;
pub const mmTPC2_RTR_HBW_RANGE_MASK_H_7: c_uint = 0xE804BC;
pub const mmTPC2_RTR_HBW_RANGE_BASE_L_0: c_uint = 0xE804C0;
pub const mmTPC2_RTR_HBW_RANGE_BASE_L_1: c_uint = 0xE804C4;
pub const mmTPC2_RTR_HBW_RANGE_BASE_L_2: c_uint = 0xE804C8;
pub const mmTPC2_RTR_HBW_RANGE_BASE_L_3: c_uint = 0xE804CC;
pub const mmTPC2_RTR_HBW_RANGE_BASE_L_4: c_uint = 0xE804D0;
pub const mmTPC2_RTR_HBW_RANGE_BASE_L_5: c_uint = 0xE804D4;
pub const mmTPC2_RTR_HBW_RANGE_BASE_L_6: c_uint = 0xE804D8;
pub const mmTPC2_RTR_HBW_RANGE_BASE_L_7: c_uint = 0xE804DC;
pub const mmTPC2_RTR_HBW_RANGE_BASE_H_0: c_uint = 0xE804E0;
pub const mmTPC2_RTR_HBW_RANGE_BASE_H_1: c_uint = 0xE804E4;
pub const mmTPC2_RTR_HBW_RANGE_BASE_H_2: c_uint = 0xE804E8;
pub const mmTPC2_RTR_HBW_RANGE_BASE_H_3: c_uint = 0xE804EC;
pub const mmTPC2_RTR_HBW_RANGE_BASE_H_4: c_uint = 0xE804F0;
pub const mmTPC2_RTR_HBW_RANGE_BASE_H_5: c_uint = 0xE804F4;
pub const mmTPC2_RTR_HBW_RANGE_BASE_H_6: c_uint = 0xE804F8;
pub const mmTPC2_RTR_HBW_RANGE_BASE_H_7: c_uint = 0xE804FC;
pub const mmTPC2_RTR_LBW_RANGE_HIT: c_uint = 0xE80500;
pub const mmTPC2_RTR_LBW_RANGE_MASK_0: c_uint = 0xE80510;
pub const mmTPC2_RTR_LBW_RANGE_MASK_1: c_uint = 0xE80514;
pub const mmTPC2_RTR_LBW_RANGE_MASK_2: c_uint = 0xE80518;
pub const mmTPC2_RTR_LBW_RANGE_MASK_3: c_uint = 0xE8051C;
pub const mmTPC2_RTR_LBW_RANGE_MASK_4: c_uint = 0xE80520;
pub const mmTPC2_RTR_LBW_RANGE_MASK_5: c_uint = 0xE80524;
pub const mmTPC2_RTR_LBW_RANGE_MASK_6: c_uint = 0xE80528;
pub const mmTPC2_RTR_LBW_RANGE_MASK_7: c_uint = 0xE8052C;
pub const mmTPC2_RTR_LBW_RANGE_MASK_8: c_uint = 0xE80530;
pub const mmTPC2_RTR_LBW_RANGE_MASK_9: c_uint = 0xE80534;
pub const mmTPC2_RTR_LBW_RANGE_MASK_10: c_uint = 0xE80538;
pub const mmTPC2_RTR_LBW_RANGE_MASK_11: c_uint = 0xE8053C;
pub const mmTPC2_RTR_LBW_RANGE_MASK_12: c_uint = 0xE80540;
pub const mmTPC2_RTR_LBW_RANGE_MASK_13: c_uint = 0xE80544;
pub const mmTPC2_RTR_LBW_RANGE_MASK_14: c_uint = 0xE80548;
pub const mmTPC2_RTR_LBW_RANGE_MASK_15: c_uint = 0xE8054C;
pub const mmTPC2_RTR_LBW_RANGE_BASE_0: c_uint = 0xE80550;
pub const mmTPC2_RTR_LBW_RANGE_BASE_1: c_uint = 0xE80554;
pub const mmTPC2_RTR_LBW_RANGE_BASE_2: c_uint = 0xE80558;
pub const mmTPC2_RTR_LBW_RANGE_BASE_3: c_uint = 0xE8055C;
pub const mmTPC2_RTR_LBW_RANGE_BASE_4: c_uint = 0xE80560;
pub const mmTPC2_RTR_LBW_RANGE_BASE_5: c_uint = 0xE80564;
pub const mmTPC2_RTR_LBW_RANGE_BASE_6: c_uint = 0xE80568;
pub const mmTPC2_RTR_LBW_RANGE_BASE_7: c_uint = 0xE8056C;
pub const mmTPC2_RTR_LBW_RANGE_BASE_8: c_uint = 0xE80570;
pub const mmTPC2_RTR_LBW_RANGE_BASE_9: c_uint = 0xE80574;
pub const mmTPC2_RTR_LBW_RANGE_BASE_10: c_uint = 0xE80578;
pub const mmTPC2_RTR_LBW_RANGE_BASE_11: c_uint = 0xE8057C;
pub const mmTPC2_RTR_LBW_RANGE_BASE_12: c_uint = 0xE80580;
pub const mmTPC2_RTR_LBW_RANGE_BASE_13: c_uint = 0xE80584;
pub const mmTPC2_RTR_LBW_RANGE_BASE_14: c_uint = 0xE80588;
pub const mmTPC2_RTR_LBW_RANGE_BASE_15: c_uint = 0xE8058C;
pub const mmTPC2_RTR_RGLTR: c_uint = 0xE80590;
pub const mmTPC2_RTR_RGLTR_WR_RESULT: c_uint = 0xE80594;
pub const mmTPC2_RTR_RGLTR_RD_RESULT: c_uint = 0xE80598;
pub const mmTPC2_RTR_SCRAMB_EN: c_uint = 0xE80600;
pub const mmTPC2_RTR_NON_LIN_SCRAMB: c_uint = 0xE80604;
