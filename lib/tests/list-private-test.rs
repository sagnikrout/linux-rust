//! Automatically rewritten from C to Rust
//! Source: lib/tests/list-private-test.c
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
// KUnit compilation/smoke test for Private list primitives.
//
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//

//
// This forces compiler to warn if you access it directly, because list
// primitives expect (struct list_head *), not (volatile struct list_head *).
//

// Redefine ACCESS_PRIVATE for this test.

    (*((struct list_head *)((unsigned long)&((p).member))))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_test_struct {
    pub data: c_int,
    pub list: list_head __private,
}

#[no_mangle]
unsafe extern "C" fn list_private_compile_test(test: *mut kunit) {
    static void list_private_compile_test(struct kunit *test)
    {
    struct list_test_struct entry;
    struct list_test_struct *pos, *n;
    LIST_HEAD(head);
    INIT_LIST_HEAD(&ACCESS_PRIVATE(&entry, list));
    list_add(&ACCESS_PRIVATE(&entry, list), &head);
    pos = &entry;
    pos = list_private_entry(&ACCESS_PRIVATE(&entry, list), struct list_test_struct, list);
    pos = list_private_first_entry(&head, struct list_test_struct, list);
    pos = list_private_last_entry(&head, struct list_test_struct, list);
    pos = list_private_next_entry(pos, list);
    pos = list_private_prev_entry(pos, list);
    pos = list_private_next_entry_circular(pos, &head, list);
    pos = list_private_prev_entry_circular(pos, &head, list);
    if (list_private_entry_is_head(pos, &head, list))
    return;
    list_private_for_each_entry(pos, &head, list) { }
    list_private_for_each_entry_reverse(pos, &head, list) { }
    list_private_for_each_entry_continue(pos, &head, list) { }
    list_private_for_each_entry_continue_reverse(pos, &head, list) { }
    list_private_for_each_entry_from(pos, &head, list) { }
    list_private_for_each_entry_from_reverse(pos, &head, list) { }
    list_private_for_each_entry_safe(pos, n, &head, list)
    list_private_safe_reset_next(pos, n, list);
    list_private_for_each_entry_safe_continue(pos, n, &head, list) { }
    list_private_for_each_entry_safe_from(pos, n, &head, list) { }
    list_private_for_each_entry_safe_reverse(pos, n, &head, list) { }
    }
    static struct kunit_case list_private_test_cases[] = {
    KUNIT_CASE(list_private_compile_test),
    {},
    };
    static struct kunit_suite list_private_test_module = {
    .name = "list-private-kunit-test",
    .test_cases = list_private_test_cases,
    };
    kunit_test_suite(list_private_test_module);
    MODULE_DESCRIPTION("KUnit compilation test for private list primitives");
    MODULE_LICENSE("GPL");
