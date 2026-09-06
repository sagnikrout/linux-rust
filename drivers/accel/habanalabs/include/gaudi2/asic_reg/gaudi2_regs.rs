//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/gaudi2_regs.h
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
// Copyright 2020-2023 HabanaLabs, Ltd.
// All Rights Reserved.
//

pub const mmGIC_DISTRIBUTOR__5_GICD_SETSPI_NSR: c_uint = 0x4800040;
pub const mmDCORE0_TPC0_EML_CFG_DBG_CNT: c_uint = 0x40000;
pub const SM_OBJS_PROT_BITS_OFFS: c_uint = 0x14000;

// RTR CTR RAZWI related offsets

// RAZWI captured hbw aw addr high

// RAZWI captured hbw aw addr low

// RAZWI captured hbw aw set

// RAZWI captured hbw ar addr high

// RAZWI captured hbw ar addr low

// RAZWI captured hbw ar set

// RAZWI captured lbw aw addr

// RAZWI captured lbw aw set

// RAZWI captured lbw ar addr

// RAZWI captured lbw ar set

// RAZWI captured shared hbw aw addr high

// RAZWI captured shared hbw aw addr low

// RAZWI captured shared hbw ar addr high

// RAZWI captured shared hbw ar addr low

// RAZWI captured shared aw XY coordinates

// RAZWI captured shared ar XY coordinates

// RAZWI hbw shared occurred due to write access

// RAZWI hbw shared occurred due to read access

// RAZWI captured shared lbw aw addr

// RAZWI captured shared lbw ar addr

// RAZWI captured shared lbw aw XY coordinates

// RAZWI captured shared lbw ar XY coordinates

// RAZWI lbw shared occurred due to write access

// RAZWI lbw shared occurred due to read access

