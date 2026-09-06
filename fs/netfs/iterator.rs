//! Automatically rewritten from C to Rust
//! Source: fs/netfs/iterator.c
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
// Iterator helpers.
//
// Copyright (C) 2022 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// netfs_extract_user_iter - Extract the pages from a user iterator into a bvec
// @orig: The original iterator
// @orig_len: The amount of iterator to copy
// @new: The iterator to be set up
// @extraction_flags: Flags to qualify the request
//
// Extract the page fragments from the given amount of the source iterator and
// build up a second iterator that refers to all of those bits.  This allows
// the original iterator to be disposed of.
//
// @extraction_flags can have ITER_ALLOW_P2PDMA set to request peer-to-peer DMA be
// allowed on the pages extracted.
//
// On success, the number of elements in the bvec is returned, the original
// iterator will have been advanced by the amount extracted.
//
// The iov_iter_extract_mode() function should be used to query how cleanup
// should be performed.
//
    ssize_t netfs_extract_user_iter(struct iov_iter *orig, size_t orig_len,
    struct iov_iter *new,
    iov_iter_extraction_t extraction_flags)
    {
    struct bio_vec *bv = core::ptr::null_mut();
    struct page **pages;
    unsigned int cur_npages;
    unsigned int max_pages;
    let mut npages: c_uint = 0;
    unsigned int i;
    let mut ret: isize = 0;
    let mut count: usize = orig_len, offset, len;
    size_t bv_size, pg_size;
    if (WARN_ON_ONCE(!iter_is_ubuf(orig) && !iter_is_iovec(orig)))
    return -EIO;
    max_pages = iov_iter_npages(orig, INT_MAX);
    bv_size = array_size(max_pages, sizeof(*bv));
    bv = kvmalloc(bv_size, GFP_KERNEL);
    if (!bv)
    return -ENOMEM;
// Put the page list at the end of the bvec list storage.  bvec
// elements are larger than page pointers, so as long as we work
// 0->last, we should be fine.
//
    pg_size = array_size(max_pages, sizeof(*pages));
    pages = (void *)bv + bv_size - pg_size;
    while (count && npages < max_pages) {
    ret = iov_iter_extract_pages(orig, &pages, count,
    max_pages - npages, extraction_flags,
    &offset);
    if (unlikely(ret <= 0)) {
    ret = ret ?: -EIO;
    break;
    }
    if (WARN(ret > count,
    "%s: extract_pages overrun %zd > %zu bytes\n",
    __func__, ret, count)) {
    ret = -EIO;
    break;
    }
    cur_npages = DIV_ROUND_UP(offset + ret, PAGE_SIZE);
    if (WARN(cur_npages > max_pages - npages,
    "%s: extract_pages overrun %u > %u pages\n",
    __func__, npages + cur_npages, max_pages)) {
    ret = -EIO;
    break;
    }
    count -= ret;
    ret += offset;
    for (i = 0; i < cur_npages; i++) {
    len = ret > PAGE_SIZE ? PAGE_SIZE : ret;
    bvec_set_page(bv + npages + i, *pages++, len - offset, offset);
    ret -= len;
    offset = 0;
    }
    npages += cur_npages;
    }
// Note: Don't try to clean up after EIO.  Either we got no pages, so
// nothing to clean up, or we got a buffer overrun, memory corruption
// and can't trust the stuff in the buffer (a WARN was emitted).
//
    if (ret < 0 && (ret == -ENOMEM || npages == 0)) {
    for (i = 0; i < npages; i++)
    unpin_user_page(bv[i].bv_page);
    kvfree(bv);
    return ret;
    }
    iov_iter_bvec(new, orig.data_source, bv, npages, orig_len - count);
    return npages;
    }
    EXPORT_SYMBOL_GPL(netfs_extract_user_iter);
//
// Select the span of a bvec iterator we're going to use.  Limit it by both maximum
// size and maximum number of segments.  Returns the size of the span in bytes.
//
    static size_t netfs_limit_bvec(const struct iov_iter *iter, size_t start_offset,
    size_t max_size, size_t max_segs)
    {
    const struct bio_vec *bvecs = iter.bvec;
    let mut nbv: c_uint = iter.nr_segs, ix = 0, nsegs = 0;
    size_t len, span = 0, n = iter.count;
    let mut skip: usize = iter.iov_offset + start_offset;
    if (WARN_ON(!iov_iter_is_bvec(iter)) ||
    WARN_ON(start_offset > n) ||
    n == 0)
    return 0;
    while (n && ix < nbv && skip) {
    len = bvecs[ix].bv_len;
    if (skip < len)
    break;
    skip -= len;
    n -= len;
    ix++;
    }
    while (n && ix < nbv) {
    len = min3(n, bvecs[ix].bv_len - skip, max_size);
    span += len;
    nsegs++;
    ix++;
    if (span >= max_size || nsegs >= max_segs)
    break;
    skip = 0;
    n -= len;
    }
    return min(span, max_size);
    }
//
// Select the span of a kvec iterator we're going to use.  Limit it by both
// maximum size and maximum number of segments.  Returns the size of the span
// in bytes.
//
    static size_t netfs_limit_kvec(const struct iov_iter *iter, size_t start_offset,
    size_t max_size, size_t max_segs)
    {
    const struct kvec *kvecs = iter.kvec;
    let mut nkv: c_uint = iter.nr_segs, ix = 0, nsegs = 0;
    size_t len, span = 0, n = iter.count;
    let mut skip: usize = iter.iov_offset + start_offset;
    if (WARN_ON(!iov_iter_is_kvec(iter)) ||
    WARN_ON(start_offset > n) ||
    n == 0)
    return 0;
    while (n && ix < nkv && skip) {
    len = kvecs[ix].iov_len;
    if (skip < len)
    break;
    skip -= len;
    n -= len;
    ix++;
    }
    while (n && ix < nkv) {
    len = min3(n, kvecs[ix].iov_len - skip, max_size);
    span += len;
    nsegs++;
    ix++;
    if (span >= max_size || nsegs >= max_segs)
    break;
    skip = 0;
    n -= len;
    }
    return min(span, max_size);
    }
//
// Select the span of an xarray iterator we're going to use.  Limit it by both
// maximum size and maximum number of segments.  It is assumed that segments
// can be larger than a page in size, provided they're physically contiguous.
// Returns the size of the span in bytes.
//
    static size_t netfs_limit_xarray(const struct iov_iter *iter, size_t start_offset,
    size_t max_size, size_t max_segs)
    {
    struct folio *folio;
    let mut nsegs: c_uint = 0;
    let mut pos: loff_t = iter.xarray_start + iter.iov_offset;
    let mut index: pgoff_t = pos / PAGE_SIZE;
    let mut span: usize = 0, n = iter.count;
    XA_STATE(xas, iter.xarray, index);
    if (WARN_ON(!iov_iter_is_xarray(iter)) ||
    WARN_ON(start_offset > n) ||
    n == 0)
    return 0;
    max_size = min(max_size, n - start_offset);
    rcu_read_lock();
    xas_for_each(&xas, folio, ULONG_MAX) {
    size_t offset, flen, len;
    if (xas_retry(&xas, folio))
    continue;
    if (WARN_ON(xa_is_value(folio)))
    break;
    if (WARN_ON(folio_test_hugetlb(folio)))
    break;
    flen = folio_size(folio);
    offset = offset_in_folio(folio, pos);
    len = min(max_size, flen - offset);
    span += len;
    nsegs++;
    if (span >= max_size || nsegs >= max_segs)
    break;
    }
    rcu_read_unlock();
    return min(span, max_size);
    }
//
// Select the span of a folio queue iterator we're going to use.  Limit it by
// both maximum size and maximum number of segments.  Returns the size of the
// span in bytes.
//
    static size_t netfs_limit_folioq(const struct iov_iter *iter, size_t start_offset,
    size_t max_size, size_t max_segs)
    {
    const struct folio_queue *folioq = iter.folioq;
    let mut nsegs: c_uint = 0;
    let mut slot: c_uint = iter.folioq_slot;
    let mut span: usize = 0, n = iter.count;
    if (WARN_ON(!iov_iter_is_folioq(iter)) ||
    WARN_ON(start_offset > n) ||
    n == 0)
    return 0;
    max_size = umin(max_size, n - start_offset);
    if (slot >= folioq_nr_slots(folioq)) {
    folioq = folioq.next;
    slot = 0;
    }
    start_offset += iter.iov_offset;
    do {
    let mut flen: usize = folioq_folio_size(folioq, slot);
    if (start_offset < flen) {
    span += flen - start_offset;
    nsegs++;
    start_offset = 0;
    } else {
    start_offset -= flen;
    }
    if (span >= max_size || nsegs >= max_segs)
    break;
    slot++;
    if (slot >= folioq_nr_slots(folioq)) {
    folioq = folioq.next;
    slot = 0;
    }
    } while (folioq);
    return umin(span, max_size);
    }
    size_t netfs_limit_iter(const struct iov_iter *iter, size_t start_offset,
    size_t max_size, size_t max_segs)
    {
    if (iov_iter_is_folioq(iter))
    return netfs_limit_folioq(iter, start_offset, max_size, max_segs);
    if (iov_iter_is_bvec(iter))
    return netfs_limit_bvec(iter, start_offset, max_size, max_segs);
    if (iov_iter_is_xarray(iter))
    return netfs_limit_xarray(iter, start_offset, max_size, max_segs);
    if (iov_iter_is_kvec(iter))
    return netfs_limit_kvec(iter, start_offset, max_size, max_segs);
    BUG();
    }
    EXPORT_SYMBOL(netfs_limit_iter);
