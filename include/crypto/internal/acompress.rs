//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/acompress.h
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
// Asynchronous Compression operations
//
// Copyright (c) 2016, Intel Corporation
// Authors: Weigang Li <weigang.li@intel.com>
// Giovanni Cabiddu <giovanni.cabiddu@intel.com>
//

//
// struct acomp_alg - asynchronous compression algorithm
//
// @compress:	Function performs a compress operation
// @decompress:	Function performs a de-compress operation
// @init:	Initialize the cryptographic transformation object.
// This function is used to initialize the cryptographic
// transformation object. This function is called only once at
// the instantiation time, right after the transformation context
// was allocated. In case the cryptographic hardware has some
// special requirements which need to be handled by software, this
// function shall check for the precise requirement of the
// transformation and put any software fallbacks in place.
// @exit:	Deinitialize the cryptographic transformation object. This is a
// counterpart to @init, used to remove various changes set in
// @init.
//
// @base:	Common crypto API algorithm data structure
// @calg:	Cmonn algorithm data structure shared with scomp
// @COMP_ALG_COMMON: see struct comp_alg_common
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acomp_alg {
    pub req): *mut *mut int (compress)(struct acomp_req,
    pub req): *mut *mut int (decompress)(struct acomp_req,
    pub tfm): *mut *mut int (init)(struct crypto_acomp,
    pub tfm): *mut *mut void (exit)(struct crypto_acomp,
    pub COMP_ALG_COMMON: struct,
    pub calg: comp_alg_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_acomp_stream {
    pub lock: spinlock_t,
    pub ctx: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_acomp_streams {
// These must come first because of struct scomp_alg.
    pub (*alloc_ctx)(void): *mut c_void,
    pub ): *mut *mut void (free_ctx)(void,
    pub streams: *mut crypto_acomp_stream __percpu,
    pub stream_work: work_struct,
    pub stream_want: cpumask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acomp_walk {
// Virtual address of the source.
    pub addr: *const *const c_void,
    pub virt: },
    pub src: },
// Private field for the API, do not use.
    pub in: scatter_walk,
}

// Virtual address of the destination.
// Private field for the API, do not use.
//
// Transform internal helpers.
//
// crypto_register_acomp() -- Register asynchronous compression algorithm
//
// Function registers an implementation of an asynchronous
// compression algorithm
//
// @alg:	algorithm definition
//
// Return:	zero on success; error code in case of error
//
extern "C" {
    pub fn crypto_register_acomp(alg: *mut acomp_alg) -> c_int;
}
//
// crypto_unregister_acomp() -- Unregister asynchronous compression algorithm
//
// Function unregisters an implementation of an asynchronous
// compression algorithm
//
// @alg:	algorithm definition
//
extern "C" {
    pub fn crypto_unregister_acomp(alg: *mut acomp_alg);
}
extern "C" {
    pub fn crypto_register_acomps(algs: *mut acomp_alg, count: c_int) -> c_int;
}
extern "C" {
    pub fn crypto_unregister_acomps(algs: *mut acomp_alg, count: c_int);
}
extern "C" {
    pub fn crypto_tfm_req_virt(_arg: &tfm->base) -> return;
}
extern "C" {
    pub fn crypto_acomp_free_streams(s: *mut crypto_acomp_streams);
}
extern "C" {
    pub fn crypto_acomp_alloc_streams(s: *mut crypto_acomp_streams) -> c_int;
}

extern "C" {
    pub fn acomp_walk_done_src(walk: *mut acomp_walk, used: c_int);
}
extern "C" {
    pub fn acomp_walk_done_dst(walk: *mut acomp_walk, used: c_int);
}
extern "C" {
    pub fn acomp_walk_next_src(walk: *mut acomp_walk) -> c_int;
}
extern "C" {
    pub fn acomp_walk_next_dst(walk: *mut acomp_walk) -> c_int;
}
extern "C" {
    pub fn __crypto_acomp_tfm(_arg: crypto_acomp_tfm(tfm)->fb) -> return;
}
