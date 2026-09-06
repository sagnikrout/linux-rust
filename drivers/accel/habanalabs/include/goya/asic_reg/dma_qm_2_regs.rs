//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/dma_qm_2_regs.h
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
// DMA_QM_2 (Prototype: QMAN)
//
pub const mmDMA_QM_2_GLBL_CFG0: c_uint = 0x410000;
pub const mmDMA_QM_2_GLBL_CFG1: c_uint = 0x410004;
pub const mmDMA_QM_2_GLBL_PROT: c_uint = 0x410008;
pub const mmDMA_QM_2_GLBL_ERR_CFG: c_uint = 0x41000C;
pub const mmDMA_QM_2_GLBL_ERR_ADDR_LO: c_uint = 0x410010;
pub const mmDMA_QM_2_GLBL_ERR_ADDR_HI: c_uint = 0x410014;
pub const mmDMA_QM_2_GLBL_ERR_WDATA: c_uint = 0x410018;
pub const mmDMA_QM_2_GLBL_SECURE_PROPS: c_uint = 0x41001C;
pub const mmDMA_QM_2_GLBL_NON_SECURE_PROPS: c_uint = 0x410020;
pub const mmDMA_QM_2_GLBL_STS0: c_uint = 0x410024;
pub const mmDMA_QM_2_GLBL_STS1: c_uint = 0x410028;
pub const mmDMA_QM_2_PQ_BASE_LO: c_uint = 0x410060;
pub const mmDMA_QM_2_PQ_BASE_HI: c_uint = 0x410064;
pub const mmDMA_QM_2_PQ_SIZE: c_uint = 0x410068;
pub const mmDMA_QM_2_PQ_PI: c_uint = 0x41006C;
pub const mmDMA_QM_2_PQ_CI: c_uint = 0x410070;
pub const mmDMA_QM_2_PQ_CFG0: c_uint = 0x410074;
pub const mmDMA_QM_2_PQ_CFG1: c_uint = 0x410078;
pub const mmDMA_QM_2_PQ_ARUSER: c_uint = 0x41007C;
pub const mmDMA_QM_2_PQ_PUSH0: c_uint = 0x410080;
pub const mmDMA_QM_2_PQ_PUSH1: c_uint = 0x410084;
pub const mmDMA_QM_2_PQ_PUSH2: c_uint = 0x410088;
pub const mmDMA_QM_2_PQ_PUSH3: c_uint = 0x41008C;
pub const mmDMA_QM_2_PQ_STS0: c_uint = 0x410090;
pub const mmDMA_QM_2_PQ_STS1: c_uint = 0x410094;
pub const mmDMA_QM_2_PQ_RD_RATE_LIM_EN: c_uint = 0x4100A0;
pub const mmDMA_QM_2_PQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0x4100A4;
pub const mmDMA_QM_2_PQ_RD_RATE_LIM_SAT: c_uint = 0x4100A8;
pub const mmDMA_QM_2_PQ_RD_RATE_LIM_TOUT: c_uint = 0x4100AC;
pub const mmDMA_QM_2_CQ_CFG0: c_uint = 0x4100B0;
pub const mmDMA_QM_2_CQ_CFG1: c_uint = 0x4100B4;
pub const mmDMA_QM_2_CQ_ARUSER: c_uint = 0x4100B8;
pub const mmDMA_QM_2_CQ_PTR_LO: c_uint = 0x4100C0;
pub const mmDMA_QM_2_CQ_PTR_HI: c_uint = 0x4100C4;
pub const mmDMA_QM_2_CQ_TSIZE: c_uint = 0x4100C8;
pub const mmDMA_QM_2_CQ_CTL: c_uint = 0x4100CC;
pub const mmDMA_QM_2_CQ_PTR_LO_STS: c_uint = 0x4100D4;
pub const mmDMA_QM_2_CQ_PTR_HI_STS: c_uint = 0x4100D8;
pub const mmDMA_QM_2_CQ_TSIZE_STS: c_uint = 0x4100DC;
pub const mmDMA_QM_2_CQ_CTL_STS: c_uint = 0x4100E0;
pub const mmDMA_QM_2_CQ_STS0: c_uint = 0x4100E4;
pub const mmDMA_QM_2_CQ_STS1: c_uint = 0x4100E8;
pub const mmDMA_QM_2_CQ_RD_RATE_LIM_EN: c_uint = 0x4100F0;
pub const mmDMA_QM_2_CQ_RD_RATE_LIM_RST_TOKEN: c_uint = 0x4100F4;
pub const mmDMA_QM_2_CQ_RD_RATE_LIM_SAT: c_uint = 0x4100F8;
pub const mmDMA_QM_2_CQ_RD_RATE_LIM_TOUT: c_uint = 0x4100FC;
pub const mmDMA_QM_2_CQ_IFIFO_CNT: c_uint = 0x410108;
pub const mmDMA_QM_2_CP_MSG_BASE0_ADDR_LO: c_uint = 0x410120;
pub const mmDMA_QM_2_CP_MSG_BASE0_ADDR_HI: c_uint = 0x410124;
pub const mmDMA_QM_2_CP_MSG_BASE1_ADDR_LO: c_uint = 0x410128;
pub const mmDMA_QM_2_CP_MSG_BASE1_ADDR_HI: c_uint = 0x41012C;
pub const mmDMA_QM_2_CP_MSG_BASE2_ADDR_LO: c_uint = 0x410130;
pub const mmDMA_QM_2_CP_MSG_BASE2_ADDR_HI: c_uint = 0x410134;
pub const mmDMA_QM_2_CP_MSG_BASE3_ADDR_LO: c_uint = 0x410138;
pub const mmDMA_QM_2_CP_MSG_BASE3_ADDR_HI: c_uint = 0x41013C;
pub const mmDMA_QM_2_CP_LDMA_TSIZE_OFFSET: c_uint = 0x410140;
pub const mmDMA_QM_2_CP_LDMA_SRC_BASE_LO_OFFSET: c_uint = 0x410144;
pub const mmDMA_QM_2_CP_LDMA_SRC_BASE_HI_OFFSET: c_uint = 0x410148;
pub const mmDMA_QM_2_CP_LDMA_DST_BASE_LO_OFFSET: c_uint = 0x41014C;
pub const mmDMA_QM_2_CP_LDMA_DST_BASE_HI_OFFSET: c_uint = 0x410150;
pub const mmDMA_QM_2_CP_LDMA_COMMIT_OFFSET: c_uint = 0x410154;
pub const mmDMA_QM_2_CP_FENCE0_RDATA: c_uint = 0x410158;
pub const mmDMA_QM_2_CP_FENCE1_RDATA: c_uint = 0x41015C;
pub const mmDMA_QM_2_CP_FENCE2_RDATA: c_uint = 0x410160;
pub const mmDMA_QM_2_CP_FENCE3_RDATA: c_uint = 0x410164;
pub const mmDMA_QM_2_CP_FENCE0_CNT: c_uint = 0x410168;
pub const mmDMA_QM_2_CP_FENCE1_CNT: c_uint = 0x41016C;
pub const mmDMA_QM_2_CP_FENCE2_CNT: c_uint = 0x410170;
pub const mmDMA_QM_2_CP_FENCE3_CNT: c_uint = 0x410174;
pub const mmDMA_QM_2_CP_STS: c_uint = 0x410178;
pub const mmDMA_QM_2_CP_CURRENT_INST_LO: c_uint = 0x41017C;
pub const mmDMA_QM_2_CP_CURRENT_INST_HI: c_uint = 0x410180;
pub const mmDMA_QM_2_CP_BARRIER_CFG: c_uint = 0x410184;
pub const mmDMA_QM_2_CP_DBG_0: c_uint = 0x410188;
pub const mmDMA_QM_2_PQ_BUF_ADDR: c_uint = 0x410300;
pub const mmDMA_QM_2_PQ_BUF_RDATA: c_uint = 0x410304;
pub const mmDMA_QM_2_CQ_BUF_ADDR: c_uint = 0x410308;
pub const mmDMA_QM_2_CQ_BUF_RDATA: c_uint = 0x41030C;
