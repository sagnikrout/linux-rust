//! Automatically rewritten from C to Rust
//! Source: lib/test_static_key_base.c
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
// Kernel module for testing static keys.
//
// Copyright 2015 Akamai Technologies Inc. All Rights Reserved
//
// Authors:
// Jason Baron       <jbaron@akamai.com>
//

// old keys
    let mut base_old_true_key: static_key = STATIC_KEY_INIT_TRUE;
    EXPORT_SYMBOL_GPL(base_old_true_key);
    let mut base_inv_old_true_key: static_key = STATIC_KEY_INIT_TRUE;
    EXPORT_SYMBOL_GPL(base_inv_old_true_key);
    let mut base_old_false_key: static_key = STATIC_KEY_INIT_FALSE;
    EXPORT_SYMBOL_GPL(base_old_false_key);
    let mut base_inv_old_false_key: static_key = STATIC_KEY_INIT_FALSE;
    EXPORT_SYMBOL_GPL(base_inv_old_false_key);
// new keys
    DEFINE_STATIC_KEY_TRUE(base_true_key);
    EXPORT_SYMBOL_GPL(base_true_key);
    DEFINE_STATIC_KEY_TRUE(base_inv_true_key);
    EXPORT_SYMBOL_GPL(base_inv_true_key);
    DEFINE_STATIC_KEY_FALSE(base_false_key);
    EXPORT_SYMBOL_GPL(base_false_key);
    DEFINE_STATIC_KEY_FALSE(base_inv_false_key);
    EXPORT_SYMBOL_GPL(base_inv_false_key);
#[no_mangle]
unsafe extern "C" fn invert_key(key: *mut static_key) {
    static void invert_key(struct static_key *key)
    {
    if (static_key_enabled(key))
    static_key_disable(key);
    else
    static_key_enable(key);
    }
#[no_mangle]
unsafe extern "C" fn test_static_key_base_init() -> int __init {
    static int __init test_static_key_base_init(void)
    {
    invert_key(&base_inv_old_true_key);
    invert_key(&base_inv_old_false_key);
    invert_key(&base_inv_true_key.key);
    invert_key(&base_inv_false_key.key);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_static_key_base_exit() -> void __exit {
    static void __exit test_static_key_base_exit(void)
    {
    }
    module_init(test_static_key_base_init);
    module_exit(test_static_key_base_exit);
    MODULE_AUTHOR("Jason Baron <jbaron@akamai.com>");
    MODULE_DESCRIPTION("Kernel module to support testing static keys");
    MODULE_LICENSE("GPL");
