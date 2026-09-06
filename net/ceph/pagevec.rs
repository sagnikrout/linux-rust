//! Automatically rewritten from C to Rust
//! Source: net/ceph/pagevec.c
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

#[no_mangle]
pub unsafe extern "C" fn ceph_release_page_vector(pages: *mut page, num_pages: c_int) {
    void ceph_release_page_vector(struct page **pages, int num_pages)
    {
    int i;
    for (i = 0; i < num_pages; i++)
    __free_pages(pages[i], 0);
    kfree(pages);
    }
    EXPORT_SYMBOL(ceph_release_page_vector);
//
// allocate a vector new pages
//
    struct page **ceph_alloc_page_vector(int num_pages, gfp_t flags)
    {
    struct page **pages;
    int i;
    pages = kmalloc_objs(*pages, num_pages, flags);
    if (!pages)
    return ERR_PTR(-ENOMEM);
    for (i = 0; i < num_pages; i++) {
    pages[i] = __page_cache_alloc(flags);
    if (pages[i] == core::ptr::null_mut()) {
    ceph_release_page_vector(pages, i);
    return ERR_PTR(-ENOMEM);
    }
    }
    return pages;
    }
    EXPORT_SYMBOL(ceph_alloc_page_vector);
    void ceph_copy_from_page_vector(struct page **pages,
    void *data,
    loff_t off, size_t len)
    {
    let mut i: c_int = 0;
    let mut po: usize = off & ~PAGE_MASK;
    let mut left: usize = len;
    while (left > 0) {
    let mut l: usize = min_t(size_t, PAGE_SIZE-po, left);
    memcpy(data, page_address(pages[i]) + po, l);
    data += l;
    left -= l;
    po += l;
    if (po == PAGE_SIZE) {
    po = 0;
    i++;
    }
    }
    }
    EXPORT_SYMBOL(ceph_copy_from_page_vector);
//
// Zero an extent within a page vector.  Offset is relative to the
// start of the first page.
//
#[no_mangle]
pub unsafe extern "C" fn ceph_zero_page_vector_range(off: c_int, len: c_int, pages: *mut page) {
    void ceph_zero_page_vector_range(int off, int len, struct page **pages)
    {
    let mut i: c_int = off >> PAGE_SHIFT;
    off &= ~PAGE_MASK;
    dout("zero_page_vector_page %u~%u\n", off, len);
// leading partial page?
    if (off) {
    let mut end: c_int = min((int)PAGE_SIZE, off + len);
    dout("zeroing %d %p head from %d\n", i, pages[i],
    (int)off);
    zero_user_segment(pages[i], off, end);
    len -= (end - off);
    i++;
    }
    while (len >= PAGE_SIZE) {
    dout("zeroing %d %p len=%d\n", i, pages[i], len);
    zero_user_segment(pages[i], 0, PAGE_SIZE);
    len -= PAGE_SIZE;
    i++;
    }
// trailing partial page?
    if (len) {
    dout("zeroing %d %p tail to %d\n", i, pages[i], (int)len);
    zero_user_segment(pages[i], 0, len);
    }
    }
    EXPORT_SYMBOL(ceph_zero_page_vector_range);
