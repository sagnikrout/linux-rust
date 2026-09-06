//! Automatically rewritten from C to Rust
//! Source: mm/fail_page_alloc.c
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

    static struct {
    struct fault_attr attr;
    bool ignore_gfp_highmem;
    bool ignore_gfp_reclaim;
    u32 min_order;
    } fail_page_alloc = {
    .attr = FAULT_ATTR_INITIALIZER,
    .ignore_gfp_reclaim = true,
    .ignore_gfp_highmem = true,
    .min_order = 1,
    };
#[no_mangle]
unsafe extern "C" fn setup_fail_page_alloc(str: *mut c_char) -> int __init {
    static int __init setup_fail_page_alloc(char *str)
    {
    return setup_fault_attr(&fail_page_alloc.attr, str);
    }
    __setup("fail_page_alloc=", setup_fail_page_alloc);
#[no_mangle]
pub unsafe extern "C" fn should_fail_alloc_page(gfp_mask: gfp_t, order: c_uint) -> bool {
    bool should_fail_alloc_page(gfp_t gfp_mask, unsigned int order)
    {
    let mut flags: c_int = 0;
    if (order < fail_page_alloc.min_order)
    return false;
    if (gfp_mask & __GFP_NOFAIL)
    return false;
    if (fail_page_alloc.ignore_gfp_highmem && (gfp_mask & __GFP_HIGHMEM))
    return false;
    if (fail_page_alloc.ignore_gfp_reclaim &&
    (gfp_mask & __GFP_DIRECT_RECLAIM))
    return false;
// See comment in __should_failslab()
    if (gfp_mask & __GFP_NOWARN)
    flags |= FAULT_NOWARN;
    return should_fail_ex(&fail_page_alloc.attr, 1 << order, flags);
    }
    ALLOW_ERROR_INJECTION(should_fail_alloc_page, TRUE);

#[no_mangle]
unsafe extern "C" fn fail_page_alloc_debugfs() -> int __init {
    static int __init fail_page_alloc_debugfs(void)
    {
    let mut mode: umode_t = S_IFREG | 0600;
    struct dentry *dir;
    dir = fault_create_debugfs_attr("fail_page_alloc", core::ptr::null_mut(),
    &fail_page_alloc.attr);
    debugfs_create_bool("ignore-gfp-wait", mode, dir,
    &fail_page_alloc.ignore_gfp_reclaim);
    debugfs_create_bool("ignore-gfp-highmem", mode, dir,
    &fail_page_alloc.ignore_gfp_highmem);
    debugfs_create_u32("min-order", mode, dir, &fail_page_alloc.min_order);
    return 0;
    }
    late_initcall(fail_page_alloc_debugfs);
