//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc1_cmdq_regs.h
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
// TPC1_CMDQ (Prototype: CMDQ)
//
pub const mmTPC1_CMDQ_GLBL_CFG0: c_uint = 0xE49000;
pub const mmTPC1_CMDQ_GLBL_CFG1: c_uint = 0xE49004;
pub const mmTPC1_CMDQ_GLBL_PROT: c_uint = 0xE49008;
pub const mmTPC1_CMDQ_GLBL_ERR_CFG: c_uint = 0xE4900C;
pub const mmTPC1_CMDQ_GLBL_ERR_ADDR_LO: c_uint = 0xE49010;
pub const mmTPC1_CMDQ_GLBL_ERR_ADDR_HI: c_uint = 0xE49014;
pub const mmTPC1_CMDQ_GLBL_ERR_WDATA: c_uint = 0xE49018;
pub const mmTPC1_CMDQ_GLBL_SECURE_PROPS: c_uint = 0xE4901C;
pub const mmTPC1_CMDQ_GLBL_NON_SECURE_PROPS: c_uint = 0xE49020;
pub const mmTPC1_CMDQ_GLBL_STS0: c_uint = 0xE49024;
pub const mmTPC1_CMDQ_GLBL_STS1: c_uint = 0xE49028;
pub const mmTPC1_CMDQ_CQ_CFG0: c_uint = 0xE490B0;
pub const mmTPC1_CMDQ_CQ_CFG1: c_uint = 0xE490B4;
pub const mmTPC1_CMDQ_CQ_ARUSER: c_uint = 0xE490B8;
pub const mmTPC1_CMDQ_CQ_PTR_LO: c_uint = 0xE490C0;
pub const mmTPC1_CMDQ_CQ_PTR_HI: c_uint = 0xE490C4;
pub const mmTPC1_CMDQ_CQ_TSIZE: c_uint = 0xE490C8;
pub const mmTPC1_CMDQ_CQ_CTL: c_uint = 0xE490CC;
pub const mmTPC1_CMDQ_CQ_PTR_LO_STS: c_uint = 0xE490D4;
pub const mmTPC1_CMDQ_CQ_PTR_HI_STS: c_uint = 0xE490D8;
pub const mmTPC1_CMDQ_CQ_TSIZE_STS: c_uint = 0xE490DC;
pub const mmTPC1_CMDQ_CQ_CTL_STS: c_uint = 0xE490E0;
pub const mmTPC1_CMDQ_CQ_STS0: c_uint = 0xE490E4;
pub const mmTPC1_CMDQ_CQ_STS1: c_uint = 0xE490E8;
pub const mmTPC1_CMDQ_CQ_RD_RATE_LIM_EN: c_uint = 0xE490F0;
pub const mmTPC1_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0xE490F4;
pub const mmTPC1_CMDQ_CQ_RD_RATE_LIM_SAT: c_uint = 0xE490F8;
pub const mmTPC1_CMDQ_CQ_RD_RATE_LIM_TOUT: c_uint = 0xE490FC;
pub const mmTPC1_CMDQ_CQ_IFIFO_CNT: c_uint = 0xE49108;
pub const mmTPC1_CMDQ_CP_MSG_BASE0_ADDR_LO: c_uint = 0xE49120;
pub const mmTPC1_CMDQ_CP_MSG_BASE0_ADDR_HI: c_uint = 0xE49124;
pub const mmTPC1_CMDQ_CP_MSG_BASE1_ADDR_LO: c_uint = 0xE49128;
pub const mmTPC1_CMDQ_CP_MSG_BASE1_ADDR_HI: c_uint = 0xE4912C;
pub const mmTPC1_CMDQ_CP_MSG_BASE2_ADDR_LO: c_uint = 0xE49130;
pub const mmTPC1_CMDQ_CP_MSG_BASE2_ADDR_HI: c_uint = 0xE49134;
pub const mmTPC1_CMDQ_CP_MSG_BASE3_ADDR_LO: c_uint = 0xE49138;
pub const mmTPC1_CMDQ_CP_MSG_BASE3_ADDR_HI: c_uint = 0xE4913C;
pub const mmTPC1_CMDQ_CP_LDMA_TSIZE_OFFSET: c_uint = 0xE49140;
pub const mmTPC1_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: c_uint = 0xE49144;
pub const mmTPC1_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: c_uint = 0xE49148;
pub const mmTPC1_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: c_uint = 0xE4914C;
pub const mmTPC1_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: c_uint = 0xE49150;
pub const mmTPC1_CMDQ_CP_LDMA_COMMIT_OFFSET: c_uint = 0xE49154;
pub const mmTPC1_CMDQ_CP_FENCE0_RDATA: c_uint = 0xE49158;
pub const mmTPC1_CMDQ_CP_FENCE1_RDATA: c_uint = 0xE4915C;
pub const mmTPC1_CMDQ_CP_FENCE2_RDATA: c_uint = 0xE49160;
pub const mmTPC1_CMDQ_CP_FENCE3_RDATA: c_uint = 0xE49164;
pub const mmTPC1_CMDQ_CP_FENCE0_CNT: c_uint = 0xE49168;
pub const mmTPC1_CMDQ_CP_FENCE1_CNT: c_uint = 0xE4916C;
pub const mmTPC1_CMDQ_CP_FENCE2_CNT: c_uint = 0xE49170;
pub const mmTPC1_CMDQ_CP_FENCE3_CNT: c_uint = 0xE49174;
pub const mmTPC1_CMDQ_CP_STS: c_uint = 0xE49178;
pub const mmTPC1_CMDQ_CP_CURRENT_INST_LO: c_uint = 0xE4917C;
pub const mmTPC1_CMDQ_CP_CURRENT_INST_HI: c_uint = 0xE49180;
pub const mmTPC1_CMDQ_CP_BARRIER_CFG: c_uint = 0xE49184;
pub const mmTPC1_CMDQ_CP_DBG_0: c_uint = 0xE49188;
pub const mmTPC1_CMDQ_CQ_BUF_ADDR: c_uint = 0xE49308;
pub const mmTPC1_CMDQ_CQ_BUF_RDATA: c_uint = 0xE4930C;
