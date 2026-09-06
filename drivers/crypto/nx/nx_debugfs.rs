//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/nx/nx_debugfs.c
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
// debugfs routines supporting the Power 7+ Nest Accelerators driver
//
// Copyright (C) 2011-2012 International Business Machines Inc.
//
// Author: Kent Yoder <yoder1@us.ibm.com>
//

//
// debugfs
//
// For documentation on these attributes, please see:
//
// Documentation/ABI/testing/debugfs-pfo-nx-crypto
//
#[no_mangle]
pub unsafe extern "C" fn nx_debugfs_init(drv: *mut nx_crypto_driver) {
    void nx_debugfs_init(struct nx_crypto_driver *drv)
    {
    struct dentry *root;
    root = debugfs_create_dir(NX_NAME, core::ptr::null_mut());
    drv.dfs_root = root;
    debugfs_create_u32("aes_ops", S_IRUSR | S_IRGRP | S_IROTH,
    root, &drv.stats.aes_ops.counter);
    debugfs_create_u32("sha256_ops", S_IRUSR | S_IRGRP | S_IROTH,
    root, &drv.stats.sha256_ops.counter);
    debugfs_create_u32("sha512_ops", S_IRUSR | S_IRGRP | S_IROTH,
    root, &drv.stats.sha512_ops.counter);
    debugfs_create_u64("aes_bytes", S_IRUSR | S_IRGRP | S_IROTH,
    root, &drv.stats.aes_bytes.counter);
    debugfs_create_u64("sha256_bytes", S_IRUSR | S_IRGRP | S_IROTH,
    root, &drv.stats.sha256_bytes.counter);
    debugfs_create_u64("sha512_bytes", S_IRUSR | S_IRGRP | S_IROTH,
    root, &drv.stats.sha512_bytes.counter);
    debugfs_create_u32("errors", S_IRUSR | S_IRGRP | S_IROTH,
    root, &drv.stats.errors.counter);
    debugfs_create_u32("last_error", S_IRUSR | S_IRGRP | S_IROTH,
    root, &drv.stats.last_error.counter);
    debugfs_create_u32("last_error_pid", S_IRUSR | S_IRGRP | S_IROTH,
    root, &drv.stats.last_error_pid.counter);
    }
    void
    nx_debugfs_fini(struct nx_crypto_driver *drv)
    {
    debugfs_remove_recursive(drv.dfs_root);
    }
