//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene/xgene_enet_ring2.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Applied Micro X-Gene SoC Ethernet Driver
//
// Copyright (c) 2015, Applied Micro Circuits Corporation
// Author: Iyappan Subramanian <isubramanian@apm.com>
//

pub const X2_NUM_RING_CONFIG: c_int = 6;
pub const INTR_MBOX_SIZE: c_int = 1024;
pub const CSR_VMID0_INTR_MBOX: c_uint = 0x0270;

pub const X2_MSG_AM_POS: c_int = 10;
pub const X2_QBASE_AM_POS: c_int = 11;
pub const X2_INTLINE_POS: c_int = 24;
pub const X2_INTLINE_LEN: c_int = 5;
pub const X2_CFGCRID_POS: c_int = 29;
pub const X2_CFGCRID_LEN: c_int = 3;
pub const X2_SELTHRSH_POS: c_int = 7;
pub const X2_SELTHRSH_LEN: c_int = 3;
pub const X2_RINGTYPE_POS: c_int = 23;
pub const X2_RINGTYPE_LEN: c_int = 2;
pub const X2_DEQINTEN_POS: c_int = 29;
pub const X2_RECOMTIMEOUT_POS: c_int = 0;
pub const X2_RECOMTIMEOUT_LEN: c_int = 7;
pub const X2_NUMMSGSINQ_POS: c_int = 0;
pub const X2_NUMMSGSINQ_LEN: c_int = 17;
