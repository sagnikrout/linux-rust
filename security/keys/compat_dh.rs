//! Automatically rewritten from C to Rust
//! Source: security/keys/compat_dh.c
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
// 32-bit compatibility syscall for 64-bit systems for DH operations
//
// Copyright (C) 2016 Stephan Mueller <smueller@chronox.de>
//

//
// Perform the DH computation or DH based key derivation.
//
// If successful, 0 will be returned.
//
    long compat_keyctl_dh_compute(struct keyctl_dh_params __user *params,
    char __user *buffer, size_t buflen,
    struct compat_keyctl_kdf_params __user *kdf)
    {
    struct keyctl_kdf_params kdfcopy;
    struct compat_keyctl_kdf_params compat_kdfcopy;
    if (!kdf)
    return __keyctl_dh_compute(params, buffer, buflen, core::ptr::null_mut());
    if (copy_from_user(&compat_kdfcopy, kdf, sizeof(compat_kdfcopy)) != 0)
    return -EFAULT;
    kdfcopy.hashname = compat_ptr(compat_kdfcopy.hashname);
    kdfcopy.otherinfo = compat_ptr(compat_kdfcopy.otherinfo);
    kdfcopy.otherinfolen = compat_kdfcopy.otherinfolen;
    memcpy(kdfcopy.__spare, compat_kdfcopy.__spare,
    sizeof(kdfcopy.__spare));
    return __keyctl_dh_compute(params, buffer, buflen, &kdfcopy);
    }
