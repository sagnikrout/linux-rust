//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/digsig.h
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
// Copyright (C) 2011 Nokia Corporation
// Copyright (C) 2011 Intel Corporation
//
// Author:
// Dmitry Kasatkin <dmitry.kasatkin@nokia.com>
// <dmitry.kasatkin@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pubkey_algo {
    PUBKEY_ALGO_RSA,
    PUBKEY_ALGO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum digest_algo {
    DIGEST_ALGO_SHA1,
    DIGEST_ALGO_SHA256,
    DIGEST_ALGO_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pubkey_hdr {
    pub /: *mut *mut uint8_t version; / key format version,
    pub /: *mut *mut uint32_t timestamp; / key made, always 0 for now,
    pub algo: u8,
    pub nmpi: u8,
    pub mpi: [c_char; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct signature_hdr {
    pub /: *mut *mut uint8_t version; / signature format version,
    pub /: *mut *mut uint32_t timestamp; / signature made,
    pub algo: u8,
    pub hash: u8,
    pub keyid: [u8; 8],
    pub nmpi: u8,
    pub mpi: [c_char; ],
    pub __packed: },

    pub digestlen): *const *const char digest, int,

    pub -EOPNOTSUPP: return,

