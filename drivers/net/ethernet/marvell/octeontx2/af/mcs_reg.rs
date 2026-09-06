//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/mcs_reg.h
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
// Marvell MCS driver
//
// Copyright (C) 2022 Marvell.
//

// Registers
pub const MCSX_IP_MODE: c_uint = 0x900c8ull;

// PAB

// PEX registers

// CNF10K-B

pub const MCSX_PEX_RX_SLAVE_ETYPE_ENABLE: c_uint = 0x6e8ull;
pub const MCSX_PEX_TX_SLAVE_ETYPE_ENABLE: c_uint = 0x968ull;
// BEE
pub const MCSX_BBE_RX_SLAVE_PADDING_CTL: c_uint = 0xe08ull;
pub const MCSX_BBE_TX_SLAVE_PADDING_CTL: c_uint = 0x12f8ull;
pub const MCSX_BBE_RX_SLAVE_CAL_ENTRY: c_uint = 0x180ull;
pub const MCSX_BBE_RX_SLAVE_CAL_LEN: c_uint = 0x188ull;

pub const MCSX_BBE_RX_SLAVE_DFIFO_OVERFLOW_0: c_uint = 0xe20;
pub const MCSX_BBE_TX_SLAVE_DFIFO_OVERFLOW_0: c_uint = 0x1298;
pub const MCSX_BBE_RX_SLAVE_PLFIFO_OVERFLOW_0: c_uint = 0xe40;
pub const MCSX_BBE_TX_SLAVE_PLFIFO_OVERFLOW_0: c_uint = 0x12b8;

// CPM registers

pub const MCSX_CPM_RX_SLAVE_FLOWID_TCAM_ENA_1: c_uint = 0x30708ull;

// TX registers

pub const MCSX_CPM_TX_SLAVE_FLOWID_TCAM_ENA_1: c_uint = 0x51d18ull;

pub const MCSX_CPM_TX_SLAVE_AUTO_REKEY_ENABLE_0: c_uint = 0x5500ull;
// CSE

// CSE TX

