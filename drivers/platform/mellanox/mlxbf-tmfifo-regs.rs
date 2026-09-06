//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/mellanox/mlxbf-tmfifo-regs.h
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
// Copyright (c) 2019, Mellanox Technologies. All rights reserved.
//

pub const MLXBF_TMFIFO_TX_DATA: c_uint = 0x00;
pub const MLXBF_TMFIFO_TX_STS: c_uint = 0x08;
pub const MLXBF_TMFIFO_TX_STS__LENGTH: c_uint = 0x0001;
pub const MLXBF_TMFIFO_TX_STS__COUNT_SHIFT: c_int = 0;
pub const MLXBF_TMFIFO_TX_STS__COUNT_WIDTH: c_int = 9;
pub const MLXBF_TMFIFO_TX_STS__COUNT_RESET_VAL: c_int = 0;

pub const MLXBF_TMFIFO_TX_CTL: c_uint = 0x10;
pub const MLXBF_TMFIFO_TX_CTL__LENGTH: c_uint = 0x0001;
pub const MLXBF_TMFIFO_TX_CTL__LWM_SHIFT: c_int = 0;
pub const MLXBF_TMFIFO_TX_CTL__LWM_WIDTH: c_int = 8;
pub const MLXBF_TMFIFO_TX_CTL__LWM_RESET_VAL: c_int = 128;

pub const MLXBF_TMFIFO_TX_CTL__HWM_SHIFT: c_int = 8;
pub const MLXBF_TMFIFO_TX_CTL__HWM_WIDTH: c_int = 8;
pub const MLXBF_TMFIFO_TX_CTL__HWM_RESET_VAL: c_int = 128;

pub const MLXBF_TMFIFO_TX_CTL__MAX_ENTRIES_SHIFT: c_int = 32;
pub const MLXBF_TMFIFO_TX_CTL__MAX_ENTRIES_WIDTH: c_int = 9;
pub const MLXBF_TMFIFO_TX_CTL__MAX_ENTRIES_RESET_VAL: c_int = 256;

pub const MLXBF_TMFIFO_RX_DATA: c_uint = 0x00;
pub const MLXBF_TMFIFO_RX_STS: c_uint = 0x08;
pub const MLXBF_TMFIFO_RX_STS__LENGTH: c_uint = 0x0001;
pub const MLXBF_TMFIFO_RX_STS__COUNT_SHIFT: c_int = 0;
pub const MLXBF_TMFIFO_RX_STS__COUNT_WIDTH: c_int = 9;
pub const MLXBF_TMFIFO_RX_STS__COUNT_RESET_VAL: c_int = 0;

pub const MLXBF_TMFIFO_RX_CTL: c_uint = 0x10;
pub const MLXBF_TMFIFO_RX_CTL__LENGTH: c_uint = 0x0001;
pub const MLXBF_TMFIFO_RX_CTL__LWM_SHIFT: c_int = 0;
pub const MLXBF_TMFIFO_RX_CTL__LWM_WIDTH: c_int = 8;
pub const MLXBF_TMFIFO_RX_CTL__LWM_RESET_VAL: c_int = 128;

pub const MLXBF_TMFIFO_RX_CTL__HWM_SHIFT: c_int = 8;
pub const MLXBF_TMFIFO_RX_CTL__HWM_WIDTH: c_int = 8;
pub const MLXBF_TMFIFO_RX_CTL__HWM_RESET_VAL: c_int = 128;

pub const MLXBF_TMFIFO_RX_CTL__MAX_ENTRIES_SHIFT: c_int = 32;
pub const MLXBF_TMFIFO_RX_CTL__MAX_ENTRIES_WIDTH: c_int = 9;
pub const MLXBF_TMFIFO_RX_CTL__MAX_ENTRIES_RESET_VAL: c_int = 256;

// BF3 register offsets within resource 0.
pub const MLXBF_TMFIFO_RX_DATA_BF3: c_uint = 0x0000;
pub const MLXBF_TMFIFO_TX_DATA_BF3: c_uint = 0x1000;
// BF3 register offsets within resource 1.
pub const MLXBF_TMFIFO_RX_STS_BF3: c_uint = 0x0000;
pub const MLXBF_TMFIFO_RX_CTL_BF3: c_uint = 0x0008;
pub const MLXBF_TMFIFO_TX_STS_BF3: c_uint = 0x0100;
pub const MLXBF_TMFIFO_TX_CTL_BF3: c_uint = 0x0108;
