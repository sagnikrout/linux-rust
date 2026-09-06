//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/futex/functional/futex_wait_private_mapped_file.c
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
// Copyright FUJITSU LIMITED 2010
// Copyright KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
//
// DESCRIPTION
// Internally, Futex has two handling mode, anon and file. The private file
// mapping is special. At first it behave as file, but after write anything
// it behave as anon. This test is intent to test such case.
//
// AUTHOR
// KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
//
// HISTORY
// 2010-Jan-6: Initial version by KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
//

pub const PAGE_SZ: c_int = 4096;
    char pad[PAGE_SZ] = {1};
    let mut val: futex_t = 1;
    char pad2[PAGE_SZ] = {1};
pub const WAKE_WAIT_US: c_int = 3000000;
    let mut wait_timeout: timespec = { .tv_sec = 5, .tv_nsec = 0};
    void *thr_futex_wait(void *arg)
    {
    struct __test_metadata *_metadata = (struct __test_metadata *)arg;
    int ret;
    TH_LOG("futex wait");
    ret = futex_wait(&val, 1, &wait_timeout, 0);
    if (ret && errno != EWOULDBLOCK && errno != ETIMEDOUT) {
    ASSERT_TRUE(0)
    TH_LOG("futex error: %s", strerror(errno));
    }
    if (ret && errno == ETIMEDOUT) {
    ASSERT_TRUE(0)
    TH_LOG("waiter timedout");
    }
    TH_LOG("futex_wait: ret = %d, errno = %d", ret, errno);
    return core::ptr::null_mut();
    }
    TEST(wait_private_mapped_file)
    {
    pthread_t thr;
    int res;
    res = pthread_create(&thr, core::ptr::null_mut(), thr_futex_wait, _metadata);
    ASSERT_EQ(res, 0)
    TH_LOG("pthread_create error");
    TH_LOG("wait a while");
    usleep(WAKE_WAIT_US);
    val = 2;
    res = futex_wake(&val, 1, 0);
    TH_LOG("futex_wake %d", res);
    EXPECT_EQ(res, 1)
    TH_LOG("FUTEX_WAKE didn't find the waiting thread");
    TH_LOG("join");
    pthread_join(thr, core::ptr::null_mut());
    }
    TEST_HARNESS_MAIN
