//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_dump.h
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


// bnx2x_dump.h: QLogic Everest network driver.
//
// Copyright (c) 2012-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// Unless you and QLogic execute a separate written software license
// agreement governing use of this software, this software is licensed to you
// under the terms of the GNU General Public License version 2, available
// at http://www.gnu.org/licenses/old-licenses/gpl-2.0.html (the "GPL").
//
// Notwithstanding the above, under no circumstances may you combine this
// software in any way with any other QLogic software provided under a
// license other than the GPL, without QLogic's express prior written
// consent.
//
// WaitP Definitions
pub const DRV_DUMP_XSTORM_WAITP_ADDRESS: c_uint = 0x2b8a80;
pub const DRV_DUMP_TSTORM_WAITP_ADDRESS: c_uint = 0x1b8a80;
pub const DRV_DUMP_USTORM_WAITP_ADDRESS: c_uint = 0x338a80;
pub const DRV_DUMP_CSTORM_WAITP_ADDRESS: c_uint = 0x238a80;
// Possible Chips
pub const DUMP_CHIP_E1: c_int = 1;
pub const DUMP_CHIP_E1H: c_int = 2;
pub const DUMP_CHIP_E2: c_int = 4;
pub const DUMP_CHIP_E3A0: c_int = 8;
pub const DUMP_CHIP_E3B0: c_int = 16;
pub const DUMP_PATH_0: c_int = 512;
pub const DUMP_PATH_1: c_int = 1024;
pub const NUM_PRESETS: c_int = 13;
pub const NUM_CHIPS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dump_header {
    pub /: *mut *mut u32 header_size; / Size in DWORDs excluding this field,
    pub version: u32,
    pub preset: u32,
    pub /: *mut *mut u32 dump_meta_data; / OR of CHIP and PATH.,
}

pub const BNX2X_DUMP_VERSION: c_uint = 0x61111111;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_addr {
    pub addr: u32,
    pub size: u32,
    pub chips: u32,
    pub presets: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wreg_addr {
    pub addr: u32,
    pub size: u32,
    pub read_regs_count: u32,
    pub read_regs: *const u32,
    pub chips: u32,
    pub presets: u32,
}

pub const PAGE_MODE_VALUES_E2: c_int = 2;
pub const PAGE_READ_REGS_E2: c_int = 1;
pub const PAGE_WRITE_REGS_E2: c_int = 1;
pub const PAGE_MODE_VALUES_E3: c_int = 2;
pub const PAGE_READ_REGS_E3: c_int = 1;
pub const PAGE_WRITE_REGS_E3: c_int = 1;

