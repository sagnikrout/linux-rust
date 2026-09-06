//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/atmel-authenc.h
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
// API for Atmel Secure Protocol Layers Improved Performances (SPLIP)
//
// Copyright (C) 2016 Atmel Corporation
//
// Author: Cyrille Pitchen <cyrille.pitchen@atmel.com>
//
// This driver is based on drivers/mtd/spi-nor/fsl-quadspi.c from Freescale.
//

extern "C" {
    pub fn int(: *mut *mut atmel_aes_authenc_fn_t)(struct atmel_aes_dev, _arg: c_int, _arg: bool) -> typedef;
}
extern "C" {
    pub fn atmel_sha_authenc_is_ready() -> bool;
}
extern "C" {
    pub fn atmel_sha_authenc_get_reqsize() -> c_uint;
}
extern "C" {
    pub fn atmel_sha_authenc_free(auth: *mut atmel_sha_authenc_ctx);
}
extern "C" {
    pub fn atmel_sha_authenc_abort(req: *mut ahash_request);
}

