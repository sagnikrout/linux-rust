//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_edma0_core_ctx_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// DCORE0_EDMA0_CORE_CTX
// (Prototype: DMA_CORE_CTX)
//
pub const mmDCORE0_EDMA0_CORE_CTX_RATE_LIM_TKN: c_uint = 0x41CB860;
pub const mmDCORE0_EDMA0_CORE_CTX_PWRLP: c_uint = 0x41CB864;
pub const mmDCORE0_EDMA0_CORE_CTX_TE_NUMROWS: c_uint = 0x41CB868;
pub const mmDCORE0_EDMA0_CORE_CTX_IDX: c_uint = 0x41CB86C;
pub const mmDCORE0_EDMA0_CORE_CTX_IDX_INC: c_uint = 0x41CB870;
pub const mmDCORE0_EDMA0_CORE_CTX_CTRL: c_uint = 0x41CB874;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_TSIZE_0: c_uint = 0x41CB878;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_TSIZE_1: c_uint = 0x41CB87C;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_STRIDE_1: c_uint = 0x41CB880;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_TSIZE_2: c_uint = 0x41CB884;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_STRIDE_2: c_uint = 0x41CB888;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_TSIZE_3: c_uint = 0x41CB88C;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_STRIDE_3: c_uint = 0x41CB890;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_TSIZE_4: c_uint = 0x41CB894;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_STRIDE_4: c_uint = 0x41CB898;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_TSIZE_1: c_uint = 0x41CB89C;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_STRIDE_1: c_uint = 0x41CB8A0;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_TSIZE_2: c_uint = 0x41CB8A4;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_STRIDE_2: c_uint = 0x41CB8A8;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_TSIZE_3: c_uint = 0x41CB8AC;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_STRIDE_3: c_uint = 0x41CB8B0;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_TSIZE_4: c_uint = 0x41CB8B4;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_STRIDE_4: c_uint = 0x41CB8B8;
pub const mmDCORE0_EDMA0_CORE_CTX_WR_COMP_ADDR_HI: c_uint = 0x41CB8BC;
pub const mmDCORE0_EDMA0_CORE_CTX_WR_COMP_ADDR_LO: c_uint = 0x41CB8C0;
pub const mmDCORE0_EDMA0_CORE_CTX_WR_COMP_WDATA: c_uint = 0x41CB8C4;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_OFFSET_LO: c_uint = 0x41CB8C8;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_OFFSET_HI: c_uint = 0x41CB8CC;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_OFFSET_LO: c_uint = 0x41CB8D0;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_OFFSET_HI: c_uint = 0x41CB8D4;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_BASE_LO: c_uint = 0x41CB8D8;
pub const mmDCORE0_EDMA0_CORE_CTX_SRC_BASE_HI: c_uint = 0x41CB8DC;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_BASE_LO: c_uint = 0x41CB8E0;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_BASE_HI: c_uint = 0x41CB8E4;
pub const mmDCORE0_EDMA0_CORE_CTX_DST_TSIZE_0: c_uint = 0x41CB8E8;
pub const mmDCORE0_EDMA0_CORE_CTX_COMMIT: c_uint = 0x41CB8EC;
