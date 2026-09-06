//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/key_gen.h
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
// CAAM/SEC 4.x definitions for handling key-generation jobs
//
// Copyright 2008-2011 Freescale Semiconductor, Inc.
//
// split_key_len - Compute MDHA split key length for a given algorithm
// @hash: Hashing algorithm selection, one of OP_ALG_ALGSEL_* - MD5, SHA1,
// SHA224, SHA384, SHA512.
//
// Return: MDHA split key length
//
// Sizes for MDHA pads (*not* keys): MD5, SHA1, 224, 256, 384, 512
//
// split_key_pad_len - Compute MDHA split key pad length for a given algorithm
// @hash: Hashing algorithm selection, one of OP_ALG_ALGSEL_* - MD5, SHA1,
// SHA224, SHA384, SHA512.
//
// Return: MDHA split key pad length
//
extern "C" {
    pub fn ALIGN(_arg: split_key_len(hash), _arg: 16) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct split_key_result {
    pub completion: completion,
    pub err: c_int,
}

extern "C" {
    pub fn split_key_done(dev: *mut device, desc: *mut u32, err: u32, context: *mut c_void);
}
