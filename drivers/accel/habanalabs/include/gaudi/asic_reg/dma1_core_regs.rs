//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/asic_reg/dma1_core_regs.h
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
// DMA1_CORE (Prototype: DMA_CORE)
//
pub const mmDMA1_CORE_CFG_0: c_uint = 0x520000;
pub const mmDMA1_CORE_CFG_1: c_uint = 0x520004;
pub const mmDMA1_CORE_LBW_MAX_OUTSTAND: c_uint = 0x520008;
pub const mmDMA1_CORE_SRC_BASE_LO: c_uint = 0x520014;
pub const mmDMA1_CORE_SRC_BASE_HI: c_uint = 0x520018;
pub const mmDMA1_CORE_DST_BASE_LO: c_uint = 0x52001C;
pub const mmDMA1_CORE_DST_BASE_HI: c_uint = 0x520020;
pub const mmDMA1_CORE_SRC_TSIZE_1: c_uint = 0x52002C;
pub const mmDMA1_CORE_SRC_STRIDE_1: c_uint = 0x520030;
pub const mmDMA1_CORE_SRC_TSIZE_2: c_uint = 0x520034;
pub const mmDMA1_CORE_SRC_STRIDE_2: c_uint = 0x520038;
pub const mmDMA1_CORE_SRC_TSIZE_3: c_uint = 0x52003C;
pub const mmDMA1_CORE_SRC_STRIDE_3: c_uint = 0x520040;
pub const mmDMA1_CORE_SRC_TSIZE_4: c_uint = 0x520044;
pub const mmDMA1_CORE_SRC_STRIDE_4: c_uint = 0x520048;
pub const mmDMA1_CORE_SRC_TSIZE_0: c_uint = 0x52004C;
pub const mmDMA1_CORE_DST_TSIZE_1: c_uint = 0x520054;
pub const mmDMA1_CORE_DST_STRIDE_1: c_uint = 0x520058;
pub const mmDMA1_CORE_DST_TSIZE_2: c_uint = 0x52005C;
pub const mmDMA1_CORE_DST_STRIDE_2: c_uint = 0x520060;
pub const mmDMA1_CORE_DST_TSIZE_3: c_uint = 0x520064;
pub const mmDMA1_CORE_DST_STRIDE_3: c_uint = 0x520068;
pub const mmDMA1_CORE_DST_TSIZE_4: c_uint = 0x52006C;
pub const mmDMA1_CORE_DST_STRIDE_4: c_uint = 0x520070;
pub const mmDMA1_CORE_DST_TSIZE_0: c_uint = 0x520074;
pub const mmDMA1_CORE_COMMIT: c_uint = 0x520078;
pub const mmDMA1_CORE_WR_COMP_WDATA: c_uint = 0x52007C;
pub const mmDMA1_CORE_WR_COMP_ADDR_LO: c_uint = 0x520080;
pub const mmDMA1_CORE_WR_COMP_ADDR_HI: c_uint = 0x520084;
pub const mmDMA1_CORE_WR_COMP_AWUSER_31_11: c_uint = 0x520088;
pub const mmDMA1_CORE_TE_NUMROWS: c_uint = 0x520094;
pub const mmDMA1_CORE_PROT: c_uint = 0x5200B8;
pub const mmDMA1_CORE_SECURE_PROPS: c_uint = 0x5200F0;
pub const mmDMA1_CORE_NON_SECURE_PROPS: c_uint = 0x5200F4;
pub const mmDMA1_CORE_RD_MAX_OUTSTAND: c_uint = 0x520100;
pub const mmDMA1_CORE_RD_MAX_SIZE: c_uint = 0x520104;
pub const mmDMA1_CORE_RD_ARCACHE: c_uint = 0x520108;
pub const mmDMA1_CORE_RD_ARUSER_31_11: c_uint = 0x520110;
pub const mmDMA1_CORE_RD_INFLIGHTS: c_uint = 0x520114;
pub const mmDMA1_CORE_WR_MAX_OUTSTAND: c_uint = 0x520120;
pub const mmDMA1_CORE_WR_MAX_AWID: c_uint = 0x520124;
pub const mmDMA1_CORE_WR_AWCACHE: c_uint = 0x520128;
pub const mmDMA1_CORE_WR_AWUSER_31_11: c_uint = 0x520130;
pub const mmDMA1_CORE_WR_INFLIGHTS: c_uint = 0x520134;
pub const mmDMA1_CORE_RD_RATE_LIM_CFG_0: c_uint = 0x520150;
pub const mmDMA1_CORE_RD_RATE_LIM_CFG_1: c_uint = 0x520154;
pub const mmDMA1_CORE_WR_RATE_LIM_CFG_0: c_uint = 0x520158;
pub const mmDMA1_CORE_WR_RATE_LIM_CFG_1: c_uint = 0x52015C;
pub const mmDMA1_CORE_ERR_CFG: c_uint = 0x520160;
pub const mmDMA1_CORE_ERR_CAUSE: c_uint = 0x520164;
pub const mmDMA1_CORE_ERRMSG_ADDR_LO: c_uint = 0x520170;
pub const mmDMA1_CORE_ERRMSG_ADDR_HI: c_uint = 0x520174;
pub const mmDMA1_CORE_ERRMSG_WDATA: c_uint = 0x520178;
pub const mmDMA1_CORE_STS0: c_uint = 0x520190;
pub const mmDMA1_CORE_STS1: c_uint = 0x520194;
pub const mmDMA1_CORE_RD_DBGMEM_ADD: c_uint = 0x520200;
pub const mmDMA1_CORE_RD_DBGMEM_DATA_WR: c_uint = 0x520204;
pub const mmDMA1_CORE_RD_DBGMEM_DATA_RD: c_uint = 0x520208;
pub const mmDMA1_CORE_RD_DBGMEM_CTRL: c_uint = 0x52020C;
pub const mmDMA1_CORE_RD_DBGMEM_RC: c_uint = 0x520210;
pub const mmDMA1_CORE_DBG_HBW_AXI_AR_CNT: c_uint = 0x520220;
pub const mmDMA1_CORE_DBG_HBW_AXI_AW_CNT: c_uint = 0x520224;
pub const mmDMA1_CORE_DBG_LBW_AXI_AW_CNT: c_uint = 0x520228;
pub const mmDMA1_CORE_DBG_DESC_CNT: c_uint = 0x52022C;
pub const mmDMA1_CORE_DBG_STS: c_uint = 0x520230;
pub const mmDMA1_CORE_DBG_RD_DESC_ID: c_uint = 0x520234;
pub const mmDMA1_CORE_DBG_WR_DESC_ID: c_uint = 0x520238;
