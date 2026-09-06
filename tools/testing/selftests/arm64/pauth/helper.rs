//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/arm64/pauth/helper.h
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
// Copyright (C) 2020 ARM Limited

pub const NKEYS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct signatures {
    pub keyia: usize,
    pub keyib: usize,
    pub keyda: usize,
    pub keydb: usize,
    pub keyg: usize,
}

extern "C" {
    pub fn pac_corruptor();
}
// PAuth sign a value with key ia and modifier value 0
extern "C" {
    pub fn keyia_sign(val: usize) -> usize;
}
extern "C" {
    pub fn keyib_sign(val: usize) -> usize;
}
extern "C" {
    pub fn keyda_sign(val: usize) -> usize;
}
extern "C" {
    pub fn keydb_sign(val: usize) -> usize;
}
extern "C" {
    pub fn keyg_sign(val: usize) -> usize;
}
