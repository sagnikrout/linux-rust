//! Automatically rewritten from C to Rust
//! Source: net/ceph/pagelist.c
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

    struct ceph_pagelist *ceph_pagelist_alloc(gfp_t gfp_flags)
    {
    struct ceph_pagelist *pl;
    pl = kmalloc_obj(*pl, gfp_flags);
    if (!pl)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&pl.head);
    pl.mapped_tail = core::ptr::null_mut();
    pl.length = 0;
    pl.room = 0;
    INIT_LIST_HEAD(&pl.free_list);
    pl.num_pages_free = 0;
    refcount_set(&pl.refcnt, 1);
    return pl;
    }
    EXPORT_SYMBOL(ceph_pagelist_alloc);
#[no_mangle]
unsafe extern "C" fn ceph_pagelist_unmap_tail(pl: *mut ceph_pagelist) {
    static void ceph_pagelist_unmap_tail(struct ceph_pagelist *pl)
    {
    if (pl.mapped_tail) {
    struct page *page = list_entry(pl.head.prev, struct page, lru);
    kunmap(page);
    pl.mapped_tail = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ceph_pagelist_release(pl: *mut ceph_pagelist) {
    void ceph_pagelist_release(struct ceph_pagelist *pl)
    {
    if (!refcount_dec_and_test(&pl.refcnt))
    return;
    ceph_pagelist_unmap_tail(pl);
    while (!list_empty(&pl.head)) {
    struct page *page = list_first_entry(&pl.head, struct page,
    lru);
    list_del(&page.lru);
    __free_page(page);
    }
    ceph_pagelist_free_reserve(pl);
    kfree(pl);
    }
    EXPORT_SYMBOL(ceph_pagelist_release);
#[no_mangle]
unsafe extern "C" fn ceph_pagelist_addpage(pl: *mut ceph_pagelist) -> c_int {
    static int ceph_pagelist_addpage(struct ceph_pagelist *pl)
    {
    struct page *page;
    if (!pl.num_pages_free) {
    page = __page_cache_alloc(GFP_NOFS);
    } else {
    page = list_first_entry(&pl.free_list, struct page, lru);
    list_del(&page.lru);
    --pl.num_pages_free;
    }
    if (!page)
    return -ENOMEM;
    pl.room += PAGE_SIZE;
    ceph_pagelist_unmap_tail(pl);
    list_add_tail(&page.lru, &pl.head);
    pl.mapped_tail = kmap(page);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ceph_pagelist_append(pl: *mut ceph_pagelist, buf: *const c_void, len: usize) -> c_int {
    int ceph_pagelist_append(struct ceph_pagelist *pl, const void *buf, size_t len)
    {
    while (pl.room < len) {
    let mut bit: usize = pl.room;
    int ret;
    memcpy(pl.mapped_tail + (pl.length & ~PAGE_MASK),
    buf, bit);
    pl.length += bit;
    pl.room -= bit;
    buf += bit;
    len -= bit;
    ret = ceph_pagelist_addpage(pl);
    if (ret)
    return ret;
    }
    memcpy(pl.mapped_tail + (pl.length & ~PAGE_MASK), buf, len);
    pl.length += len;
    pl.room -= len;
    return 0;
    }
    EXPORT_SYMBOL(ceph_pagelist_append);
// Allocate enough pages for a pagelist to append the given amount
// of data without allocating.
// Returns: 0 on success, -ENOMEM on error.
//
#[no_mangle]
pub unsafe extern "C" fn ceph_pagelist_reserve(pl: *mut ceph_pagelist, space: usize) -> c_int {
    int ceph_pagelist_reserve(struct ceph_pagelist *pl, size_t space)
    {
    if (space <= pl.room)
    return 0;
    space -= pl.room;
    space = (space + PAGE_SIZE - 1) >> PAGE_SHIFT;   /* conv to num pages */
    while (space > pl.num_pages_free) {
    struct page *page = __page_cache_alloc(GFP_NOFS);
    if (!page)
    return -ENOMEM;
    list_add_tail(&page.lru, &pl.free_list);
    ++pl.num_pages_free;
    }
    return 0;
    }
    EXPORT_SYMBOL(ceph_pagelist_reserve);
// Free any pages that have been preallocated.
#[no_mangle]
pub unsafe extern "C" fn ceph_pagelist_free_reserve(pl: *mut ceph_pagelist) -> c_int {
    int ceph_pagelist_free_reserve(struct ceph_pagelist *pl)
    {
    while (!list_empty(&pl.free_list)) {
    struct page *page = list_first_entry(&pl.free_list,
    struct page, lru);
    list_del(&page.lru);
    __free_page(page);
    --pl.num_pages_free;
    }
    BUG_ON(pl.num_pages_free);
    return 0;
    }
    EXPORT_SYMBOL(ceph_pagelist_free_reserve);
