//! Automatically rewritten from C to Rust
//! Source: samples/kmemleak/kmemleak-test.c
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
//
// samples/kmemleak/kmemleak-test.c
//
// Copyright (C) 2008 ARM Limited
// Written by Catalin Marinas <catalin.marinas@arm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_node {
    pub header: [c_long; 25],
    pub list: list_head,
    pub footer: [c_long; 25],
}

    static LIST_HEAD(test_list);
    static DEFINE_PER_CPU(void *, kmemleak_test_pointer);
//
// Some very simple testing. This function needs to be extended for
// proper testing.
//
#[no_mangle]
unsafe extern "C" fn kmemleak_test_init() -> c_int {
    static int kmemleak_test_init(void)
    {
    struct test_node *elem;
    int i;
    pr_info("Kmemleak testing\n");
// make some orphan objects
    pr_info("kmalloc(32) = 0x%px\n", kmalloc(32, GFP_KERNEL));
    pr_info("kmalloc(32) = 0x%px\n", kmalloc(32, GFP_KERNEL));
    pr_info("kmalloc(1024) = 0x%px\n", kmalloc(1024, GFP_KERNEL));
    pr_info("kmalloc(1024) = 0x%px\n", kmalloc(1024, GFP_KERNEL));
    pr_info("kmalloc(2048) = 0x%px\n", kmalloc(2048, GFP_KERNEL));
    pr_info("kmalloc(2048) = 0x%px\n", kmalloc(2048, GFP_KERNEL));
    pr_info("kmalloc(4096) = 0x%px\n", kmalloc(4096, GFP_KERNEL));
    pr_info("kmalloc(4096) = 0x%px\n", kmalloc(4096, GFP_KERNEL));

    pr_info("kmem_cache_alloc(files_cachep) = 0x%px\n",
    kmem_cache_alloc(files_cachep, GFP_KERNEL));
    pr_info("kmem_cache_alloc(files_cachep) = 0x%px\n",
    kmem_cache_alloc(files_cachep, GFP_KERNEL));

    pr_info("vmalloc(64) = 0x%px\n", vmalloc(64));
    pr_info("vmalloc(64) = 0x%px\n", vmalloc(64));
    pr_info("vmalloc(64) = 0x%px\n", vmalloc(64));
    pr_info("vmalloc(64) = 0x%px\n", vmalloc(64));
    pr_info("vmalloc(64) = 0x%px\n", vmalloc(64));
//
// Add elements to a list. They should only appear as orphan
// after the module is removed.
//
    for (i = 0; i < 10; i++) {
    elem = kzalloc(sizeof(*elem), GFP_KERNEL);
    pr_info("kzalloc(sizeof(*elem)) = 0x%px\n", elem);
    if (!elem)
    return -ENOMEM;
    INIT_LIST_HEAD(&elem.list);
    list_add_tail(&elem.list, &test_list);
    }
    for_each_possible_cpu(i) {
    per_cpu(kmemleak_test_pointer, i) = kmalloc(129, GFP_KERNEL);
    pr_info("kmalloc(129) = 0x%px\n",
    per_cpu(kmemleak_test_pointer, i));
    }
    pr_info("__alloc_percpu(64, 4) = 0x%px\n", __alloc_percpu(64, 4));
    return 0;
    }
    module_init(kmemleak_test_init);
#[no_mangle]
unsafe extern "C" fn kmemleak_test_exit() -> void __exit {
    static void __exit kmemleak_test_exit(void)
    {
    struct test_node *elem, *tmp;
//
// Remove the list elements without actually freeing the
// memory.
//
    list_for_each_entry_safe(elem, tmp, &test_list, list)
    list_del(&elem.list);
    }
    module_exit(kmemleak_test_exit);
    MODULE_DESCRIPTION("Sample module to leak memory for kmemleak testing");
    MODULE_LICENSE("GPL");
