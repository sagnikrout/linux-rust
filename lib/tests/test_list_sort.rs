//! Automatically rewritten from C to Rust
//! Source: lib/tests/test_list_sort.c
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
// The pattern of set bits in the list length determines which cases
// are hit in list_sort().
//

pub const TEST_POISON1: c_uint = 0xDEADBEEF;
pub const TEST_POISON2: c_uint = 0xA324354C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_el {
    pub poison1: c_uint,
    pub list: list_head,
    pub poison2: c_uint,
    pub value: c_int,
    pub serial: c_uint,
}

#[no_mangle]
unsafe extern "C" fn check(test: *mut kunit, ela: *const debug_el, elb: *const debug_el) {
    static void check(struct kunit *test, const struct debug_el *ela, const struct debug_el *elb)
    {
    struct debug_el **elts = test.priv;
    KUNIT_EXPECT_LT_MSG(test, ela.serial, (unsigned int)TEST_LIST_LEN, "incorrect serial");
    KUNIT_EXPECT_LT_MSG(test, elb.serial, (unsigned int)TEST_LIST_LEN, "incorrect serial");
    KUNIT_EXPECT_PTR_EQ_MSG(test, elts[ela.serial], ela, "phantom element");
    KUNIT_EXPECT_PTR_EQ_MSG(test, elts[elb.serial], elb, "phantom element");
    KUNIT_EXPECT_EQ_MSG(test, ela.poison1, TEST_POISON1, "bad poison");
    KUNIT_EXPECT_EQ_MSG(test, ela.poison2, TEST_POISON2, "bad poison");
    KUNIT_EXPECT_EQ_MSG(test, elb.poison1, TEST_POISON1, "bad poison");
    KUNIT_EXPECT_EQ_MSG(test, elb.poison2, TEST_POISON2, "bad poison");
    }
// `priv` is the test pointer so check() can fail the test if the list is invalid.
#[no_mangle]
unsafe extern "C" fn cmp(priv: *mut c_void, a: *const list_head, b: *const list_head) -> c_int {
    static int cmp(void *priv, const struct list_head *a, const struct list_head *b)
    {
    const struct debug_el *ela, *elb;
    ela = container_of(a, struct debug_el, list);
    elb = container_of(b, struct debug_el, list);
    check(priv, ela, elb);
    return ela.value - elb.value;
    }
#[no_mangle]
unsafe extern "C" fn list_sort_test(test: *mut kunit) {
    static void list_sort_test(struct kunit *test)
    {
    int i, count = 1;
    struct debug_el *el, **elts;
    struct list_head *cur;
    LIST_HEAD(head);
    elts = kunit_kcalloc(test, TEST_LIST_LEN, sizeof(*elts), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, elts);
    test.priv = elts;
    for (i = 0; i < TEST_LIST_LEN; i++) {
    el = kunit_kmalloc(test, sizeof(*el), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, el);
// force some equivalencies
    el.value = get_random_u32_below(TEST_LIST_LEN / 3);
    el.serial = i;
    el.poison1 = TEST_POISON1;
    el.poison2 = TEST_POISON2;
    elts[i] = el;
    list_add_tail(&el.list, &head);
    }
    list_sort(test, &head, cmp);
    for (cur = head.next; cur.next != &head; cur = cur.next) {
    struct debug_el *el1;
    int cmp_result;
    KUNIT_ASSERT_PTR_EQ_MSG(test, cur.next.prev, cur,
    "list is corrupted");
    cmp_result = cmp(test, cur, cur.next);
    KUNIT_ASSERT_LE_MSG(test, cmp_result, 0, "list is not sorted");
    el = container_of(cur, struct debug_el, list);
    el1 = container_of(cur.next, struct debug_el, list);
    if (cmp_result == 0) {
    KUNIT_ASSERT_LE_MSG(test, el.serial, el1.serial,
    "order of equivalent elements not preserved");
    }
    check(test, el, el1);
    count++;
    }
    KUNIT_EXPECT_PTR_EQ_MSG(test, head.prev, cur, "list is corrupted");
    KUNIT_EXPECT_EQ_MSG(test, count, TEST_LIST_LEN,
    "list length changed after sorting!");
    }
    static struct kunit_case list_sort_cases[] = {
    KUNIT_CASE(list_sort_test),
    {}
    };
    static struct kunit_suite list_sort_suite = {
    .name = "list_sort",
    .test_cases = list_sort_cases,
    };
    kunit_test_suites(&list_sort_suite);
    MODULE_DESCRIPTION("list_sort() KUnit test suite");
    MODULE_LICENSE("GPL");
