//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/mcdi_pcol_mae.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2019 Solarflare Communications Inc.
// Copyright 2019-2022 Xilinx, Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//
// MCDI definitions for Match-Action Engine functionality, that are
// missing from the main mcdi_pcol.h
//
// MC_CMD_MAE_COUNTER_LIST_ALLOC is not (yet) a released API, but the
// following value is needed as an argument to MC_CMD_MAE_ACTION_SET_ALLOC.
//
// enum: A counter ID that is guaranteed never to represent a real counter
pub const MC_CMD_MAE_COUNTER_LIST_ALLOC_OUT_COUNTER_LIST_ID_NULL: c_uint = 0xffffffff;
