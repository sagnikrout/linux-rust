//! Automatically rewritten from C to Rust
//! Source: drivers/greybus/debugfs.c
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
// Greybus debugfs code
//
// Copyright 2014 Google Inc.
// Copyright 2014 Linaro Ltd.
//

    static struct dentry *gb_debug_root;
#[no_mangle]
pub unsafe extern "C" fn gb_debugfs_init() -> void __init {
    void __init gb_debugfs_init(void)
    {
    gb_debug_root = debugfs_create_dir("greybus", core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn gb_debugfs_cleanup() {
    void gb_debugfs_cleanup(void)
    {
    debugfs_remove_recursive(gb_debug_root);
    gb_debug_root = core::ptr::null_mut();
    }
    struct dentry *gb_debugfs_get(void)
    {
    return gb_debug_root;
    }
    EXPORT_SYMBOL_GPL(gb_debugfs_get);
