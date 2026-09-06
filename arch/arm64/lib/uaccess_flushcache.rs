//! Automatically rewritten from C to Rust
//! Source: arch/arm64/lib/uaccess_flushcache.c
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
// Copyright (C) 2017 ARM Ltd.
//

#[no_mangle]
pub unsafe extern "C" fn memcpy_flushcache(dst: *mut c_void, src: *const c_void, cnt: usize) {
    void memcpy_flushcache(void *dst, const void *src, size_t cnt)
    {
//
// We assume this should not be called with @dst pointing to
// non-cacheable memory, such that we don't need an explicit
// barrier to order the cache maintenance against the memcpy.
//
    memcpy(dst, src, cnt);
    dcache_clean_pop((unsigned long)dst, (unsigned long)dst + cnt);
    }
    EXPORT_SYMBOL_GPL(memcpy_flushcache);
    unsigned long __copy_user_flushcache(void *to, const void __user *from,
    unsigned long n)
    {
    unsigned long rc;
    rc = raw_copy_from_user(to, from, n);
// See above
    dcache_clean_pop((unsigned long)to, (unsigned long)to + n - rc);
    return rc;
    }
