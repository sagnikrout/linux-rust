//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/dcp.h
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
// Copyright (C) 2021 sigma star gmbh
//
// Specifies paes key slot handles for NXP's DCP (Data Co-Processor) to be used
// with the crypto_skcipher_setkey().
//
pub const DCP_PAES_KEYSIZE: c_int = 1;
pub const DCP_PAES_KEY_SLOT0: c_uint = 0x00;
pub const DCP_PAES_KEY_SLOT1: c_uint = 0x01;
pub const DCP_PAES_KEY_SLOT2: c_uint = 0x02;
pub const DCP_PAES_KEY_SLOT3: c_uint = 0x03;
pub const DCP_PAES_KEY_UNIQUE: c_uint = 0xfe;
pub const DCP_PAES_KEY_OTP: c_uint = 0xff;
