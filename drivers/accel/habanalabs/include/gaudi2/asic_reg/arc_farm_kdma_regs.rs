//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/arc_farm_kdma_regs.h
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
// ARC_FARM_KDMA
// (Prototype: DMA_CORE)
//
pub const mmARC_FARM_KDMA_CFG_0: c_uint = 0x4E8B000;
pub const mmARC_FARM_KDMA_CFG_1: c_uint = 0x4E8B004;
pub const mmARC_FARM_KDMA_PROT: c_uint = 0x4E8B008;
pub const mmARC_FARM_KDMA_CKG: c_uint = 0x4E8B00C;
pub const mmARC_FARM_KDMA_RD_GLBL: c_uint = 0x4E8B07C;
pub const mmARC_FARM_KDMA_RD_HBW_MAX_OUTSTAND: c_uint = 0x4E8B080;
pub const mmARC_FARM_KDMA_RD_HBW_MAX_SIZE: c_uint = 0x4E8B084;
pub const mmARC_FARM_KDMA_RD_HBW_ARCACHE: c_uint = 0x4E8B088;
pub const mmARC_FARM_KDMA_RD_HBW_INFLIGHTS: c_uint = 0x4E8B090;
pub const mmARC_FARM_KDMA_RD_HBW_RATE_LIM_CFG: c_uint = 0x4E8B094;
pub const mmARC_FARM_KDMA_RD_LBW_MAX_OUTSTAND: c_uint = 0x4E8B0C0;
pub const mmARC_FARM_KDMA_RD_LBW_MAX_SIZE: c_uint = 0x4E8B0C4;
pub const mmARC_FARM_KDMA_RD_LBW_ARCACHE: c_uint = 0x4E8B0C8;
pub const mmARC_FARM_KDMA_RD_LBW_INFLIGHTS: c_uint = 0x4E8B0D0;
pub const mmARC_FARM_KDMA_RD_LBW_RATE_LIM_CFG: c_uint = 0x4E8B0D4;
pub const mmARC_FARM_KDMA_WR_HBW_MAX_OUTSTAND: c_uint = 0x4E8B100;
pub const mmARC_FARM_KDMA_WR_HBW_MAX_AWID: c_uint = 0x4E8B104;
pub const mmARC_FARM_KDMA_WR_HBW_AWCACHE: c_uint = 0x4E8B108;
pub const mmARC_FARM_KDMA_WR_HBW_INFLIGHTS: c_uint = 0x4E8B10C;
pub const mmARC_FARM_KDMA_WR_HBW_RATE_LIM_CFG: c_uint = 0x4E8B110;
pub const mmARC_FARM_KDMA_WR_LBW_MAX_OUTSTAND: c_uint = 0x4E8B140;
pub const mmARC_FARM_KDMA_WR_LBW_MAX_AWID: c_uint = 0x4E8B144;
pub const mmARC_FARM_KDMA_WR_LBW_AWCACHE: c_uint = 0x4E8B148;
pub const mmARC_FARM_KDMA_WR_LBW_INFLIGHTS: c_uint = 0x4E8B14C;
pub const mmARC_FARM_KDMA_WR_LBW_RATE_LIM_CFG: c_uint = 0x4E8B150;
pub const mmARC_FARM_KDMA_WR_COMP_MAX_OUTSTAND: c_uint = 0x4E8B180;
pub const mmARC_FARM_KDMA_WR_COMP_AWUSER: c_uint = 0x4E8B184;
pub const mmARC_FARM_KDMA_ERR_CFG: c_uint = 0x4E8B300;
pub const mmARC_FARM_KDMA_ERR_CAUSE: c_uint = 0x4E8B304;
pub const mmARC_FARM_KDMA_ERRMSG_ADDR_LO: c_uint = 0x4E8B308;
pub const mmARC_FARM_KDMA_ERRMSG_ADDR_HI: c_uint = 0x4E8B30C;
pub const mmARC_FARM_KDMA_ERRMSG_WDATA: c_uint = 0x4E8B310;
pub const mmARC_FARM_KDMA_STS0: c_uint = 0x4E8B380;
pub const mmARC_FARM_KDMA_STS1: c_uint = 0x4E8B384;
pub const mmARC_FARM_KDMA_STS_RD_CTX_SEL: c_uint = 0x4E8B400;
pub const mmARC_FARM_KDMA_STS_RD_CTX_SIZE: c_uint = 0x4E8B404;
pub const mmARC_FARM_KDMA_STS_RD_CTX_BASE_LO: c_uint = 0x4E8B408;
pub const mmARC_FARM_KDMA_STS_RD_CTX_BASE_HI: c_uint = 0x4E8B40C;
pub const mmARC_FARM_KDMA_STS_RD_CTX_ID: c_uint = 0x4E8B410;
pub const mmARC_FARM_KDMA_STS_RD_HB_AXI_ADDR_LO: c_uint = 0x4E8B414;
pub const mmARC_FARM_KDMA_STS_RD_HB_AXI_ADDR_HI: c_uint = 0x4E8B418;
pub const mmARC_FARM_KDMA_STS_RD_LB_AXI_ADDR: c_uint = 0x4E8B41C;
pub const mmARC_FARM_KDMA_STS_WR_CTX_SEL: c_uint = 0x4E8B420;
pub const mmARC_FARM_KDMA_STS_WR_CTX_SIZE: c_uint = 0x4E8B424;
pub const mmARC_FARM_KDMA_STS_WR_CTX_BASE_LO: c_uint = 0x4E8B428;
pub const mmARC_FARM_KDMA_STS_WR_CTX_BASE_HI: c_uint = 0x4E8B42C;
pub const mmARC_FARM_KDMA_STS_WR_CTX_ID: c_uint = 0x4E8B430;
pub const mmARC_FARM_KDMA_STS_WR_HB_AXI_ADDR_LO: c_uint = 0x4E8B434;
pub const mmARC_FARM_KDMA_STS_WR_HB_AXI_ADDR_HI: c_uint = 0x4E8B438;
pub const mmARC_FARM_KDMA_STS_WR_LB_AXI_ADDR: c_uint = 0x4E8B43C;
pub const mmARC_FARM_KDMA_PWRLP_CFG: c_uint = 0x4E8B700;
pub const mmARC_FARM_KDMA_PWRLP_STS: c_uint = 0x4E8B704;
pub const mmARC_FARM_KDMA_DBG_DESC_CNT: c_uint = 0x4E8B710;
pub const mmARC_FARM_KDMA_DBG_STS: c_uint = 0x4E8B714;
pub const mmARC_FARM_KDMA_DBG_BUF_STS: c_uint = 0x4E8B718;
pub const mmARC_FARM_KDMA_DBG_RD_DESC_ID: c_uint = 0x4E8B720;
pub const mmARC_FARM_KDMA_DBG_WR_DESC_ID: c_uint = 0x4E8B724;
pub const mmARC_FARM_KDMA_APB_DMA_LBW_BASE: c_uint = 0x4E8B728;
pub const mmARC_FARM_KDMA_APB_MSTR_IF_LBW_BASE: c_uint = 0x4E8B72C;
pub const mmARC_FARM_KDMA_E2E_CRED_ASYNC_CFG: c_uint = 0x4E8B730;
pub const mmARC_FARM_KDMA_DBG_APB_ENABLER: c_uint = 0x4E8BE1C;
pub const mmARC_FARM_KDMA_L2H_CMPR_LO: c_uint = 0x4E8BE20;
pub const mmARC_FARM_KDMA_L2H_CMPR_HI: c_uint = 0x4E8BE24;
pub const mmARC_FARM_KDMA_L2H_MASK_LO: c_uint = 0x4E8BE28;
pub const mmARC_FARM_KDMA_L2H_MASK_HI: c_uint = 0x4E8BE2C;
pub const mmARC_FARM_KDMA_IDLE_IND_MASK: c_uint = 0x4E8BE30;
pub const mmARC_FARM_KDMA_APB_ENABLER: c_uint = 0x4E8BE34;
