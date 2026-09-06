//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/scompress.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Synchronous Compression operations
//
// Copyright 2015 LG Electronics Inc.
// Copyright (c) 2016, Intel Corporation
// Author: Giovanni Cabiddu <giovanni.cabiddu@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_scomp {
    pub base: crypto_tfm,
}

//
// struct scomp_alg - synchronous compression algorithm
//
// @compress:	Function performs a compress operation
// @decompress:	Function performs a de-compress operation
// @streams:	Per-cpu memory for algorithm
// @calg:	Cmonn algorithm data structure shared with acomp
// @COMP_ALG_COMMON: see struct comp_alg_common
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scomp_alg {
    pub ctx): *mut c_void,
    pub ctx): *mut c_void,
    pub streams: crypto_acomp_streams,
    pub COMP_ALG_COMMON: struct,
    pub calg: comp_alg_common,
}

extern "C" {
    pub fn container_of(_arg: alg, scomp_alg: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: tfm, crypto_scomp: struct, _arg: base) -> return;
}
extern "C" {
    pub fn __crypto_scomp_alg(_arg: crypto_scomp_tfm(tfm)->__crt_alg) -> return;
}
extern "C" {
    pub fn crypto_scomp_alg(_arg: tfm)->compress(tfm, _arg: src, _arg: slen, _arg: dst, _arg: dlen, _arg: ctx) -> return;
}
//
// crypto_register_scomp() -- Register synchronous compression algorithm
//
// Function registers an implementation of a synchronous
// compression algorithm
//
// @alg:	algorithm definition
//
// Return: zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_register_scomp(alg: *mut scomp_alg) -> c_int;
}
//
// crypto_unregister_scomp() -- Unregister synchronous compression algorithm
//
// Function unregisters an implementation of a synchronous
// compression algorithm
//
// @alg:	algorithm definition
//
extern "C" {
    pub fn crypto_unregister_scomp(alg: *mut scomp_alg);
}
extern "C" {
    pub fn crypto_register_scomps(algs: *mut scomp_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_scomps(algs: *mut scomp_alg, count: c_int);
}
