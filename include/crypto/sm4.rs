//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/sm4.h
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
// Common values for the SM4 algorithm
// Copyright (C) 2018 ARM Limited or its affiliates.
// Copyright (c) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
//

pub const SM4_KEY_SIZE: c_int = 16;
pub const SM4_BLOCK_SIZE: c_int = 16;
pub const SM4_RKEY_WORDS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm4_ctx {
    pub rkey_enc: [u32; SM4_RKEY_WORDS],
    pub rkey_dec: [u32; SM4_RKEY_WORDS],
}

//
// sm4_expandkey - Expands the SM4 key as described in GB/T 32907-2016
// @ctx:	The location where the computed key will be stored.
// @in_key:	The supplied key.
// @key_len:	The length of the supplied key.
//
// Returns 0 on success. The function fails only if an invalid key size (or
// pointer) is supplied.
//
// sm4_crypt_block - Encrypt or decrypt a single SM4 block
// @rk:		The rkey_enc for encrypt or rkey_dec for decrypt
// @out:	Buffer to store output data
// @in: 	Buffer containing the input data
//
extern "C" {
    pub fn sm4_crypt_block(rk: *const u32, out: *mut u8, in: *const u8);
}
