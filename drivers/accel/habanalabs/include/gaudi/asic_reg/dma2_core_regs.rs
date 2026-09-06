//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/asic_reg/dma2_core_regs.h
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
// DMA2_CORE (Prototype: DMA_CORE)
//
pub const mmDMA2_CORE_CFG_0: c_uint = 0x540000;
pub const mmDMA2_CORE_CFG_1: c_uint = 0x540004;
pub const mmDMA2_CORE_LBW_MAX_OUTSTAND: c_uint = 0x540008;
pub const mmDMA2_CORE_SRC_BASE_LO: c_uint = 0x540014;
pub const mmDMA2_CORE_SRC_BASE_HI: c_uint = 0x540018;
pub const mmDMA2_CORE_DST_BASE_LO: c_uint = 0x54001C;
pub const mmDMA2_CORE_DST_BASE_HI: c_uint = 0x540020;
pub const mmDMA2_CORE_SRC_TSIZE_1: c_uint = 0x54002C;
pub const mmDMA2_CORE_SRC_STRIDE_1: c_uint = 0x540030;
pub const mmDMA2_CORE_SRC_TSIZE_2: c_uint = 0x540034;
pub const mmDMA2_CORE_SRC_STRIDE_2: c_uint = 0x540038;
pub const mmDMA2_CORE_SRC_TSIZE_3: c_uint = 0x54003C;
pub const mmDMA2_CORE_SRC_STRIDE_3: c_uint = 0x540040;
pub const mmDMA2_CORE_SRC_TSIZE_4: c_uint = 0x540044;
pub const mmDMA2_CORE_SRC_STRIDE_4: c_uint = 0x540048;
pub const mmDMA2_CORE_SRC_TSIZE_0: c_uint = 0x54004C;
pub const mmDMA2_CORE_DST_TSIZE_1: c_uint = 0x540054;
pub const mmDMA2_CORE_DST_STRIDE_1: c_uint = 0x540058;
pub const mmDMA2_CORE_DST_TSIZE_2: c_uint = 0x54005C;
pub const mmDMA2_CORE_DST_STRIDE_2: c_uint = 0x540060;
pub const mmDMA2_CORE_DST_TSIZE_3: c_uint = 0x540064;
pub const mmDMA2_CORE_DST_STRIDE_3: c_uint = 0x540068;
pub const mmDMA2_CORE_DST_TSIZE_4: c_uint = 0x54006C;
pub const mmDMA2_CORE_DST_STRIDE_4: c_uint = 0x540070;
pub const mmDMA2_CORE_DST_TSIZE_0: c_uint = 0x540074;
pub const mmDMA2_CORE_COMMIT: c_uint = 0x540078;
pub const mmDMA2_CORE_WR_COMP_WDATA: c_uint = 0x54007C;
pub const mmDMA2_CORE_WR_COMP_ADDR_LO: c_uint = 0x540080;
pub const mmDMA2_CORE_WR_COMP_ADDR_HI: c_uint = 0x540084;
pub const mmDMA2_CORE_WR_COMP_AWUSER_31_11: c_uint = 0x540088;
pub const mmDMA2_CORE_TE_NUMROWS: c_uint = 0x540094;
pub const mmDMA2_CORE_PROT: c_uint = 0x5400B8;
pub const mmDMA2_CORE_SECURE_PROPS: c_uint = 0x5400F0;
pub const mmDMA2_CORE_NON_SECURE_PROPS: c_uint = 0x5400F4;
pub const mmDMA2_CORE_RD_MAX_OUTSTAND: c_uint = 0x540100;
pub const mmDMA2_CORE_RD_MAX_SIZE: c_uint = 0x540104;
pub const mmDMA2_CORE_RD_ARCACHE: c_uint = 0x540108;
pub const mmDMA2_CORE_RD_ARUSER_31_11: c_uint = 0x540110;
pub const mmDMA2_CORE_RD_INFLIGHTS: c_uint = 0x540114;
pub const mmDMA2_CORE_WR_MAX_OUTSTAND: c_uint = 0x540120;
pub const mmDMA2_CORE_WR_MAX_AWID: c_uint = 0x540124;
pub const mmDMA2_CORE_WR_AWCACHE: c_uint = 0x540128;
pub const mmDMA2_CORE_WR_AWUSER_31_11: c_uint = 0x540130;
pub const mmDMA2_CORE_WR_INFLIGHTS: c_uint = 0x540134;
pub const mmDMA2_CORE_RD_RATE_LIM_CFG_0: c_uint = 0x540150;
pub const mmDMA2_CORE_RD_RATE_LIM_CFG_1: c_uint = 0x540154;
pub const mmDMA2_CORE_WR_RATE_LIM_CFG_0: c_uint = 0x540158;
pub const mmDMA2_CORE_WR_RATE_LIM_CFG_1: c_uint = 0x54015C;
pub const mmDMA2_CORE_ERR_CFG: c_uint = 0x540160;
pub const mmDMA2_CORE_ERR_CAUSE: c_uint = 0x540164;
pub const mmDMA2_CORE_ERRMSG_ADDR_LO: c_uint = 0x540170;
pub const mmDMA2_CORE_ERRMSG_ADDR_HI: c_uint = 0x540174;
pub const mmDMA2_CORE_ERRMSG_WDATA: c_uint = 0x540178;
pub const mmDMA2_CORE_STS0: c_uint = 0x540190;
pub const mmDMA2_CORE_STS1: c_uint = 0x540194;
pub const mmDMA2_CORE_RD_DBGMEM_ADD: c_uint = 0x540200;
pub const mmDMA2_CORE_RD_DBGMEM_DATA_WR: c_uint = 0x540204;
pub const mmDMA2_CORE_RD_DBGMEM_DATA_RD: c_uint = 0x540208;
pub const mmDMA2_CORE_RD_DBGMEM_CTRL: c_uint = 0x54020C;
pub const mmDMA2_CORE_RD_DBGMEM_RC: c_uint = 0x540210;
pub const mmDMA2_CORE_DBG_HBW_AXI_AR_CNT: c_uint = 0x540220;
pub const mmDMA2_CORE_DBG_HBW_AXI_AW_CNT: c_uint = 0x540224;
pub const mmDMA2_CORE_DBG_LBW_AXI_AW_CNT: c_uint = 0x540228;
pub const mmDMA2_CORE_DBG_DESC_CNT: c_uint = 0x54022C;
pub const mmDMA2_CORE_DBG_STS: c_uint = 0x540230;
pub const mmDMA2_CORE_DBG_RD_DESC_ID: c_uint = 0x540234;
pub const mmDMA2_CORE_DBG_WR_DESC_ID: c_uint = 0x540238;
