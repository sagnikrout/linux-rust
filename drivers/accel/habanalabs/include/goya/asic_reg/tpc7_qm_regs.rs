//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc7_qm_regs.h
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
// TPC7_QM (Prototype: QMAN)
//
pub const mmTPC7_QM_GLBL_CFG0: c_uint = 0xFC8000;
pub const mmTPC7_QM_GLBL_CFG1: c_uint = 0xFC8004;
pub const mmTPC7_QM_GLBL_PROT: c_uint = 0xFC8008;
pub const mmTPC7_QM_GLBL_ERR_CFG: c_uint = 0xFC800C;
pub const mmTPC7_QM_GLBL_ERR_ADDR_LO: c_uint = 0xFC8010;
pub const mmTPC7_QM_GLBL_ERR_ADDR_HI: c_uint = 0xFC8014;
pub const mmTPC7_QM_GLBL_ERR_WDATA: c_uint = 0xFC8018;
pub const mmTPC7_QM_GLBL_SECURE_PROPS: c_uint = 0xFC801C;
pub const mmTPC7_QM_GLBL_NON_SECURE_PROPS: c_uint = 0xFC8020;
pub const mmTPC7_QM_GLBL_STS0: c_uint = 0xFC8024;
pub const mmTPC7_QM_GLBL_STS1: c_uint = 0xFC8028;
pub const mmTPC7_QM_PQ_BASE_LO: c_uint = 0xFC8060;
pub const mmTPC7_QM_PQ_BASE_HI: c_uint = 0xFC8064;
pub const mmTPC7_QM_PQ_SIZE: c_uint = 0xFC8068;
pub const mmTPC7_QM_PQ_PI: c_uint = 0xFC806C;
pub const mmTPC7_QM_PQ_CI: c_uint = 0xFC8070;
pub const mmTPC7_QM_PQ_CFG0: c_uint = 0xFC8074;
pub const mmTPC7_QM_PQ_CFG1: c_uint = 0xFC8078;
pub const mmTPC7_QM_PQ_ARUSER: c_uint = 0xFC807C;
pub const mmTPC7_QM_PQ_PUSH0: c_uint = 0xFC8080;
pub const mmTPC7_QM_PQ_PUSH1: c_uint = 0xFC8084;
pub const mmTPC7_QM_PQ_PUSH2: c_uint = 0xFC8088;
pub const mmTPC7_QM_PQ_PUSH3: c_uint = 0xFC808C;
pub const mmTPC7_QM_PQ_STS0: c_uint = 0xFC8090;
pub const mmTPC7_QM_PQ_STS1: c_uint = 0xFC8094;
pub const mmTPC7_QM_PQ_RD_RATE_LIM_EN: c_uint = 0xFC80A0;
pub const mmTPC7_QM_PQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0xFC80A4;
pub const mmTPC7_QM_PQ_RD_RATE_LIM_SAT: c_uint = 0xFC80A8;
pub const mmTPC7_QM_PQ_RD_RATE_LIM_TOUT: c_uint = 0xFC80AC;
pub const mmTPC7_QM_CQ_CFG0: c_uint = 0xFC80B0;
pub const mmTPC7_QM_CQ_CFG1: c_uint = 0xFC80B4;
pub const mmTPC7_QM_CQ_ARUSER: c_uint = 0xFC80B8;
pub const mmTPC7_QM_CQ_PTR_LO: c_uint = 0xFC80C0;
pub const mmTPC7_QM_CQ_PTR_HI: c_uint = 0xFC80C4;
pub const mmTPC7_QM_CQ_TSIZE: c_uint = 0xFC80C8;
pub const mmTPC7_QM_CQ_CTL: c_uint = 0xFC80CC;
pub const mmTPC7_QM_CQ_PTR_LO_STS: c_uint = 0xFC80D4;
pub const mmTPC7_QM_CQ_PTR_HI_STS: c_uint = 0xFC80D8;
pub const mmTPC7_QM_CQ_TSIZE_STS: c_uint = 0xFC80DC;
pub const mmTPC7_QM_CQ_CTL_STS: c_uint = 0xFC80E0;
pub const mmTPC7_QM_CQ_STS0: c_uint = 0xFC80E4;
pub const mmTPC7_QM_CQ_STS1: c_uint = 0xFC80E8;
pub const mmTPC7_QM_CQ_RD_RATE_LIM_EN: c_uint = 0xFC80F0;
pub const mmTPC7_QM_CQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0xFC80F4;
pub const mmTPC7_QM_CQ_RD_RATE_LIM_SAT: c_uint = 0xFC80F8;
pub const mmTPC7_QM_CQ_RD_RATE_LIM_TOUT: c_uint = 0xFC80FC;
pub const mmTPC7_QM_CQ_IFIFO_CNT: c_uint = 0xFC8108;
pub const mmTPC7_QM_CP_MSG_BASE0_ADDR_LO: c_uint = 0xFC8120;
pub const mmTPC7_QM_CP_MSG_BASE0_ADDR_HI: c_uint = 0xFC8124;
pub const mmTPC7_QM_CP_MSG_BASE1_ADDR_LO: c_uint = 0xFC8128;
pub const mmTPC7_QM_CP_MSG_BASE1_ADDR_HI: c_uint = 0xFC812C;
pub const mmTPC7_QM_CP_MSG_BASE2_ADDR_LO: c_uint = 0xFC8130;
pub const mmTPC7_QM_CP_MSG_BASE2_ADDR_HI: c_uint = 0xFC8134;
pub const mmTPC7_QM_CP_MSG_BASE3_ADDR_LO: c_uint = 0xFC8138;
pub const mmTPC7_QM_CP_MSG_BASE3_ADDR_HI: c_uint = 0xFC813C;
pub const mmTPC7_QM_CP_LDMA_TSIZE_OFFSET: c_uint = 0xFC8140;
pub const mmTPC7_QM_CP_LDMA_SRC_BASE_LO_OFFSET: c_uint = 0xFC8144;
pub const mmTPC7_QM_CP_LDMA_SRC_BASE_HI_OFFSET: c_uint = 0xFC8148;
pub const mmTPC7_QM_CP_LDMA_DST_BASE_LO_OFFSET: c_uint = 0xFC814C;
pub const mmTPC7_QM_CP_LDMA_DST_BASE_HI_OFFSET: c_uint = 0xFC8150;
pub const mmTPC7_QM_CP_LDMA_COMMIT_OFFSET: c_uint = 0xFC8154;
pub const mmTPC7_QM_CP_FENCE0_RDATA: c_uint = 0xFC8158;
pub const mmTPC7_QM_CP_FENCE1_RDATA: c_uint = 0xFC815C;
pub const mmTPC7_QM_CP_FENCE2_RDATA: c_uint = 0xFC8160;
pub const mmTPC7_QM_CP_FENCE3_RDATA: c_uint = 0xFC8164;
pub const mmTPC7_QM_CP_FENCE0_CNT: c_uint = 0xFC8168;
pub const mmTPC7_QM_CP_FENCE1_CNT: c_uint = 0xFC816C;
pub const mmTPC7_QM_CP_FENCE2_CNT: c_uint = 0xFC8170;
pub const mmTPC7_QM_CP_FENCE3_CNT: c_uint = 0xFC8174;
pub const mmTPC7_QM_CP_STS: c_uint = 0xFC8178;
pub const mmTPC7_QM_CP_CURRENT_INST_LO: c_uint = 0xFC817C;
pub const mmTPC7_QM_CP_CURRENT_INST_HI: c_uint = 0xFC8180;
pub const mmTPC7_QM_CP_BARRIER_CFG: c_uint = 0xFC8184;
pub const mmTPC7_QM_CP_DBG_0: c_uint = 0xFC8188;
pub const mmTPC7_QM_PQ_BUF_ADDR: c_uint = 0xFC8300;
pub const mmTPC7_QM_PQ_BUF_RDATA: c_uint = 0xFC8304;
pub const mmTPC7_QM_CQ_BUF_ADDR: c_uint = 0xFC8308;
pub const mmTPC7_QM_CQ_BUF_RDATA: c_uint = 0xFC830C;
