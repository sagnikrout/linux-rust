//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc4_qm_regs.h
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
// TPC4_QM (Prototype: QMAN)
//
pub const mmTPC4_QM_GLBL_CFG0: c_uint = 0xF08000;
pub const mmTPC4_QM_GLBL_CFG1: c_uint = 0xF08004;
pub const mmTPC4_QM_GLBL_PROT: c_uint = 0xF08008;
pub const mmTPC4_QM_GLBL_ERR_CFG: c_uint = 0xF0800C;
pub const mmTPC4_QM_GLBL_ERR_ADDR_LO: c_uint = 0xF08010;
pub const mmTPC4_QM_GLBL_ERR_ADDR_HI: c_uint = 0xF08014;
pub const mmTPC4_QM_GLBL_ERR_WDATA: c_uint = 0xF08018;
pub const mmTPC4_QM_GLBL_SECURE_PROPS: c_uint = 0xF0801C;
pub const mmTPC4_QM_GLBL_NON_SECURE_PROPS: c_uint = 0xF08020;
pub const mmTPC4_QM_GLBL_STS0: c_uint = 0xF08024;
pub const mmTPC4_QM_GLBL_STS1: c_uint = 0xF08028;
pub const mmTPC4_QM_PQ_BASE_LO: c_uint = 0xF08060;
pub const mmTPC4_QM_PQ_BASE_HI: c_uint = 0xF08064;
pub const mmTPC4_QM_PQ_SIZE: c_uint = 0xF08068;
pub const mmTPC4_QM_PQ_PI: c_uint = 0xF0806C;
pub const mmTPC4_QM_PQ_CI: c_uint = 0xF08070;
pub const mmTPC4_QM_PQ_CFG0: c_uint = 0xF08074;
pub const mmTPC4_QM_PQ_CFG1: c_uint = 0xF08078;
pub const mmTPC4_QM_PQ_ARUSER: c_uint = 0xF0807C;
pub const mmTPC4_QM_PQ_PUSH0: c_uint = 0xF08080;
pub const mmTPC4_QM_PQ_PUSH1: c_uint = 0xF08084;
pub const mmTPC4_QM_PQ_PUSH2: c_uint = 0xF08088;
pub const mmTPC4_QM_PQ_PUSH3: c_uint = 0xF0808C;
pub const mmTPC4_QM_PQ_STS0: c_uint = 0xF08090;
pub const mmTPC4_QM_PQ_STS1: c_uint = 0xF08094;
pub const mmTPC4_QM_PQ_RD_RATE_LIM_EN: c_uint = 0xF080A0;
pub const mmTPC4_QM_PQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0xF080A4;
pub const mmTPC4_QM_PQ_RD_RATE_LIM_SAT: c_uint = 0xF080A8;
pub const mmTPC4_QM_PQ_RD_RATE_LIM_TOUT: c_uint = 0xF080AC;
pub const mmTPC4_QM_CQ_CFG0: c_uint = 0xF080B0;
pub const mmTPC4_QM_CQ_CFG1: c_uint = 0xF080B4;
pub const mmTPC4_QM_CQ_ARUSER: c_uint = 0xF080B8;
pub const mmTPC4_QM_CQ_PTR_LO: c_uint = 0xF080C0;
pub const mmTPC4_QM_CQ_PTR_HI: c_uint = 0xF080C4;
pub const mmTPC4_QM_CQ_TSIZE: c_uint = 0xF080C8;
pub const mmTPC4_QM_CQ_CTL: c_uint = 0xF080CC;
pub const mmTPC4_QM_CQ_PTR_LO_STS: c_uint = 0xF080D4;
pub const mmTPC4_QM_CQ_PTR_HI_STS: c_uint = 0xF080D8;
pub const mmTPC4_QM_CQ_TSIZE_STS: c_uint = 0xF080DC;
pub const mmTPC4_QM_CQ_CTL_STS: c_uint = 0xF080E0;
pub const mmTPC4_QM_CQ_STS0: c_uint = 0xF080E4;
pub const mmTPC4_QM_CQ_STS1: c_uint = 0xF080E8;
pub const mmTPC4_QM_CQ_RD_RATE_LIM_EN: c_uint = 0xF080F0;
pub const mmTPC4_QM_CQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0xF080F4;
pub const mmTPC4_QM_CQ_RD_RATE_LIM_SAT: c_uint = 0xF080F8;
pub const mmTPC4_QM_CQ_RD_RATE_LIM_TOUT: c_uint = 0xF080FC;
pub const mmTPC4_QM_CQ_IFIFO_CNT: c_uint = 0xF08108;
pub const mmTPC4_QM_CP_MSG_BASE0_ADDR_LO: c_uint = 0xF08120;
pub const mmTPC4_QM_CP_MSG_BASE0_ADDR_HI: c_uint = 0xF08124;
pub const mmTPC4_QM_CP_MSG_BASE1_ADDR_LO: c_uint = 0xF08128;
pub const mmTPC4_QM_CP_MSG_BASE1_ADDR_HI: c_uint = 0xF0812C;
pub const mmTPC4_QM_CP_MSG_BASE2_ADDR_LO: c_uint = 0xF08130;
pub const mmTPC4_QM_CP_MSG_BASE2_ADDR_HI: c_uint = 0xF08134;
pub const mmTPC4_QM_CP_MSG_BASE3_ADDR_LO: c_uint = 0xF08138;
pub const mmTPC4_QM_CP_MSG_BASE3_ADDR_HI: c_uint = 0xF0813C;
pub const mmTPC4_QM_CP_LDMA_TSIZE_OFFSET: c_uint = 0xF08140;
pub const mmTPC4_QM_CP_LDMA_SRC_BASE_LO_OFFSET: c_uint = 0xF08144;
pub const mmTPC4_QM_CP_LDMA_SRC_BASE_HI_OFFSET: c_uint = 0xF08148;
pub const mmTPC4_QM_CP_LDMA_DST_BASE_LO_OFFSET: c_uint = 0xF0814C;
pub const mmTPC4_QM_CP_LDMA_DST_BASE_HI_OFFSET: c_uint = 0xF08150;
pub const mmTPC4_QM_CP_LDMA_COMMIT_OFFSET: c_uint = 0xF08154;
pub const mmTPC4_QM_CP_FENCE0_RDATA: c_uint = 0xF08158;
pub const mmTPC4_QM_CP_FENCE1_RDATA: c_uint = 0xF0815C;
pub const mmTPC4_QM_CP_FENCE2_RDATA: c_uint = 0xF08160;
pub const mmTPC4_QM_CP_FENCE3_RDATA: c_uint = 0xF08164;
pub const mmTPC4_QM_CP_FENCE0_CNT: c_uint = 0xF08168;
pub const mmTPC4_QM_CP_FENCE1_CNT: c_uint = 0xF0816C;
pub const mmTPC4_QM_CP_FENCE2_CNT: c_uint = 0xF08170;
pub const mmTPC4_QM_CP_FENCE3_CNT: c_uint = 0xF08174;
pub const mmTPC4_QM_CP_STS: c_uint = 0xF08178;
pub const mmTPC4_QM_CP_CURRENT_INST_LO: c_uint = 0xF0817C;
pub const mmTPC4_QM_CP_CURRENT_INST_HI: c_uint = 0xF08180;
pub const mmTPC4_QM_CP_BARRIER_CFG: c_uint = 0xF08184;
pub const mmTPC4_QM_CP_DBG_0: c_uint = 0xF08188;
pub const mmTPC4_QM_PQ_BUF_ADDR: c_uint = 0xF08300;
pub const mmTPC4_QM_PQ_BUF_RDATA: c_uint = 0xF08304;
pub const mmTPC4_QM_CQ_BUF_ADDR: c_uint = 0xF08308;
pub const mmTPC4_QM_CQ_BUF_RDATA: c_uint = 0xF0830C;
