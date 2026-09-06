//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/secvar-ops.c
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
// This file initializes secvar operations for PowerPC Secureboot
//

    let mut __ro_after_init: *const secvar_operations secvar_ops = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn set_secvar_ops(ops: *const secvar_operations) -> c_int {
    int set_secvar_ops(const struct secvar_operations *ops)
    {
    if (WARN_ON_ONCE(secvar_ops))
    return -EBUSY;
    secvar_ops = ops;
    return 0;
    }
