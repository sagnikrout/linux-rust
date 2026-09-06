//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/arm64/signal/testcases/testcases.h
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
// Copyright (C) 2019 ARM Limited

// Architecture specific sigframe definitions

pub const KSFT_BAD_MAGIC: c_uint = 0xdeadbeef;

//
// A simple record-walker for __reserved area: it walks through assuming
// only to find a proper struct __aarch64_ctx header descriptor.
//
// Instead it makes no assumptions on the content and ordering of the
// records, any needed bounds checking must be enforced by the caller
// if wanted: this way can be used by caller on any maliciously built bad
// contexts.
//
// head->size accounts both for payload and header _aarch64_ctx size !
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fake_sigframe {
    pub info: siginfo_t,
    pub uc: ucontext_t,
}

extern "C" {
    pub fn validate_reserved(uc: *mut ucontext_t, resv_sz: usize, err: *mut c_char) -> bool;
}
// offset = offs;
extern "C" {
    pub fn get_header(_arg: head, _arg: 0, _arg: resv_sz, _arg: offset) -> return;
}
