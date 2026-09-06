//! Automatically rewritten from C to Rust
//! Source: fs/netfs/fscache_main.c
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
// General filesystem local caching manager
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

// Macro flag: #define CREATE_TRACE_POINTS

    EXPORT_TRACEPOINT_SYMBOL(fscache_access_cache);
    EXPORT_TRACEPOINT_SYMBOL(fscache_access_volume);
    EXPORT_TRACEPOINT_SYMBOL(fscache_access);
    struct workqueue_struct *fscache_wq;
    EXPORT_SYMBOL(fscache_wq);
//
// Mixing scores (in bits) for (7,20):
// Input delta: 1-bit      2-bit
// 1 round:     330.3     9201.6
// 2 rounds:   1246.4    25475.4
// 3 rounds:   1907.1    31295.1
// 4 rounds:   2042.3    31718.6
// Perfect:    2048      31744
// (32*64)   (32*31/2 * 64)
//

    (	x ^= (a),	\
    y ^= x,	x = rol32(x, 7),\
    x += y,	y = rol32(y,20),\
    y *= 9			)
#[no_mangle]
pub unsafe extern "C" fn fold_hash(x: c_ulong, y: c_ulong) -> c_uint {
    static inline unsigned int fold_hash(unsigned long x, unsigned long y)
    {
// Use arch-optimized multiply if one exists
    return __hash_32(y ^ __hash_32(x));
    }
//
// Generate a hash.  This is derived from full_name_hash(), but we want to be
// sure it is arch independent and that it doesn't change as bits of the
// computed hash value might appear on disk.  The caller must guarantee that
// the source data is a multiple of four bytes in size.
//
#[no_mangle]
pub unsafe extern "C" fn fscache_hash(salt: c_uint, data: *const c_void, len: usize) -> c_uint {
    unsigned int fscache_hash(unsigned int salt, const void *data, size_t len)
    {
    const __le32 *p = data;
    unsigned int a, x = 0, y = salt, n = len / sizeof(__le32);
    for (; n; n--) {
    a = le32_to_cpu(*p++);
    HASH_MIX(x, y, a);
    }
    return fold_hash(x, y);
    }
//
// initialise the fs caching module
//
#[no_mangle]
pub unsafe extern "C" fn fscache_init() -> int __init {
    int __init fscache_init(void)
    {
    let mut ret: c_int = -ENOMEM;
    fscache_wq = alloc_workqueue("fscache", WQ_UNBOUND | WQ_FREEZABLE, 0);
    if (!fscache_wq)
    goto error_wq;
    ret = fscache_proc_init();
    if (ret < 0)
    goto error_proc;
    fscache_cookie_jar = kmem_cache_create("fscache_cookie_jar",
    sizeof(struct fscache_cookie),
    0, 0, core::ptr::null_mut());
    if (!fscache_cookie_jar) {
    pr_notice("Failed to allocate a cookie jar\n");
    ret = -ENOMEM;
    goto error_cookie_jar;
    }
    pr_notice("FS-Cache loaded\n");
    return 0;
    error_cookie_jar:
    fscache_proc_cleanup();
    error_proc:
    destroy_workqueue(fscache_wq);
    error_wq:
    return ret;
    }
//
// clean up on module removal
//
#[no_mangle]
pub unsafe extern "C" fn fscache_exit() -> void __exit {
    void __exit fscache_exit(void)
    {
    _enter("");
    kmem_cache_destroy(fscache_cookie_jar);
    fscache_proc_cleanup();
    timer_shutdown_sync(&fscache_cookie_lru_timer);
    destroy_workqueue(fscache_wq);
    pr_notice("FS-Cache unloaded\n");
    }
