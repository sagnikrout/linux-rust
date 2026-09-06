//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/asic_reg/dma7_core_regs.h
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
// DMA7_CORE (Prototype: DMA_CORE)
//
pub const mmDMA7_CORE_CFG_0: c_uint = 0x5E0000;
pub const mmDMA7_CORE_CFG_1: c_uint = 0x5E0004;
pub const mmDMA7_CORE_LBW_MAX_OUTSTAND: c_uint = 0x5E0008;
pub const mmDMA7_CORE_SRC_BASE_LO: c_uint = 0x5E0014;
pub const mmDMA7_CORE_SRC_BASE_HI: c_uint = 0x5E0018;
pub const mmDMA7_CORE_DST_BASE_LO: c_uint = 0x5E001C;
pub const mmDMA7_CORE_DST_BASE_HI: c_uint = 0x5E0020;
pub const mmDMA7_CORE_SRC_TSIZE_1: c_uint = 0x5E002C;
pub const mmDMA7_CORE_SRC_STRIDE_1: c_uint = 0x5E0030;
pub const mmDMA7_CORE_SRC_TSIZE_2: c_uint = 0x5E0034;
pub const mmDMA7_CORE_SRC_STRIDE_2: c_uint = 0x5E0038;
pub const mmDMA7_CORE_SRC_TSIZE_3: c_uint = 0x5E003C;
pub const mmDMA7_CORE_SRC_STRIDE_3: c_uint = 0x5E0040;
pub const mmDMA7_CORE_SRC_TSIZE_4: c_uint = 0x5E0044;
pub const mmDMA7_CORE_SRC_STRIDE_4: c_uint = 0x5E0048;
pub const mmDMA7_CORE_SRC_TSIZE_0: c_uint = 0x5E004C;
pub const mmDMA7_CORE_DST_TSIZE_1: c_uint = 0x5E0054;
pub const mmDMA7_CORE_DST_STRIDE_1: c_uint = 0x5E0058;
pub const mmDMA7_CORE_DST_TSIZE_2: c_uint = 0x5E005C;
pub const mmDMA7_CORE_DST_STRIDE_2: c_uint = 0x5E0060;
pub const mmDMA7_CORE_DST_TSIZE_3: c_uint = 0x5E0064;
pub const mmDMA7_CORE_DST_STRIDE_3: c_uint = 0x5E0068;
pub const mmDMA7_CORE_DST_TSIZE_4: c_uint = 0x5E006C;
pub const mmDMA7_CORE_DST_STRIDE_4: c_uint = 0x5E0070;
pub const mmDMA7_CORE_DST_TSIZE_0: c_uint = 0x5E0074;
pub const mmDMA7_CORE_COMMIT: c_uint = 0x5E0078;
pub const mmDMA7_CORE_WR_COMP_WDATA: c_uint = 0x5E007C;
pub const mmDMA7_CORE_WR_COMP_ADDR_LO: c_uint = 0x5E0080;
pub const mmDMA7_CORE_WR_COMP_ADDR_HI: c_uint = 0x5E0084;
pub const mmDMA7_CORE_WR_COMP_AWUSER_31_11: c_uint = 0x5E0088;
pub const mmDMA7_CORE_TE_NUMROWS: c_uint = 0x5E0094;
pub const mmDMA7_CORE_PROT: c_uint = 0x5E00B8;
pub const mmDMA7_CORE_SECURE_PROPS: c_uint = 0x5E00F0;
pub const mmDMA7_CORE_NON_SECURE_PROPS: c_uint = 0x5E00F4;
pub const mmDMA7_CORE_RD_MAX_OUTSTAND: c_uint = 0x5E0100;
pub const mmDMA7_CORE_RD_MAX_SIZE: c_uint = 0x5E0104;
pub const mmDMA7_CORE_RD_ARCACHE: c_uint = 0x5E0108;
pub const mmDMA7_CORE_RD_ARUSER_31_11: c_uint = 0x5E0110;
pub const mmDMA7_CORE_RD_INFLIGHTS: c_uint = 0x5E0114;
pub const mmDMA7_CORE_WR_MAX_OUTSTAND: c_uint = 0x5E0120;
pub const mmDMA7_CORE_WR_MAX_AWID: c_uint = 0x5E0124;
pub const mmDMA7_CORE_WR_AWCACHE: c_uint = 0x5E0128;
pub const mmDMA7_CORE_WR_AWUSER_31_11: c_uint = 0x5E0130;
pub const mmDMA7_CORE_WR_INFLIGHTS: c_uint = 0x5E0134;
pub const mmDMA7_CORE_RD_RATE_LIM_CFG_0: c_uint = 0x5E0150;
pub const mmDMA7_CORE_RD_RATE_LIM_CFG_1: c_uint = 0x5E0154;
pub const mmDMA7_CORE_WR_RATE_LIM_CFG_0: c_uint = 0x5E0158;
pub const mmDMA7_CORE_WR_RATE_LIM_CFG_1: c_uint = 0x5E015C;
pub const mmDMA7_CORE_ERR_CFG: c_uint = 0x5E0160;
pub const mmDMA7_CORE_ERR_CAUSE: c_uint = 0x5E0164;
pub const mmDMA7_CORE_ERRMSG_ADDR_LO: c_uint = 0x5E0170;
pub const mmDMA7_CORE_ERRMSG_ADDR_HI: c_uint = 0x5E0174;
pub const mmDMA7_CORE_ERRMSG_WDATA: c_uint = 0x5E0178;
pub const mmDMA7_CORE_STS0: c_uint = 0x5E0190;
pub const mmDMA7_CORE_STS1: c_uint = 0x5E0194;
pub const mmDMA7_CORE_RD_DBGMEM_ADD: c_uint = 0x5E0200;
pub const mmDMA7_CORE_RD_DBGMEM_DATA_WR: c_uint = 0x5E0204;
pub const mmDMA7_CORE_RD_DBGMEM_DATA_RD: c_uint = 0x5E0208;
pub const mmDMA7_CORE_RD_DBGMEM_CTRL: c_uint = 0x5E020C;
pub const mmDMA7_CORE_RD_DBGMEM_RC: c_uint = 0x5E0210;
pub const mmDMA7_CORE_DBG_HBW_AXI_AR_CNT: c_uint = 0x5E0220;
pub const mmDMA7_CORE_DBG_HBW_AXI_AW_CNT: c_uint = 0x5E0224;
pub const mmDMA7_CORE_DBG_LBW_AXI_AW_CNT: c_uint = 0x5E0228;
pub const mmDMA7_CORE_DBG_DESC_CNT: c_uint = 0x5E022C;
pub const mmDMA7_CORE_DBG_STS: c_uint = 0x5E0230;
pub const mmDMA7_CORE_DBG_RD_DESC_ID: c_uint = 0x5E0234;
pub const mmDMA7_CORE_DBG_WR_DESC_ID: c_uint = 0x5E0238;
