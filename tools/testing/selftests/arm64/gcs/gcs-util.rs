//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/arm64/gcs/gcs-util.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2023 ARM Limited.
//

pub const __NR_map_shadow_stack: c_int = 453;

pub const __NR_prctl: c_int = 167;

pub const NT_ARM_GCS: c_uint = 0x410;

// Shadow Stack/Guarded Control Stack interface
pub const PR_GET_SHADOW_STACK_STATUS: c_int = 74;
pub const PR_SET_SHADOW_STACK_STATUS: c_int = 75;
pub const PR_LOCK_SHADOW_STACK_STATUS: c_int = 76;

pub const GCS_CAP_VALID_TOKEN: c_int = 1;
pub const GCS_CAP_IN_PROGRESS_TOKEN: c_int = 5;

// CHKFEAT x16
