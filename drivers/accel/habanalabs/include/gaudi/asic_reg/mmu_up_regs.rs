//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/asic_reg/mmu_up_regs.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// MMU_UP (Prototype: MMU)
//
pub const mmMMU_UP_MMU_ENABLE: c_uint = 0xC1100C;
pub const mmMMU_UP_FORCE_ORDERING: c_uint = 0xC11010;
pub const mmMMU_UP_FEATURE_ENABLE: c_uint = 0xC11014;
pub const mmMMU_UP_VA_ORDERING_MASK_31_7: c_uint = 0xC11018;
pub const mmMMU_UP_VA_ORDERING_MASK_49_32: c_uint = 0xC1101C;
pub const mmMMU_UP_LOG2_DDR_SIZE: c_uint = 0xC11020;
pub const mmMMU_UP_SCRAMBLER: c_uint = 0xC11024;
pub const mmMMU_UP_MEM_INIT_BUSY: c_uint = 0xC11028;
pub const mmMMU_UP_SPI_MASK: c_uint = 0xC1102C;
pub const mmMMU_UP_SPI_CAUSE: c_uint = 0xC11030;
pub const mmMMU_UP_PAGE_ERROR_CAPTURE: c_uint = 0xC11034;
pub const mmMMU_UP_PAGE_ERROR_CAPTURE_VA: c_uint = 0xC11038;
pub const mmMMU_UP_ACCESS_ERROR_CAPTURE: c_uint = 0xC1103C;
pub const mmMMU_UP_ACCESS_ERROR_CAPTURE_VA: c_uint = 0xC11040;
pub const mmMMU_UP_SPI_INTERRUPT_CLR: c_uint = 0xC11044;
pub const mmMMU_UP_SPI_INTERRUPT_MASK: c_uint = 0xC11048;
pub const mmMMU_UP_DBG_MEM_WRAP_RM: c_uint = 0xC1104C;
pub const mmMMU_UP_SPI_CAUSE_CLR: c_uint = 0xC11050;
pub const mmMMU_UP_SLICE_CREDIT: c_uint = 0xC11054;
pub const mmMMU_UP_PIPE_CREDIT: c_uint = 0xC11058;
pub const mmMMU_UP_RAZWI_WRITE_VLD: c_uint = 0xC1105C;
pub const mmMMU_UP_RAZWI_WRITE_ID: c_uint = 0xC11060;
pub const mmMMU_UP_RAZWI_READ_VLD: c_uint = 0xC11064;
pub const mmMMU_UP_RAZWI_READ_ID: c_uint = 0xC11068;
pub const mmMMU_UP_MMU_BYPASS: c_uint = 0xC1106C;
