//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/stmmac_est.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2023, Intel Corporation
// stmmac EST(802.3 Qbv) handling
//
pub const EST_GMAC4_OFFSET: c_uint = 0x00000c50;
pub const EST_XGMAC_OFFSET: c_uint = 0x00001050;
pub const EST_CONTROL: c_uint = 0x00000000;

pub const EST_GMAC5_PTOV_SHIFT: c_int = 24;
pub const EST_GMAC5_PTOV_MUL: c_int = 6;

pub const EST_XGMAC_PTOV_SHIFT: c_int = 23;
pub const EST_XGMAC_PTOV_MUL: c_int = 9;

pub const EST_STATUS: c_uint = 0x00000008;

pub const EST_SWOL_SHIFT: c_int = 7;

pub const EST_SCH_ERR: c_uint = 0x00000010;
pub const EST_FRM_SZ_ERR: c_uint = 0x00000014;
pub const EST_FRM_SZ_CAP: c_uint = 0x00000018;

pub const EST_SZ_CAP_HBFQ_SHIFT: c_int = 16;

pub const EST_INT_EN: c_uint = 0x00000020;

pub const EST_GCL_CONTROL: c_uint = 0x00000030;
pub const EST_BTR_LOW: c_uint = 0x0;
pub const EST_BTR_HIGH: c_uint = 0x1;
pub const EST_CTR_LOW: c_uint = 0x2;
pub const EST_CTR_HIGH: c_uint = 0x3;
pub const EST_TER: c_uint = 0x4;
pub const EST_LLR: c_uint = 0x5;
pub const EST_ADDR_SHIFT: c_int = 8;

pub const EST_GCL_DATA: c_uint = 0x00000034;
