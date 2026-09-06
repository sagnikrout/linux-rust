//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memory/renesas-rpc-if-regs.h
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
// R-Car RPC Interface Registers Definitions
//
// Copyright (C) 2025 Renesas Electronics Corporation
//

pub const RPCIF_CMNCR: c_uint = 0x0000	/* R/W */;

pub const RPCIF_SSLDR: c_uint = 0x0004	/* R/W */;

pub const RPCIF_DRCR: c_uint = 0x000C	/* R/W */;

pub const RPCIF_DRCMR: c_uint = 0x0010	/* R/W */;

pub const RPCIF_DREAR: c_uint = 0x0014	/* R/W */;

pub const RPCIF_DROPR: c_uint = 0x0018	/* R/W */;
pub const RPCIF_DRENR: c_uint = 0x001C	/* R/W */;

pub const RPCIF_SMCR: c_uint = 0x0020	/* R/W */;

pub const RPCIF_SMCMR: c_uint = 0x0024	/* R/W */;

pub const RPCIF_SMADR: c_uint = 0x0028	/* R/W */;
pub const RPCIF_SMOPR: c_uint = 0x002C	/* R/W */;

pub const RPCIF_SMENR: c_uint = 0x0030	/* R/W */;

pub const RPCIF_SMRDR0: c_uint = 0x0038	/* R */;
pub const RPCIF_SMRDR1: c_uint = 0x003C	/* R */;
pub const RPCIF_SMWDR0: c_uint = 0x0040	/* W */;
pub const RPCIF_SMWDR1: c_uint = 0x0044	/* W */;
pub const RPCIF_CMNSR: c_uint = 0x0048	/* R */;

pub const RPCIF_DRDMCR: c_uint = 0x0058	/* R/W */;

pub const RPCIF_DRDRENR: c_uint = 0x005C	/* R/W */;

pub const RPCIF_SMDMCR: c_uint = 0x0060	/* R/W */;

pub const RPCIF_SMDRENR: c_uint = 0x0064	/* R/W */;

pub const RPCIF_PHYADD: c_uint = 0x0070	/* R/W available on R-Car E3/D3/V3M and RZ/G2{E,L} */;
pub const RPCIF_PHYWR: c_uint = 0x0074	/* R/W available on R-Car E3/D3/V3M and RZ/G2{E,L} */;
pub const RPCIF_PHYCNT: c_uint = 0x007C	/* R/W */;

pub const RPCIF_PHYOFFSET1: c_uint = 0x0080	/* R/W */;

pub const RPCIF_PHYOFFSET2: c_uint = 0x0084	/* R/W */;

pub const RPCIF_PHYINT: c_uint = 0x0088	/* R/W */;

