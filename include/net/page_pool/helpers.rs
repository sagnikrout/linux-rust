//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/page_pool/helpers.h
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
// page_pool/helpers.h
// Author:	Jesper Dangaard Brouer <netoptimizer@brouer.com>
// Copyright (C) 2016 Red Hat, Inc.
//
// DOC: page_pool allocator
//
// The page_pool allocator is optimized for recycling page or page fragment used
// by skb packet and xdp frame.
//
// Basic use involves replacing any alloc_pages() calls with page_pool_alloc(),
// which allocate memory with or without page splitting depending on the
// requested memory size.
//
// If the driver knows that it always requires full pages or its allocations are
// always smaller than half a page, it can use one of the more specific API
// calls:
//
// 1. page_pool_alloc_pages(): allocate memory without page splitting when
// driver knows that the memory it need is always bigger than half of the page
// allocated from page pool. There is no cache line dirtying for 'struct page'
// when a page is recycled back to the page pool.
//
// 2. page_pool_alloc_frag(): allocate memory with page splitting when driver
// knows that the memory it need is always smaller than or equal to half of the
// page allocated from page pool. Page splitting enables memory saving and thus
// avoids TLB/cache miss for data access, but there also is some cost to
// implement page splitting, mainly some cache line dirtying/bouncing for
// 'struct page' and atomic operation for page->pp_ref_count.
//
// The API keeps track of in-flight pages, in order to let API users know when
// it is safe to free a page_pool object, the API users must call
// page_pool_put_page() or page_pool_free_va() to free the page_pool object, or
// attach the page_pool object to a page_pool-aware object like skbs marked with
// skb_mark_for_recycle().
//
// page_pool_put_page() may be called multiple times on the same page if a page
// is split into multiple fragments. For the last fragment, it will either
// recycle the page, or in case of page->_refcount > 1, it will release the DMA
// mapping and in-flight state accounting.
//
// dma_sync_single_range_for_device() is only called for the last fragment when
// page_pool is created with PP_FLAG_DMA_SYNC_DEV flag, so it depends on the
// last freed fragment to do the sync_for_device operation for all fragments in
// the same page when a page is split. The API user must setup pool->p.max_len
// and pool->p.offset correctly and ensure that page_pool_put_page() is called
// with dma_sync_size being -1 for fragment API.
//

// Deprecated driver-facing API, use netlink instead
extern "C" {
    pub fn page_pool_ethtool_stats_get_count() -> c_int;
}

//
// page_pool_dev_alloc_pages() - allocate a page.
// @pool:	pool from which to allocate
//
// Get a page from the page allocator or page_pool caches.
//
extern "C" {
    pub fn page_pool_alloc_pages(_arg: pool, _arg: gfp) -> return;
}
//
// page_pool_dev_alloc_frag() - allocate a page fragment.
// @pool: pool from which to allocate
// @offset: offset to the allocated page
// @size: requested size
//
// Get a page fragment from the page allocator or page_pool caches.
//
// Return: allocated page fragment, otherwise return NULL.
//
extern "C" {
    pub fn page_pool_alloc_frag(_arg: pool, _arg: offset, _arg: size, _arg: gfp) -> return;
}
// size = max_size;
// offset = 0;
extern "C" {
    pub fn page_pool_alloc_netmems(_arg: pool, _arg: gfp) -> return;
}
// There is very likely not enough space for another fragment, so append
// the remaining size to the current fragment to avoid truesize
// underestimate problem.
//
// size = max_size - *offset;
extern "C" {
    pub fn page_pool_alloc_netmem(_arg: pool, _arg: offset, _arg: size, _arg: gfp) -> return;
}
extern "C" {
    pub fn page_pool_alloc_netmems(_arg: pool, _arg: gfp) -> return;
}
extern "C" {
    pub fn netmem_to_page(_arg: page_pool_alloc_netmem(pool, _arg: offset, _arg: size, _arg: gfp)) -> return;
}
//
// page_pool_dev_alloc() - allocate a page or a page fragment.
// @pool: pool from which to allocate
// @offset: offset to the allocated page
// @size: in as the requested size, out as the allocated size
//
// Get a page or a page fragment from the page allocator or page_pool caches
// depending on the requested size in order to allocate memory with least memory
// utilization and performance penalty.
//
// Return: allocated page or page fragment, otherwise return NULL.
//
extern "C" {
    pub fn page_pool_alloc(_arg: pool, _arg: offset, _arg: size, _arg: gfp) -> return;
}
// Mask off __GFP_HIGHMEM to ensure we can use page_address()
//
// page_pool_dev_alloc_va() - allocate a page or a page fragment and return its
// va.
// @pool: pool from which to allocate
// @size: in as the requested size, out as the allocated size
//
// This is just a thin wrapper around the page_pool_alloc() API, and
// it returns va of the allocated page or page fragment.
//
// Return: the va for the allocated page or page fragment, otherwise return NULL.
//
extern "C" {
    pub fn page_pool_alloc_va(_arg: pool, _arg: size, _arg: gfp) -> return;
}
//
// page_pool_get_dma_dir() - Retrieve the stored DMA direction.
// @pool:	pool from which page was allocated
//
// Get the stored dma direction. A driver might decide to store this locally
// and avoid the extra cache line from page_pool to determine the direction.
//
// page_pool_fragment_page() - split a fresh page into fragments
// @page:	page to split
// @nr:		references to set
//
// pp_ref_count represents the number of outstanding references to the page,
// which will be freed using page_pool APIs (rather than page allocator APIs
// like put_page()). Such references are usually held by page_pool-aware
// objects like skbs marked for page pool recycling.
//
// This helper allows the caller to take (set) multiple references to a
// freshly allocated page. The page must be freshly allocated (have a
// pp_ref_count of 1). This is commonly done by drivers and
// "fragment allocators" to save atomic operations - either when they know
// upfront how many references they will need; or to take MAX references and
// return the unused ones with a single atomic dec(), instead of performing
// multiple atomic inc() operations.
//
// If nr == pp_ref_count then we have cleared all remaining
// references to the page:
// 1. 'n == 1': no need to actually overwrite it.
// 2. 'n != 1': overwrite it with one, which is the rare case
// for pp_ref_count draining.
//
// The main advantage to doing this is that not only we avoid a atomic
// update, as an atomic_read is generally a much cheaper operation than
// an atomic update, especially when dealing with a page that may be
// referenced by only 2 or 3 users; but also unify the pp_ref_count
// handling by ensuring all pages have partitioned into only 1 piece
// initially, and only overwrite it when the page is partitioned into
// more than one piece.
//
// As we have ensured nr is always one for constant case using
// the BUILD_BUG_ON(), only need to handle the non-constant case
// here for pp_ref_count draining, which is a rare case.
//
// We are the last user here too, reset pp_ref_count back to 1 to
// ensure all pages have been partitioned into 1 piece initially,
// this should be the rare case when the last two fragment users call
// page_pool_unref_page() currently.
//
extern "C" {
    pub fn page_pool_unref_netmem(_arg: page_to_netmem(page), _arg: nr) -> return;
}
// If page_pool_unref_page() returns 0, we were the last user
// When page_pool isn't compiled-in, net/core/xdp.c doesn't
// allow registering MEM_TYPE_PAGE_POOL, but shield linker.
//

//
// page_pool_put_page() - release a reference to a page pool page
// @pool:	pool from which page was allocated
// @page:	page to release a reference on
// @dma_sync_size: how much of the page may have been touched by the device
// @allow_direct: released by the consumer, allow lockless caching
//
// The outcome of this depends on the page refcnt. If the driver bumps
// the refcnt > 1 this will unmap the page. If the page refcnt is 1
// the allocator owns the page and will try to recycle it in one of the pool
// caches. If PP_FLAG_DMA_SYNC_DEV is set, the page will be synced for_device
// using dma_sync_single_range_for_device().
//
// page_pool_put_full_page() - release a reference on a page pool page
// @pool:	pool from which page was allocated
// @page:	page to release a reference on
// @allow_direct: released by the consumer, allow lockless caching
//
// Similar to page_pool_put_page(), but will DMA sync the entire memory area
// as configured in &page_pool_params.max_len.
//
// page_pool_recycle_direct() - release a reference on a page pool page
// @pool:	pool from which page was allocated
// @page:	page to release a reference on
//
// Similar to page_pool_put_full_page() but caller must guarantee safe context
// (e.g NAPI), since it will recycle the page directly into the pool fast cache.
//

//
// page_pool_free_va() - free a va into the page_pool
// @pool: pool from which va was allocated
// @va: va to be freed
// @allow_direct: freed by the consumer, allow lockless caching
//
// Free a va allocated from page_pool_allo_va().
//
// page_pool_get_dma_addr() - Retrieve the stored DMA address.
// @page:	page allocated from a page pool
//
// Fetch the DMA address of the page. The page pool to which the page belongs
// must had been created with PP_FLAG_DMA_MAP.
//
extern "C" {
    pub fn page_pool_get_dma_addr_netmem(_arg: page_to_netmem(page)) -> return;
}
//
// page_pool_dma_sync_for_cpu - sync Rx page for CPU after it's written by HW
// @pool: &page_pool the @page belongs to
// @page: page to sync
// @offset: offset from page start to "hard" start if using PP frags
// @dma_sync_size: size of the data written to the page
//
// Can be used as a shorthand to sync Rx pages before accessing them in the
// driver. Caller must ensure the pool was created with ``PP_FLAG_DMA_MAP``.
// Note that this version performs DMA sync unconditionally, even if the
// associated PP doesn't perform sync-for-device.
//
extern "C" {
    pub fn refcount_dec_and_test(_arg: &pool->user_cnt) -> return;
}
//
// page_pool_is_unreadable() - will allocated buffers be unreadable for the CPU
// @pool: queried page pool
//
// Check if page pool will return buffers which are unreadable to the CPU
// kernel. This will only be the case if user space bound a memory provider (mp)
// which returns unreadable memory to the queue served by the page pool.
// If %PP_FLAG_ALLOW_UNREADABLE_NETMEM was set but there is no mp bound
// this helper will return false. See also netif_rxq_has_unreadable_mp().
//
// Return: true if memory allocated by the page pool may be unreadable
//
