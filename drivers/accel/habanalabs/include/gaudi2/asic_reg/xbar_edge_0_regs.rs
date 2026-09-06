//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/xbar_edge_0_regs.h
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
// XBAR_EDGE_0
// (Prototype: XBAR)
//
pub const mmXBAR_EDGE_0_LBW_HIF0_BASE_ADDR: c_uint = 0x4D48000;
pub const mmXBAR_EDGE_0_LBW_HIF0_ADDR_MASK: c_uint = 0x4D48004;
pub const mmXBAR_EDGE_0_LBW_HIF1_BASE_ADDR: c_uint = 0x4D48008;
pub const mmXBAR_EDGE_0_LBW_HIF1_ADDR_MASK: c_uint = 0x4D4800C;
pub const mmXBAR_EDGE_0_LBW_HMMU0_BASE_ADDR: c_uint = 0x4D48010;
pub const mmXBAR_EDGE_0_LBW_HMMU0_ADDR_MASK: c_uint = 0x4D48014;
pub const mmXBAR_EDGE_0_LBW_HMMU1_BASE_ADDR: c_uint = 0x4D48018;
pub const mmXBAR_EDGE_0_LBW_HMMU1_ADDR_MASK: c_uint = 0x4D4801C;
pub const mmXBAR_EDGE_0_LBW_EDMA_BASE_ADDR0: c_uint = 0x4D48020;
pub const mmXBAR_EDGE_0_LBW_EDMA_ADDR_MASK0: c_uint = 0x4D48024;
pub const mmXBAR_EDGE_0_LBW_EDMA_BASE_ADDR1: c_uint = 0x4D48028;
pub const mmXBAR_EDGE_0_LBW_EDMA_ADDR_MASK1: c_uint = 0x4D4802C;
pub const mmXBAR_EDGE_0_LBW_HBM_BASE_ADDR0: c_uint = 0x4D48030;
pub const mmXBAR_EDGE_0_LBW_HBM_ADDR_MASK0: c_uint = 0x4D48034;
pub const mmXBAR_EDGE_0_LBW_HBM_BASE_ADDR1: c_uint = 0x4D48038;
pub const mmXBAR_EDGE_0_LBW_HBM_ADDR_MASK1: c_uint = 0x4D4803C;
pub const mmXBAR_EDGE_0_LBW_XBAR_BASE_ADDR0: c_uint = 0x4D48040;
pub const mmXBAR_EDGE_0_LBW_XBAR_ADDR_MASK0: c_uint = 0x4D48044;
pub const mmXBAR_EDGE_0_LBW_XBAR_BASE_ADDR1: c_uint = 0x4D48048;
pub const mmXBAR_EDGE_0_LBW_XBAR_ADDR_MASK1: c_uint = 0x4D4804C;
pub const mmXBAR_EDGE_0_DBG_HIF0_BASE_ADDR: c_uint = 0x4D48080;
pub const mmXBAR_EDGE_0_DBG_HIF0_ADDR_MASK: c_uint = 0x4D48084;
pub const mmXBAR_EDGE_0_DBG_HIF1_BASE_ADDR: c_uint = 0x4D48088;
pub const mmXBAR_EDGE_0_DBG_HIF1_ADDR_MASK: c_uint = 0x4D4808C;
pub const mmXBAR_EDGE_0_DBG_HMMU0_BASE_ADDR: c_uint = 0x4D48090;
pub const mmXBAR_EDGE_0_DBG_HMMU0_ADDR_MASK: c_uint = 0x4D48094;
pub const mmXBAR_EDGE_0_DBG_HMMU1_BASE_ADDR: c_uint = 0x4D48098;
pub const mmXBAR_EDGE_0_DBG_HMMU1_ADDR_MASK: c_uint = 0x4D4809C;
pub const mmXBAR_EDGE_0_DBG_EDMA_BASE_ADDR0: c_uint = 0x4D480A0;
pub const mmXBAR_EDGE_0_DBG_EDMA_ADDR_MASK0: c_uint = 0x4D480A4;
pub const mmXBAR_EDGE_0_DBG_EDMA_BASE_ADDR1: c_uint = 0x4D480A8;
pub const mmXBAR_EDGE_0_DBG_EDMA_ADDR_MASK1: c_uint = 0x4D480AC;
pub const mmXBAR_EDGE_0_DBG_HBM_BASE_ADDR0: c_uint = 0x4D480B0;
pub const mmXBAR_EDGE_0_DBG_HBM_ADDR_MASK0: c_uint = 0x4D480B4;
pub const mmXBAR_EDGE_0_DBG_HBM_BASE_ADDR1: c_uint = 0x4D480B8;
pub const mmXBAR_EDGE_0_DBG_HBM_ADDR_MASK1: c_uint = 0x4D480BC;
pub const mmXBAR_EDGE_0_DBG_XBAR_BASE_ADDR0: c_uint = 0x4D480C0;
pub const mmXBAR_EDGE_0_DBG_XBAR_ADDR_MASK0: c_uint = 0x4D480C4;
pub const mmXBAR_EDGE_0_DBG_XBAR_BASE_ADDR1: c_uint = 0x4D480C8;
pub const mmXBAR_EDGE_0_DBG_XBAR_ADDR_MASK1: c_uint = 0x4D480CC;
pub const mmXBAR_EDGE_0_LBW_INTERNAL_ADDR_RGF: c_uint = 0x4D480D0;
pub const mmXBAR_EDGE_0_DBG_INTERNAL_ADDR_FUN: c_uint = 0x4D480D4;
pub const mmXBAR_EDGE_0_EMEM_HBM_BIT_LOCATION: c_uint = 0x4D48100;
pub const mmXBAR_EDGE_0_EMEM_PC_BIT_LOCATION: c_uint = 0x4D48104;
pub const mmXBAR_EDGE_0_HIF_WR_RS_CH_LOCATION: c_uint = 0x4D48108;
pub const mmXBAR_EDGE_0_HBW_MST_ARB_WEIGHT: c_uint = 0x4D4810C;
pub const mmXBAR_EDGE_0_MMU_PC_IDX_MAP_0: c_uint = 0x4D48110;
pub const mmXBAR_EDGE_0_MMU_PC_IDX_MAP_1: c_uint = 0x4D48114;
pub const mmXBAR_EDGE_0_MMU_RD_LL_ARB_0: c_uint = 0x4D48120;
pub const mmXBAR_EDGE_0_MMU_RD_LL_ARB_1: c_uint = 0x4D48124;
pub const mmXBAR_EDGE_0_MMU_WR_LL_ARB_0: c_uint = 0x4D48128;
pub const mmXBAR_EDGE_0_MMU_WR_LL_ARB_1: c_uint = 0x4D4812C;
pub const mmXBAR_EDGE_0_HBM_USER_RESP_OVR_0: c_uint = 0x4D48130;
pub const mmXBAR_EDGE_0_HBM_USER_RESP_OVR_1: c_uint = 0x4D48134;
pub const mmXBAR_EDGE_0_RL_RD_0: c_uint = 0x4D48140;
pub const mmXBAR_EDGE_0_RL_RD_1: c_uint = 0x4D48144;
pub const mmXBAR_EDGE_0_RL_RD_2: c_uint = 0x4D48148;
pub const mmXBAR_EDGE_0_RL_RD_3: c_uint = 0x4D4814C;
pub const mmXBAR_EDGE_0_RL_RD_4: c_uint = 0x4D48150;
pub const mmXBAR_EDGE_0_RL_RD_5: c_uint = 0x4D48154;
pub const mmXBAR_EDGE_0_RL_RD_6: c_uint = 0x4D48158;
pub const mmXBAR_EDGE_0_RL_RD_7: c_uint = 0x4D4815C;
pub const mmXBAR_EDGE_0_RL_RD_8: c_uint = 0x4D48160;
pub const mmXBAR_EDGE_0_RL_RD_9: c_uint = 0x4D48164;
pub const mmXBAR_EDGE_0_RL_RD_10: c_uint = 0x4D48168;
pub const mmXBAR_EDGE_0_RL_RD_11: c_uint = 0x4D4816C;
pub const mmXBAR_EDGE_0_RL_WR_0: c_uint = 0x4D48180;
pub const mmXBAR_EDGE_0_RL_WR_1: c_uint = 0x4D48184;
pub const mmXBAR_EDGE_0_RL_WR_2: c_uint = 0x4D48188;
pub const mmXBAR_EDGE_0_RL_WR_3: c_uint = 0x4D4818C;
pub const mmXBAR_EDGE_0_RL_WR_4: c_uint = 0x4D48190;
pub const mmXBAR_EDGE_0_RL_WR_5: c_uint = 0x4D48194;
pub const mmXBAR_EDGE_0_RL_WR_6: c_uint = 0x4D48198;
pub const mmXBAR_EDGE_0_RL_WR_7: c_uint = 0x4D4819C;
pub const mmXBAR_EDGE_0_RL_WR_8: c_uint = 0x4D481A0;
pub const mmXBAR_EDGE_0_RL_WR_9: c_uint = 0x4D481A4;
pub const mmXBAR_EDGE_0_RL_WR_10: c_uint = 0x4D481A8;
pub const mmXBAR_EDGE_0_RL_WR_11: c_uint = 0x4D481AC;
pub const mmXBAR_EDGE_0_E2E_CRDT_SLV_0: c_uint = 0x4D481B0;
pub const mmXBAR_EDGE_0_E2E_CRDT_SLV_1: c_uint = 0x4D481B4;
pub const mmXBAR_EDGE_0_E2E_CRDT_SLV_2: c_uint = 0x4D481B8;
pub const mmXBAR_EDGE_0_E2E_CRDT_DEBUG: c_uint = 0x4D481BC;
pub const mmXBAR_EDGE_0_UPSCALE: c_uint = 0x4D481C0;
pub const mmXBAR_EDGE_0_DOWN_CONV: c_uint = 0x4D481C4;
pub const mmXBAR_EDGE_0_DOWN_CONV_LFSR_EN: c_uint = 0x4D481D0;
pub const mmXBAR_EDGE_0_DOWN_CONV_LFSR_SET_VLD: c_uint = 0x4D481D4;
pub const mmXBAR_EDGE_0_DOWN_CONV_LFSR_SET_VALUE: c_uint = 0x4D481D8;
pub const mmXBAR_EDGE_0_DOWN_CONV_LFSR_CFG_POLY: c_uint = 0x4D481DC;
