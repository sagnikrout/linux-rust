//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/mme_cmdq_regs.h
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
// MME_CMDQ (Prototype: CMDQ)
//
pub const mmMME_CMDQ_GLBL_CFG0: c_uint = 0xD9000;
pub const mmMME_CMDQ_GLBL_CFG1: c_uint = 0xD9004;
pub const mmMME_CMDQ_GLBL_PROT: c_uint = 0xD9008;
pub const mmMME_CMDQ_GLBL_ERR_CFG: c_uint = 0xD900C;
pub const mmMME_CMDQ_GLBL_ERR_ADDR_LO: c_uint = 0xD9010;
pub const mmMME_CMDQ_GLBL_ERR_ADDR_HI: c_uint = 0xD9014;
pub const mmMME_CMDQ_GLBL_ERR_WDATA: c_uint = 0xD9018;
pub const mmMME_CMDQ_GLBL_SECURE_PROPS: c_uint = 0xD901C;
pub const mmMME_CMDQ_GLBL_NON_SECURE_PROPS: c_uint = 0xD9020;
pub const mmMME_CMDQ_GLBL_STS0: c_uint = 0xD9024;
pub const mmMME_CMDQ_GLBL_STS1: c_uint = 0xD9028;
pub const mmMME_CMDQ_CQ_CFG0: c_uint = 0xD90B0;
pub const mmMME_CMDQ_CQ_CFG1: c_uint = 0xD90B4;
pub const mmMME_CMDQ_CQ_ARUSER: c_uint = 0xD90B8;
pub const mmMME_CMDQ_CQ_PTR_LO: c_uint = 0xD90C0;
pub const mmMME_CMDQ_CQ_PTR_HI: c_uint = 0xD90C4;
pub const mmMME_CMDQ_CQ_TSIZE: c_uint = 0xD90C8;
pub const mmMME_CMDQ_CQ_CTL: c_uint = 0xD90CC;
pub const mmMME_CMDQ_CQ_PTR_LO_STS: c_uint = 0xD90D4;
pub const mmMME_CMDQ_CQ_PTR_HI_STS: c_uint = 0xD90D8;
pub const mmMME_CMDQ_CQ_TSIZE_STS: c_uint = 0xD90DC;
pub const mmMME_CMDQ_CQ_CTL_STS: c_uint = 0xD90E0;
pub const mmMME_CMDQ_CQ_STS0: c_uint = 0xD90E4;
pub const mmMME_CMDQ_CQ_STS1: c_uint = 0xD90E8;
pub const mmMME_CMDQ_CQ_RD_RATE_LIM_EN: c_uint = 0xD90F0;
pub const mmMME_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0xD90F4;
pub const mmMME_CMDQ_CQ_RD_RATE_LIM_SAT: c_uint = 0xD90F8;
pub const mmMME_CMDQ_CQ_RD_RATE_LIM_TOUT: c_uint = 0xD90FC;
pub const mmMME_CMDQ_CQ_IFIFO_CNT: c_uint = 0xD9108;
pub const mmMME_CMDQ_CP_MSG_BASE0_ADDR_LO: c_uint = 0xD9120;
pub const mmMME_CMDQ_CP_MSG_BASE0_ADDR_HI: c_uint = 0xD9124;
pub const mmMME_CMDQ_CP_MSG_BASE1_ADDR_LO: c_uint = 0xD9128;
pub const mmMME_CMDQ_CP_MSG_BASE1_ADDR_HI: c_uint = 0xD912C;
pub const mmMME_CMDQ_CP_MSG_BASE2_ADDR_LO: c_uint = 0xD9130;
pub const mmMME_CMDQ_CP_MSG_BASE2_ADDR_HI: c_uint = 0xD9134;
pub const mmMME_CMDQ_CP_MSG_BASE3_ADDR_LO: c_uint = 0xD9138;
pub const mmMME_CMDQ_CP_MSG_BASE3_ADDR_HI: c_uint = 0xD913C;
pub const mmMME_CMDQ_CP_LDMA_TSIZE_OFFSET: c_uint = 0xD9140;
pub const mmMME_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: c_uint = 0xD9144;
pub const mmMME_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: c_uint = 0xD9148;
pub const mmMME_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: c_uint = 0xD914C;
pub const mmMME_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: c_uint = 0xD9150;
pub const mmMME_CMDQ_CP_LDMA_COMMIT_OFFSET: c_uint = 0xD9154;
pub const mmMME_CMDQ_CP_FENCE0_RDATA: c_uint = 0xD9158;
pub const mmMME_CMDQ_CP_FENCE1_RDATA: c_uint = 0xD915C;
pub const mmMME_CMDQ_CP_FENCE2_RDATA: c_uint = 0xD9160;
pub const mmMME_CMDQ_CP_FENCE3_RDATA: c_uint = 0xD9164;
pub const mmMME_CMDQ_CP_FENCE0_CNT: c_uint = 0xD9168;
pub const mmMME_CMDQ_CP_FENCE1_CNT: c_uint = 0xD916C;
pub const mmMME_CMDQ_CP_FENCE2_CNT: c_uint = 0xD9170;
pub const mmMME_CMDQ_CP_FENCE3_CNT: c_uint = 0xD9174;
pub const mmMME_CMDQ_CP_STS: c_uint = 0xD9178;
pub const mmMME_CMDQ_CP_CURRENT_INST_LO: c_uint = 0xD917C;
pub const mmMME_CMDQ_CP_CURRENT_INST_HI: c_uint = 0xD9180;
pub const mmMME_CMDQ_CP_BARRIER_CFG: c_uint = 0xD9184;
pub const mmMME_CMDQ_CP_DBG_0: c_uint = 0xD9188;
pub const mmMME_CMDQ_CQ_BUF_ADDR: c_uint = 0xD9308;
pub const mmMME_CMDQ_CQ_BUF_RDATA: c_uint = 0xD930C;
