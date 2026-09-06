//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/streebog.h
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


// SPDX-License-Identifier: GPL-2.0+ OR BSD-2-Clause
//
// Copyright (c) 2013 Alexey Degtyarev <alexey@renatasystems.org>
// Copyright (c) 2018 Vitaly Chikunov <vt@altlinux.org>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//

pub const STREEBOG256_DIGEST_SIZE: c_int = 32;
pub const STREEBOG512_DIGEST_SIZE: c_int = 64;
pub const STREEBOG_BLOCK_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct streebog_uint512 {
    pub qword: [__le64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct streebog_state {
    pub hash: streebog_uint512,
    pub h: streebog_uint512,
    pub N: streebog_uint512,
    pub Sigma: streebog_uint512,
}
