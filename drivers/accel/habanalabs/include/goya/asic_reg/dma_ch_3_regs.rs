//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/dma_ch_3_regs.h
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
// DMA_CH_3 (Prototype: DMA_CH)
//
pub const mmDMA_CH_3_CFG0: c_uint = 0x419000;
pub const mmDMA_CH_3_CFG1: c_uint = 0x419004;
pub const mmDMA_CH_3_ERRMSG_ADDR_LO: c_uint = 0x419008;
pub const mmDMA_CH_3_ERRMSG_ADDR_HI: c_uint = 0x41900C;
pub const mmDMA_CH_3_ERRMSG_WDATA: c_uint = 0x419010;
pub const mmDMA_CH_3_RD_COMP_ADDR_LO: c_uint = 0x419014;
pub const mmDMA_CH_3_RD_COMP_ADDR_HI: c_uint = 0x419018;
pub const mmDMA_CH_3_RD_COMP_WDATA: c_uint = 0x41901C;
pub const mmDMA_CH_3_WR_COMP_ADDR_LO: c_uint = 0x419020;
pub const mmDMA_CH_3_WR_COMP_ADDR_HI: c_uint = 0x419024;
pub const mmDMA_CH_3_WR_COMP_WDATA: c_uint = 0x419028;
pub const mmDMA_CH_3_LDMA_SRC_ADDR_LO: c_uint = 0x41902C;
pub const mmDMA_CH_3_LDMA_SRC_ADDR_HI: c_uint = 0x419030;
pub const mmDMA_CH_3_LDMA_DST_ADDR_LO: c_uint = 0x419034;
pub const mmDMA_CH_3_LDMA_DST_ADDR_HI: c_uint = 0x419038;
pub const mmDMA_CH_3_LDMA_TSIZE: c_uint = 0x41903C;
pub const mmDMA_CH_3_COMIT_TRANSFER: c_uint = 0x419040;
pub const mmDMA_CH_3_STS0: c_uint = 0x419044;
pub const mmDMA_CH_3_STS1: c_uint = 0x419048;
pub const mmDMA_CH_3_STS2: c_uint = 0x41904C;
pub const mmDMA_CH_3_STS3: c_uint = 0x419050;
pub const mmDMA_CH_3_STS4: c_uint = 0x419054;
pub const mmDMA_CH_3_SRC_ADDR_LO_STS: c_uint = 0x419058;
pub const mmDMA_CH_3_SRC_ADDR_HI_STS: c_uint = 0x41905C;
pub const mmDMA_CH_3_SRC_TSIZE_STS: c_uint = 0x419060;
pub const mmDMA_CH_3_DST_ADDR_LO_STS: c_uint = 0x419064;
pub const mmDMA_CH_3_DST_ADDR_HI_STS: c_uint = 0x419068;
pub const mmDMA_CH_3_DST_TSIZE_STS: c_uint = 0x41906C;
pub const mmDMA_CH_3_RD_RATE_LIM_EN: c_uint = 0x419070;
pub const mmDMA_CH_3_RD_RATE_LIM_RST_TOKEN: c_uint = 0x419074;
pub const mmDMA_CH_3_RD_RATE_LIM_SAT: c_uint = 0x419078;
pub const mmDMA_CH_3_RD_RATE_LIM_TOUT: c_uint = 0x41907C;
pub const mmDMA_CH_3_WR_RATE_LIM_EN: c_uint = 0x419080;
pub const mmDMA_CH_3_WR_RATE_LIM_RST_TOKEN: c_uint = 0x419084;
pub const mmDMA_CH_3_WR_RATE_LIM_SAT: c_uint = 0x419088;
pub const mmDMA_CH_3_WR_RATE_LIM_TOUT: c_uint = 0x41908C;
pub const mmDMA_CH_3_CFG2: c_uint = 0x419090;
pub const mmDMA_CH_3_TDMA_CTL: c_uint = 0x419100;
pub const mmDMA_CH_3_TDMA_SRC_BASE_ADDR_LO: c_uint = 0x419104;
pub const mmDMA_CH_3_TDMA_SRC_BASE_ADDR_HI: c_uint = 0x419108;
pub const mmDMA_CH_3_TDMA_SRC_ROI_BASE_0: c_uint = 0x41910C;
pub const mmDMA_CH_3_TDMA_SRC_ROI_SIZE_0: c_uint = 0x419110;
pub const mmDMA_CH_3_TDMA_SRC_VALID_ELEMENTS_0: c_uint = 0x419114;
pub const mmDMA_CH_3_TDMA_SRC_START_OFFSET_0: c_uint = 0x419118;
pub const mmDMA_CH_3_TDMA_SRC_STRIDE_0: c_uint = 0x41911C;
pub const mmDMA_CH_3_TDMA_SRC_ROI_BASE_1: c_uint = 0x419120;
pub const mmDMA_CH_3_TDMA_SRC_ROI_SIZE_1: c_uint = 0x419124;
pub const mmDMA_CH_3_TDMA_SRC_VALID_ELEMENTS_1: c_uint = 0x419128;
pub const mmDMA_CH_3_TDMA_SRC_START_OFFSET_1: c_uint = 0x41912C;
pub const mmDMA_CH_3_TDMA_SRC_STRIDE_1: c_uint = 0x419130;
pub const mmDMA_CH_3_TDMA_SRC_ROI_BASE_2: c_uint = 0x419134;
pub const mmDMA_CH_3_TDMA_SRC_ROI_SIZE_2: c_uint = 0x419138;
pub const mmDMA_CH_3_TDMA_SRC_VALID_ELEMENTS_2: c_uint = 0x41913C;
pub const mmDMA_CH_3_TDMA_SRC_START_OFFSET_2: c_uint = 0x419140;
pub const mmDMA_CH_3_TDMA_SRC_STRIDE_2: c_uint = 0x419144;
pub const mmDMA_CH_3_TDMA_SRC_ROI_BASE_3: c_uint = 0x419148;
pub const mmDMA_CH_3_TDMA_SRC_ROI_SIZE_3: c_uint = 0x41914C;
pub const mmDMA_CH_3_TDMA_SRC_VALID_ELEMENTS_3: c_uint = 0x419150;
pub const mmDMA_CH_3_TDMA_SRC_START_OFFSET_3: c_uint = 0x419154;
pub const mmDMA_CH_3_TDMA_SRC_STRIDE_3: c_uint = 0x419158;
pub const mmDMA_CH_3_TDMA_SRC_ROI_BASE_4: c_uint = 0x41915C;
pub const mmDMA_CH_3_TDMA_SRC_ROI_SIZE_4: c_uint = 0x419160;
pub const mmDMA_CH_3_TDMA_SRC_VALID_ELEMENTS_4: c_uint = 0x419164;
pub const mmDMA_CH_3_TDMA_SRC_START_OFFSET_4: c_uint = 0x419168;
pub const mmDMA_CH_3_TDMA_SRC_STRIDE_4: c_uint = 0x41916C;
pub const mmDMA_CH_3_TDMA_DST_BASE_ADDR_LO: c_uint = 0x419170;
pub const mmDMA_CH_3_TDMA_DST_BASE_ADDR_HI: c_uint = 0x419174;
pub const mmDMA_CH_3_TDMA_DST_ROI_BASE_0: c_uint = 0x419178;
pub const mmDMA_CH_3_TDMA_DST_ROI_SIZE_0: c_uint = 0x41917C;
pub const mmDMA_CH_3_TDMA_DST_VALID_ELEMENTS_0: c_uint = 0x419180;
pub const mmDMA_CH_3_TDMA_DST_START_OFFSET_0: c_uint = 0x419184;
pub const mmDMA_CH_3_TDMA_DST_STRIDE_0: c_uint = 0x419188;
pub const mmDMA_CH_3_TDMA_DST_ROI_BASE_1: c_uint = 0x41918C;
pub const mmDMA_CH_3_TDMA_DST_ROI_SIZE_1: c_uint = 0x419190;
pub const mmDMA_CH_3_TDMA_DST_VALID_ELEMENTS_1: c_uint = 0x419194;
pub const mmDMA_CH_3_TDMA_DST_START_OFFSET_1: c_uint = 0x419198;
pub const mmDMA_CH_3_TDMA_DST_STRIDE_1: c_uint = 0x41919C;
pub const mmDMA_CH_3_TDMA_DST_ROI_BASE_2: c_uint = 0x4191A0;
pub const mmDMA_CH_3_TDMA_DST_ROI_SIZE_2: c_uint = 0x4191A4;
pub const mmDMA_CH_3_TDMA_DST_VALID_ELEMENTS_2: c_uint = 0x4191A8;
pub const mmDMA_CH_3_TDMA_DST_START_OFFSET_2: c_uint = 0x4191AC;
pub const mmDMA_CH_3_TDMA_DST_STRIDE_2: c_uint = 0x4191B0;
pub const mmDMA_CH_3_TDMA_DST_ROI_BASE_3: c_uint = 0x4191B4;
pub const mmDMA_CH_3_TDMA_DST_ROI_SIZE_3: c_uint = 0x4191B8;
pub const mmDMA_CH_3_TDMA_DST_VALID_ELEMENTS_3: c_uint = 0x4191BC;
pub const mmDMA_CH_3_TDMA_DST_START_OFFSET_3: c_uint = 0x4191C0;
pub const mmDMA_CH_3_TDMA_DST_STRIDE_3: c_uint = 0x4191C4;
pub const mmDMA_CH_3_TDMA_DST_ROI_BASE_4: c_uint = 0x4191C8;
pub const mmDMA_CH_3_TDMA_DST_ROI_SIZE_4: c_uint = 0x4191CC;
pub const mmDMA_CH_3_TDMA_DST_VALID_ELEMENTS_4: c_uint = 0x4191D0;
pub const mmDMA_CH_3_TDMA_DST_START_OFFSET_4: c_uint = 0x4191D4;
pub const mmDMA_CH_3_TDMA_DST_STRIDE_4: c_uint = 0x4191D8;
pub const mmDMA_CH_3_MEM_INIT_BUSY: c_uint = 0x4191FC;
