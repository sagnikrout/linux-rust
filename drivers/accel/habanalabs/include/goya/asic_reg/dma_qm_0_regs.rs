//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/dma_qm_0_regs.h
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
// DMA_QM_0 (Prototype: QMAN)
//
pub const mmDMA_QM_0_GLBL_CFG0: c_uint = 0x400000;
pub const mmDMA_QM_0_GLBL_CFG1: c_uint = 0x400004;
pub const mmDMA_QM_0_GLBL_PROT: c_uint = 0x400008;
pub const mmDMA_QM_0_GLBL_ERR_CFG: c_uint = 0x40000C;
pub const mmDMA_QM_0_GLBL_ERR_ADDR_LO: c_uint = 0x400010;
pub const mmDMA_QM_0_GLBL_ERR_ADDR_HI: c_uint = 0x400014;
pub const mmDMA_QM_0_GLBL_ERR_WDATA: c_uint = 0x400018;
pub const mmDMA_QM_0_GLBL_SECURE_PROPS: c_uint = 0x40001C;
pub const mmDMA_QM_0_GLBL_NON_SECURE_PROPS: c_uint = 0x400020;
pub const mmDMA_QM_0_GLBL_STS0: c_uint = 0x400024;
pub const mmDMA_QM_0_GLBL_STS1: c_uint = 0x400028;
pub const mmDMA_QM_0_PQ_BASE_LO: c_uint = 0x400060;
pub const mmDMA_QM_0_PQ_BASE_HI: c_uint = 0x400064;
pub const mmDMA_QM_0_PQ_SIZE: c_uint = 0x400068;
pub const mmDMA_QM_0_PQ_PI: c_uint = 0x40006C;
pub const mmDMA_QM_0_PQ_CI: c_uint = 0x400070;
pub const mmDMA_QM_0_PQ_CFG0: c_uint = 0x400074;
pub const mmDMA_QM_0_PQ_CFG1: c_uint = 0x400078;
pub const mmDMA_QM_0_PQ_ARUSER: c_uint = 0x40007C;
pub const mmDMA_QM_0_PQ_PUSH0: c_uint = 0x400080;
pub const mmDMA_QM_0_PQ_PUSH1: c_uint = 0x400084;
pub const mmDMA_QM_0_PQ_PUSH2: c_uint = 0x400088;
pub const mmDMA_QM_0_PQ_PUSH3: c_uint = 0x40008C;
pub const mmDMA_QM_0_PQ_STS0: c_uint = 0x400090;
pub const mmDMA_QM_0_PQ_STS1: c_uint = 0x400094;
pub const mmDMA_QM_0_PQ_RD_RATE_LIM_EN: c_uint = 0x4000A0;
pub const mmDMA_QM_0_PQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0x4000A4;
pub const mmDMA_QM_0_PQ_RD_RATE_LIM_SAT: c_uint = 0x4000A8;
pub const mmDMA_QM_0_PQ_RD_RATE_LIM_TOUT: c_uint = 0x4000AC;
pub const mmDMA_QM_0_CQ_CFG0: c_uint = 0x4000B0;
pub const mmDMA_QM_0_CQ_CFG1: c_uint = 0x4000B4;
pub const mmDMA_QM_0_CQ_ARUSER: c_uint = 0x4000B8;
pub const mmDMA_QM_0_CQ_PTR_LO: c_uint = 0x4000C0;
pub const mmDMA_QM_0_CQ_PTR_HI: c_uint = 0x4000C4;
pub const mmDMA_QM_0_CQ_TSIZE: c_uint = 0x4000C8;
pub const mmDMA_QM_0_CQ_CTL: c_uint = 0x4000CC;
pub const mmDMA_QM_0_CQ_PTR_LO_STS: c_uint = 0x4000D4;
pub const mmDMA_QM_0_CQ_PTR_HI_STS: c_uint = 0x4000D8;
pub const mmDMA_QM_0_CQ_TSIZE_STS: c_uint = 0x4000DC;
pub const mmDMA_QM_0_CQ_CTL_STS: c_uint = 0x4000E0;
pub const mmDMA_QM_0_CQ_STS0: c_uint = 0x4000E4;
pub const mmDMA_QM_0_CQ_STS1: c_uint = 0x4000E8;
pub const mmDMA_QM_0_CQ_RD_RATE_LIM_EN: c_uint = 0x4000F0;
pub const mmDMA_QM_0_CQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0x4000F4;
pub const mmDMA_QM_0_CQ_RD_RATE_LIM_SAT: c_uint = 0x4000F8;
pub const mmDMA_QM_0_CQ_RD_RATE_LIM_TOUT: c_uint = 0x4000FC;
pub const mmDMA_QM_0_CQ_IFIFO_CNT: c_uint = 0x400108;
pub const mmDMA_QM_0_CP_MSG_BASE0_ADDR_LO: c_uint = 0x400120;
pub const mmDMA_QM_0_CP_MSG_BASE0_ADDR_HI: c_uint = 0x400124;
pub const mmDMA_QM_0_CP_MSG_BASE1_ADDR_LO: c_uint = 0x400128;
pub const mmDMA_QM_0_CP_MSG_BASE1_ADDR_HI: c_uint = 0x40012C;
pub const mmDMA_QM_0_CP_MSG_BASE2_ADDR_LO: c_uint = 0x400130;
pub const mmDMA_QM_0_CP_MSG_BASE2_ADDR_HI: c_uint = 0x400134;
pub const mmDMA_QM_0_CP_MSG_BASE3_ADDR_LO: c_uint = 0x400138;
pub const mmDMA_QM_0_CP_MSG_BASE3_ADDR_HI: c_uint = 0x40013C;
pub const mmDMA_QM_0_CP_LDMA_TSIZE_OFFSET: c_uint = 0x400140;
pub const mmDMA_QM_0_CP_LDMA_SRC_BASE_LO_OFFSET: c_uint = 0x400144;
pub const mmDMA_QM_0_CP_LDMA_SRC_BASE_HI_OFFSET: c_uint = 0x400148;
pub const mmDMA_QM_0_CP_LDMA_DST_BASE_LO_OFFSET: c_uint = 0x40014C;
pub const mmDMA_QM_0_CP_LDMA_DST_BASE_HI_OFFSET: c_uint = 0x400150;
pub const mmDMA_QM_0_CP_LDMA_COMMIT_OFFSET: c_uint = 0x400154;
pub const mmDMA_QM_0_CP_FENCE0_RDATA: c_uint = 0x400158;
pub const mmDMA_QM_0_CP_FENCE1_RDATA: c_uint = 0x40015C;
pub const mmDMA_QM_0_CP_FENCE2_RDATA: c_uint = 0x400160;
pub const mmDMA_QM_0_CP_FENCE3_RDATA: c_uint = 0x400164;
pub const mmDMA_QM_0_CP_FENCE0_CNT: c_uint = 0x400168;
pub const mmDMA_QM_0_CP_FENCE1_CNT: c_uint = 0x40016C;
pub const mmDMA_QM_0_CP_FENCE2_CNT: c_uint = 0x400170;
pub const mmDMA_QM_0_CP_FENCE3_CNT: c_uint = 0x400174;
pub const mmDMA_QM_0_CP_STS: c_uint = 0x400178;
pub const mmDMA_QM_0_CP_CURRENT_INST_LO: c_uint = 0x40017C;
pub const mmDMA_QM_0_CP_CURRENT_INST_HI: c_uint = 0x400180;
pub const mmDMA_QM_0_CP_BARRIER_CFG: c_uint = 0x400184;
pub const mmDMA_QM_0_CP_DBG_0: c_uint = 0x400188;
pub const mmDMA_QM_0_PQ_BUF_ADDR: c_uint = 0x400300;
pub const mmDMA_QM_0_PQ_BUF_RDATA: c_uint = 0x400304;
pub const mmDMA_QM_0_CQ_BUF_ADDR: c_uint = 0x400308;
pub const mmDMA_QM_0_CQ_BUF_RDATA: c_uint = 0x40030C;
