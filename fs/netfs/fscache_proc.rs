//! Automatically rewritten from C to Rust
//! Source: fs/netfs/fscache_proc.c
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
// FS-Cache statistics viewing interface
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Add files to /proc/fs/netfs/.
//
#[no_mangle]
pub unsafe extern "C" fn fscache_proc_init() -> int __init {
    int __init fscache_proc_init(void)
    {
    if (!proc_symlink("fs/fscache", core::ptr::null_mut(), "netfs"))
    goto error_sym;
    if (!proc_create_seq("fs/netfs/caches", S_IFREG | 0444, core::ptr::null_mut(),
    &fscache_caches_seq_ops))
    goto error;
    if (!proc_create_seq("fs/netfs/volumes", S_IFREG | 0444, core::ptr::null_mut(),
    &fscache_volumes_seq_ops))
    goto error;
    if (!proc_create_seq("fs/netfs/cookies", S_IFREG | 0444, core::ptr::null_mut(),
    &fscache_cookies_seq_ops))
    goto error;
    return 0;
    error:
    remove_proc_entry("fs/fscache", core::ptr::null_mut());
    error_sym:
    return -ENOMEM;
    }
//
// Clean up the /proc/fs/fscache symlink.
//
#[no_mangle]
pub unsafe extern "C" fn fscache_proc_cleanup() {
    void fscache_proc_cleanup(void)
    {
    remove_proc_subtree("fs/fscache", core::ptr::null_mut());
    }
