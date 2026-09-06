//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/goya.h
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
// Copyright 2016-2019 HabanaLabs, Ltd.
// All Rights Reserved.
//
pub const SRAM_CFG_BAR_ID: c_int = 0;
pub const MSIX_BAR_ID: c_int = 2;
pub const DDR_BAR_ID: c_int = 4;
pub const CFG_BAR_SIZE: c_uint = 0x10000000ull		/* 256MB */;
pub const MSIX_BAR_SIZE: c_uint = 0x1000ull		/* 4KB */;
pub const CFG_BASE: c_uint = 0x7FFC000000ull;
pub const CFG_SIZE: c_uint = 0x4000000		/* 32MB CFG + 32MB DBG*/;
pub const SRAM_BASE_ADDR: c_uint = 0x7FF0000000ull;
pub const SRAM_SIZE: c_uint = 0x32A0000		/* 50.625MB */;
pub const DRAM_PHYS_BASE: c_uint = 0x0ull;
pub const HOST_PHYS_BASE: c_uint = 0x8000000000ull		/* 0.5TB */;
pub const HOST_PHYS_SIZE: c_uint = 0x1000000000000ull	/* 0.25PB (48 bits) */;
pub const GOYA_MSIX_ENTRIES: c_int = 8;

pub const MAX_ASID: c_int = 2;
pub const PROT_BITS_OFFS: c_uint = 0xF80;
pub const DMA_MAX_NUM: c_int = 5;
pub const TPC_MAX_NUM: c_int = 8;
pub const MME_MAX_NUM: c_int = 1;
