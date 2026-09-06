//! Automatically rewritten from C to Rust
//! Source: net/core/hwbm.c
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
// Support for hardware buffer manager.
//
// Copyright (C) 2016 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
//

#[no_mangle]
pub unsafe extern "C" fn hwbm_buf_free(bm_pool: *mut hwbm_pool, buf: *mut c_void) {
    void hwbm_buf_free(struct hwbm_pool *bm_pool, void *buf)
    {
    if (likely(bm_pool.frag_size <= PAGE_SIZE))
    skb_free_frag(buf);
    else
    kfree(buf);
    }
    EXPORT_SYMBOL_GPL(hwbm_buf_free);
// Refill processing for HW buffer management
#[no_mangle]
pub unsafe extern "C" fn hwbm_pool_refill(bm_pool: *mut hwbm_pool, gfp: gfp_t) -> c_int {
    int hwbm_pool_refill(struct hwbm_pool *bm_pool, gfp_t gfp)
    {
    let mut frag_size: c_int = bm_pool.frag_size;
    void *buf;
    if (likely(frag_size <= PAGE_SIZE))
    buf = netdev_alloc_frag(frag_size);
    else
    buf = kmalloc(frag_size, gfp);
    if (!buf)
    return -ENOMEM;
    if (bm_pool.construct)
    if (bm_pool.construct(bm_pool, buf)) {
    hwbm_buf_free(bm_pool, buf);
    return -ENOMEM;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(hwbm_pool_refill);
#[no_mangle]
pub unsafe extern "C" fn hwbm_pool_add(bm_pool: *mut hwbm_pool, buf_num: c_uint) -> c_int {
    int hwbm_pool_add(struct hwbm_pool *bm_pool, unsigned int buf_num)
    {
    int err, i;
    mutex_lock(&bm_pool.buf_lock);
    if (bm_pool.buf_num == bm_pool.size) {
    pr_warn("pool already filled\n");
    mutex_unlock(&bm_pool.buf_lock);
    return bm_pool.buf_num;
    }
    if (buf_num + bm_pool.buf_num > bm_pool.size) {
    pr_warn("cannot allocate %d buffers for pool\n",
    buf_num);
    mutex_unlock(&bm_pool.buf_lock);
    return 0;
    }
    if ((buf_num + bm_pool.buf_num) < bm_pool.buf_num) {
    pr_warn("Adding %d buffers to the %d current buffers will overflow\n",
    buf_num,  bm_pool.buf_num);
    mutex_unlock(&bm_pool.buf_lock);
    return 0;
    }
    for (i = 0; i < buf_num; i++) {
    err = hwbm_pool_refill(bm_pool, GFP_KERNEL);
    if (err < 0)
    break;
    }
// Update BM driver with number of buffers added to pool
    bm_pool.buf_num += i;
    pr_debug("hwpm pool: %d of %d buffers added\n", i, buf_num);
    mutex_unlock(&bm_pool.buf_lock);
    return i;
    }
    EXPORT_SYMBOL_GPL(hwbm_pool_add);
