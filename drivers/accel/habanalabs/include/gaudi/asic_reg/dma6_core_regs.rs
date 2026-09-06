//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/asic_reg/dma6_core_regs.h
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
// DMA6_CORE (Prototype: DMA_CORE)
//
pub const mmDMA6_CORE_CFG_0: c_uint = 0x5C0000;
pub const mmDMA6_CORE_CFG_1: c_uint = 0x5C0004;
pub const mmDMA6_CORE_LBW_MAX_OUTSTAND: c_uint = 0x5C0008;
pub const mmDMA6_CORE_SRC_BASE_LO: c_uint = 0x5C0014;
pub const mmDMA6_CORE_SRC_BASE_HI: c_uint = 0x5C0018;
pub const mmDMA6_CORE_DST_BASE_LO: c_uint = 0x5C001C;
pub const mmDMA6_CORE_DST_BASE_HI: c_uint = 0x5C0020;
pub const mmDMA6_CORE_SRC_TSIZE_1: c_uint = 0x5C002C;
pub const mmDMA6_CORE_SRC_STRIDE_1: c_uint = 0x5C0030;
pub const mmDMA6_CORE_SRC_TSIZE_2: c_uint = 0x5C0034;
pub const mmDMA6_CORE_SRC_STRIDE_2: c_uint = 0x5C0038;
pub const mmDMA6_CORE_SRC_TSIZE_3: c_uint = 0x5C003C;
pub const mmDMA6_CORE_SRC_STRIDE_3: c_uint = 0x5C0040;
pub const mmDMA6_CORE_SRC_TSIZE_4: c_uint = 0x5C0044;
pub const mmDMA6_CORE_SRC_STRIDE_4: c_uint = 0x5C0048;
pub const mmDMA6_CORE_SRC_TSIZE_0: c_uint = 0x5C004C;
pub const mmDMA6_CORE_DST_TSIZE_1: c_uint = 0x5C0054;
pub const mmDMA6_CORE_DST_STRIDE_1: c_uint = 0x5C0058;
pub const mmDMA6_CORE_DST_TSIZE_2: c_uint = 0x5C005C;
pub const mmDMA6_CORE_DST_STRIDE_2: c_uint = 0x5C0060;
pub const mmDMA6_CORE_DST_TSIZE_3: c_uint = 0x5C0064;
pub const mmDMA6_CORE_DST_STRIDE_3: c_uint = 0x5C0068;
pub const mmDMA6_CORE_DST_TSIZE_4: c_uint = 0x5C006C;
pub const mmDMA6_CORE_DST_STRIDE_4: c_uint = 0x5C0070;
pub const mmDMA6_CORE_DST_TSIZE_0: c_uint = 0x5C0074;
pub const mmDMA6_CORE_COMMIT: c_uint = 0x5C0078;
pub const mmDMA6_CORE_WR_COMP_WDATA: c_uint = 0x5C007C;
pub const mmDMA6_CORE_WR_COMP_ADDR_LO: c_uint = 0x5C0080;
pub const mmDMA6_CORE_WR_COMP_ADDR_HI: c_uint = 0x5C0084;
pub const mmDMA6_CORE_WR_COMP_AWUSER_31_11: c_uint = 0x5C0088;
pub const mmDMA6_CORE_TE_NUMROWS: c_uint = 0x5C0094;
pub const mmDMA6_CORE_PROT: c_uint = 0x5C00B8;
pub const mmDMA6_CORE_SECURE_PROPS: c_uint = 0x5C00F0;
pub const mmDMA6_CORE_NON_SECURE_PROPS: c_uint = 0x5C00F4;
pub const mmDMA6_CORE_RD_MAX_OUTSTAND: c_uint = 0x5C0100;
pub const mmDMA6_CORE_RD_MAX_SIZE: c_uint = 0x5C0104;
pub const mmDMA6_CORE_RD_ARCACHE: c_uint = 0x5C0108;
pub const mmDMA6_CORE_RD_ARUSER_31_11: c_uint = 0x5C0110;
pub const mmDMA6_CORE_RD_INFLIGHTS: c_uint = 0x5C0114;
pub const mmDMA6_CORE_WR_MAX_OUTSTAND: c_uint = 0x5C0120;
pub const mmDMA6_CORE_WR_MAX_AWID: c_uint = 0x5C0124;
pub const mmDMA6_CORE_WR_AWCACHE: c_uint = 0x5C0128;
pub const mmDMA6_CORE_WR_AWUSER_31_11: c_uint = 0x5C0130;
pub const mmDMA6_CORE_WR_INFLIGHTS: c_uint = 0x5C0134;
pub const mmDMA6_CORE_RD_RATE_LIM_CFG_0: c_uint = 0x5C0150;
pub const mmDMA6_CORE_RD_RATE_LIM_CFG_1: c_uint = 0x5C0154;
pub const mmDMA6_CORE_WR_RATE_LIM_CFG_0: c_uint = 0x5C0158;
pub const mmDMA6_CORE_WR_RATE_LIM_CFG_1: c_uint = 0x5C015C;
pub const mmDMA6_CORE_ERR_CFG: c_uint = 0x5C0160;
pub const mmDMA6_CORE_ERR_CAUSE: c_uint = 0x5C0164;
pub const mmDMA6_CORE_ERRMSG_ADDR_LO: c_uint = 0x5C0170;
pub const mmDMA6_CORE_ERRMSG_ADDR_HI: c_uint = 0x5C0174;
pub const mmDMA6_CORE_ERRMSG_WDATA: c_uint = 0x5C0178;
pub const mmDMA6_CORE_STS0: c_uint = 0x5C0190;
pub const mmDMA6_CORE_STS1: c_uint = 0x5C0194;
pub const mmDMA6_CORE_RD_DBGMEM_ADD: c_uint = 0x5C0200;
pub const mmDMA6_CORE_RD_DBGMEM_DATA_WR: c_uint = 0x5C0204;
pub const mmDMA6_CORE_RD_DBGMEM_DATA_RD: c_uint = 0x5C0208;
pub const mmDMA6_CORE_RD_DBGMEM_CTRL: c_uint = 0x5C020C;
pub const mmDMA6_CORE_RD_DBGMEM_RC: c_uint = 0x5C0210;
pub const mmDMA6_CORE_DBG_HBW_AXI_AR_CNT: c_uint = 0x5C0220;
pub const mmDMA6_CORE_DBG_HBW_AXI_AW_CNT: c_uint = 0x5C0224;
pub const mmDMA6_CORE_DBG_LBW_AXI_AW_CNT: c_uint = 0x5C0228;
pub const mmDMA6_CORE_DBG_DESC_CNT: c_uint = 0x5C022C;
pub const mmDMA6_CORE_DBG_STS: c_uint = 0x5C0230;
pub const mmDMA6_CORE_DBG_RD_DESC_ID: c_uint = 0x5C0234;
pub const mmDMA6_CORE_DBG_WR_DESC_ID: c_uint = 0x5C0238;
