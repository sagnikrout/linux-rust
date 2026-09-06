//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/goya_regs.h
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

pub const mmPCIE_DBI_DEVICE_ID_VENDOR_ID_REG: c_uint = 0xC02000;
pub const mmPCIE_DBI_MSIX_DOORBELL_OFF: c_uint = 0xC02948;
pub const mmSYNC_MNGR_MON_PAY_ADDRL_0: c_uint = 0x113000;
pub const mmSYNC_MNGR_SOB_OBJ_0: c_uint = 0x112000;
pub const mmSYNC_MNGR_SOB_OBJ_1000: c_uint = 0x112FA0;
pub const mmSYNC_MNGR_SOB_OBJ_1007: c_uint = 0x112FBC;
pub const mmSYNC_MNGR_SOB_OBJ_1023: c_uint = 0x112FFC;
pub const mmSYNC_MNGR_MON_STATUS_0: c_uint = 0x114000;
pub const mmSYNC_MNGR_MON_STATUS_255: c_uint = 0x1143FC;
pub const mmGIC_DISTRIBUTOR__5_GICD_SETSPI_NSR: c_uint = 0x800040;
