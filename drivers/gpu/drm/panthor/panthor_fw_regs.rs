//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_fw_regs.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
// Copyright 2026 ARM Limited. All rights reserved.
pub const MCU_CONTROL_BASE: c_uint = 0x700;
pub const MCU_CONTROL: c_uint = 0x0;
pub const MCU_CONTROL_ENABLE: c_int = 1;
pub const MCU_CONTROL_AUTO: c_int = 2;
pub const MCU_CONTROL_DISABLE: c_int = 0;
pub const MCU_STATUS: c_uint = 0x4;
pub const MCU_STATUS_DISABLED: c_int = 0;
pub const MCU_STATUS_ENABLED: c_int = 1;
pub const MCU_STATUS_HALT: c_int = 2;
pub const MCU_STATUS_FATAL: c_int = 3;
pub const JOB_INT_BASE: c_uint = 0x1000;

pub const CSF_GPU_LATEST_FLUSH_ID: c_uint = 0x10000;

pub const CSF_GLB_DOORBELL_ID: c_int = 0;
