//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/rot0_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// ROT0
// (Prototype: ROTATOR)
//
pub const mmROT0_KMD_MODE: c_uint = 0x4E0B000;
pub const mmROT0_CPL_QUEUE_EN: c_uint = 0x4E0B004;
pub const mmROT0_CPL_QUEUE_ADDR_L: c_uint = 0x4E0B008;
pub const mmROT0_CPL_QUEUE_ADDR_H: c_uint = 0x4E0B00C;
pub const mmROT0_CPL_QUEUE_DATA: c_uint = 0x4E0B010;
pub const mmROT0_CPL_QUEUE_AWUSER: c_uint = 0x4E0B014;
pub const mmROT0_CPL_QUEUE_AXI: c_uint = 0x4E0B018;
pub const mmROT0_CPL_MSG_THRESHOLD: c_uint = 0x4E0B020;
pub const mmROT0_CPL_MSG_AXI: c_uint = 0x4E0B024;
pub const mmROT0_AXI_WB: c_uint = 0x4E0B028;
pub const mmROT0_ERR_CFG: c_uint = 0x4E0B02C;
pub const mmROT0_ERR_STATUS: c_uint = 0x4E0B030;
pub const mmROT0_WBC_MAX_OUTSTANDING: c_uint = 0x4E0B038;
pub const mmROT0_WBC_RL: c_uint = 0x4E0B03C;
pub const mmROT0_WBC_INFLIGHTS: c_uint = 0x4E0B040;
pub const mmROT0_WBC_INFO: c_uint = 0x4E0B044;
pub const mmROT0_WBC_MON: c_uint = 0x4E0B048;
pub const mmROT0_RSB_CAM_MAX_SIZE: c_uint = 0x4E0B04C;
pub const mmROT0_RSB_CFG: c_uint = 0x4E0B050;
pub const mmROT0_RSB_MAX_OS: c_uint = 0x4E0B054;
pub const mmROT0_RSB_RL: c_uint = 0x4E0B058;
pub const mmROT0_RSB_INFLIGHTS: c_uint = 0x4E0B05C;
pub const mmROT0_RSB_OCCUPANCY: c_uint = 0x4E0B060;
pub const mmROT0_RSB_INFO: c_uint = 0x4E0B064;
pub const mmROT0_RSB_MON: c_uint = 0x4E0B068;
pub const mmROT0_RSB_MON_CONTEXT_ID: c_uint = 0x4E0B06C;
pub const mmROT0_MSS_HALT: c_uint = 0x4E0B070;
pub const mmROT0_MSS_SEI_STATUS: c_uint = 0x4E0B074;
pub const mmROT0_MSS_SEI_MASK: c_uint = 0x4E0B078;
pub const mmROT0_MSS_SPI_STATUS: c_uint = 0x4E0B07C;
pub const mmROT0_MSS_SPI_MASK: c_uint = 0x4E0B080;
pub const mmROT0_DISABLE_PAD_CALC: c_uint = 0x4E0B084;
pub const mmROT0_QMAN_CFG: c_uint = 0x4E0B088;
pub const mmROT0_CLK_EN: c_uint = 0x4E0B08C;
pub const mmROT0_MRSB_CAM_MAX_SIZE: c_uint = 0x4E0B090;
pub const mmROT0_MRSB_CFG: c_uint = 0x4E0B094;
pub const mmROT0_MRSB_MAX_OS: c_uint = 0x4E0B098;
pub const mmROT0_MRSB_RL: c_uint = 0x4E0B09C;
pub const mmROT0_MRSB_INFLIGHTS: c_uint = 0x4E0B0A0;
pub const mmROT0_MRSB_OCCUPANCY: c_uint = 0x4E0B0A4;
pub const mmROT0_MRSB_INFO: c_uint = 0x4E0B0A8;
pub const mmROT0_MRSB_MON: c_uint = 0x4E0B0AC;
pub const mmROT0_MRSB_MON_CONTEXT_ID: c_uint = 0x4E0B0B0;
pub const mmROT0_MSS_STS: c_uint = 0x4E0B0B4;
