//! Automatically rewritten from C to Rust
//! Source: security/integrity/evm/evm_posix_acl.c
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
// Copyright (C) 2011 IBM Corporation
//
// Author:
// Mimi Zohar <zohar@us.ibm.com>
//

#[no_mangle]
pub unsafe extern "C" fn posix_xattr_acl(xattr: *const c_char) -> c_int {
    int posix_xattr_acl(const char *xattr)
    {
    let mut xattr_len: c_int = strlen(xattr);
    if ((strlen(XATTR_NAME_POSIX_ACL_ACCESS) == xattr_len)
    && (strncmp(XATTR_NAME_POSIX_ACL_ACCESS, xattr, xattr_len) == 0))
    return 1;
    if ((strlen(XATTR_NAME_POSIX_ACL_DEFAULT) == xattr_len)
    && (strncmp(XATTR_NAME_POSIX_ACL_DEFAULT, xattr, xattr_len) == 0))
    return 1;
    return 0;
    }
