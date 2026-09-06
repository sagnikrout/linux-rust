//! Automatically rewritten from C Header to Rust Module
//! Source: crypto/aegis.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AEGIS common definitions
//
// Copyright (c) 2018 Ondrej Mosnacek <omosnacek@gmail.com>
// Copyright (c) 2018 Red Hat, Inc. All rights reserved.
//

pub const AEGIS_BLOCK_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub union aegis_block {
    pub sizeof(__le64)]: __le64 words64[AEGIS_BLOCK_SIZE /,
    pub sizeof(__le32)]: __le32 words32[AEGIS_BLOCK_SIZE /,
    pub bytes: [u8; AEGIS_BLOCK_SIZE],
}

extern "C" {
    pub fn crypto_aegis128_have_simd() -> bool;
}
extern "C" {
    pub fn crypto_aegis128_update_simd(state: *mut aegis_state, msg: *const c_void);
}
