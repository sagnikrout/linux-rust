//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/futex/functional/futex_wait_uninitialized_heap.c
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
// Wait on uninitialized heap. It shold be zero and FUTEX_WAIT should
// return immediately. This test is intent to test zero page handling in
// futex.
//
// AUTHOR
// KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
//
// HISTORY
// 2010-Jan-6: Initial version by KOSAKI Motohiro <kosaki.motohiro@jp.fujitsu.com>
//

pub const WAIT_US: c_int = 5000000;
    let mut child_blocked: static int = 1;
    static bool child_ret;
    void *buf;
    void *wait_thread(void *arg)
    {
    struct __test_metadata *_metadata = (struct __test_metadata *)arg;
    int res;
    child_ret = true;
    res = futex_wait(buf, 1, core::ptr::null_mut(), 0);
    child_blocked = 0;
    if (res != 0 && errno != EWOULDBLOCK) {
    EXPECT_EQ(res, 0)
    TH_LOG("futex failure: %s", strerror(errno));
    child_ret = false;
    }
    pthread_exit(core::ptr::null_mut());
    }
    TEST(futex_wait_uninitialized_heap)
    {
    long page_size;
    pthread_t thr;
    int ret;
    page_size = sysconf(_SC_PAGESIZE);
    buf = mmap(core::ptr::null_mut(), page_size, PROT_READ|PROT_WRITE,
    MAP_PRIVATE|MAP_ANONYMOUS, 0, 0);
    ASSERT_NE(buf, MAP_FAILED)
    TH_LOG("mmap failed: %s", strerror(errno));
    ret = pthread_create(&thr, core::ptr::null_mut(), wait_thread, _metadata);
    ASSERT_EQ(ret, 0)
    TH_LOG("pthread_create failed");
    TH_LOG("waiting %dus for child to return", WAIT_US);
    usleep(WAIT_US);
    EXPECT_EQ(child_blocked, 0)
    TH_LOG("child blocked in kernel");
    EXPECT_TRUE(child_ret)
    TH_LOG("child error");
    pthread_join(thr, core::ptr::null_mut());
    munmap(buf, page_size);
    }
    TEST_HARNESS_MAIN
