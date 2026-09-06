//! Automatically rewritten from C to Rust
//! Source: mm/kmsan/shadow.c
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
// KMSAN shadow implementation.
//
// Copyright (C) 2017-2022 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

    static void *shadow_ptr_for(struct page *page)
    {
    return page_address(shadow_page_for(page));
    }
    static void *origin_ptr_for(struct page *page)
    {
    return page_address(origin_page_for(page));
    }
#[no_mangle]
unsafe extern "C" fn page_has_metadata(page: *mut page) -> bool {
    static bool page_has_metadata(struct page *page)
    {
    return shadow_page_for(page) && origin_page_for(page);
    }
#[no_mangle]
unsafe extern "C" fn set_no_shadow_origin_page(page: *mut page) {
    static void set_no_shadow_origin_page(struct page *page)
    {
    shadow_page_for(page) = core::ptr::null_mut();
    origin_page_for(page) = core::ptr::null_mut();
    }
//
// Dummy load and store pages to be used when the real metadata is unavailable.
// There are separate pages for loads and stores, so that every load returns a
// zero, and every store doesn't affect other loads.
//
    static char dummy_load_page[PAGE_SIZE] __aligned(PAGE_SIZE);
    static char dummy_store_page[PAGE_SIZE] __aligned(PAGE_SIZE);
#[no_mangle]
unsafe extern "C" fn vmalloc_meta(addr: *mut c_void, is_origin: bool) -> c_ulong {
    static unsigned long vmalloc_meta(void *addr, bool is_origin)
    {
    let mut addr64: c_ulong = (unsigned long)addr, off;
    KMSAN_WARN_ON(is_origin && !IS_ALIGNED(addr64, KMSAN_ORIGIN_SIZE));
    if (kmsan_internal_is_vmalloc_addr(addr)) {
    off = addr64 - VMALLOC_START;
    return off + (is_origin ? KMSAN_VMALLOC_ORIGIN_START :
    KMSAN_VMALLOC_SHADOW_START);
    }
    if (kmsan_internal_is_module_addr(addr)) {
    off = addr64 - MODULES_VADDR;
    return off + (is_origin ? KMSAN_MODULES_ORIGIN_START :
    KMSAN_MODULES_SHADOW_START);
    }
    return 0;
    }
    static struct page *virt_to_page_or_null(void *vaddr)
    {
    if (kmsan_virt_addr_valid(vaddr))
    return virt_to_page(vaddr);
    else
    return core::ptr::null_mut();
    }
    struct shadow_origin_ptr kmsan_get_shadow_origin_ptr(void *address, u64 size,
    bool store)
    {
    struct shadow_origin_ptr ret;
    void *shadow;
//
// Even if we redirect this memory access to the dummy page, it will
// go out of bounds.
//
    KMSAN_WARN_ON(size > PAGE_SIZE);
    if (!kmsan_enabled)
    goto return_dummy;
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(address, size));
    shadow = kmsan_get_metadata(address, KMSAN_META_SHADOW);
    if (!shadow)
    goto return_dummy;
    ret.shadow = shadow;
    ret.origin = kmsan_get_metadata(address, KMSAN_META_ORIGIN);
    return ret;
    return_dummy:
    if (store) {
// Ignore this store.
    ret.shadow = dummy_store_page;
    ret.origin = dummy_store_page;
    } else {
// This load will return zero.
    ret.shadow = dummy_load_page;
    ret.origin = dummy_load_page;
    }
    return ret;
    }
//
// Obtain the shadow or origin pointer for the given address, or NULL if there's
// none. The caller must check the return value for being non-NULL if needed.
// The return value of this function should not depend on whether we're in the
// runtime or not.
//
    void *kmsan_get_metadata(void *address, bool is_origin)
    {
    let mut addr: u64 = (u64)address, off;
    struct page *page;
    void *ret;
    if (is_origin)
    addr = ALIGN_DOWN(addr, KMSAN_ORIGIN_SIZE);
    address = (void *)addr;
    if (kmsan_internal_is_vmalloc_addr(address) ||
    kmsan_internal_is_module_addr(address))
    return (void *)vmalloc_meta(address, is_origin);
    ret = arch_kmsan_get_meta_or_null(address, is_origin);
    if (ret)
    return ret;
    page = virt_to_page_or_null(address);
    if (!page)
    return core::ptr::null_mut();
    if (!page_has_metadata(page))
    return core::ptr::null_mut();
    off = offset_in_page(addr);
    return (is_origin ? origin_ptr_for(page) : shadow_ptr_for(page)) + off;
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_copy_page_meta(dst: *mut page, src: *mut page) {
    void kmsan_copy_page_meta(struct page *dst, struct page *src)
    {
    if (!kmsan_enabled || kmsan_in_runtime())
    return;
    if (!dst || !page_has_metadata(dst))
    return;
    if (!src || !page_has_metadata(src)) {
    kmsan_internal_unpoison_memory(page_address(dst), PAGE_SIZE,
// checked*/ false);
    return;
    }
    kmsan_enter_runtime();
    __memcpy(shadow_ptr_for(dst), shadow_ptr_for(src), PAGE_SIZE);
    __memcpy(origin_ptr_for(dst), origin_ptr_for(src), PAGE_SIZE);
    kmsan_leave_runtime();
    }
    EXPORT_SYMBOL(kmsan_copy_page_meta);
#[no_mangle]
pub unsafe extern "C" fn kmsan_alloc_page(page: *mut page, order: c_uint, flags: gfp_t) {
    void kmsan_alloc_page(struct page *page, unsigned int order, gfp_t flags)
    {
    let mut initialized: bool = (flags & __GFP_ZERO) || !kmsan_enabled;
    struct page *shadow, *origin;
    depot_stack_handle_t handle;
    let mut pages: c_int = 1 << order;
    if (!page)
    return;
    shadow = shadow_page_for(page);
    origin = origin_page_for(page);
    if (initialized) {
    __memset(page_address(shadow), 0, PAGE_SIZE * pages);
    __memset(page_address(origin), 0, PAGE_SIZE * pages);
    return;
    }
// Zero pages allocated by the runtime should also be initialized.
    if (kmsan_in_runtime())
    return;
    __memset(page_address(shadow), -1, PAGE_SIZE * pages);
    kmsan_enter_runtime();
    handle = kmsan_save_stack_with_flags(flags, /*extra_bits*/ 0);
    kmsan_leave_runtime();
//
// Addresses are page-aligned, pages are contiguous, so it's ok
// to just fill the origin pages with @handle.
//
    for (int i = 0; i < PAGE_SIZE * pages / sizeof(handle); i++)
    ((depot_stack_handle_t *)page_address(origin))[i] = handle;
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_free_page(page: *mut page, order: c_uint) {
    void kmsan_free_page(struct page *page, unsigned int order)
    {
    if (!kmsan_enabled || kmsan_in_runtime())
    return;
    kmsan_enter_runtime();
    kmsan_internal_poison_memory(page_address(page), PAGE_SIZE << order,
    GFP_KERNEL & ~(__GFP_RECLAIM),
    KMSAN_POISON_CHECK | KMSAN_POISON_FREE);
    kmsan_leave_runtime();
    }
    int kmsan_vmap_pages_range_noflush(unsigned long start, unsigned long end,
    pgprot_t prot, struct page **pages,
    unsigned int page_shift, gfp_t gfp_mask)
    {
    unsigned long shadow_start, origin_start, shadow_end, origin_end;
    struct page **s_pages, **o_pages;
    int nr, mapped, err = 0;
    if (!kmsan_enabled)
    return 0;
    shadow_start = vmalloc_meta((void *)start, KMSAN_META_SHADOW);
    shadow_end = vmalloc_meta((void *)end, KMSAN_META_SHADOW);
    if (!shadow_start)
    return 0;
    nr = (end - start) / PAGE_SIZE;
    s_pages = kzalloc_objs(*s_pages, nr, gfp_mask);
    o_pages = kzalloc_objs(*o_pages, nr, gfp_mask);
    if (!s_pages || !o_pages) {
    err = -ENOMEM;
    goto ret;
    }
    for (int i = 0; i < nr; i++) {
    s_pages[i] = shadow_page_for(pages[i]);
    o_pages[i] = origin_page_for(pages[i]);
    }
    prot = PAGE_KERNEL;
    origin_start = vmalloc_meta((void *)start, KMSAN_META_ORIGIN);
    origin_end = vmalloc_meta((void *)end, KMSAN_META_ORIGIN);
    kmsan_enter_runtime();
    mapped = __vmap_pages_range_noflush(shadow_start, shadow_end, prot,
    s_pages, page_shift);
    kmsan_leave_runtime();
    if (mapped) {
    err = mapped;
    goto ret;
    }
    kmsan_enter_runtime();
    mapped = __vmap_pages_range_noflush(origin_start, origin_end, prot,
    o_pages, page_shift);
    kmsan_leave_runtime();
    if (mapped) {
    err = mapped;
    goto ret;
    }
    flush_tlb_kernel_range(shadow_start, shadow_end);
    flush_tlb_kernel_range(origin_start, origin_end);
    flush_cache_vmap(shadow_start, shadow_end);
    flush_cache_vmap(origin_start, origin_end);
    ret:
    kfree(s_pages);
    kfree(o_pages);
    return err;
    }
// Allocate metadata for pages allocated at boot time.
#[no_mangle]
pub unsafe extern "C" fn kmsan_init_alloc_meta_for_range(start: *mut c_void, end: *mut c_void) -> void __init {
    void __init kmsan_init_alloc_meta_for_range(void *start, void *end)
    {
    struct page *shadow_p, *origin_p;
    void *shadow, *origin;
    struct page *page;
    u64 size;
    start = (void *)PAGE_ALIGN_DOWN((u64)start);
    size = PAGE_ALIGN((u64)end - (u64)start);
    shadow = memblock_alloc_or_panic(size, PAGE_SIZE);
    origin = memblock_alloc_or_panic(size, PAGE_SIZE);
    for (u64 addr = 0; addr < size; addr += PAGE_SIZE) {
    page = virt_to_page_or_null((char *)start + addr);
    shadow_p = virt_to_page((char *)shadow + addr);
    set_no_shadow_origin_page(shadow_p);
    shadow_page_for(page) = shadow_p;
    origin_p = virt_to_page((char *)origin + addr);
    set_no_shadow_origin_page(origin_p);
    origin_page_for(page) = origin_p;
    }
    }
    void kmsan_setup_meta(struct page *page, struct page *shadow,
    struct page *origin, int order)
    {
    for (int i = 0; i < (1 << order); i++) {
    set_no_shadow_origin_page(&shadow[i]);
    set_no_shadow_origin_page(&origin[i]);
    shadow_page_for(&page[i]) = &shadow[i];
    origin_page_for(&page[i]) = &origin[i];
    }
    }
