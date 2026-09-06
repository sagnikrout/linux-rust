//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/mmu_regs.h
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
// MMU (Prototype: MMU)
//
pub const mmMMU_INPUT_FIFO_THRESHOLD: c_uint = 0x480000;
pub const mmMMU_MMU_ENABLE: c_uint = 0x48000C;
pub const mmMMU_FORCE_ORDERING: c_uint = 0x480010;
pub const mmMMU_FEATURE_ENABLE: c_uint = 0x480014;
pub const mmMMU_VA_ORDERING_MASK_31_7: c_uint = 0x480018;
pub const mmMMU_VA_ORDERING_MASK_49_32: c_uint = 0x48001C;
pub const mmMMU_LOG2_DDR_SIZE: c_uint = 0x480020;
pub const mmMMU_SCRAMBLER: c_uint = 0x480024;
pub const mmMMU_MEM_INIT_BUSY: c_uint = 0x480028;
pub const mmMMU_SPI_MASK: c_uint = 0x48002C;
pub const mmMMU_SPI_CAUSE: c_uint = 0x480030;
pub const mmMMU_PAGE_ERROR_CAPTURE: c_uint = 0x480034;
pub const mmMMU_PAGE_ERROR_CAPTURE_VA: c_uint = 0x480038;
pub const mmMMU_ACCESS_ERROR_CAPTURE: c_uint = 0x48003C;
pub const mmMMU_ACCESS_ERROR_CAPTURE_VA: c_uint = 0x480040;
