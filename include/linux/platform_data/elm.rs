//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/elm.h
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
// BCH Error Location Module
//
// Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bch_ecc {
    BCH4_ECC = 0,
    BCH8_ECC,
    BCH16_ECC,
}

// ELM support 8 error syndrome process
pub const ERROR_VECTOR_MAX: c_int = 8;
//
// struct elm_errorvec - error vector for elm
// @error_reported:		set true for vectors error is reported
// @error_uncorrectable:	number of uncorrectable errors
// @error_count:		number of correctable errors in the sector
// @error_loc:			buffer for error location
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elm_errorvec {
    pub error_reported: bool,
    pub error_uncorrectable: bool,
    pub error_count: c_int,
    pub error_loc: [c_int; 16],
}

