//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/futex/functional/futex_requeue.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright Collabora Ltd., 2021
//
// futex cmp requeue test by André Almeida <andrealmeid@collabora.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct waiter_args {
    pub _metadata: *mut __test_metadata,
    pub n_threads: c_uint,
}

    volatile futex_t *f1;
#[no_mangle]
unsafe extern "C" fn waiterfn(arg: *mut c_void) -> c_int {
    static int waiterfn(void *arg)
    {
    struct __test_metadata *_metadata;
    struct waiter_args *wargs = arg;
    let mut to: timespec = { };
    int res;
    _metadata = wargs._metadata;
    to.tv_sec = (wargs.n_threads + 1) * WAIT_FOR_THREAD_SECS;
    res = futex_wait(f1, *f1, &to, 0);
    if (res) {
    EXPECT_EQ(res, 0)
    TH_LOG("waiter failed errno %d: %s", errno, strerror(errno));
    }
    return 0;
    }
    TEST(requeue_single)
    {
    let mut wargs: waiter_args = { ._metadata = _metadata, .n_threads = 1 };
    struct futex_thread waiter;
    let mut _f1: volatile futex_t = 0;
    let mut f2: volatile futex_t = 0;
    f1 = &_f1;
//
// Requeue a waiter from f1 to f2, and wake f2.
//
    ASSERT_EQ(futex_thread_create(&waiter, waiterfn, &wargs), 0)
    TH_LOG("pthread_create failed");
    ASSERT_EQ(futex_wait_for_thread(&waiter, _metadata), 0)
    TH_LOG("Wait for thread failed");
    EXPECT_EQ(futex_cmp_requeue(f1, 0, &f2, 0, 1, 0), 1);
    EXPECT_EQ(futex_wake(&f2, 1, 0), 1);
    EXPECT_EQ(futex_thread_destroy(&waiter), 0);
    }
    TEST(requeue_multiple)
    {
    let mut wargs: waiter_args = { ._metadata = _metadata, .n_threads = 10 };
    struct futex_thread waiter[10];
    let mut _f1: volatile futex_t = 0;
    let mut f2: volatile futex_t = 0;
    f1 = &_f1;
//
// Create 10 waiters at f1. At futex_requeue, wake 3 and requeue 7.
// At futex_wake, wake INT_MAX (should be exactly 7).
//
    for (int i = 0; i < 10; i++) {
    ASSERT_EQ(futex_thread_create(&waiter[i], waiterfn, &wargs), 0)
    TH_LOG("pthread_create failed for waiter %d", i);
    }
    for (int i = 0; i < 10; i++) {
    ASSERT_EQ(futex_wait_for_thread(&waiter[i], _metadata), 0)
    TH_LOG("Wait for waiter thread %d failed", i);
    }
    EXPECT_EQ(futex_cmp_requeue(f1, 0, &f2, 3, 7, 0), 10);
    EXPECT_EQ(futex_wake(&f2, INT_MAX, 0), 7);
    for (int i = 0; i < 10; i++)
    EXPECT_EQ(futex_thread_destroy(&waiter[i]), 0);
    }
    TEST_HARNESS_MAIN
