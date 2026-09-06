//! Automatically rewritten from C to Rust
//! Source: mm/page_poison.c
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

    bool _page_poisoning_enabled_early;
    EXPORT_SYMBOL(_page_poisoning_enabled_early);
    DEFINE_STATIC_KEY_FALSE(_page_poisoning_enabled);
    EXPORT_SYMBOL(_page_poisoning_enabled);
#[no_mangle]
unsafe extern "C" fn early_page_poison_param(buf: *mut c_char) -> int __init {
    static int __init early_page_poison_param(char *buf)
    {
    return kstrtobool(buf, &_page_poisoning_enabled_early);
    }
    early_param("page_poison", early_page_poison_param);
#[no_mangle]
unsafe extern "C" fn poison_page(page: *mut page) {
    static void poison_page(struct page *page)
    {
    void *addr = kmap_local_page(page);
// KASAN still think the page is in-use, so skip it.
    kasan_disable_current();
    memset(kasan_reset_tag(addr), PAGE_POISON, PAGE_SIZE);
    kasan_enable_current();
    kunmap_local(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn __kernel_poison_pages(page: *mut page, n: c_int) {
    void __kernel_poison_pages(struct page *page, int n)
    {
    int i;
    for (i = 0; i < n; i++)
    poison_page(page + i);
    }
#[no_mangle]
unsafe extern "C" fn single_bit_flip(a: c_uchar, b: c_uchar) -> bool {
    static bool single_bit_flip(unsigned char a, unsigned char b)
    {
    let mut error: c_uchar = a ^ b;
    return error && !(error & (error - 1));
    }
#[no_mangle]
unsafe extern "C" fn check_poison_mem(page: *mut page, mem: *mut c_uchar, bytes: usize) {
    static void check_poison_mem(struct page *page, unsigned char *mem, size_t bytes)
    {
    static DEFINE_RATELIMIT_STATE(ratelimit, 5 * HZ, 10);
    unsigned char *start;
    unsigned char *end;
    start = memchr_inv(mem, PAGE_POISON, bytes);
    if (!start)
    return;
    for (end = mem + bytes - 1; end > start; end--) {
    if (*end != PAGE_POISON)
    break;
    }
    if (!__ratelimit(&ratelimit))
    return;
#[no_mangle]
pub unsafe extern "C" fn if(single_bit_flip(*start: *mut start == end &&, _arg: PAGE_POISON)) -> else {
    else if (start == end && single_bit_flip(*start, PAGE_POISON))
    pr_err("pagealloc: single bit error\n");
    else
    pr_err("pagealloc: memory corruption\n");
    print_hex_dump(KERN_ERR, "", DUMP_PREFIX_ADDRESS, 16, 1, start,
    end - start + 1, 1);
    dump_stack();
    dump_page(page, "pagealloc: corrupted page details");
    }
#[no_mangle]
unsafe extern "C" fn unpoison_page(page: *mut page) {
    static void unpoison_page(struct page *page)
    {
    void *addr;
    addr = kmap_local_page(page);
    kasan_disable_current();
//
// Page poisoning when enabled poisons each and every page
// that is freed to buddy. Thus no extra check is done to
// see if a page was poisoned.
//
    check_poison_mem(page, kasan_reset_tag(addr), PAGE_SIZE);
    kasan_enable_current();
    kunmap_local(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn __kernel_unpoison_pages(page: *mut page, n: c_int) {
    void __kernel_unpoison_pages(struct page *page, int n)
    {
    int i;
    for (i = 0; i < n; i++)
    unpoison_page(page + i);
    }

#[no_mangle]
pub unsafe extern "C" fn __kernel_map_pages(page: *mut page, numpages: c_int, enable: c_int) {
    void __kernel_map_pages(struct page *page, int numpages, int enable)
    {
// This function does nothing, all work is done via poison pages
    }
