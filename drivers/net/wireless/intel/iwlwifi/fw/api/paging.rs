//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/paging.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_paging_h__

//
// struct iwl_fw_paging_cmd - paging layout
//
// Send to FW the paging layout in the driver.
//
// @flags: various flags for the command
// @block_size: the block size in powers of 2
// @block_num: number of blocks specified in the command.
// @device_phy_addr: virtual addresses from device side
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_paging_cmd {
    pub flags: __le32,
    pub block_size: __le32,
    pub block_num: __le32,
    pub device_phy_addr: [__le32; NUM_OF_FW_PAGING_BLOCKS],
    pub /: *mut *mut } __packed; / FW_PAGING_BLOCK_CMD_API_S_VER_1,
