//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc6_cmdq_regs.h
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
// TPC6_CMDQ (Prototype: CMDQ)
//
pub const mmTPC6_CMDQ_GLBL_CFG0: c_uint = 0xF89000;
pub const mmTPC6_CMDQ_GLBL_CFG1: c_uint = 0xF89004;
pub const mmTPC6_CMDQ_GLBL_PROT: c_uint = 0xF89008;
pub const mmTPC6_CMDQ_GLBL_ERR_CFG: c_uint = 0xF8900C;
pub const mmTPC6_CMDQ_GLBL_ERR_ADDR_LO: c_uint = 0xF89010;
pub const mmTPC6_CMDQ_GLBL_ERR_ADDR_HI: c_uint = 0xF89014;
pub const mmTPC6_CMDQ_GLBL_ERR_WDATA: c_uint = 0xF89018;
pub const mmTPC6_CMDQ_GLBL_SECURE_PROPS: c_uint = 0xF8901C;
pub const mmTPC6_CMDQ_GLBL_NON_SECURE_PROPS: c_uint = 0xF89020;
pub const mmTPC6_CMDQ_GLBL_STS0: c_uint = 0xF89024;
pub const mmTPC6_CMDQ_GLBL_STS1: c_uint = 0xF89028;
pub const mmTPC6_CMDQ_CQ_CFG0: c_uint = 0xF890B0;
pub const mmTPC6_CMDQ_CQ_CFG1: c_uint = 0xF890B4;
pub const mmTPC6_CMDQ_CQ_ARUSER: c_uint = 0xF890B8;
pub const mmTPC6_CMDQ_CQ_PTR_LO: c_uint = 0xF890C0;
pub const mmTPC6_CMDQ_CQ_PTR_HI: c_uint = 0xF890C4;
pub const mmTPC6_CMDQ_CQ_TSIZE: c_uint = 0xF890C8;
pub const mmTPC6_CMDQ_CQ_CTL: c_uint = 0xF890CC;
pub const mmTPC6_CMDQ_CQ_PTR_LO_STS: c_uint = 0xF890D4;
pub const mmTPC6_CMDQ_CQ_PTR_HI_STS: c_uint = 0xF890D8;
pub const mmTPC6_CMDQ_CQ_TSIZE_STS: c_uint = 0xF890DC;
pub const mmTPC6_CMDQ_CQ_CTL_STS: c_uint = 0xF890E0;
pub const mmTPC6_CMDQ_CQ_STS0: c_uint = 0xF890E4;
pub const mmTPC6_CMDQ_CQ_STS1: c_uint = 0xF890E8;
pub const mmTPC6_CMDQ_CQ_RD_RATE_LIM_EN: c_uint = 0xF890F0;
pub const mmTPC6_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0xF890F4;
pub const mmTPC6_CMDQ_CQ_RD_RATE_LIM_SAT: c_uint = 0xF890F8;
pub const mmTPC6_CMDQ_CQ_RD_RATE_LIM_TOUT: c_uint = 0xF890FC;
pub const mmTPC6_CMDQ_CQ_IFIFO_CNT: c_uint = 0xF89108;
pub const mmTPC6_CMDQ_CP_MSG_BASE0_ADDR_LO: c_uint = 0xF89120;
pub const mmTPC6_CMDQ_CP_MSG_BASE0_ADDR_HI: c_uint = 0xF89124;
pub const mmTPC6_CMDQ_CP_MSG_BASE1_ADDR_LO: c_uint = 0xF89128;
pub const mmTPC6_CMDQ_CP_MSG_BASE1_ADDR_HI: c_uint = 0xF8912C;
pub const mmTPC6_CMDQ_CP_MSG_BASE2_ADDR_LO: c_uint = 0xF89130;
pub const mmTPC6_CMDQ_CP_MSG_BASE2_ADDR_HI: c_uint = 0xF89134;
pub const mmTPC6_CMDQ_CP_MSG_BASE3_ADDR_LO: c_uint = 0xF89138;
pub const mmTPC6_CMDQ_CP_MSG_BASE3_ADDR_HI: c_uint = 0xF8913C;
pub const mmTPC6_CMDQ_CP_LDMA_TSIZE_OFFSET: c_uint = 0xF89140;
pub const mmTPC6_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: c_uint = 0xF89144;
pub const mmTPC6_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: c_uint = 0xF89148;
pub const mmTPC6_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: c_uint = 0xF8914C;
pub const mmTPC6_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: c_uint = 0xF89150;
pub const mmTPC6_CMDQ_CP_LDMA_COMMIT_OFFSET: c_uint = 0xF89154;
pub const mmTPC6_CMDQ_CP_FENCE0_RDATA: c_uint = 0xF89158;
pub const mmTPC6_CMDQ_CP_FENCE1_RDATA: c_uint = 0xF8915C;
pub const mmTPC6_CMDQ_CP_FENCE2_RDATA: c_uint = 0xF89160;
pub const mmTPC6_CMDQ_CP_FENCE3_RDATA: c_uint = 0xF89164;
pub const mmTPC6_CMDQ_CP_FENCE0_CNT: c_uint = 0xF89168;
pub const mmTPC6_CMDQ_CP_FENCE1_CNT: c_uint = 0xF8916C;
pub const mmTPC6_CMDQ_CP_FENCE2_CNT: c_uint = 0xF89170;
pub const mmTPC6_CMDQ_CP_FENCE3_CNT: c_uint = 0xF89174;
pub const mmTPC6_CMDQ_CP_STS: c_uint = 0xF89178;
pub const mmTPC6_CMDQ_CP_CURRENT_INST_LO: c_uint = 0xF8917C;
pub const mmTPC6_CMDQ_CP_CURRENT_INST_HI: c_uint = 0xF89180;
pub const mmTPC6_CMDQ_CP_BARRIER_CFG: c_uint = 0xF89184;
pub const mmTPC6_CMDQ_CP_DBG_0: c_uint = 0xF89188;
pub const mmTPC6_CMDQ_CQ_BUF_ADDR: c_uint = 0xF89308;
pub const mmTPC6_CMDQ_CQ_BUF_RDATA: c_uint = 0xF8930C;
