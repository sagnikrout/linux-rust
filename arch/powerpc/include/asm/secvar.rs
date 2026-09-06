//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/secvar.h
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
// Copyright (C) 2019 IBM Corporation
// Author: Nayna Jain
//
// PowerPC secure variable operations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct secvar_operations {
    pub data_size): *const *const *const *const int (get)(char key, u64 key_len, u8 data, u64,
    pub keybufsize): *const *const *const *const int (get_next)(char key, u64 key_len, u64,
    pub data_size): *const *const *const *const int (set)(char key, u64 key_len, u8 data, u64,
    pub bufsize): *mut *mut *mut ssize_t (format)(char buf, size_t,
    pub max_size): *mut *mut int (max_size)(u64,
// NULL-terminated array of fixed variable names
// Only used if get_next() isn't provided
    pub var_names: *const *const c_char,
}

extern "C" {
    pub fn set_secvar_ops(ops: *const secvar_operations) -> c_int;
}

