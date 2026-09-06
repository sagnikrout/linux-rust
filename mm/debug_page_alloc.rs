//! Automatically rewritten from C to Rust
//! Source: mm/debug_page_alloc.c
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

    unsigned int _debug_guardpage_minorder;
    bool _debug_pagealloc_enabled_early __read_mostly
    = IS_ENABLED(CONFIG_DEBUG_PAGEALLOC_ENABLE_DEFAULT);
    EXPORT_SYMBOL(_debug_pagealloc_enabled_early);
    DEFINE_STATIC_KEY_FALSE(_debug_pagealloc_enabled);
    EXPORT_SYMBOL(_debug_pagealloc_enabled);
    DEFINE_STATIC_KEY_FALSE(_debug_guardpage_enabled);
#[no_mangle]
unsafe extern "C" fn early_debug_pagealloc(buf: *mut c_char) -> int __init {
    static int __init early_debug_pagealloc(char *buf)
    {
    return kstrtobool(buf, &_debug_pagealloc_enabled_early);
    }
    early_param("debug_pagealloc", early_debug_pagealloc);
#[no_mangle]
unsafe extern "C" fn debug_guardpage_minorder_setup(buf: *mut c_char) -> int __init {
    static int __init debug_guardpage_minorder_setup(char *buf)
    {
    unsigned int res;
    if (!buf || kstrtouint(buf, 10, &res) < 0 || res > MAX_PAGE_ORDER / 2) {
    pr_err("Bad debug_guardpage_minorder value: %s\n", buf ?: "(missing)");
    return 0;
    }
    _debug_guardpage_minorder = res;
    pr_info("Setting debug_guardpage_minorder to %u\n", res);
    return 0;
    }
    early_param("debug_guardpage_minorder", debug_guardpage_minorder_setup);
#[no_mangle]
pub unsafe extern "C" fn __set_page_guard(zone: *mut zone, page: *mut page, order: c_uint) -> bool {
    bool __set_page_guard(struct zone *zone, struct page *page, unsigned int order)
    {
    if (order >= debug_guardpage_minorder())
    return false;
    __SetPageGuard(page);
    INIT_LIST_HEAD(&page.buddy_list);
    set_page_private(page, order);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __clear_page_guard(zone: *mut zone, page: *mut page, order: c_uint) {
    void __clear_page_guard(struct zone *zone, struct page *page, unsigned int order)
    {
    __ClearPageGuard(page);
    set_page_private(page, 0);
    }
