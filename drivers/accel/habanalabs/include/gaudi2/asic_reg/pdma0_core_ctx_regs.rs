//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pdma0_core_ctx_regs.h
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
// PDMA0_CORE_CTX
// (Prototype: DMA_CORE_CTX)
//
pub const mmPDMA0_CORE_CTX_RATE_LIM_TKN: c_uint = 0x4C8B860;
pub const mmPDMA0_CORE_CTX_PWRLP: c_uint = 0x4C8B864;
pub const mmPDMA0_CORE_CTX_TE_NUMROWS: c_uint = 0x4C8B868;
pub const mmPDMA0_CORE_CTX_IDX: c_uint = 0x4C8B86C;
pub const mmPDMA0_CORE_CTX_IDX_INC: c_uint = 0x4C8B870;
pub const mmPDMA0_CORE_CTX_CTRL: c_uint = 0x4C8B874;
pub const mmPDMA0_CORE_CTX_SRC_TSIZE_0: c_uint = 0x4C8B878;
pub const mmPDMA0_CORE_CTX_SRC_TSIZE_1: c_uint = 0x4C8B87C;
pub const mmPDMA0_CORE_CTX_SRC_STRIDE_1: c_uint = 0x4C8B880;
pub const mmPDMA0_CORE_CTX_SRC_TSIZE_2: c_uint = 0x4C8B884;
pub const mmPDMA0_CORE_CTX_SRC_STRIDE_2: c_uint = 0x4C8B888;
pub const mmPDMA0_CORE_CTX_SRC_TSIZE_3: c_uint = 0x4C8B88C;
pub const mmPDMA0_CORE_CTX_SRC_STRIDE_3: c_uint = 0x4C8B890;
pub const mmPDMA0_CORE_CTX_SRC_TSIZE_4: c_uint = 0x4C8B894;
pub const mmPDMA0_CORE_CTX_SRC_STRIDE_4: c_uint = 0x4C8B898;
pub const mmPDMA0_CORE_CTX_DST_TSIZE_1: c_uint = 0x4C8B89C;
pub const mmPDMA0_CORE_CTX_DST_STRIDE_1: c_uint = 0x4C8B8A0;
pub const mmPDMA0_CORE_CTX_DST_TSIZE_2: c_uint = 0x4C8B8A4;
pub const mmPDMA0_CORE_CTX_DST_STRIDE_2: c_uint = 0x4C8B8A8;
pub const mmPDMA0_CORE_CTX_DST_TSIZE_3: c_uint = 0x4C8B8AC;
pub const mmPDMA0_CORE_CTX_DST_STRIDE_3: c_uint = 0x4C8B8B0;
pub const mmPDMA0_CORE_CTX_DST_TSIZE_4: c_uint = 0x4C8B8B4;
pub const mmPDMA0_CORE_CTX_DST_STRIDE_4: c_uint = 0x4C8B8B8;
pub const mmPDMA0_CORE_CTX_WR_COMP_ADDR_HI: c_uint = 0x4C8B8BC;
pub const mmPDMA0_CORE_CTX_WR_COMP_ADDR_LO: c_uint = 0x4C8B8C0;
pub const mmPDMA0_CORE_CTX_WR_COMP_WDATA: c_uint = 0x4C8B8C4;
pub const mmPDMA0_CORE_CTX_SRC_OFFSET_LO: c_uint = 0x4C8B8C8;
pub const mmPDMA0_CORE_CTX_SRC_OFFSET_HI: c_uint = 0x4C8B8CC;
pub const mmPDMA0_CORE_CTX_DST_OFFSET_LO: c_uint = 0x4C8B8D0;
pub const mmPDMA0_CORE_CTX_DST_OFFSET_HI: c_uint = 0x4C8B8D4;
pub const mmPDMA0_CORE_CTX_SRC_BASE_LO: c_uint = 0x4C8B8D8;
pub const mmPDMA0_CORE_CTX_SRC_BASE_HI: c_uint = 0x4C8B8DC;
pub const mmPDMA0_CORE_CTX_DST_BASE_LO: c_uint = 0x4C8B8E0;
pub const mmPDMA0_CORE_CTX_DST_BASE_HI: c_uint = 0x4C8B8E4;
pub const mmPDMA0_CORE_CTX_DST_TSIZE_0: c_uint = 0x4C8B8E8;
pub const mmPDMA0_CORE_CTX_COMMIT: c_uint = 0x4C8B8EC;
