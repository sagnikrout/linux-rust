//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/camss/camss-csid-gen2.h
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
// camss-csid-gen1.h
//
// Qualcomm MSM Camera Subsystem - CSID (CSI Decoder) Module Generation 1
//
// Copyright (C) 2021 Linaro Ltd.
//
pub const DECODE_FORMAT_UNCOMPRESSED_6_BIT: c_uint = 0x0;
pub const DECODE_FORMAT_UNCOMPRESSED_8_BIT: c_uint = 0x1;
pub const DECODE_FORMAT_UNCOMPRESSED_10_BIT: c_uint = 0x2;
pub const DECODE_FORMAT_UNCOMPRESSED_12_BIT: c_uint = 0x3;
pub const DECODE_FORMAT_UNCOMPRESSED_14_BIT: c_uint = 0x4;
pub const DECODE_FORMAT_UNCOMPRESSED_16_BIT: c_uint = 0x5;
pub const DECODE_FORMAT_UNCOMPRESSED_20_BIT: c_uint = 0x6;
pub const DECODE_FORMAT_DPCM_10_6_10: c_uint = 0x7;
pub const DECODE_FORMAT_DPCM_10_8_10: c_uint = 0x8;
pub const DECODE_FORMAT_DPCM_12_6_12: c_uint = 0x9;
pub const DECODE_FORMAT_DPCM_12_8_12: c_uint = 0xa;
pub const DECODE_FORMAT_DPCM_14_8_14: c_uint = 0xb;
pub const DECODE_FORMAT_DPCM_14_10_14: c_uint = 0xc;
pub const DECODE_FORMAT_DPCM_12_10_12: c_uint = 0xd;
pub const DECODE_FORMAT_USER_DEFINED: c_uint = 0xe;
pub const DECODE_FORMAT_PAYLOAD_ONLY: c_uint = 0xf;
pub const ENCODE_FORMAT_RAW_8_BIT: c_uint = 0x1;
pub const ENCODE_FORMAT_RAW_10_BIT: c_uint = 0x2;
pub const ENCODE_FORMAT_RAW_12_BIT: c_uint = 0x3;
pub const ENCODE_FORMAT_RAW_14_BIT: c_uint = 0x4;
pub const ENCODE_FORMAT_RAW_16_BIT: c_uint = 0x5;
pub const PLAIN_FORMAT_PLAIN8: c_uint = 0x0 /* supports DPCM, UNCOMPRESSED_6/8_BIT */;
pub const PLAIN_FORMAT_PLAIN16: c_uint = 0x1 /* supports DPCM, UNCOMPRESSED_10/16_BIT */;
pub const PLAIN_FORMAT_PLAIN32: c_uint = 0x2 /* supports UNCOMPRESSED_20_BIT */;
