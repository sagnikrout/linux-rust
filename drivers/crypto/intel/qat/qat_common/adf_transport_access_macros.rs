//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_transport_access_macros.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

pub const ADF_RING_CONFIG_NEAR_FULL_WM: c_uint = 0x0A;
pub const ADF_RING_CONFIG_NEAR_EMPTY_WM: c_uint = 0x05;
pub const ADF_COALESCING_MIN_TIME: c_uint = 0x1FF;
pub const ADF_COALESCING_MAX_TIME: c_uint = 0xFFFFF;
pub const ADF_COALESCING_DEF_TIME: c_uint = 0x27FF;
pub const ADF_RING_NEAR_WATERMARK_512: c_uint = 0x08;
pub const ADF_RING_NEAR_WATERMARK_0: c_uint = 0x00;
pub const ADF_RING_EMPTY_SIG: c_uint = 0x7F7F7F7F;
// Valid internal ring size values
pub const ADF_RING_SIZE_128: c_uint = 0x01;
pub const ADF_RING_SIZE_256: c_uint = 0x02;
pub const ADF_RING_SIZE_512: c_uint = 0x03;
pub const ADF_RING_SIZE_4K: c_uint = 0x06;
pub const ADF_RING_SIZE_16K: c_uint = 0x08;
pub const ADF_RING_SIZE_4M: c_uint = 0x10;

// Valid internal msg size values
pub const ADF_MSG_SIZE_32: c_uint = 0x01;
pub const ADF_MSG_SIZE_64: c_uint = 0x02;
pub const ADF_MSG_SIZE_128: c_uint = 0x04;

// Size to bytes conversion macros for ring and msg size values

// Minimum ring buffer size for memory allocation

// Max outstanding requests

