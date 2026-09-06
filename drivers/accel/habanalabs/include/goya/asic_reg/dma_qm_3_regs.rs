//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/dma_qm_3_regs.h
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
// DMA_QM_3 (Prototype: QMAN)
//
pub const mmDMA_QM_3_GLBL_CFG0: c_uint = 0x418000;
pub const mmDMA_QM_3_GLBL_CFG1: c_uint = 0x418004;
pub const mmDMA_QM_3_GLBL_PROT: c_uint = 0x418008;
pub const mmDMA_QM_3_GLBL_ERR_CFG: c_uint = 0x41800C;
pub const mmDMA_QM_3_GLBL_ERR_ADDR_LO: c_uint = 0x418010;
pub const mmDMA_QM_3_GLBL_ERR_ADDR_HI: c_uint = 0x418014;
pub const mmDMA_QM_3_GLBL_ERR_WDATA: c_uint = 0x418018;
pub const mmDMA_QM_3_GLBL_SECURE_PROPS: c_uint = 0x41801C;
pub const mmDMA_QM_3_GLBL_NON_SECURE_PROPS: c_uint = 0x418020;
pub const mmDMA_QM_3_GLBL_STS0: c_uint = 0x418024;
pub const mmDMA_QM_3_GLBL_STS1: c_uint = 0x418028;
pub const mmDMA_QM_3_PQ_BASE_LO: c_uint = 0x418060;
pub const mmDMA_QM_3_PQ_BASE_HI: c_uint = 0x418064;
pub const mmDMA_QM_3_PQ_SIZE: c_uint = 0x418068;
pub const mmDMA_QM_3_PQ_PI: c_uint = 0x41806C;
pub const mmDMA_QM_3_PQ_CI: c_uint = 0x418070;
pub const mmDMA_QM_3_PQ_CFG0: c_uint = 0x418074;
pub const mmDMA_QM_3_PQ_CFG1: c_uint = 0x418078;
pub const mmDMA_QM_3_PQ_ARUSER: c_uint = 0x41807C;
pub const mmDMA_QM_3_PQ_PUSH0: c_uint = 0x418080;
pub const mmDMA_QM_3_PQ_PUSH1: c_uint = 0x418084;
pub const mmDMA_QM_3_PQ_PUSH2: c_uint = 0x418088;
pub const mmDMA_QM_3_PQ_PUSH3: c_uint = 0x41808C;
pub const mmDMA_QM_3_PQ_STS0: c_uint = 0x418090;
pub const mmDMA_QM_3_PQ_STS1: c_uint = 0x418094;
pub const mmDMA_QM_3_PQ_RD_RATE_LIM_EN: c_uint = 0x4180A0;
pub const mmDMA_QM_3_PQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0x4180A4;
pub const mmDMA_QM_3_PQ_RD_RATE_LIM_SAT: c_uint = 0x4180A8;
pub const mmDMA_QM_3_PQ_RD_RATE_LIM_TOUT: c_uint = 0x4180AC;
pub const mmDMA_QM_3_CQ_CFG0: c_uint = 0x4180B0;
pub const mmDMA_QM_3_CQ_CFG1: c_uint = 0x4180B4;
pub const mmDMA_QM_3_CQ_ARUSER: c_uint = 0x4180B8;
pub const mmDMA_QM_3_CQ_PTR_LO: c_uint = 0x4180C0;
pub const mmDMA_QM_3_CQ_PTR_HI: c_uint = 0x4180C4;
pub const mmDMA_QM_3_CQ_TSIZE: c_uint = 0x4180C8;
pub const mmDMA_QM_3_CQ_CTL: c_uint = 0x4180CC;
pub const mmDMA_QM_3_CQ_PTR_LO_STS: c_uint = 0x4180D4;
pub const mmDMA_QM_3_CQ_PTR_HI_STS: c_uint = 0x4180D8;
pub const mmDMA_QM_3_CQ_TSIZE_STS: c_uint = 0x4180DC;
pub const mmDMA_QM_3_CQ_CTL_STS: c_uint = 0x4180E0;
pub const mmDMA_QM_3_CQ_STS0: c_uint = 0x4180E4;
pub const mmDMA_QM_3_CQ_STS1: c_uint = 0x4180E8;
pub const mmDMA_QM_3_CQ_RD_RATE_LIM_EN: c_uint = 0x4180F0;
pub const mmDMA_QM_3_CQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0x4180F4;
pub const mmDMA_QM_3_CQ_RD_RATE_LIM_SAT: c_uint = 0x4180F8;
pub const mmDMA_QM_3_CQ_RD_RATE_LIM_TOUT: c_uint = 0x4180FC;
pub const mmDMA_QM_3_CQ_IFIFO_CNT: c_uint = 0x418108;
pub const mmDMA_QM_3_CP_MSG_BASE0_ADDR_LO: c_uint = 0x418120;
pub const mmDMA_QM_3_CP_MSG_BASE0_ADDR_HI: c_uint = 0x418124;
pub const mmDMA_QM_3_CP_MSG_BASE1_ADDR_LO: c_uint = 0x418128;
pub const mmDMA_QM_3_CP_MSG_BASE1_ADDR_HI: c_uint = 0x41812C;
pub const mmDMA_QM_3_CP_MSG_BASE2_ADDR_LO: c_uint = 0x418130;
pub const mmDMA_QM_3_CP_MSG_BASE2_ADDR_HI: c_uint = 0x418134;
pub const mmDMA_QM_3_CP_MSG_BASE3_ADDR_LO: c_uint = 0x418138;
pub const mmDMA_QM_3_CP_MSG_BASE3_ADDR_HI: c_uint = 0x41813C;
pub const mmDMA_QM_3_CP_LDMA_TSIZE_OFFSET: c_uint = 0x418140;
pub const mmDMA_QM_3_CP_LDMA_SRC_BASE_LO_OFFSET: c_uint = 0x418144;
pub const mmDMA_QM_3_CP_LDMA_SRC_BASE_HI_OFFSET: c_uint = 0x418148;
pub const mmDMA_QM_3_CP_LDMA_DST_BASE_LO_OFFSET: c_uint = 0x41814C;
pub const mmDMA_QM_3_CP_LDMA_DST_BASE_HI_OFFSET: c_uint = 0x418150;
pub const mmDMA_QM_3_CP_LDMA_COMMIT_OFFSET: c_uint = 0x418154;
pub const mmDMA_QM_3_CP_FENCE0_RDATA: c_uint = 0x418158;
pub const mmDMA_QM_3_CP_FENCE1_RDATA: c_uint = 0x41815C;
pub const mmDMA_QM_3_CP_FENCE2_RDATA: c_uint = 0x418160;
pub const mmDMA_QM_3_CP_FENCE3_RDATA: c_uint = 0x418164;
pub const mmDMA_QM_3_CP_FENCE0_CNT: c_uint = 0x418168;
pub const mmDMA_QM_3_CP_FENCE1_CNT: c_uint = 0x41816C;
pub const mmDMA_QM_3_CP_FENCE2_CNT: c_uint = 0x418170;
pub const mmDMA_QM_3_CP_FENCE3_CNT: c_uint = 0x418174;
pub const mmDMA_QM_3_CP_STS: c_uint = 0x418178;
pub const mmDMA_QM_3_CP_CURRENT_INST_LO: c_uint = 0x41817C;
pub const mmDMA_QM_3_CP_CURRENT_INST_HI: c_uint = 0x418180;
pub const mmDMA_QM_3_CP_BARRIER_CFG: c_uint = 0x418184;
pub const mmDMA_QM_3_CP_DBG_0: c_uint = 0x418188;
pub const mmDMA_QM_3_PQ_BUF_ADDR: c_uint = 0x418300;
pub const mmDMA_QM_3_PQ_BUF_RDATA: c_uint = 0x418304;
pub const mmDMA_QM_3_CQ_BUF_ADDR: c_uint = 0x418308;
pub const mmDMA_QM_3_CQ_BUF_RDATA: c_uint = 0x41830C;
