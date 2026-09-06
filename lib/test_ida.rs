//! Automatically rewritten from C to Rust
//! Source: lib/test_ida.c
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
// test_ida.c: Test the IDA API
// Copyright (c) 2016-2018 Microsoft Corporation
// Copyright (c) 2018 Oracle Corporation
// Author: Matthew Wilcox <willy@infradead.org>
//

    static unsigned int tests_run;
    static unsigned int tests_passed;

    static void ida_dump(struct ida *ida) { }

    tests_run++;							\
    if (x) {							\
    ida_dump(ida);						\
    dump_stack();						\
    } else {							\
    tests_passed++;						\
    }								\
    } while (0)
//
// Straightforward checks that allocating and freeing IDs work.
//
#[no_mangle]
unsafe extern "C" fn ida_check_alloc(ida: *mut ida) {
    static void ida_check_alloc(struct ida *ida)
    {
    int i, id;
    for (i = 0; i < 10000; i++)
    IDA_BUG_ON(ida, ida_alloc(ida, GFP_KERNEL) != i);
    ida_free(ida, 20);
    ida_free(ida, 21);
    for (i = 0; i < 3; i++) {
    id = ida_alloc(ida, GFP_KERNEL);
    IDA_BUG_ON(ida, id < 0);
    if (i == 2)
    IDA_BUG_ON(ida, id != 10000);
    }
    for (i = 0; i < 5000; i++)
    ida_free(ida, i);
    IDA_BUG_ON(ida, ida_alloc_min(ida, 5000, GFP_KERNEL) != 10001);
    ida_destroy(ida);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    }
// Destroy an IDA with a single entry at @base
#[no_mangle]
unsafe extern "C" fn ida_check_destroy_1(ida: *mut ida, base: c_uint) {
    static void ida_check_destroy_1(struct ida *ida, unsigned int base)
    {
    IDA_BUG_ON(ida, ida_alloc_min(ida, base, GFP_KERNEL) != base);
    IDA_BUG_ON(ida, ida_is_empty(ida));
    ida_destroy(ida);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    }
// Check that ida_destroy and ida_is_empty work
#[no_mangle]
unsafe extern "C" fn ida_check_destroy(ida: *mut ida) {
    static void ida_check_destroy(struct ida *ida)
    {
// Destroy an already-empty IDA
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    ida_destroy(ida);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    ida_check_destroy_1(ida, 0);
    ida_check_destroy_1(ida, 1);
    ida_check_destroy_1(ida, 1023);
    ida_check_destroy_1(ida, 1024);
    ida_check_destroy_1(ida, 12345678);
    }
//
// Check what happens when we fill a leaf and then delete it.  This may
// discover mishandling of IDR_FREE.
//
#[no_mangle]
unsafe extern "C" fn ida_check_leaf(ida: *mut ida, base: c_uint) {
    static void ida_check_leaf(struct ida *ida, unsigned int base)
    {
    unsigned long i;
    for (i = 0; i < IDA_BITMAP_BITS; i++) {
    IDA_BUG_ON(ida, ida_alloc_min(ida, base, GFP_KERNEL) !=
    base + i);
    }
    ida_destroy(ida);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    IDA_BUG_ON(ida, ida_alloc(ida, GFP_KERNEL) != 0);
    IDA_BUG_ON(ida, ida_is_empty(ida));
    ida_free(ida, 0);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    }
//
// Check allocations up to and slightly above the maximum allowed (2^31-1) ID.
// Allocating up to 2^31-1 should succeed, and then allocating the next one
// should fail.
//
#[no_mangle]
unsafe extern "C" fn ida_check_max(ida: *mut ida) {
    static void ida_check_max(struct ida *ida)
    {
    unsigned long i, j;
    for (j = 1; j < 65537; j *= 2) {
    let mut base: c_ulong = (1UL << 31) - j;
    for (i = 0; i < j; i++) {
    IDA_BUG_ON(ida, ida_alloc_min(ida, base, GFP_KERNEL) !=
    base + i);
    }
    IDA_BUG_ON(ida, ida_alloc_min(ida, base, GFP_KERNEL) !=
    -ENOSPC);
    ida_destroy(ida);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    }
    }
//
// Check handling of conversions between exceptional entries and full bitmaps.
//
#[no_mangle]
unsafe extern "C" fn ida_check_conv(ida: *mut ida) {
    static void ida_check_conv(struct ida *ida)
    {
    unsigned long i;
    for (i = 0; i < IDA_BITMAP_BITS * 2; i += IDA_BITMAP_BITS) {
    IDA_BUG_ON(ida, ida_alloc_min(ida, i + 1, GFP_KERNEL) != i + 1);
    IDA_BUG_ON(ida, ida_alloc_min(ida, i + BITS_PER_LONG,
    GFP_KERNEL) != i + BITS_PER_LONG);
    ida_free(ida, i + 1);
    ida_free(ida, i + BITS_PER_LONG);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    }
    for (i = 0; i < IDA_BITMAP_BITS * 2; i++)
    IDA_BUG_ON(ida, ida_alloc(ida, GFP_KERNEL) != i);
    for (i = IDA_BITMAP_BITS * 2; i > 0; i--)
    ida_free(ida, i - 1);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    for (i = 0; i < IDA_BITMAP_BITS + BITS_PER_LONG - 4; i++)
    IDA_BUG_ON(ida, ida_alloc(ida, GFP_KERNEL) != i);
    for (i = IDA_BITMAP_BITS + BITS_PER_LONG - 4; i > 0; i--)
    ida_free(ida, i - 1);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    }
//
// Check various situations where we attempt to free an ID we don't own.
//
#[no_mangle]
unsafe extern "C" fn ida_check_bad_free(ida: *mut ida) {
    static void ida_check_bad_free(struct ida *ida)
    {
    unsigned long i;
    printk("vvv Ignore \"not allocated\" warnings\n");
// IDA is empty; all of these will fail
    ida_free(ida, 0);
    for (i = 0; i < 31; i++)
    ida_free(ida, 1 << i);
// IDA contains a single value entry
    IDA_BUG_ON(ida, ida_alloc_min(ida, 3, GFP_KERNEL) != 3);
    ida_free(ida, 0);
    for (i = 0; i < 31; i++)
    ida_free(ida, 1 << i);
// IDA contains a single bitmap
    IDA_BUG_ON(ida, ida_alloc_min(ida, 1023, GFP_KERNEL) != 1023);
    ida_free(ida, 0);
    for (i = 0; i < 31; i++)
    ida_free(ida, 1 << i);
// IDA contains a tree
    IDA_BUG_ON(ida, ida_alloc_min(ida, (1 << 20) - 1, GFP_KERNEL) != (1 << 20) - 1);
    ida_free(ida, 0);
    for (i = 0; i < 31; i++)
    ida_free(ida, 1 << i);
    printk("^^^ \"not allocated\" warnings over\n");
    ida_free(ida, 3);
    ida_free(ida, 1023);
    ida_free(ida, (1 << 20) - 1);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    }
//
// Check ida_find_first_range() and varriants.
//
#[no_mangle]
unsafe extern "C" fn ida_check_find_first(ida: *mut ida) {
    static void ida_check_find_first(struct ida *ida)
    {
// IDA is empty; all of the below should be not exist
    IDA_BUG_ON(ida, ida_exists(ida, 0));
    IDA_BUG_ON(ida, ida_exists(ida, 3));
    IDA_BUG_ON(ida, ida_exists(ida, 63));
    IDA_BUG_ON(ida, ida_exists(ida, 1023));
    IDA_BUG_ON(ida, ida_exists(ida, (1 << 20) - 1));
// IDA contains a single value entry
    IDA_BUG_ON(ida, ida_alloc_min(ida, 3, GFP_KERNEL) != 3);
    IDA_BUG_ON(ida, ida_exists(ida, 0));
    IDA_BUG_ON(ida, !ida_exists(ida, 3));
    IDA_BUG_ON(ida, ida_exists(ida, 63));
    IDA_BUG_ON(ida, ida_exists(ida, 1023));
    IDA_BUG_ON(ida, ida_exists(ida, (1 << 20) - 1));
    IDA_BUG_ON(ida, ida_alloc_min(ida, 63, GFP_KERNEL) != 63);
    IDA_BUG_ON(ida, ida_exists(ida, 0));
    IDA_BUG_ON(ida, !ida_exists(ida, 3));
    IDA_BUG_ON(ida, !ida_exists(ida, 63));
    IDA_BUG_ON(ida, ida_exists(ida, 1023));
    IDA_BUG_ON(ida, ida_exists(ida, (1 << 20) - 1));
// IDA contains a single bitmap
    IDA_BUG_ON(ida, ida_alloc_min(ida, 1023, GFP_KERNEL) != 1023);
    IDA_BUG_ON(ida, ida_exists(ida, 0));
    IDA_BUG_ON(ida, !ida_exists(ida, 3));
    IDA_BUG_ON(ida, !ida_exists(ida, 63));
    IDA_BUG_ON(ida, !ida_exists(ida, 1023));
    IDA_BUG_ON(ida, ida_exists(ida, (1 << 20) - 1));
// IDA contains a tree
    IDA_BUG_ON(ida, ida_alloc_min(ida, (1 << 20) - 1, GFP_KERNEL) != (1 << 20) - 1);
    IDA_BUG_ON(ida, ida_exists(ida, 0));
    IDA_BUG_ON(ida, !ida_exists(ida, 3));
    IDA_BUG_ON(ida, !ida_exists(ida, 63));
    IDA_BUG_ON(ida, !ida_exists(ida, 1023));
    IDA_BUG_ON(ida, !ida_exists(ida, (1 << 20) - 1));
// Now try to find first
    IDA_BUG_ON(ida, ida_find_first(ida) != 3);
    IDA_BUG_ON(ida, ida_find_first_range(ida, -1, 2) != -EINVAL);
    IDA_BUG_ON(ida, ida_find_first_range(ida, 0, 2) != -ENOENT); // no used ID
    IDA_BUG_ON(ida, ida_find_first_range(ida, 0, 3) != 3);
    IDA_BUG_ON(ida, ida_find_first_range(ida, 1, 3) != 3);
    IDA_BUG_ON(ida, ida_find_first_range(ida, 3, 3) != 3);
    IDA_BUG_ON(ida, ida_find_first_range(ida, 2, 4) != 3);
    IDA_BUG_ON(ida, ida_find_first_range(ida, 4, 3) != -ENOENT); // min > max, fail
    IDA_BUG_ON(ida, ida_find_first_range(ida, 4, 60) != -ENOENT); // no used ID
    IDA_BUG_ON(ida, ida_find_first_range(ida, 4, 64) != 63);
    IDA_BUG_ON(ida, ida_find_first_range(ida, 63, 63) != 63);
    IDA_BUG_ON(ida, ida_find_first_range(ida, 64, 1026) != 1023);
    IDA_BUG_ON(ida, ida_find_first_range(ida, 1023, 1023) != 1023);
    IDA_BUG_ON(ida, ida_find_first_range(ida, 1023, (1 << 20) - 1) != 1023);
    IDA_BUG_ON(ida, ida_find_first_range(ida, 1024, (1 << 20) - 1) != (1 << 20) - 1);
    IDA_BUG_ON(ida, ida_find_first_range(ida, (1 << 20), INT_MAX) != -ENOENT);
    ida_free(ida, 3);
    ida_free(ida, 63);
    ida_free(ida, 1023);
    ida_free(ida, (1 << 20) - 1);
    IDA_BUG_ON(ida, !ida_is_empty(ida));
    }
    static DEFINE_IDA(ida);
#[no_mangle]
unsafe extern "C" fn ida_checks() -> c_int {
    static int ida_checks(void)
    {
    IDA_BUG_ON(&ida, !ida_is_empty(&ida));
    ida_check_alloc(&ida);
    ida_check_destroy(&ida);
    ida_check_leaf(&ida, 0);
    ida_check_leaf(&ida, 1024);
    ida_check_leaf(&ida, 1024 * 64);
    ida_check_max(&ida);
    ida_check_conv(&ida);
    ida_check_bad_free(&ida);
    ida_check_find_first(&ida);
    printk("IDA: %u of %u tests passed\n", tests_passed, tests_run);
    return (tests_run != tests_passed) ? 0 : -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn ida_exit() {
    static void ida_exit(void)
    {
    }
    module_init(ida_checks);
    module_exit(ida_exit);
    MODULE_AUTHOR("Matthew Wilcox <willy@infradead.org>");
    MODULE_DESCRIPTION("Test the IDA API");
    MODULE_LICENSE("GPL");
