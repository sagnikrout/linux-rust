//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/tls.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB)
//
// Copyright (c) 2016-2017, Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// TLS socket options

// Supported versions

pub const TLS_1_2_VERSION_MAJOR: c_uint = 0x3;
pub const TLS_1_2_VERSION_MINOR: c_uint = 0x3;

// Supported ciphers
pub const TLS_CIPHER_AES_GCM_128: c_int = 51;
pub const TLS_CIPHER_AES_GCM_128_IV_SIZE: c_int = 8;
pub const TLS_CIPHER_AES_GCM_128_KEY_SIZE: c_int = 16;
pub const TLS_CIPHER_AES_GCM_128_SALT_SIZE: c_int = 4;
pub const TLS_CIPHER_AES_GCM_128_TAG_SIZE: c_int = 16;
pub const TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE: c_int = 8;
pub const TLS_SET_RECORD_TYPE: c_int = 1;
pub const TLS_GET_RECORD_TYPE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_crypto_info {
    pub version: __u16,
    pub cipher_type: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls12_crypto_info_aes_gcm_128 {
    pub info: tls_crypto_info,
    pub iv: [c_uchar; TLS_CIPHER_AES_GCM_128_IV_SIZE],
    pub key: [c_uchar; TLS_CIPHER_AES_GCM_128_KEY_SIZE],
    pub salt: [c_uchar; TLS_CIPHER_AES_GCM_128_SALT_SIZE],
    pub rec_seq: [c_uchar; TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE],
}
