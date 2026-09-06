//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/xbar_mid_0_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// XBAR_MID_0
// (Prototype: XBAR)
//
pub const mmXBAR_MID_0_LBW_HIF0_BASE_ADDR: c_uint = 0x4D40000;
pub const mmXBAR_MID_0_LBW_HIF0_ADDR_MASK: c_uint = 0x4D40004;
pub const mmXBAR_MID_0_LBW_HIF1_BASE_ADDR: c_uint = 0x4D40008;
pub const mmXBAR_MID_0_LBW_HIF1_ADDR_MASK: c_uint = 0x4D4000C;
pub const mmXBAR_MID_0_LBW_HMMU0_BASE_ADDR: c_uint = 0x4D40010;
pub const mmXBAR_MID_0_LBW_HMMU0_ADDR_MASK: c_uint = 0x4D40014;
pub const mmXBAR_MID_0_LBW_HMMU1_BASE_ADDR: c_uint = 0x4D40018;
pub const mmXBAR_MID_0_LBW_HMMU1_ADDR_MASK: c_uint = 0x4D4001C;
pub const mmXBAR_MID_0_LBW_EDMA_BASE_ADDR0: c_uint = 0x4D40020;
pub const mmXBAR_MID_0_LBW_EDMA_ADDR_MASK0: c_uint = 0x4D40024;
pub const mmXBAR_MID_0_LBW_EDMA_BASE_ADDR1: c_uint = 0x4D40028;
pub const mmXBAR_MID_0_LBW_EDMA_ADDR_MASK1: c_uint = 0x4D4002C;
pub const mmXBAR_MID_0_LBW_HBM_BASE_ADDR0: c_uint = 0x4D40030;
pub const mmXBAR_MID_0_LBW_HBM_ADDR_MASK0: c_uint = 0x4D40034;
pub const mmXBAR_MID_0_LBW_HBM_BASE_ADDR1: c_uint = 0x4D40038;
pub const mmXBAR_MID_0_LBW_HBM_ADDR_MASK1: c_uint = 0x4D4003C;
pub const mmXBAR_MID_0_LBW_XBAR_BASE_ADDR0: c_uint = 0x4D40040;
pub const mmXBAR_MID_0_LBW_XBAR_ADDR_MASK0: c_uint = 0x4D40044;
pub const mmXBAR_MID_0_LBW_XBAR_BASE_ADDR1: c_uint = 0x4D40048;
pub const mmXBAR_MID_0_LBW_XBAR_ADDR_MASK1: c_uint = 0x4D4004C;
pub const mmXBAR_MID_0_DBG_HIF0_BASE_ADDR: c_uint = 0x4D40080;
pub const mmXBAR_MID_0_DBG_HIF0_ADDR_MASK: c_uint = 0x4D40084;
pub const mmXBAR_MID_0_DBG_HIF1_BASE_ADDR: c_uint = 0x4D40088;
pub const mmXBAR_MID_0_DBG_HIF1_ADDR_MASK: c_uint = 0x4D4008C;
pub const mmXBAR_MID_0_DBG_HMMU0_BASE_ADDR: c_uint = 0x4D40090;
pub const mmXBAR_MID_0_DBG_HMMU0_ADDR_MASK: c_uint = 0x4D40094;
pub const mmXBAR_MID_0_DBG_HMMU1_BASE_ADDR: c_uint = 0x4D40098;
pub const mmXBAR_MID_0_DBG_HMMU1_ADDR_MASK: c_uint = 0x4D4009C;
pub const mmXBAR_MID_0_DBG_EDMA_BASE_ADDR0: c_uint = 0x4D400A0;
pub const mmXBAR_MID_0_DBG_EDMA_ADDR_MASK0: c_uint = 0x4D400A4;
pub const mmXBAR_MID_0_DBG_EDMA_BASE_ADDR1: c_uint = 0x4D400A8;
pub const mmXBAR_MID_0_DBG_EDMA_ADDR_MASK1: c_uint = 0x4D400AC;
pub const mmXBAR_MID_0_DBG_HBM_BASE_ADDR0: c_uint = 0x4D400B0;
pub const mmXBAR_MID_0_DBG_HBM_ADDR_MASK0: c_uint = 0x4D400B4;
pub const mmXBAR_MID_0_DBG_HBM_BASE_ADDR1: c_uint = 0x4D400B8;
pub const mmXBAR_MID_0_DBG_HBM_ADDR_MASK1: c_uint = 0x4D400BC;
pub const mmXBAR_MID_0_DBG_XBAR_BASE_ADDR0: c_uint = 0x4D400C0;
pub const mmXBAR_MID_0_DBG_XBAR_ADDR_MASK0: c_uint = 0x4D400C4;
pub const mmXBAR_MID_0_DBG_XBAR_BASE_ADDR1: c_uint = 0x4D400C8;
pub const mmXBAR_MID_0_DBG_XBAR_ADDR_MASK1: c_uint = 0x4D400CC;
pub const mmXBAR_MID_0_LBW_INTERNAL_ADDR_RGF: c_uint = 0x4D400D0;
pub const mmXBAR_MID_0_DBG_INTERNAL_ADDR_FUN: c_uint = 0x4D400D4;
pub const mmXBAR_MID_0_EMEM_HBM_BIT_LOCATION: c_uint = 0x4D40100;
pub const mmXBAR_MID_0_EMEM_PC_BIT_LOCATION: c_uint = 0x4D40104;
pub const mmXBAR_MID_0_HIF_WR_RS_CH_LOCATION: c_uint = 0x4D40108;
pub const mmXBAR_MID_0_HBW_MST_ARB_WEIGHT: c_uint = 0x4D4010C;
pub const mmXBAR_MID_0_MMU_PC_IDX_MAP_0: c_uint = 0x4D40110;
pub const mmXBAR_MID_0_MMU_PC_IDX_MAP_1: c_uint = 0x4D40114;
pub const mmXBAR_MID_0_MMU_RD_LL_ARB_0: c_uint = 0x4D40120;
pub const mmXBAR_MID_0_MMU_RD_LL_ARB_1: c_uint = 0x4D40124;
pub const mmXBAR_MID_0_MMU_WR_LL_ARB_0: c_uint = 0x4D40128;
pub const mmXBAR_MID_0_MMU_WR_LL_ARB_1: c_uint = 0x4D4012C;
pub const mmXBAR_MID_0_HBM_USER_RESP_OVR_0: c_uint = 0x4D40130;
pub const mmXBAR_MID_0_HBM_USER_RESP_OVR_1: c_uint = 0x4D40134;
pub const mmXBAR_MID_0_RL_RD_0: c_uint = 0x4D40140;
pub const mmXBAR_MID_0_RL_RD_1: c_uint = 0x4D40144;
pub const mmXBAR_MID_0_RL_RD_2: c_uint = 0x4D40148;
pub const mmXBAR_MID_0_RL_RD_3: c_uint = 0x4D4014C;
pub const mmXBAR_MID_0_RL_RD_4: c_uint = 0x4D40150;
pub const mmXBAR_MID_0_RL_RD_5: c_uint = 0x4D40154;
pub const mmXBAR_MID_0_RL_RD_6: c_uint = 0x4D40158;
pub const mmXBAR_MID_0_RL_RD_7: c_uint = 0x4D4015C;
pub const mmXBAR_MID_0_RL_RD_8: c_uint = 0x4D40160;
pub const mmXBAR_MID_0_RL_RD_9: c_uint = 0x4D40164;
pub const mmXBAR_MID_0_RL_RD_10: c_uint = 0x4D40168;
pub const mmXBAR_MID_0_RL_RD_11: c_uint = 0x4D4016C;
pub const mmXBAR_MID_0_RL_WR_0: c_uint = 0x4D40180;
pub const mmXBAR_MID_0_RL_WR_1: c_uint = 0x4D40184;
pub const mmXBAR_MID_0_RL_WR_2: c_uint = 0x4D40188;
pub const mmXBAR_MID_0_RL_WR_3: c_uint = 0x4D4018C;
pub const mmXBAR_MID_0_RL_WR_4: c_uint = 0x4D40190;
pub const mmXBAR_MID_0_RL_WR_5: c_uint = 0x4D40194;
pub const mmXBAR_MID_0_RL_WR_6: c_uint = 0x4D40198;
pub const mmXBAR_MID_0_RL_WR_7: c_uint = 0x4D4019C;
pub const mmXBAR_MID_0_RL_WR_8: c_uint = 0x4D401A0;
pub const mmXBAR_MID_0_RL_WR_9: c_uint = 0x4D401A4;
pub const mmXBAR_MID_0_RL_WR_10: c_uint = 0x4D401A8;
pub const mmXBAR_MID_0_RL_WR_11: c_uint = 0x4D401AC;
pub const mmXBAR_MID_0_E2E_CRDT_SLV_0: c_uint = 0x4D401B0;
pub const mmXBAR_MID_0_E2E_CRDT_SLV_1: c_uint = 0x4D401B4;
pub const mmXBAR_MID_0_E2E_CRDT_SLV_2: c_uint = 0x4D401B8;
pub const mmXBAR_MID_0_E2E_CRDT_DEBUG: c_uint = 0x4D401BC;
pub const mmXBAR_MID_0_UPSCALE: c_uint = 0x4D401C0;
pub const mmXBAR_MID_0_DOWN_CONV: c_uint = 0x4D401C4;
pub const mmXBAR_MID_0_DOWN_CONV_LFSR_EN: c_uint = 0x4D401D0;
pub const mmXBAR_MID_0_DOWN_CONV_LFSR_SET_VLD: c_uint = 0x4D401D4;
pub const mmXBAR_MID_0_DOWN_CONV_LFSR_SET_VALUE: c_uint = 0x4D401D8;
pub const mmXBAR_MID_0_DOWN_CONV_LFSR_CFG_POLY: c_uint = 0x4D401DC;
