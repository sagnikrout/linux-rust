//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/inc/dml2_debug.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// private helper macros

// fatal errors for unrecoverable DML states until a full reset
pub const DML_LOG_LEVEL_FATAL: c_int = 0;
// unexpected but recoverable failures inside DML
pub const DML_LOG_LEVEL_ERROR: c_int = 1;
// unexpected inputs or events to DML
pub const DML_LOG_LEVEL_WARN: c_int = 2;
// high level tracing of DML interfaces
pub const DML_LOG_LEVEL_INFO: c_int = 3;
// tracing of DML internal executions
pub const DML_LOG_LEVEL_DEBUG: c_int = 4;
// detailed tracing of DML calculation procedure
pub const DML_LOG_LEVEL_VERBOSE: c_int = 5;

// public macros for DML_LOG_LEVEL_FATAL and up

// public macros for DML_LOG_LEVEL_ERROR and up

// public macros for DML_LOG_LEVEL_WARN and up

// public macros for DML_LOG_LEVEL_INFO and up

// public macros for DML_LOG_LEVEL_DEBUG and up

// public macros for DML_LOG_LEVEL_VERBOSE

