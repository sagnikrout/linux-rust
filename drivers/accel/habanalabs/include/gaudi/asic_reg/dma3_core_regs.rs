//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/asic_reg/dma3_core_regs.h
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
// DMA3_CORE (Prototype: DMA_CORE)
//
pub const mmDMA3_CORE_CFG_0: c_uint = 0x560000;
pub const mmDMA3_CORE_CFG_1: c_uint = 0x560004;
pub const mmDMA3_CORE_LBW_MAX_OUTSTAND: c_uint = 0x560008;
pub const mmDMA3_CORE_SRC_BASE_LO: c_uint = 0x560014;
pub const mmDMA3_CORE_SRC_BASE_HI: c_uint = 0x560018;
pub const mmDMA3_CORE_DST_BASE_LO: c_uint = 0x56001C;
pub const mmDMA3_CORE_DST_BASE_HI: c_uint = 0x560020;
pub const mmDMA3_CORE_SRC_TSIZE_1: c_uint = 0x56002C;
pub const mmDMA3_CORE_SRC_STRIDE_1: c_uint = 0x560030;
pub const mmDMA3_CORE_SRC_TSIZE_2: c_uint = 0x560034;
pub const mmDMA3_CORE_SRC_STRIDE_2: c_uint = 0x560038;
pub const mmDMA3_CORE_SRC_TSIZE_3: c_uint = 0x56003C;
pub const mmDMA3_CORE_SRC_STRIDE_3: c_uint = 0x560040;
pub const mmDMA3_CORE_SRC_TSIZE_4: c_uint = 0x560044;
pub const mmDMA3_CORE_SRC_STRIDE_4: c_uint = 0x560048;
pub const mmDMA3_CORE_SRC_TSIZE_0: c_uint = 0x56004C;
pub const mmDMA3_CORE_DST_TSIZE_1: c_uint = 0x560054;
pub const mmDMA3_CORE_DST_STRIDE_1: c_uint = 0x560058;
pub const mmDMA3_CORE_DST_TSIZE_2: c_uint = 0x56005C;
pub const mmDMA3_CORE_DST_STRIDE_2: c_uint = 0x560060;
pub const mmDMA3_CORE_DST_TSIZE_3: c_uint = 0x560064;
pub const mmDMA3_CORE_DST_STRIDE_3: c_uint = 0x560068;
pub const mmDMA3_CORE_DST_TSIZE_4: c_uint = 0x56006C;
pub const mmDMA3_CORE_DST_STRIDE_4: c_uint = 0x560070;
pub const mmDMA3_CORE_DST_TSIZE_0: c_uint = 0x560074;
pub const mmDMA3_CORE_COMMIT: c_uint = 0x560078;
pub const mmDMA3_CORE_WR_COMP_WDATA: c_uint = 0x56007C;
pub const mmDMA3_CORE_WR_COMP_ADDR_LO: c_uint = 0x560080;
pub const mmDMA3_CORE_WR_COMP_ADDR_HI: c_uint = 0x560084;
pub const mmDMA3_CORE_WR_COMP_AWUSER_31_11: c_uint = 0x560088;
pub const mmDMA3_CORE_TE_NUMROWS: c_uint = 0x560094;
pub const mmDMA3_CORE_PROT: c_uint = 0x5600B8;
pub const mmDMA3_CORE_SECURE_PROPS: c_uint = 0x5600F0;
pub const mmDMA3_CORE_NON_SECURE_PROPS: c_uint = 0x5600F4;
pub const mmDMA3_CORE_RD_MAX_OUTSTAND: c_uint = 0x560100;
pub const mmDMA3_CORE_RD_MAX_SIZE: c_uint = 0x560104;
pub const mmDMA3_CORE_RD_ARCACHE: c_uint = 0x560108;
pub const mmDMA3_CORE_RD_ARUSER_31_11: c_uint = 0x560110;
pub const mmDMA3_CORE_RD_INFLIGHTS: c_uint = 0x560114;
pub const mmDMA3_CORE_WR_MAX_OUTSTAND: c_uint = 0x560120;
pub const mmDMA3_CORE_WR_MAX_AWID: c_uint = 0x560124;
pub const mmDMA3_CORE_WR_AWCACHE: c_uint = 0x560128;
pub const mmDMA3_CORE_WR_AWUSER_31_11: c_uint = 0x560130;
pub const mmDMA3_CORE_WR_INFLIGHTS: c_uint = 0x560134;
pub const mmDMA3_CORE_RD_RATE_LIM_CFG_0: c_uint = 0x560150;
pub const mmDMA3_CORE_RD_RATE_LIM_CFG_1: c_uint = 0x560154;
pub const mmDMA3_CORE_WR_RATE_LIM_CFG_0: c_uint = 0x560158;
pub const mmDMA3_CORE_WR_RATE_LIM_CFG_1: c_uint = 0x56015C;
pub const mmDMA3_CORE_ERR_CFG: c_uint = 0x560160;
pub const mmDMA3_CORE_ERR_CAUSE: c_uint = 0x560164;
pub const mmDMA3_CORE_ERRMSG_ADDR_LO: c_uint = 0x560170;
pub const mmDMA3_CORE_ERRMSG_ADDR_HI: c_uint = 0x560174;
pub const mmDMA3_CORE_ERRMSG_WDATA: c_uint = 0x560178;
pub const mmDMA3_CORE_STS0: c_uint = 0x560190;
pub const mmDMA3_CORE_STS1: c_uint = 0x560194;
pub const mmDMA3_CORE_RD_DBGMEM_ADD: c_uint = 0x560200;
pub const mmDMA3_CORE_RD_DBGMEM_DATA_WR: c_uint = 0x560204;
pub const mmDMA3_CORE_RD_DBGMEM_DATA_RD: c_uint = 0x560208;
pub const mmDMA3_CORE_RD_DBGMEM_CTRL: c_uint = 0x56020C;
pub const mmDMA3_CORE_RD_DBGMEM_RC: c_uint = 0x560210;
pub const mmDMA3_CORE_DBG_HBW_AXI_AR_CNT: c_uint = 0x560220;
pub const mmDMA3_CORE_DBG_HBW_AXI_AW_CNT: c_uint = 0x560224;
pub const mmDMA3_CORE_DBG_LBW_AXI_AW_CNT: c_uint = 0x560228;
pub const mmDMA3_CORE_DBG_DESC_CNT: c_uint = 0x56022C;
pub const mmDMA3_CORE_DBG_STS: c_uint = 0x560230;
pub const mmDMA3_CORE_DBG_RD_DESC_ID: c_uint = 0x560234;
pub const mmDMA3_CORE_DBG_WR_DESC_ID: c_uint = 0x560238;
