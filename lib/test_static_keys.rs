//! Automatically rewritten from C to Rust
//! Source: lib/test_static_keys.c
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
    let mut old_true_key: static_key = STATIC_KEY_INIT_TRUE;
    let mut old_false_key: static_key = STATIC_KEY_INIT_FALSE;
// new api
    DEFINE_STATIC_KEY_TRUE(true_key);
    DEFINE_STATIC_KEY_FALSE(false_key);
// external
    extern struct static_key base_old_true_key;
    extern struct static_key base_inv_old_true_key;
    extern struct static_key base_old_false_key;
    extern struct static_key base_inv_old_false_key;
// new api
    extern struct static_key_true base_true_key;
    extern struct static_key_true base_inv_true_key;
    extern struct static_key_false base_false_key;
    extern struct static_key_false base_inv_false_key;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_key {
    pub init_state: bool,
    pub key: *mut static_key,
    pub (*test_key)(void): *mut bool,
}

    static bool key ## _ ## branch(void)	\
    {					\
    return branch(&key);		\
    }
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
unsafe extern "C" fn invert_keys(keys: *mut test_key, size: c_int) {
    static void invert_keys(struct test_key *keys, int size)
    {
    struct static_key *previous = core::ptr::null_mut();
    int i;
    for (i = 0; i < size; i++) {
    if (previous != keys[i].key) {
    invert_key(keys[i].key);
    previous = keys[i].key;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn verify_keys(keys: *mut test_key, size: c_int, invert: bool) -> c_int {
    static int verify_keys(struct test_key *keys, int size, bool invert)
    {
    int i;
    bool ret, init;
    for (i = 0; i < size; i++) {
    ret = static_key_enabled(keys[i].key);
    init = keys[i].init_state;
    if (ret != (invert ? !init : init))
    return -EINVAL;
    ret = keys[i].test_key();
    if (static_key_enabled(keys[i].key)) {
    if (!ret)
    return -EINVAL;
    } else {
    if (ret)
    return -EINVAL;
    }
    }
    return 0;
    }
    test_key_func(old_true_key, static_key_true)
    test_key_func(old_false_key, static_key_false)
    test_key_func(true_key, static_branch_likely)
    test_key_func(true_key, static_branch_unlikely)
    test_key_func(false_key, static_branch_likely)
    test_key_func(false_key, static_branch_unlikely)
    test_key_func(base_old_true_key, static_key_true)
    test_key_func(base_inv_old_true_key, static_key_true)
    test_key_func(base_old_false_key, static_key_false)
    test_key_func(base_inv_old_false_key, static_key_false)
    test_key_func(base_true_key, static_branch_likely)
    test_key_func(base_true_key, static_branch_unlikely)
    test_key_func(base_inv_true_key, static_branch_likely)
    test_key_func(base_inv_true_key, static_branch_unlikely)
    test_key_func(base_false_key, static_branch_likely)
    test_key_func(base_false_key, static_branch_unlikely)
    test_key_func(base_inv_false_key, static_branch_likely)
    test_key_func(base_inv_false_key, static_branch_unlikely)
#[no_mangle]
unsafe extern "C" fn test_static_key_init() -> int __init {
    static int __init test_static_key_init(void)
    {
    int ret;
    int size;
    struct test_key static_key_tests[] = {
// internal keys - old keys
    {
    .init_state	= true,
    .key		= &old_true_key,
    .test_key	= &old_true_key_static_key_true,
    },
    {
    .init_state	= false,
    .key		= &old_false_key,
    .test_key	= &old_false_key_static_key_false,
    },
// internal keys - new keys
    {
    .init_state	= true,
    .key		= &true_key.key,
    .test_key	= &true_key_static_branch_likely,
    },
    {
    .init_state	= true,
    .key		= &true_key.key,
    .test_key	= &true_key_static_branch_unlikely,
    },
    {
    .init_state	= false,
    .key		= &false_key.key,
    .test_key	= &false_key_static_branch_likely,
    },
    {
    .init_state	= false,
    .key		= &false_key.key,
    .test_key	= &false_key_static_branch_unlikely,
    },
// external keys - old keys
    {
    .init_state	= true,
    .key		= &base_old_true_key,
    .test_key	= &base_old_true_key_static_key_true,
    },
    {
    .init_state	= false,
    .key		= &base_inv_old_true_key,
    .test_key	= &base_inv_old_true_key_static_key_true,
    },
    {
    .init_state	= false,
    .key		= &base_old_false_key,
    .test_key	= &base_old_false_key_static_key_false,
    },
    {
    .init_state	= true,
    .key		= &base_inv_old_false_key,
    .test_key	= &base_inv_old_false_key_static_key_false,
    },
// external keys - new keys
    {
    .init_state	= true,
    .key		= &base_true_key.key,
    .test_key	= &base_true_key_static_branch_likely,
    },
    {
    .init_state	= true,
    .key		= &base_true_key.key,
    .test_key	= &base_true_key_static_branch_unlikely,
    },
    {
    .init_state	= false,
    .key		= &base_inv_true_key.key,
    .test_key	= &base_inv_true_key_static_branch_likely,
    },
    {
    .init_state	= false,
    .key		= &base_inv_true_key.key,
    .test_key	= &base_inv_true_key_static_branch_unlikely,
    },
    {
    .init_state	= false,
    .key		= &base_false_key.key,
    .test_key	= &base_false_key_static_branch_likely,
    },
    {
    .init_state	= false,
    .key		= &base_false_key.key,
    .test_key	= &base_false_key_static_branch_unlikely,
    },
    {
    .init_state	= true,
    .key		= &base_inv_false_key.key,
    .test_key	= &base_inv_false_key_static_branch_likely,
    },
    {
    .init_state	= true,
    .key		= &base_inv_false_key.key,
    .test_key	= &base_inv_false_key_static_branch_unlikely,
    },
    };
    size = ARRAY_SIZE(static_key_tests);
    ret = verify_keys(static_key_tests, size, false);
    if (ret)
    goto out;
    invert_keys(static_key_tests, size);
    ret = verify_keys(static_key_tests, size, true);
    if (ret)
    goto out;
    invert_keys(static_key_tests, size);
    ret = verify_keys(static_key_tests, size, false);
    if (ret)
    goto out;
    return 0;
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_static_key_exit() -> void __exit {
    static void __exit test_static_key_exit(void)
    {
    }
    module_init(test_static_key_init);
    module_exit(test_static_key_exit);
    MODULE_AUTHOR("Jason Baron <jbaron@akamai.com>");
    MODULE_DESCRIPTION("Kernel module for testing static keys");
    MODULE_LICENSE("GPL");
