//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/dma_ch_2_regs.h
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
// DMA_CH_2 (Prototype: DMA_CH)
//
pub const mmDMA_CH_2_CFG0: c_uint = 0x411000;
pub const mmDMA_CH_2_CFG1: c_uint = 0x411004;
pub const mmDMA_CH_2_ERRMSG_ADDR_LO: c_uint = 0x411008;
pub const mmDMA_CH_2_ERRMSG_ADDR_HI: c_uint = 0x41100C;
pub const mmDMA_CH_2_ERRMSG_WDATA: c_uint = 0x411010;
pub const mmDMA_CH_2_RD_COMP_ADDR_LO: c_uint = 0x411014;
pub const mmDMA_CH_2_RD_COMP_ADDR_HI: c_uint = 0x411018;
pub const mmDMA_CH_2_RD_COMP_WDATA: c_uint = 0x41101C;
pub const mmDMA_CH_2_WR_COMP_ADDR_LO: c_uint = 0x411020;
pub const mmDMA_CH_2_WR_COMP_ADDR_HI: c_uint = 0x411024;
pub const mmDMA_CH_2_WR_COMP_WDATA: c_uint = 0x411028;
pub const mmDMA_CH_2_LDMA_SRC_ADDR_LO: c_uint = 0x41102C;
pub const mmDMA_CH_2_LDMA_SRC_ADDR_HI: c_uint = 0x411030;
pub const mmDMA_CH_2_LDMA_DST_ADDR_LO: c_uint = 0x411034;
pub const mmDMA_CH_2_LDMA_DST_ADDR_HI: c_uint = 0x411038;
pub const mmDMA_CH_2_LDMA_TSIZE: c_uint = 0x41103C;
pub const mmDMA_CH_2_COMIT_TRANSFER: c_uint = 0x411040;
pub const mmDMA_CH_2_STS0: c_uint = 0x411044;
pub const mmDMA_CH_2_STS1: c_uint = 0x411048;
pub const mmDMA_CH_2_STS2: c_uint = 0x41104C;
pub const mmDMA_CH_2_STS3: c_uint = 0x411050;
pub const mmDMA_CH_2_STS4: c_uint = 0x411054;
pub const mmDMA_CH_2_SRC_ADDR_LO_STS: c_uint = 0x411058;
pub const mmDMA_CH_2_SRC_ADDR_HI_STS: c_uint = 0x41105C;
pub const mmDMA_CH_2_SRC_TSIZE_STS: c_uint = 0x411060;
pub const mmDMA_CH_2_DST_ADDR_LO_STS: c_uint = 0x411064;
pub const mmDMA_CH_2_DST_ADDR_HI_STS: c_uint = 0x411068;
pub const mmDMA_CH_2_DST_TSIZE_STS: c_uint = 0x41106C;
pub const mmDMA_CH_2_RD_RATE_LIM_EN: c_uint = 0x411070;
pub const mmDMA_CH_2_RD_RATE_LIM_RST_TOKEN: c_uint = 0x411074;
pub const mmDMA_CH_2_RD_RATE_LIM_SAT: c_uint = 0x411078;
pub const mmDMA_CH_2_RD_RATE_LIM_TOUT: c_uint = 0x41107C;
pub const mmDMA_CH_2_WR_RATE_LIM_EN: c_uint = 0x411080;
pub const mmDMA_CH_2_WR_RATE_LIM_RST_TOKEN: c_uint = 0x411084;
pub const mmDMA_CH_2_WR_RATE_LIM_SAT: c_uint = 0x411088;
pub const mmDMA_CH_2_WR_RATE_LIM_TOUT: c_uint = 0x41108C;
pub const mmDMA_CH_2_CFG2: c_uint = 0x411090;
pub const mmDMA_CH_2_TDMA_CTL: c_uint = 0x411100;
pub const mmDMA_CH_2_TDMA_SRC_BASE_ADDR_LO: c_uint = 0x411104;
pub const mmDMA_CH_2_TDMA_SRC_BASE_ADDR_HI: c_uint = 0x411108;
pub const mmDMA_CH_2_TDMA_SRC_ROI_BASE_0: c_uint = 0x41110C;
pub const mmDMA_CH_2_TDMA_SRC_ROI_SIZE_0: c_uint = 0x411110;
pub const mmDMA_CH_2_TDMA_SRC_VALID_ELEMENTS_0: c_uint = 0x411114;
pub const mmDMA_CH_2_TDMA_SRC_START_OFFSET_0: c_uint = 0x411118;
pub const mmDMA_CH_2_TDMA_SRC_STRIDE_0: c_uint = 0x41111C;
pub const mmDMA_CH_2_TDMA_SRC_ROI_BASE_1: c_uint = 0x411120;
pub const mmDMA_CH_2_TDMA_SRC_ROI_SIZE_1: c_uint = 0x411124;
pub const mmDMA_CH_2_TDMA_SRC_VALID_ELEMENTS_1: c_uint = 0x411128;
pub const mmDMA_CH_2_TDMA_SRC_START_OFFSET_1: c_uint = 0x41112C;
pub const mmDMA_CH_2_TDMA_SRC_STRIDE_1: c_uint = 0x411130;
pub const mmDMA_CH_2_TDMA_SRC_ROI_BASE_2: c_uint = 0x411134;
pub const mmDMA_CH_2_TDMA_SRC_ROI_SIZE_2: c_uint = 0x411138;
pub const mmDMA_CH_2_TDMA_SRC_VALID_ELEMENTS_2: c_uint = 0x41113C;
pub const mmDMA_CH_2_TDMA_SRC_START_OFFSET_2: c_uint = 0x411140;
pub const mmDMA_CH_2_TDMA_SRC_STRIDE_2: c_uint = 0x411144;
pub const mmDMA_CH_2_TDMA_SRC_ROI_BASE_3: c_uint = 0x411148;
pub const mmDMA_CH_2_TDMA_SRC_ROI_SIZE_3: c_uint = 0x41114C;
pub const mmDMA_CH_2_TDMA_SRC_VALID_ELEMENTS_3: c_uint = 0x411150;
pub const mmDMA_CH_2_TDMA_SRC_START_OFFSET_3: c_uint = 0x411154;
pub const mmDMA_CH_2_TDMA_SRC_STRIDE_3: c_uint = 0x411158;
pub const mmDMA_CH_2_TDMA_SRC_ROI_BASE_4: c_uint = 0x41115C;
pub const mmDMA_CH_2_TDMA_SRC_ROI_SIZE_4: c_uint = 0x411160;
pub const mmDMA_CH_2_TDMA_SRC_VALID_ELEMENTS_4: c_uint = 0x411164;
pub const mmDMA_CH_2_TDMA_SRC_START_OFFSET_4: c_uint = 0x411168;
pub const mmDMA_CH_2_TDMA_SRC_STRIDE_4: c_uint = 0x41116C;
pub const mmDMA_CH_2_TDMA_DST_BASE_ADDR_LO: c_uint = 0x411170;
pub const mmDMA_CH_2_TDMA_DST_BASE_ADDR_HI: c_uint = 0x411174;
pub const mmDMA_CH_2_TDMA_DST_ROI_BASE_0: c_uint = 0x411178;
pub const mmDMA_CH_2_TDMA_DST_ROI_SIZE_0: c_uint = 0x41117C;
pub const mmDMA_CH_2_TDMA_DST_VALID_ELEMENTS_0: c_uint = 0x411180;
pub const mmDMA_CH_2_TDMA_DST_START_OFFSET_0: c_uint = 0x411184;
pub const mmDMA_CH_2_TDMA_DST_STRIDE_0: c_uint = 0x411188;
pub const mmDMA_CH_2_TDMA_DST_ROI_BASE_1: c_uint = 0x41118C;
pub const mmDMA_CH_2_TDMA_DST_ROI_SIZE_1: c_uint = 0x411190;
pub const mmDMA_CH_2_TDMA_DST_VALID_ELEMENTS_1: c_uint = 0x411194;
pub const mmDMA_CH_2_TDMA_DST_START_OFFSET_1: c_uint = 0x411198;
pub const mmDMA_CH_2_TDMA_DST_STRIDE_1: c_uint = 0x41119C;
pub const mmDMA_CH_2_TDMA_DST_ROI_BASE_2: c_uint = 0x4111A0;
pub const mmDMA_CH_2_TDMA_DST_ROI_SIZE_2: c_uint = 0x4111A4;
pub const mmDMA_CH_2_TDMA_DST_VALID_ELEMENTS_2: c_uint = 0x4111A8;
pub const mmDMA_CH_2_TDMA_DST_START_OFFSET_2: c_uint = 0x4111AC;
pub const mmDMA_CH_2_TDMA_DST_STRIDE_2: c_uint = 0x4111B0;
pub const mmDMA_CH_2_TDMA_DST_ROI_BASE_3: c_uint = 0x4111B4;
pub const mmDMA_CH_2_TDMA_DST_ROI_SIZE_3: c_uint = 0x4111B8;
pub const mmDMA_CH_2_TDMA_DST_VALID_ELEMENTS_3: c_uint = 0x4111BC;
pub const mmDMA_CH_2_TDMA_DST_START_OFFSET_3: c_uint = 0x4111C0;
pub const mmDMA_CH_2_TDMA_DST_STRIDE_3: c_uint = 0x4111C4;
pub const mmDMA_CH_2_TDMA_DST_ROI_BASE_4: c_uint = 0x4111C8;
pub const mmDMA_CH_2_TDMA_DST_ROI_SIZE_4: c_uint = 0x4111CC;
pub const mmDMA_CH_2_TDMA_DST_VALID_ELEMENTS_4: c_uint = 0x4111D0;
pub const mmDMA_CH_2_TDMA_DST_START_OFFSET_4: c_uint = 0x4111D4;
pub const mmDMA_CH_2_TDMA_DST_STRIDE_4: c_uint = 0x4111D8;
pub const mmDMA_CH_2_MEM_INIT_BUSY: c_uint = 0x4111FC;
