//! Automatically rewritten from C to Rust
//! Source: fs/cachefiles/main.c
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
// Network filesystem caching backend to use cache files on a premounted
// filesystem
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

// Macro flag: #define CREATE_TRACE_POINTS

    unsigned cachefiles_debug;
    module_param_named(debug, cachefiles_debug, uint, S_IWUSR | S_IRUGO);
    MODULE_PARM_DESC(cachefiles_debug, "CacheFiles debugging mask");
    MODULE_DESCRIPTION("Mounted-filesystem based cache");
    MODULE_AUTHOR("Red Hat, Inc.");
    MODULE_LICENSE("GPL");
    struct kmem_cache *cachefiles_object_jar;
    static struct miscdevice cachefiles_dev = {
    .minor	= MISC_DYNAMIC_MINOR,
    .name	= "cachefiles",
    .fops	= &cachefiles_daemon_fops,
    };
//
// initialise the fs caching module
//
#[no_mangle]
unsafe extern "C" fn cachefiles_init() -> int __init {
    static int __init cachefiles_init(void)
    {
    int ret;
    ret = cachefiles_register_error_injection();
    if (ret < 0)
    goto error_einj;
    ret = misc_register(&cachefiles_dev);
    if (ret < 0)
    goto error_dev;
// create an object jar
    ret = -ENOMEM;
    cachefiles_object_jar =
    kmem_cache_create("cachefiles_object_jar",
    sizeof(struct cachefiles_object),
    0, SLAB_HWCACHE_ALIGN, core::ptr::null_mut());
    if (!cachefiles_object_jar) {
    pr_notice("Failed to allocate an object jar\n");
    goto error_object_jar;
    }
    pr_info("Loaded\n");
    return 0;
    error_object_jar:
    misc_deregister(&cachefiles_dev);
    error_dev:
    cachefiles_unregister_error_injection();
    error_einj:
    pr_err("failed to register: %d\n", ret);
    return ret;
    }
    fs_initcall(cachefiles_init);
//
// clean up on module removal
//
#[no_mangle]
unsafe extern "C" fn cachefiles_exit() -> void __exit {
    static void __exit cachefiles_exit(void)
    {
    pr_info("Unloading\n");
    kmem_cache_destroy(cachefiles_object_jar);
    misc_deregister(&cachefiles_dev);
    cachefiles_unregister_error_injection();
    }
    module_exit(cachefiles_exit);
