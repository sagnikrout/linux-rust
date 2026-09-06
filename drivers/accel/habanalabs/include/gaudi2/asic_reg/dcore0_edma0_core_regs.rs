//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_edma0_core_regs.h
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
// DCORE0_EDMA0_CORE
// (Prototype: DMA_CORE)
//
pub const mmDCORE0_EDMA0_CORE_CFG_0: c_uint = 0x41CB000;
pub const mmDCORE0_EDMA0_CORE_CFG_1: c_uint = 0x41CB004;
pub const mmDCORE0_EDMA0_CORE_PROT: c_uint = 0x41CB008;
pub const mmDCORE0_EDMA0_CORE_CKG: c_uint = 0x41CB00C;
pub const mmDCORE0_EDMA0_CORE_RD_GLBL: c_uint = 0x41CB07C;
pub const mmDCORE0_EDMA0_CORE_RD_HBW_MAX_OUTSTAND: c_uint = 0x41CB080;
pub const mmDCORE0_EDMA0_CORE_RD_HBW_MAX_SIZE: c_uint = 0x41CB084;
pub const mmDCORE0_EDMA0_CORE_RD_HBW_ARCACHE: c_uint = 0x41CB088;
pub const mmDCORE0_EDMA0_CORE_RD_HBW_INFLIGHTS: c_uint = 0x41CB090;
pub const mmDCORE0_EDMA0_CORE_RD_HBW_RATE_LIM_CFG: c_uint = 0x41CB094;
pub const mmDCORE0_EDMA0_CORE_RD_LBW_MAX_OUTSTAND: c_uint = 0x41CB0C0;
pub const mmDCORE0_EDMA0_CORE_RD_LBW_MAX_SIZE: c_uint = 0x41CB0C4;
pub const mmDCORE0_EDMA0_CORE_RD_LBW_ARCACHE: c_uint = 0x41CB0C8;
pub const mmDCORE0_EDMA0_CORE_RD_LBW_INFLIGHTS: c_uint = 0x41CB0D0;
pub const mmDCORE0_EDMA0_CORE_RD_LBW_RATE_LIM_CFG: c_uint = 0x41CB0D4;
pub const mmDCORE0_EDMA0_CORE_WR_HBW_MAX_OUTSTAND: c_uint = 0x41CB100;
pub const mmDCORE0_EDMA0_CORE_WR_HBW_MAX_AWID: c_uint = 0x41CB104;
pub const mmDCORE0_EDMA0_CORE_WR_HBW_AWCACHE: c_uint = 0x41CB108;
pub const mmDCORE0_EDMA0_CORE_WR_HBW_INFLIGHTS: c_uint = 0x41CB10C;
pub const mmDCORE0_EDMA0_CORE_WR_HBW_RATE_LIM_CFG: c_uint = 0x41CB110;
pub const mmDCORE0_EDMA0_CORE_WR_LBW_MAX_OUTSTAND: c_uint = 0x41CB140;
pub const mmDCORE0_EDMA0_CORE_WR_LBW_MAX_AWID: c_uint = 0x41CB144;
pub const mmDCORE0_EDMA0_CORE_WR_LBW_AWCACHE: c_uint = 0x41CB148;
pub const mmDCORE0_EDMA0_CORE_WR_LBW_INFLIGHTS: c_uint = 0x41CB14C;
pub const mmDCORE0_EDMA0_CORE_WR_LBW_RATE_LIM_CFG: c_uint = 0x41CB150;
pub const mmDCORE0_EDMA0_CORE_WR_COMP_MAX_OUTSTAND: c_uint = 0x41CB180;
pub const mmDCORE0_EDMA0_CORE_WR_COMP_AWUSER: c_uint = 0x41CB184;
pub const mmDCORE0_EDMA0_CORE_ERR_CFG: c_uint = 0x41CB300;
pub const mmDCORE0_EDMA0_CORE_ERR_CAUSE: c_uint = 0x41CB304;
pub const mmDCORE0_EDMA0_CORE_ERRMSG_ADDR_LO: c_uint = 0x41CB308;
pub const mmDCORE0_EDMA0_CORE_ERRMSG_ADDR_HI: c_uint = 0x41CB30C;
pub const mmDCORE0_EDMA0_CORE_ERRMSG_WDATA: c_uint = 0x41CB310;
pub const mmDCORE0_EDMA0_CORE_STS0: c_uint = 0x41CB380;
pub const mmDCORE0_EDMA0_CORE_STS1: c_uint = 0x41CB384;
pub const mmDCORE0_EDMA0_CORE_STS_RD_CTX_SEL: c_uint = 0x41CB400;
pub const mmDCORE0_EDMA0_CORE_STS_RD_CTX_SIZE: c_uint = 0x41CB404;
pub const mmDCORE0_EDMA0_CORE_STS_RD_CTX_BASE_LO: c_uint = 0x41CB408;
pub const mmDCORE0_EDMA0_CORE_STS_RD_CTX_BASE_HI: c_uint = 0x41CB40C;
pub const mmDCORE0_EDMA0_CORE_STS_RD_CTX_ID: c_uint = 0x41CB410;
pub const mmDCORE0_EDMA0_CORE_STS_RD_HB_AXI_ADDR_LO: c_uint = 0x41CB414;
pub const mmDCORE0_EDMA0_CORE_STS_RD_HB_AXI_ADDR_HI: c_uint = 0x41CB418;
pub const mmDCORE0_EDMA0_CORE_STS_RD_LB_AXI_ADDR: c_uint = 0x41CB41C;
pub const mmDCORE0_EDMA0_CORE_STS_WR_CTX_SEL: c_uint = 0x41CB420;
pub const mmDCORE0_EDMA0_CORE_STS_WR_CTX_SIZE: c_uint = 0x41CB424;
pub const mmDCORE0_EDMA0_CORE_STS_WR_CTX_BASE_LO: c_uint = 0x41CB428;
pub const mmDCORE0_EDMA0_CORE_STS_WR_CTX_BASE_HI: c_uint = 0x41CB42C;
pub const mmDCORE0_EDMA0_CORE_STS_WR_CTX_ID: c_uint = 0x41CB430;
pub const mmDCORE0_EDMA0_CORE_STS_WR_HB_AXI_ADDR_LO: c_uint = 0x41CB434;
pub const mmDCORE0_EDMA0_CORE_STS_WR_HB_AXI_ADDR_HI: c_uint = 0x41CB438;
pub const mmDCORE0_EDMA0_CORE_STS_WR_LB_AXI_ADDR: c_uint = 0x41CB43C;
pub const mmDCORE0_EDMA0_CORE_PWRLP_CFG: c_uint = 0x41CB700;
pub const mmDCORE0_EDMA0_CORE_PWRLP_STS: c_uint = 0x41CB704;
pub const mmDCORE0_EDMA0_CORE_DBG_DESC_CNT: c_uint = 0x41CB710;
pub const mmDCORE0_EDMA0_CORE_DBG_STS: c_uint = 0x41CB714;
pub const mmDCORE0_EDMA0_CORE_DBG_BUF_STS: c_uint = 0x41CB718;
pub const mmDCORE0_EDMA0_CORE_DBG_RD_DESC_ID: c_uint = 0x41CB720;
pub const mmDCORE0_EDMA0_CORE_DBG_WR_DESC_ID: c_uint = 0x41CB724;
pub const mmDCORE0_EDMA0_CORE_APB_DMA_LBW_BASE: c_uint = 0x41CB728;
pub const mmDCORE0_EDMA0_CORE_APB_MSTR_IF_LBW_BASE: c_uint = 0x41CB72C;
pub const mmDCORE0_EDMA0_CORE_E2E_CRED_ASYNC_CFG: c_uint = 0x41CB730;
pub const mmDCORE0_EDMA0_CORE_DBG_APB_ENABLER: c_uint = 0x41CBE1C;
pub const mmDCORE0_EDMA0_CORE_L2H_CMPR_LO: c_uint = 0x41CBE20;
pub const mmDCORE0_EDMA0_CORE_L2H_CMPR_HI: c_uint = 0x41CBE24;
pub const mmDCORE0_EDMA0_CORE_L2H_MASK_LO: c_uint = 0x41CBE28;
pub const mmDCORE0_EDMA0_CORE_L2H_MASK_HI: c_uint = 0x41CBE2C;
pub const mmDCORE0_EDMA0_CORE_IDLE_IND_MASK: c_uint = 0x41CBE30;
pub const mmDCORE0_EDMA0_CORE_APB_ENABLER: c_uint = 0x41CBE34;
