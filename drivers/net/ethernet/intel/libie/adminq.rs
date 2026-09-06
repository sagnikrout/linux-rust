//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/libie/adminq.c
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
// Copyright (C) 2025 Intel Corporation

    static const char * const libie_aq_str_arr[] = {

    [LIBIE_AQ_RC_##x]	= "LIBIE_AQ_RC_" #x
    LIBIE_AQ_STR(OK),
    LIBIE_AQ_STR(EPERM),
    LIBIE_AQ_STR(ENOENT),
    LIBIE_AQ_STR(ESRCH),
    LIBIE_AQ_STR(EIO),
    LIBIE_AQ_STR(EAGAIN),
    LIBIE_AQ_STR(ENOMEM),
    LIBIE_AQ_STR(EACCES),
    LIBIE_AQ_STR(EBUSY),
    LIBIE_AQ_STR(EEXIST),
    LIBIE_AQ_STR(EINVAL),
    LIBIE_AQ_STR(ENOSPC),
    LIBIE_AQ_STR(ENOSYS),
    LIBIE_AQ_STR(EMODE),
    LIBIE_AQ_STR(ENOSEC),
    LIBIE_AQ_STR(EBADSIG),
    LIBIE_AQ_STR(ESVN),
    LIBIE_AQ_STR(EBADMAN),
    LIBIE_AQ_STR(EBADBUF),

    "LIBIE_AQ_RC_UNKNOWN",
    };

//
// libie_aq_str - get error string based on aq error
// @err: admin queue error type
//
// Return: error string for passed error code
//
    const char *libie_aq_str(enum libie_aq_err err)
    {
    if (err >= ARRAY_SIZE(libie_aq_str_arr) ||
    !libie_aq_str_arr[err])
    err = __LIBIE_AQ_STR_NUM;
    return libie_aq_str_arr[err];
    }
    EXPORT_SYMBOL_NS_GPL(libie_aq_str, "LIBIE_ADMINQ");
    MODULE_DESCRIPTION("Intel(R) Ethernet common library - adminq helpers");
    MODULE_LICENSE("GPL");
