//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/cc_platform.c
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
// Confidential Computing Platform Capability checks
//
// Copyright (C) 2021 Advanced Micro Devices, Inc.
//
// Author: Tom Lendacky <thomas.lendacky@amd.com>
//

#[no_mangle]
pub unsafe extern "C" fn cc_platform_has(attr: enum cc_attr) -> bool {
    bool cc_platform_has(enum cc_attr attr)
    {
    switch (attr) {
    case CC_ATTR_MEM_ENCRYPT:
    case CC_ATTR_GUEST_MEM_ENCRYPT:
    return is_secure_guest();
    default:
    return false;
    }
    }
    EXPORT_SYMBOL_GPL(cc_platform_has);
