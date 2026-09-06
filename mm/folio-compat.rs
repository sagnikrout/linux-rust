//! Automatically rewritten from C to Rust
//! Source: mm/folio-compat.c
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
// Compatibility functions which bloat the callers too much to make inline.
// All of the callers of these functions should be converted to use folios
// eventually.
//

#[no_mangle]
pub unsafe extern "C" fn unlock_page(page: *mut page) {
    void unlock_page(struct page *page)
    {
    return folio_unlock(page_folio(page));
    }
    EXPORT_SYMBOL(unlock_page);
#[no_mangle]
pub unsafe extern "C" fn end_page_writeback(page: *mut page) {
    void end_page_writeback(struct page *page)
    {
    return folio_end_writeback(page_folio(page));
    }
    EXPORT_SYMBOL(end_page_writeback);
#[no_mangle]
pub unsafe extern "C" fn wait_on_page_writeback(page: *mut page) {
    void wait_on_page_writeback(struct page *page)
    {
    return folio_wait_writeback(page_folio(page));
    }
    EXPORT_SYMBOL_GPL(wait_on_page_writeback);
#[no_mangle]
pub unsafe extern "C" fn mark_page_accessed(page: *mut page) {
    void mark_page_accessed(struct page *page)
    {
    folio_mark_accessed(page_folio(page));
    }
    EXPORT_SYMBOL(mark_page_accessed);
#[no_mangle]
pub unsafe extern "C" fn set_page_writeback(page: *mut page) {
    void set_page_writeback(struct page *page)
    {
    folio_start_writeback(page_folio(page));
    }
    EXPORT_SYMBOL(set_page_writeback);
// Read the comment above folio_mark_dirty() regarding required locks!
#[no_mangle]
pub unsafe extern "C" fn set_page_dirty(page: *mut page) -> bool {
    bool set_page_dirty(struct page *page)
    {
    return folio_mark_dirty(page_folio(page));
    }
    EXPORT_SYMBOL(set_page_dirty);
#[no_mangle]
pub unsafe extern "C" fn set_page_dirty_lock(page: *mut page) -> c_int {
    int set_page_dirty_lock(struct page *page)
    {
    return folio_mark_dirty_lock(page_folio(page));
    }
    EXPORT_SYMBOL(set_page_dirty_lock);
#[no_mangle]
pub unsafe extern "C" fn clear_page_dirty_for_io(page: *mut page) -> bool {
    bool clear_page_dirty_for_io(struct page *page)
    {
    return folio_clear_dirty_for_io(page_folio(page));
    }
    EXPORT_SYMBOL(clear_page_dirty_for_io);
    bool redirty_page_for_writepage(struct writeback_control *wbc,
    struct page *page)
    {
    return folio_redirty_for_writepage(wbc, page_folio(page));
    }
    EXPORT_SYMBOL(redirty_page_for_writepage);
    int add_to_page_cache_lru(struct page *page, struct address_space *mapping,
    pgoff_t index, gfp_t gfp)
    {
    return filemap_add_folio(mapping, page_folio(page), index, gfp);
    }
    EXPORT_SYMBOL(add_to_page_cache_lru);
    noinline
    struct page *pagecache_get_page(struct address_space *mapping, pgoff_t index,
    fgf_t fgp_flags, gfp_t gfp)
    {
    struct folio *folio;
    folio = __filemap_get_folio(mapping, index, fgp_flags, gfp);
    if (IS_ERR(folio))
    return core::ptr::null_mut();
    return folio_file_page(folio, index);
    }
    EXPORT_SYMBOL(pagecache_get_page);
