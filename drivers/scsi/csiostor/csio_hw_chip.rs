//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/csiostor/csio_hw_chip.h
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


//
// This file is part of the Chelsio FCoE driver for Linux.
//
// Copyright (c) 2008-2013 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// Define MACRO values
pub const CSIO_HW_T5: c_uint = 0x5000;
pub const CSIO_T5_FCOE_ASIC: c_uint = 0x5600;
pub const CSIO_HW_T6: c_uint = 0x6000;
pub const CSIO_T6_FCOE_ASIC: c_uint = 0x6600;
pub const CSIO_HW_CHIP_MASK: c_uint = 0xF000;

pub const CHELSIO_CHIP_FPGA: c_uint = 0x100;

pub const CHELSIO_T5: c_uint = 0x5;
pub const CHELSIO_T6: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chip_type {
    T5_A0 = CHELSIO_CHIP_CODE(CHELSIO_T5, 0),
    T5_A1 = CHELSIO_CHIP_CODE(CHELSIO_T5, 1),
    T5_FIRST_REV	= T5_A0,
    T5_LAST_REV	= T5_A1,

    T6_A0 = CHELSIO_CHIP_CODE(CHELSIO_T6, 0),
    T6_FIRST_REV    = T6_A0,
    T6_LAST_REV     = T6_A0,
}

// Define MACRO DEFINITIONS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_info {
    pub chip: u8,
    pub fs_name: *mut c_char,
    pub fw_mod_name: *mut c_char,
    pub fw_hdr: fw_hdr,
}

// Declare ENUMS
// Slow path handlers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intr_info {
    pub /: *mut *mut unsigned int mask; / bits to check in interrupt status,
    pub /: *const *const *const char msg; / message to print or NULL,
    pub /: *mut *mut short stat_idx; / stat counter to increment or -1,
    pub /: *mut *mut unsigned short fatal; / whether the condition reported is fatal,
}

// T4/T5 Chip specific ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_hw_chip_ops {
    pub uint32_t): *mut *mut *mut int (chip_set_mem_win)(struct csio_hw ,,
    pub ): *mut *mut void (chip_pcie_intr_handler)(struct csio_hw,
    pub ): *mut *mut uint32_t (chip_flash_cfg_addr)(struct csio_hw,
    pub ): *mut *mut __be32 , uint64_t,
    pub ): *mut *mut __be32 , uint64_t,
    pub int): *mut *mut u32, uint32_t ,,
    pub ): *mut *mut void (chip_dfs_create_ext_mem)(struct csio_hw,
}
