//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/issei/hw_heci_regs.h
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
// Copyright (C) 2023-2026 Intel Corporation

// H_CB_WW - Host Circular Buffer (CB) Write Window register
pub const H_CB_WW: c_uint = 0x0;
// H_CSR - Host Control Status register
pub const H_CSR: c_uint = 0x4;

// FW_CB_RW - FW Circular Buffer Read Window register (read only)
pub const FW_CB_RW: c_uint = 0x8;
// FW_CSR_HA - FW Control Status Host Access register (read only)
pub const FW_CSR_HA: c_uint = 0xC;

