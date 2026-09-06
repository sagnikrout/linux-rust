//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc2_qm_regs.h
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
// TPC2_QM (Prototype: QMAN)
//
pub const mmTPC2_QM_GLBL_CFG0: c_uint = 0xE88000;
pub const mmTPC2_QM_GLBL_CFG1: c_uint = 0xE88004;
pub const mmTPC2_QM_GLBL_PROT: c_uint = 0xE88008;
pub const mmTPC2_QM_GLBL_ERR_CFG: c_uint = 0xE8800C;
pub const mmTPC2_QM_GLBL_ERR_ADDR_LO: c_uint = 0xE88010;
pub const mmTPC2_QM_GLBL_ERR_ADDR_HI: c_uint = 0xE88014;
pub const mmTPC2_QM_GLBL_ERR_WDATA: c_uint = 0xE88018;
pub const mmTPC2_QM_GLBL_SECURE_PROPS: c_uint = 0xE8801C;
pub const mmTPC2_QM_GLBL_NON_SECURE_PROPS: c_uint = 0xE88020;
pub const mmTPC2_QM_GLBL_STS0: c_uint = 0xE88024;
pub const mmTPC2_QM_GLBL_STS1: c_uint = 0xE88028;
pub const mmTPC2_QM_PQ_BASE_LO: c_uint = 0xE88060;
pub const mmTPC2_QM_PQ_BASE_HI: c_uint = 0xE88064;
pub const mmTPC2_QM_PQ_SIZE: c_uint = 0xE88068;
pub const mmTPC2_QM_PQ_PI: c_uint = 0xE8806C;
pub const mmTPC2_QM_PQ_CI: c_uint = 0xE88070;
pub const mmTPC2_QM_PQ_CFG0: c_uint = 0xE88074;
pub const mmTPC2_QM_PQ_CFG1: c_uint = 0xE88078;
pub const mmTPC2_QM_PQ_ARUSER: c_uint = 0xE8807C;
pub const mmTPC2_QM_PQ_PUSH0: c_uint = 0xE88080;
pub const mmTPC2_QM_PQ_PUSH1: c_uint = 0xE88084;
pub const mmTPC2_QM_PQ_PUSH2: c_uint = 0xE88088;
pub const mmTPC2_QM_PQ_PUSH3: c_uint = 0xE8808C;
pub const mmTPC2_QM_PQ_STS0: c_uint = 0xE88090;
pub const mmTPC2_QM_PQ_STS1: c_uint = 0xE88094;
pub const mmTPC2_QM_PQ_RD_RATE_LIM_EN: c_uint = 0xE880A0;
pub const mmTPC2_QM_PQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0xE880A4;
pub const mmTPC2_QM_PQ_RD_RATE_LIM_SAT: c_uint = 0xE880A8;
pub const mmTPC2_QM_PQ_RD_RATE_LIM_TOUT: c_uint = 0xE880AC;
pub const mmTPC2_QM_CQ_CFG0: c_uint = 0xE880B0;
pub const mmTPC2_QM_CQ_CFG1: c_uint = 0xE880B4;
pub const mmTPC2_QM_CQ_ARUSER: c_uint = 0xE880B8;
pub const mmTPC2_QM_CQ_PTR_LO: c_uint = 0xE880C0;
pub const mmTPC2_QM_CQ_PTR_HI: c_uint = 0xE880C4;
pub const mmTPC2_QM_CQ_TSIZE: c_uint = 0xE880C8;
pub const mmTPC2_QM_CQ_CTL: c_uint = 0xE880CC;
pub const mmTPC2_QM_CQ_PTR_LO_STS: c_uint = 0xE880D4;
pub const mmTPC2_QM_CQ_PTR_HI_STS: c_uint = 0xE880D8;
pub const mmTPC2_QM_CQ_TSIZE_STS: c_uint = 0xE880DC;
pub const mmTPC2_QM_CQ_CTL_STS: c_uint = 0xE880E0;
pub const mmTPC2_QM_CQ_STS0: c_uint = 0xE880E4;
pub const mmTPC2_QM_CQ_STS1: c_uint = 0xE880E8;
pub const mmTPC2_QM_CQ_RD_RATE_LIM_EN: c_uint = 0xE880F0;
pub const mmTPC2_QM_CQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0xE880F4;
pub const mmTPC2_QM_CQ_RD_RATE_LIM_SAT: c_uint = 0xE880F8;
pub const mmTPC2_QM_CQ_RD_RATE_LIM_TOUT: c_uint = 0xE880FC;
pub const mmTPC2_QM_CQ_IFIFO_CNT: c_uint = 0xE88108;
pub const mmTPC2_QM_CP_MSG_BASE0_ADDR_LO: c_uint = 0xE88120;
pub const mmTPC2_QM_CP_MSG_BASE0_ADDR_HI: c_uint = 0xE88124;
pub const mmTPC2_QM_CP_MSG_BASE1_ADDR_LO: c_uint = 0xE88128;
pub const mmTPC2_QM_CP_MSG_BASE1_ADDR_HI: c_uint = 0xE8812C;
pub const mmTPC2_QM_CP_MSG_BASE2_ADDR_LO: c_uint = 0xE88130;
pub const mmTPC2_QM_CP_MSG_BASE2_ADDR_HI: c_uint = 0xE88134;
pub const mmTPC2_QM_CP_MSG_BASE3_ADDR_LO: c_uint = 0xE88138;
pub const mmTPC2_QM_CP_MSG_BASE3_ADDR_HI: c_uint = 0xE8813C;
pub const mmTPC2_QM_CP_LDMA_TSIZE_OFFSET: c_uint = 0xE88140;
pub const mmTPC2_QM_CP_LDMA_SRC_BASE_LO_OFFSET: c_uint = 0xE88144;
pub const mmTPC2_QM_CP_LDMA_SRC_BASE_HI_OFFSET: c_uint = 0xE88148;
pub const mmTPC2_QM_CP_LDMA_DST_BASE_LO_OFFSET: c_uint = 0xE8814C;
pub const mmTPC2_QM_CP_LDMA_DST_BASE_HI_OFFSET: c_uint = 0xE88150;
pub const mmTPC2_QM_CP_LDMA_COMMIT_OFFSET: c_uint = 0xE88154;
pub const mmTPC2_QM_CP_FENCE0_RDATA: c_uint = 0xE88158;
pub const mmTPC2_QM_CP_FENCE1_RDATA: c_uint = 0xE8815C;
pub const mmTPC2_QM_CP_FENCE2_RDATA: c_uint = 0xE88160;
pub const mmTPC2_QM_CP_FENCE3_RDATA: c_uint = 0xE88164;
pub const mmTPC2_QM_CP_FENCE0_CNT: c_uint = 0xE88168;
pub const mmTPC2_QM_CP_FENCE1_CNT: c_uint = 0xE8816C;
pub const mmTPC2_QM_CP_FENCE2_CNT: c_uint = 0xE88170;
pub const mmTPC2_QM_CP_FENCE3_CNT: c_uint = 0xE88174;
pub const mmTPC2_QM_CP_STS: c_uint = 0xE88178;
pub const mmTPC2_QM_CP_CURRENT_INST_LO: c_uint = 0xE8817C;
pub const mmTPC2_QM_CP_CURRENT_INST_HI: c_uint = 0xE88180;
pub const mmTPC2_QM_CP_BARRIER_CFG: c_uint = 0xE88184;
pub const mmTPC2_QM_CP_DBG_0: c_uint = 0xE88188;
pub const mmTPC2_QM_PQ_BUF_ADDR: c_uint = 0xE88300;
pub const mmTPC2_QM_PQ_BUF_RDATA: c_uint = 0xE88304;
pub const mmTPC2_QM_CQ_BUF_ADDR: c_uint = 0xE88308;
pub const mmTPC2_QM_CQ_BUF_RDATA: c_uint = 0xE8830C;
