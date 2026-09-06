//! Automatically rewritten from C Header to Rust Module
//! Source: include/keys/encrypted-type.h
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
// Copyright (C) 2010 IBM Corporation
// Copyright (C) 2010 Politecnico di Torino, Italy
// TORSEC group -- https://security.polito.it
//
// Authors:
// Mimi Zohar <zohar@us.ibm.com>
// Roberto Sassu <roberto.sassu@polito.it>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encrypted_key_payload {
    pub rcu: rcu_head,
    pub /: *mut *mut *mut char format; / datablob: format,
    pub /: *mut *mut *mut char master_desc; / datablob: master key name,
    pub /: *mut *mut *mut char datalen; / datablob: decrypted key length,
    pub /: *mut *mut *mut u8 iv; / datablob: iv,
    pub /: *mut *mut *mut u8 encrypted_data; / datablob: encrypted data,
    pub /: *mut *mut unsigned short datablob_len; / length of datablob,
    pub /: *mut *mut unsigned short decrypted_datalen; / decrypted data length,
    pub /: *mut *mut unsigned short payload_datalen; / payload data length,
    pub /: *mut *mut unsigned short encrypted_key_format; / encrypted key format,
    pub /: *mut *mut *mut u8 decrypted_data; / decrypted data,
    pub /: *mut *mut u8 payload_data[]; / payload data + datablob + hmac,
}
