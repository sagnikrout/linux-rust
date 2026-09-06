//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/inside-secure/eip93/eip93-cipher.h
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
// Copyright (C) 2019 - 2021
//
// Richard van Schagen <vschagen@icloud.com>
// Christian Marangi <ansuelsmth@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eip93_crypto_ctx {
    pub eip93: *mut eip93_device,
    pub flags: u32,
    pub sa_record: *mut sa_record,
    pub sa_nonce: u32,
    pub blksize: c_int,
    pub sa_record_base: dma_addr_t,
// AEAD specific
    pub authsize: c_uint,
    pub assoclen: c_uint,
    pub set_assoc: bool,
    pub type: eip93_alg_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eip93_cipher_reqctx {
    pub desc_flags: u16,
    pub flags: u16,
    pub blksize: c_uint,
    pub ivsize: c_uint,
    pub textsize: c_uint,
    pub assoclen: c_uint,
    pub authsize: c_uint,
    pub sa_record_base: dma_addr_t,
    pub sa_state: *mut sa_state,
    pub sa_state_base: dma_addr_t,
    pub cdesc: *mut eip93_descriptor,
    pub sg_src: *mut scatterlist,
    pub sg_dst: *mut scatterlist,
    pub src_nents: c_int,
    pub dst_nents: c_int,
    pub sa_state_ctr: *mut sa_state,
    pub sa_state_ctr_base: dma_addr_t,
}

extern "C" {
    pub fn check_valid_request(rctx: *mut eip93_cipher_reqctx) -> c_int;
}
extern "C" {
    pub fn eip93_skcipher_handle_result(async: *mut crypto_async_request, err: c_int);
}
