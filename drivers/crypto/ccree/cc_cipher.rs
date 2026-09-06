//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccree/cc_cipher.h
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
// Copyright (C) 2012-2019 ARM Limited (or its affiliates).
// \file cc_cipher.h
// ARM CryptoCell Cipher Crypto API
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cipher_req_ctx {
    pub gen_ctx: async_gen_req_ctx,
    pub dma_buf_type: cc_req_dma_buf_type,
    pub in_nents: u32,
    pub in_mlli_nents: u32,
    pub out_nents: u32,
    pub out_mlli_nents: u32,
    pub iv: *mut u8,
    pub mlli_params: mlli_params,
}

extern "C" {
    pub fn cc_cipher_alloc(drvdata: *mut cc_drvdata) -> c_int;
}
extern "C" {
    pub fn cc_cipher_free(drvdata: *mut cc_drvdata) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_hkey_info {
    pub keylen: u16,
    pub hw_key1: u8,
    pub hw_key2: u8,
    pub __packed: },

