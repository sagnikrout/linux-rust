//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cdx/edac_cdx_pcol.h
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
// Driver for AMD network controllers and boards
//
// Copyright (C) 2021, Xilinx, Inc.
// Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
//

pub const MC_CMD_EDAC_GET_DDR_CONFIG_OUT_WORD_LENGTH_LEN: c_int = 4;
// Number of registers for the DDR controller
pub const MC_CMD_GET_DDR_CONFIG_OFST: c_int = 4;
pub const MC_CMD_GET_DDR_CONFIG_LEN: c_int = 4;
//
// MC_CMD_EDAC_GET_DDR_CONFIG
// Provides detailed configuration for the DDR controller of the given index.
//
pub const MC_CMD_EDAC_GET_DDR_CONFIG: c_uint = 0x3;
// MC_CMD_EDAC_GET_DDR_CONFIG_IN msgrequest
pub const MC_CMD_EDAC_GET_DDR_CONFIG_IN_CONTROLLER_INDEX_OFST: c_int = 0;
pub const MC_CMD_EDAC_GET_DDR_CONFIG_IN_CONTROLLER_INDEX_LEN: c_int = 4;
