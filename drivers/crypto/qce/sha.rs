//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/qce/sha.h
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
// Copyright (c) 2010-2014, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_sha_ctx {
    pub authkey: [u8; QCE_SHA_MAX_BLOCKSIZE],
}

//
// struct qce_sha_reqctx - holds private ahash objects per request
// @buf: used during update, import and export
// @tmpbuf: buffer for internal use
// @digest: calculated digest buffer
// @buflen: length of the buffer
// @flags: operation flags
// @src_orig: original request sg list
// @nbytes_orig: original request number of bytes
// @src_nents: source number of entries
// @byte_count: byte count
// @count: save count in states during update, import and export
// @first_blk: is it the first block
// @last_blk: is it the last block
// @sg: used to chain sg lists
// @authkey: pointer to auth key in sha ctx
// @authklen: auth key length
// @result_sg: scatterlist used for result buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qce_sha_reqctx {
    pub buf: [u8; QCE_SHA_MAX_BLOCKSIZE],
    pub tmpbuf: [u8; QCE_SHA_MAX_BLOCKSIZE],
    pub digest: [u8; QCE_SHA_MAX_DIGESTSIZE],
    pub buflen: c_uint,
    pub flags: c_ulong,
    pub src_orig: *mut scatterlist,
    pub nbytes_orig: c_uint,
    pub src_nents: c_int,
    pub byte_count: [__be32; 2],
    pub count: u64,
    pub first_blk: bool,
    pub last_blk: bool,
    pub sg: [scatterlist; 2],
    pub authkey: *mut u8,
    pub authklen: c_uint,
    pub result_sg: scatterlist,
}

extern "C" {
    pub fn container_of(_arg: alg, qce_alg_template: struct, _arg: alg.ahash) -> return;
}
