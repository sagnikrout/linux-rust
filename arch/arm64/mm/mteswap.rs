//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/mteswap.c
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

    static DEFINE_XARRAY(mte_pages);
    void *mte_allocate_tag_storage(void)
    {
// tags granule is 16 bytes, 2 tags stored per byte
    return kmalloc(MTE_PAGE_TAG_STORAGE, GFP_KERNEL);
    }
#[no_mangle]
pub unsafe extern "C" fn mte_free_tag_storage(storage: *mut c_char) {
    void mte_free_tag_storage(char *storage)
    {
    kfree(storage);
    }
#[no_mangle]
pub unsafe extern "C" fn mte_save_tags(page: *mut page) -> c_int {
    int mte_save_tags(struct page *page)
    {
    void *tag_storage, *ret;
    if (!page_mte_tagged(page))
    return 0;
    tag_storage = mte_allocate_tag_storage();
    if (!tag_storage)
    return -ENOMEM;
    mte_save_page_tags(page_address(page), tag_storage);
// lookup the swap entry.val from the page
    ret = xa_store(&mte_pages, page_swap_entry(page).val, tag_storage,
    GFP_KERNEL);
    if (WARN(xa_is_err(ret), "Failed to store MTE tags")) {
    mte_free_tag_storage(tag_storage);
    return xa_err(ret);
    } else if (ret) {
// Entry is being replaced, free the old entry
    mte_free_tag_storage(ret);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mte_restore_tags(entry: swp_entry_t, page: *mut page) {
    void mte_restore_tags(swp_entry_t entry, struct page *page)
    {
    void *tags = xa_load(&mte_pages, entry.val);
    if (!tags)
    return;
    if (try_page_mte_tagging(page)) {
    mte_restore_page_tags(page_address(page), tags);
    set_page_mte_tagged(page);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mte_invalidate_tags(type: c_int, offset: pgoff_t) {
    void mte_invalidate_tags(int type, pgoff_t offset)
    {
    let mut entry: swp_entry_t = swp_entry(type, offset);
    void *tags = xa_erase(&mte_pages, entry.val);
    mte_free_tag_storage(tags);
    }
#[no_mangle]
pub unsafe extern "C" fn __mte_invalidate_tags(page: *mut page) {
    static inline void __mte_invalidate_tags(struct page *page)
    {
    let mut entry: swp_entry_t = page_swap_entry(page);
    mte_invalidate_tags(swp_type(entry), swp_offset(entry));
    }
#[no_mangle]
pub unsafe extern "C" fn mte_invalidate_tags_area(type: c_int) {
    void mte_invalidate_tags_area(int type)
    {
    let mut entry: swp_entry_t = swp_entry(type, 0);
    let mut last_entry: swp_entry_t = swp_entry(type + 1, 0);
    void *tags;
    XA_STATE(xa_state, &mte_pages, entry.val);
    xa_lock(&mte_pages);
    xas_for_each(&xa_state, tags, last_entry.val - 1) {
    __xa_erase(&mte_pages, xa_state.xa_index);
    mte_free_tag_storage(tags);
    }
    xa_unlock(&mte_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prepare_to_swap(folio: *mut folio) -> c_int {
    int arch_prepare_to_swap(struct folio *folio)
    {
    long i, nr;
    int err;
    if (!system_supports_mte())
    return 0;
    nr = folio_nr_pages(folio);
    for (i = 0; i < nr; i++) {
    err = mte_save_tags(folio_page(folio, i));
    if (err)
    goto out;
    }
    return 0;
    out:
    while (i--)
    __mte_invalidate_tags(folio_page(folio, i));
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_swap_restore(entry: swp_entry_t, folio: *mut folio) {
    void arch_swap_restore(swp_entry_t entry, struct folio *folio)
    {
    long i, nr;
    if (!system_supports_mte())
    return;
    nr = folio_nr_pages(folio);
    for (i = 0; i < nr; i++) {
    mte_restore_tags(entry, folio_page(folio, i));
    entry.val++;
    }
    }
