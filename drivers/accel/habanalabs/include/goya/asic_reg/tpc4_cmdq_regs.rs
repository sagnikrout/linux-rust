//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc4_cmdq_regs.h
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
// TPC4_CMDQ (Prototype: CMDQ)
//
pub const mmTPC4_CMDQ_GLBL_CFG0: c_uint = 0xF09000;
pub const mmTPC4_CMDQ_GLBL_CFG1: c_uint = 0xF09004;
pub const mmTPC4_CMDQ_GLBL_PROT: c_uint = 0xF09008;
pub const mmTPC4_CMDQ_GLBL_ERR_CFG: c_uint = 0xF0900C;
pub const mmTPC4_CMDQ_GLBL_ERR_ADDR_LO: c_uint = 0xF09010;
pub const mmTPC4_CMDQ_GLBL_ERR_ADDR_HI: c_uint = 0xF09014;
pub const mmTPC4_CMDQ_GLBL_ERR_WDATA: c_uint = 0xF09018;
pub const mmTPC4_CMDQ_GLBL_SECURE_PROPS: c_uint = 0xF0901C;
pub const mmTPC4_CMDQ_GLBL_NON_SECURE_PROPS: c_uint = 0xF09020;
pub const mmTPC4_CMDQ_GLBL_STS0: c_uint = 0xF09024;
pub const mmTPC4_CMDQ_GLBL_STS1: c_uint = 0xF09028;
pub const mmTPC4_CMDQ_CQ_CFG0: c_uint = 0xF090B0;
pub const mmTPC4_CMDQ_CQ_CFG1: c_uint = 0xF090B4;
pub const mmTPC4_CMDQ_CQ_ARUSER: c_uint = 0xF090B8;
pub const mmTPC4_CMDQ_CQ_PTR_LO: c_uint = 0xF090C0;
pub const mmTPC4_CMDQ_CQ_PTR_HI: c_uint = 0xF090C4;
pub const mmTPC4_CMDQ_CQ_TSIZE: c_uint = 0xF090C8;
pub const mmTPC4_CMDQ_CQ_CTL: c_uint = 0xF090CC;
pub const mmTPC4_CMDQ_CQ_PTR_LO_STS: c_uint = 0xF090D4;
pub const mmTPC4_CMDQ_CQ_PTR_HI_STS: c_uint = 0xF090D8;
pub const mmTPC4_CMDQ_CQ_TSIZE_STS: c_uint = 0xF090DC;
pub const mmTPC4_CMDQ_CQ_CTL_STS: c_uint = 0xF090E0;
pub const mmTPC4_CMDQ_CQ_STS0: c_uint = 0xF090E4;
pub const mmTPC4_CMDQ_CQ_STS1: c_uint = 0xF090E8;
pub const mmTPC4_CMDQ_CQ_RD_RATE_LIM_EN: c_uint = 0xF090F0;
pub const mmTPC4_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0xF090F4;
pub const mmTPC4_CMDQ_CQ_RD_RATE_LIM_SAT: c_uint = 0xF090F8;
pub const mmTPC4_CMDQ_CQ_RD_RATE_LIM_TOUT: c_uint = 0xF090FC;
pub const mmTPC4_CMDQ_CQ_IFIFO_CNT: c_uint = 0xF09108;
pub const mmTPC4_CMDQ_CP_MSG_BASE0_ADDR_LO: c_uint = 0xF09120;
pub const mmTPC4_CMDQ_CP_MSG_BASE0_ADDR_HI: c_uint = 0xF09124;
pub const mmTPC4_CMDQ_CP_MSG_BASE1_ADDR_LO: c_uint = 0xF09128;
pub const mmTPC4_CMDQ_CP_MSG_BASE1_ADDR_HI: c_uint = 0xF0912C;
pub const mmTPC4_CMDQ_CP_MSG_BASE2_ADDR_LO: c_uint = 0xF09130;
pub const mmTPC4_CMDQ_CP_MSG_BASE2_ADDR_HI: c_uint = 0xF09134;
pub const mmTPC4_CMDQ_CP_MSG_BASE3_ADDR_LO: c_uint = 0xF09138;
pub const mmTPC4_CMDQ_CP_MSG_BASE3_ADDR_HI: c_uint = 0xF0913C;
pub const mmTPC4_CMDQ_CP_LDMA_TSIZE_OFFSET: c_uint = 0xF09140;
pub const mmTPC4_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: c_uint = 0xF09144;
pub const mmTPC4_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: c_uint = 0xF09148;
pub const mmTPC4_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: c_uint = 0xF0914C;
pub const mmTPC4_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: c_uint = 0xF09150;
pub const mmTPC4_CMDQ_CP_LDMA_COMMIT_OFFSET: c_uint = 0xF09154;
pub const mmTPC4_CMDQ_CP_FENCE0_RDATA: c_uint = 0xF09158;
pub const mmTPC4_CMDQ_CP_FENCE1_RDATA: c_uint = 0xF0915C;
pub const mmTPC4_CMDQ_CP_FENCE2_RDATA: c_uint = 0xF09160;
pub const mmTPC4_CMDQ_CP_FENCE3_RDATA: c_uint = 0xF09164;
pub const mmTPC4_CMDQ_CP_FENCE0_CNT: c_uint = 0xF09168;
pub const mmTPC4_CMDQ_CP_FENCE1_CNT: c_uint = 0xF0916C;
pub const mmTPC4_CMDQ_CP_FENCE2_CNT: c_uint = 0xF09170;
pub const mmTPC4_CMDQ_CP_FENCE3_CNT: c_uint = 0xF09174;
pub const mmTPC4_CMDQ_CP_STS: c_uint = 0xF09178;
pub const mmTPC4_CMDQ_CP_CURRENT_INST_LO: c_uint = 0xF0917C;
pub const mmTPC4_CMDQ_CP_CURRENT_INST_HI: c_uint = 0xF09180;
pub const mmTPC4_CMDQ_CP_BARRIER_CFG: c_uint = 0xF09184;
pub const mmTPC4_CMDQ_CP_DBG_0: c_uint = 0xF09188;
pub const mmTPC4_CMDQ_CQ_BUF_ADDR: c_uint = 0xF09308;
pub const mmTPC4_CMDQ_CQ_BUF_RDATA: c_uint = 0xF0930C;
