//! Automatically rewritten from C to Rust
//! Source: lib/tests/test_sort.c
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

// a simple boot-time regression test
pub const TEST_LEN: c_int = 1000;
#[no_mangle]
unsafe extern "C" fn cmpint(a: *const c_void, b: *const c_void) -> c_int {
    static int cmpint(const void *a, const void *b)
    {
    return *(int *)a - *(int *)b;
    }
#[no_mangle]
unsafe extern "C" fn test_sort(test: *mut kunit) {
    static void test_sort(struct kunit *test)
    {
    int *a, i, r = 1;
    a = kunit_kmalloc_array(test, TEST_LEN, sizeof(*a), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, a);
    for (i = 0; i < TEST_LEN; i++) {
    r = (r * 725861) % 6599;
    a[i] = r;
    }
    sort(a, TEST_LEN, sizeof(*a), cmpint, core::ptr::null_mut());
    for (i = 0; i < TEST_LEN - 1; i++)
    KUNIT_ASSERT_LE(test, a[i], a[i + 1]);
    r = 48;
    for (i = 0; i < TEST_LEN - 1; i++) {
    r = (r * 725861) % 6599;
    a[i] = r;
    }
    sort(a, TEST_LEN - 1, sizeof(*a), cmpint, core::ptr::null_mut());
    for (i = 0; i < TEST_LEN - 2; i++)
    KUNIT_ASSERT_LE(test, a[i], a[i + 1]);
    }
    static struct kunit_case sort_test_cases[] = {
    KUNIT_CASE(test_sort),
    {}
    };
    static struct kunit_suite sort_test_suite = {
    .name = "lib_sort",
    .test_cases = sort_test_cases,
    };
    kunit_test_suites(&sort_test_suite);
    MODULE_DESCRIPTION("sort() KUnit test suite");
    MODULE_LICENSE("GPL");
