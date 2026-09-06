//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/hyp/nvhe/page_alloc.c
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
// Copyright (C) 2020 Google LLC
// Author: Quentin Perret <qperret@google.com>
//

    u64 __hyp_vmemmap;
//
// Index the hyp_vmemmap to find a potential buddy page, but make no assumption
// about its current state.
//
// Example buddy-tree for a 4-pages physically contiguous pool:
//
// o : Page 3
//
// o-o : Page 2
//
// /   o : Page 1
// /
// o---o-o : Page 0
// Order  2   1 0
//
// Example of requests on this pool:
// __find_buddy_nocheck(pool, page 0, order 0) => page 1
// __find_buddy_nocheck(pool, page 0, order 1) => page 2
// __find_buddy_nocheck(pool, page 1, order 0) => page 0
// __find_buddy_nocheck(pool, page 2, order 0) => page 3
//
    static struct hyp_page *__find_buddy_nocheck(struct hyp_pool *pool,
    struct hyp_page *p,
    u8 order)
    {
    let mut addr: phys_addr_t = hyp_page_to_phys(p);
    addr ^= (PAGE_SIZE << order);
//
// Don't return a page outside the pool range -- it belongs to
// something else and may not be mapped in hyp_vmemmap.
//
    if (addr < pool.range_start || addr >= pool.range_end)
    return core::ptr::null_mut();
    return hyp_phys_to_page(addr);
    }
// Find a buddy page currently available for allocation
    static struct hyp_page *__find_buddy_avail(struct hyp_pool *pool,
    struct hyp_page *p,
    u8 order)
    {
    struct hyp_page *buddy = __find_buddy_nocheck(pool, p, order);
    if (!buddy || buddy.order != order || buddy.refcount)
    return core::ptr::null_mut();
    return buddy;
    }
//
// Pages that are available for allocation are tracked in free-lists, so we use
// the pages themselves to store the list nodes to avoid wasting space. As the
// allocator always returns zeroed pages (which are zeroed on the hyp_put_page()
// path to optimize allocation speed), we also need to clean-up the list node in
// each page when we take it out of the list.
//
#[no_mangle]
pub unsafe extern "C" fn page_remove_from_list(p: *mut hyp_page) {
    static inline void page_remove_from_list(struct hyp_page *p)
    {
    struct list_head *node = hyp_page_to_virt(p);
    __list_del_entry(node);
    memset(node, 0, sizeof(*node));
    }
#[no_mangle]
pub unsafe extern "C" fn page_add_to_list(p: *mut hyp_page, head: *mut list_head) {
    static inline void page_add_to_list(struct hyp_page *p, struct list_head *head)
    {
    struct list_head *node = hyp_page_to_virt(p);
    INIT_LIST_HEAD(node);
    list_add_tail(node, head);
    }
    static inline struct hyp_page *node_to_page(struct list_head *node)
    {
    return hyp_virt_to_page(node);
    }
    static void __hyp_attach_page(struct hyp_pool *pool,
    struct hyp_page *p)
    {
    let mut phys: phys_addr_t = hyp_page_to_phys(p);
    struct hyp_page *buddy;
    let mut coalesce: bool = true;
    let mut order: u8 = p.order;
//
// 'external' pages are never coalesced and their ->order field
// untrusted as they bypass hyp_pool_init(). Enforce order-0.
//
    if (phys < pool.range_start || phys >= pool.range_end) {
    order = 0;
    coalesce = false;
    }
    memset(hyp_page_to_virt(p), 0, PAGE_SIZE << order);
    if (!coalesce)
    goto insert;
//
// Only the first struct hyp_page of a high-order page (otherwise known
// as the 'head') should have p->order set. The non-head pages should
// have p->order = HYP_NO_ORDER. Here @p may no longer be the head
// after coalescing, so make sure to mark it HYP_NO_ORDER proactively.
//
    p.order = HYP_NO_ORDER;
    for (; (order + 1) <= pool.max_order; order++) {
    buddy = __find_buddy_avail(pool, p, order);
    if (!buddy)
    break;
// Take the buddy out of its list, and coalesce with @p
    page_remove_from_list(buddy);
    buddy.order = HYP_NO_ORDER;
    p = min(p, buddy);
    }
    insert:
// Mark the new head, and insert it
    p.order = order;
    page_add_to_list(p, &pool.free_area[order]);
    }
    static struct hyp_page *__hyp_extract_page(struct hyp_pool *pool,
    struct hyp_page *p,
    u8 order)
    {
    struct hyp_page *buddy;
    page_remove_from_list(p);
    while (p.order > order) {
//
// The buddy of order n - 1 currently has HYP_NO_ORDER as it
// is covered by a higher-level page (whose head is @p). Use
// __find_buddy_nocheck() to find it and inject it in the
// free_list[n - 1], effectively splitting @p in half.
//
    p.order--;
    buddy = __find_buddy_nocheck(pool, p, p.order);
    buddy.order = p.order;
    page_add_to_list(buddy, &pool.free_area[buddy.order]);
    }
    return p;
    }
#[no_mangle]
unsafe extern "C" fn __hyp_put_page(pool: *mut hyp_pool, p: *mut hyp_page) {
    static void __hyp_put_page(struct hyp_pool *pool, struct hyp_page *p)
    {
    if (hyp_page_ref_dec_and_test(p))
    __hyp_attach_page(pool, p);
    }
//
// Changes to the buddy tree and page refcounts must be done with the hyp_pool
// lock held. If a refcount change requires an update to the buddy tree (e.g.
// hyp_put_page()), both operations must be done within the same critical
// section to guarantee transient states (e.g. a page with null refcount but
// not yet attached to a free list) can't be observed by well-behaved readers.
//
#[no_mangle]
pub unsafe extern "C" fn hyp_put_page(pool: *mut hyp_pool, addr: *mut c_void) {
    void hyp_put_page(struct hyp_pool *pool, void *addr)
    {
    struct hyp_page *p = hyp_virt_to_page(addr);
    hyp_spin_lock(&pool.lock);
    __hyp_put_page(pool, p);
    hyp_spin_unlock(&pool.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn hyp_get_page(pool: *mut hyp_pool, addr: *mut c_void) {
    void hyp_get_page(struct hyp_pool *pool, void *addr)
    {
    struct hyp_page *p = hyp_virt_to_page(addr);
    hyp_spin_lock(&pool.lock);
    hyp_page_ref_inc(p);
    hyp_spin_unlock(&pool.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn hyp_split_page(p: *mut hyp_page) {
    void hyp_split_page(struct hyp_page *p)
    {
    let mut order: u8 = p.order;
    unsigned int i;
    p.order = 0;
    for (i = 1; i < (1 << order); i++) {
    struct hyp_page *tail = p + i;
    tail.order = 0;
    hyp_set_page_refcounted(tail);
    }
    }
    void *hyp_alloc_pages(struct hyp_pool *pool, u8 order)
    {
    struct hyp_page *p;
    let mut i: u8 = order;
    hyp_spin_lock(&pool.lock);
// Look for a high-enough-order page
    while (i <= pool.max_order && list_empty(&pool.free_area[i]))
    i++;
    if (i > pool.max_order) {
    hyp_spin_unlock(&pool.lock);
    return core::ptr::null_mut();
    }
// Extract it from the tree at the right order
    p = node_to_page(pool.free_area[i].next);
    p = __hyp_extract_page(pool, p, order);
    hyp_set_page_refcounted(p);
    hyp_spin_unlock(&pool.lock);
    return hyp_page_to_virt(p);
    }
    int hyp_pool_init(struct hyp_pool *pool, u64 pfn, unsigned int nr_pages,
    unsigned int reserved_pages)
    {
    let mut phys: phys_addr_t = hyp_pfn_to_phys(pfn);
    struct hyp_page *p;
    int i;
    hyp_spin_lock_init(&pool.lock);
    pool.max_order = min(MAX_PAGE_ORDER,
    get_order(nr_pages << PAGE_SHIFT));
    for (i = 0; i <= pool.max_order; i++)
    INIT_LIST_HEAD(&pool.free_area[i]);
    pool.range_start = phys;
    pool.range_end = phys + (nr_pages << PAGE_SHIFT);
// Init the vmemmap portion
    p = hyp_phys_to_page(phys);
    for (i = 0; i < nr_pages; i++) {
    hyp_set_page_refcounted(&p[i]);
    p[i].order = 0;
    }
// Attach the unused pages to the buddy tree
    for (i = reserved_pages; i < nr_pages; i++)
    __hyp_put_page(pool, &p[i]);
    return 0;
    }
