//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/dma_ch_4_regs.h
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
// DMA_CH_4 (Prototype: DMA_CH)
//
pub const mmDMA_CH_4_CFG0: c_uint = 0x421000;
pub const mmDMA_CH_4_CFG1: c_uint = 0x421004;
pub const mmDMA_CH_4_ERRMSG_ADDR_LO: c_uint = 0x421008;
pub const mmDMA_CH_4_ERRMSG_ADDR_HI: c_uint = 0x42100C;
pub const mmDMA_CH_4_ERRMSG_WDATA: c_uint = 0x421010;
pub const mmDMA_CH_4_RD_COMP_ADDR_LO: c_uint = 0x421014;
pub const mmDMA_CH_4_RD_COMP_ADDR_HI: c_uint = 0x421018;
pub const mmDMA_CH_4_RD_COMP_WDATA: c_uint = 0x42101C;
pub const mmDMA_CH_4_WR_COMP_ADDR_LO: c_uint = 0x421020;
pub const mmDMA_CH_4_WR_COMP_ADDR_HI: c_uint = 0x421024;
pub const mmDMA_CH_4_WR_COMP_WDATA: c_uint = 0x421028;
pub const mmDMA_CH_4_LDMA_SRC_ADDR_LO: c_uint = 0x42102C;
pub const mmDMA_CH_4_LDMA_SRC_ADDR_HI: c_uint = 0x421030;
pub const mmDMA_CH_4_LDMA_DST_ADDR_LO: c_uint = 0x421034;
pub const mmDMA_CH_4_LDMA_DST_ADDR_HI: c_uint = 0x421038;
pub const mmDMA_CH_4_LDMA_TSIZE: c_uint = 0x42103C;
pub const mmDMA_CH_4_COMIT_TRANSFER: c_uint = 0x421040;
pub const mmDMA_CH_4_STS0: c_uint = 0x421044;
pub const mmDMA_CH_4_STS1: c_uint = 0x421048;
pub const mmDMA_CH_4_STS2: c_uint = 0x42104C;
pub const mmDMA_CH_4_STS3: c_uint = 0x421050;
pub const mmDMA_CH_4_STS4: c_uint = 0x421054;
pub const mmDMA_CH_4_SRC_ADDR_LO_STS: c_uint = 0x421058;
pub const mmDMA_CH_4_SRC_ADDR_HI_STS: c_uint = 0x42105C;
pub const mmDMA_CH_4_SRC_TSIZE_STS: c_uint = 0x421060;
pub const mmDMA_CH_4_DST_ADDR_LO_STS: c_uint = 0x421064;
pub const mmDMA_CH_4_DST_ADDR_HI_STS: c_uint = 0x421068;
pub const mmDMA_CH_4_DST_TSIZE_STS: c_uint = 0x42106C;
pub const mmDMA_CH_4_RD_RATE_LIM_EN: c_uint = 0x421070;
pub const mmDMA_CH_4_RD_RATE_LIM_RST_TOKEN: c_uint = 0x421074;
pub const mmDMA_CH_4_RD_RATE_LIM_SAT: c_uint = 0x421078;
pub const mmDMA_CH_4_RD_RATE_LIM_TOUT: c_uint = 0x42107C;
pub const mmDMA_CH_4_WR_RATE_LIM_EN: c_uint = 0x421080;
pub const mmDMA_CH_4_WR_RATE_LIM_RST_TOKEN: c_uint = 0x421084;
pub const mmDMA_CH_4_WR_RATE_LIM_SAT: c_uint = 0x421088;
pub const mmDMA_CH_4_WR_RATE_LIM_TOUT: c_uint = 0x42108C;
pub const mmDMA_CH_4_CFG2: c_uint = 0x421090;
pub const mmDMA_CH_4_TDMA_CTL: c_uint = 0x421100;
pub const mmDMA_CH_4_TDMA_SRC_BASE_ADDR_LO: c_uint = 0x421104;
pub const mmDMA_CH_4_TDMA_SRC_BASE_ADDR_HI: c_uint = 0x421108;
pub const mmDMA_CH_4_TDMA_SRC_ROI_BASE_0: c_uint = 0x42110C;
pub const mmDMA_CH_4_TDMA_SRC_ROI_SIZE_0: c_uint = 0x421110;
pub const mmDMA_CH_4_TDMA_SRC_VALID_ELEMENTS_0: c_uint = 0x421114;
pub const mmDMA_CH_4_TDMA_SRC_START_OFFSET_0: c_uint = 0x421118;
pub const mmDMA_CH_4_TDMA_SRC_STRIDE_0: c_uint = 0x42111C;
pub const mmDMA_CH_4_TDMA_SRC_ROI_BASE_1: c_uint = 0x421120;
pub const mmDMA_CH_4_TDMA_SRC_ROI_SIZE_1: c_uint = 0x421124;
pub const mmDMA_CH_4_TDMA_SRC_VALID_ELEMENTS_1: c_uint = 0x421128;
pub const mmDMA_CH_4_TDMA_SRC_START_OFFSET_1: c_uint = 0x42112C;
pub const mmDMA_CH_4_TDMA_SRC_STRIDE_1: c_uint = 0x421130;
pub const mmDMA_CH_4_TDMA_SRC_ROI_BASE_2: c_uint = 0x421134;
pub const mmDMA_CH_4_TDMA_SRC_ROI_SIZE_2: c_uint = 0x421138;
pub const mmDMA_CH_4_TDMA_SRC_VALID_ELEMENTS_2: c_uint = 0x42113C;
pub const mmDMA_CH_4_TDMA_SRC_START_OFFSET_2: c_uint = 0x421140;
pub const mmDMA_CH_4_TDMA_SRC_STRIDE_2: c_uint = 0x421144;
pub const mmDMA_CH_4_TDMA_SRC_ROI_BASE_3: c_uint = 0x421148;
pub const mmDMA_CH_4_TDMA_SRC_ROI_SIZE_3: c_uint = 0x42114C;
pub const mmDMA_CH_4_TDMA_SRC_VALID_ELEMENTS_3: c_uint = 0x421150;
pub const mmDMA_CH_4_TDMA_SRC_START_OFFSET_3: c_uint = 0x421154;
pub const mmDMA_CH_4_TDMA_SRC_STRIDE_3: c_uint = 0x421158;
pub const mmDMA_CH_4_TDMA_SRC_ROI_BASE_4: c_uint = 0x42115C;
pub const mmDMA_CH_4_TDMA_SRC_ROI_SIZE_4: c_uint = 0x421160;
pub const mmDMA_CH_4_TDMA_SRC_VALID_ELEMENTS_4: c_uint = 0x421164;
pub const mmDMA_CH_4_TDMA_SRC_START_OFFSET_4: c_uint = 0x421168;
pub const mmDMA_CH_4_TDMA_SRC_STRIDE_4: c_uint = 0x42116C;
pub const mmDMA_CH_4_TDMA_DST_BASE_ADDR_LO: c_uint = 0x421170;
pub const mmDMA_CH_4_TDMA_DST_BASE_ADDR_HI: c_uint = 0x421174;
pub const mmDMA_CH_4_TDMA_DST_ROI_BASE_0: c_uint = 0x421178;
pub const mmDMA_CH_4_TDMA_DST_ROI_SIZE_0: c_uint = 0x42117C;
pub const mmDMA_CH_4_TDMA_DST_VALID_ELEMENTS_0: c_uint = 0x421180;
pub const mmDMA_CH_4_TDMA_DST_START_OFFSET_0: c_uint = 0x421184;
pub const mmDMA_CH_4_TDMA_DST_STRIDE_0: c_uint = 0x421188;
pub const mmDMA_CH_4_TDMA_DST_ROI_BASE_1: c_uint = 0x42118C;
pub const mmDMA_CH_4_TDMA_DST_ROI_SIZE_1: c_uint = 0x421190;
pub const mmDMA_CH_4_TDMA_DST_VALID_ELEMENTS_1: c_uint = 0x421194;
pub const mmDMA_CH_4_TDMA_DST_START_OFFSET_1: c_uint = 0x421198;
pub const mmDMA_CH_4_TDMA_DST_STRIDE_1: c_uint = 0x42119C;
pub const mmDMA_CH_4_TDMA_DST_ROI_BASE_2: c_uint = 0x4211A0;
pub const mmDMA_CH_4_TDMA_DST_ROI_SIZE_2: c_uint = 0x4211A4;
pub const mmDMA_CH_4_TDMA_DST_VALID_ELEMENTS_2: c_uint = 0x4211A8;
pub const mmDMA_CH_4_TDMA_DST_START_OFFSET_2: c_uint = 0x4211AC;
pub const mmDMA_CH_4_TDMA_DST_STRIDE_2: c_uint = 0x4211B0;
pub const mmDMA_CH_4_TDMA_DST_ROI_BASE_3: c_uint = 0x4211B4;
pub const mmDMA_CH_4_TDMA_DST_ROI_SIZE_3: c_uint = 0x4211B8;
pub const mmDMA_CH_4_TDMA_DST_VALID_ELEMENTS_3: c_uint = 0x4211BC;
pub const mmDMA_CH_4_TDMA_DST_START_OFFSET_3: c_uint = 0x4211C0;
pub const mmDMA_CH_4_TDMA_DST_STRIDE_3: c_uint = 0x4211C4;
pub const mmDMA_CH_4_TDMA_DST_ROI_BASE_4: c_uint = 0x4211C8;
pub const mmDMA_CH_4_TDMA_DST_ROI_SIZE_4: c_uint = 0x4211CC;
pub const mmDMA_CH_4_TDMA_DST_VALID_ELEMENTS_4: c_uint = 0x4211D0;
pub const mmDMA_CH_4_TDMA_DST_START_OFFSET_4: c_uint = 0x4211D4;
pub const mmDMA_CH_4_TDMA_DST_STRIDE_4: c_uint = 0x4211D8;
pub const mmDMA_CH_4_MEM_INIT_BUSY: c_uint = 0x4211FC;
