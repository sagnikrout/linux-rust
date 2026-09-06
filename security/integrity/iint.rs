//! Automatically rewritten from C to Rust
//! Source: security/integrity/iint.c
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
// Copyright (C) 2008 IBM Corporation
//
// Authors:
// Mimi Zohar <zohar@us.ibm.com>
//
// File: integrity_iint.c
// - initialize the integrity directory in securityfs
// - load IMA and EVM keys
//

    struct dentry *integrity_dir;
//
// integrity_kernel_read - read data from the file
//
// This is a function for reading file content instead of kernel_read().
// It does not perform locking checks to ensure it cannot be blocked.
// It does not perform security checks because it is irrelevant for IMA.
//
    int integrity_kernel_read(struct file *file, loff_t offset,
    void *addr, unsigned long count)
    {
    return __kernel_read(file, addr, count, &offset);
    }
//
// integrity_load_keys - load integrity keys hook
//
// Hooks is called from init/main.c:kernel_init_freeable()
// when rootfs is ready
//
#[no_mangle]
pub unsafe extern "C" fn integrity_load_keys() -> void __init {
    void __init integrity_load_keys(void)
    {
    ima_load_x509();
    if (!IS_ENABLED(CONFIG_IMA_LOAD_X509))
    evm_load_x509();
    }
#[no_mangle]
pub unsafe extern "C" fn integrity_fs_init() -> int __init {
    int __init integrity_fs_init(void)
    {
    if (integrity_dir)
    return 0;
    integrity_dir = securityfs_create_dir("integrity", core::ptr::null_mut());
    if (IS_ERR(integrity_dir)) {
    let mut ret: c_int = PTR_ERR(integrity_dir);
    if (ret != -ENODEV)
    pr_err("Unable to create integrity sysfs dir: %d\n",
    ret);
    integrity_dir = core::ptr::null_mut();
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn integrity_fs_fini() -> void __init {
    void __init integrity_fs_fini(void)
    {
    if (!integrity_dir || !simple_empty(integrity_dir))
    return;
    securityfs_remove(integrity_dir);
    integrity_dir = core::ptr::null_mut();
    }
