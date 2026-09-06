//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_alg.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// if_alg: User-space algorithm interface
//
// Copyright (c) 2010 Herbert Xu <herbert@gondor.apana.org.au>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_alg {
    pub salg_family: __u16,
    pub salg_type: [__u8; 14],
    pub salg_feat: __u32,
    pub salg_mask: __u32,
    pub salg_name: [__u8; 64],
}

//
// Linux v4.12 and later removed the 64-byte limit on salg_name[]; it's now an
// arbitrary-length field.  We had to keep the original struct above for source
// compatibility with existing userspace programs, though.  Use the new struct
// below if support for very long algorithm names is needed.  To do this,
// allocate 'sizeof(struct sockaddr_alg_new) + strlen(algname) + 1' bytes, and
// copy algname (including the null terminator) into salg_name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_alg_new {
    pub salg_family: __u16,
    pub salg_type: [__u8; 14],
    pub salg_feat: __u32,
    pub salg_mask: __u32,
    pub salg_name: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct af_alg_iv {
    pub ivlen: __u32,
    pub __counted_by(ivlen): __u8 iv[],
}

// Socket options
pub const ALG_SET_KEY: c_int = 1;
pub const ALG_SET_IV: c_int = 2;
pub const ALG_SET_OP: c_int = 3;
pub const ALG_SET_AEAD_ASSOCLEN: c_int = 4;
pub const ALG_SET_AEAD_AUTHSIZE: c_int = 5;
pub const ALG_SET_DRBG_ENTROPY: c_int = 6;
pub const ALG_SET_KEY_BY_KEY_SERIAL: c_int = 7;
// Operations
pub const ALG_OP_DECRYPT: c_int = 0;
pub const ALG_OP_ENCRYPT: c_int = 1;
