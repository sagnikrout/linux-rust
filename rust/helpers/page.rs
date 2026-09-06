//! Automatically rewritten from C to Rust
//! Source: rust/helpers/page.c
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

    __rust_helper struct page *rust_helper_alloc_pages(gfp_t gfp_mask,
    unsigned int order)
    {
    return alloc_pages(gfp_mask, order);
    }
    __rust_helper void *rust_helper_kmap_local_page(struct page *page)
    {
    return kmap_local_page(page);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_kunmap_local(addr: *const c_void) -> __rust_helper void {
    __rust_helper void rust_helper_kunmap_local(const void *addr)
    {
    kunmap_local(addr);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_page_to_nid(page: *const page) -> __rust_helper int {
    __rust_helper int rust_helper_page_to_nid(const struct page *page)
    {
    return page_to_nid(page);
    }
