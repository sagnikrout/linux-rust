//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/polynomial.h
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
// Copyright (C) 2020 BAIKAL ELECTRONICS, JSC
//
// struct polynomial_term - one term descriptor of a polynomial
// @deg: degree of the term.
// @coef: multiplication factor of the term.
// @divider: distributed divider per each degree.
// @divider_leftover: divider leftover, which couldn't be redistributed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct polynomial_term {
    pub deg: c_uint,
    pub coef: c_long,
    pub divider: c_long,
    pub divider_leftover: c_long,
}

//
// struct polynomial - a polynomial descriptor
// @total_divider: total data divider.
// @terms: polynomial terms, last term must have degree of 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct polynomial {
    pub total_divider: c_long,
    pub terms: [polynomial_term; ],
}

extern "C" {
    pub fn polynomial_calc(poly: *const polynomial, data: c_long) -> c_long;
}
