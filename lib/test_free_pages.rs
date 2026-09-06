//! Automatically rewritten from C to Rust
//! Source: lib/test_free_pages.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// test_free_pages.c: Check that free_pages() doesn't leak memory
// Copyright (c) 2020 Oracle
// Author: Matthew Wilcox <willy@infradead.org>
//

#[no_mangle]
unsafe extern "C" fn test_free_pages(gfp: gfp_t) {
    static void test_free_pages(gfp_t gfp)
    {
    unsigned int i;
    for (i = 0; i < 1000 * 1000; i++) {
    let mut addr: c_ulong = __get_free_pages(gfp, 3);
    struct page *page = virt_to_page((void *)addr);
// Simulate page cache getting a speculative reference
    get_page(page);
    free_pages(addr, 3);
    put_page(page);
    }
    }
#[no_mangle]
unsafe extern "C" fn m_in() -> c_int {
    static int m_in(void)
    {
    pr_info("Testing with GFP_KERNEL\n");
    test_free_pages(GFP_KERNEL);
    pr_info("Testing with GFP_KERNEL | __GFP_COMP\n");
    test_free_pages(GFP_KERNEL | __GFP_COMP);
    pr_info("Test completed\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn m_ex() {
    static void m_ex(void)
    {
    }
    module_init(m_in);
    module_exit(m_ex);
    MODULE_AUTHOR("Matthew Wilcox <willy@infradead.org>");
    MODULE_DESCRIPTION("Check that free_pages() doesn't leak memory");
    MODULE_LICENSE("GPL");
