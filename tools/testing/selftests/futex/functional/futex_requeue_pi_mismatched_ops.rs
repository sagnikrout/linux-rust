//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/futex/functional/futex_requeue_pi_mismatched_ops.c
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
// Copyright © International Business Machines  Corp., 2009
//
// DESCRIPTION
// 1. Block a thread using FUTEX_WAIT
// 2. Attempt to use FUTEX_CMP_REQUEUE_PI on the futex from 1.
// 3. The kernel must detect the mismatch and return -EINVAL.
//
// AUTHOR
// Darren Hart <dvhart@linux.intel.com>
//
// HISTORY
// 2009-Nov-9: Initial version by Darren Hart <dvhart@linux.intel.com>
//

    let mut f1: futex_t = FUTEX_INITIALIZER;
    let mut f2: futex_t = FUTEX_INITIALIZER;
    int child_ret;
    void *blocking_child(void *arg)
    {
    struct __test_metadata *_metadata = (struct __test_metadata *)arg;
    child_ret = futex_wait(&f1, f1, core::ptr::null_mut(), FUTEX_PRIVATE_FLAG);
    if (child_ret < 0) {
    child_ret = -errno;
    ASSERT_EQ(child_ret, 0)
    TH_LOG("futex_wait failed: %s", strerror(errno));
    }
    return (void *)&child_ret;
    }
    TEST(requeue_pi_mismatched_ops)
    {
    pthread_t child;
    int ret;
    ASSERT_EQ(pthread_create(&child, core::ptr::null_mut(), blocking_child, _metadata), 0)
    TH_LOG("pthread_create failed");
// Allow the child to block in the kernel.
    sleep(1);
//
// The kernel should detect the waiter did not setup the
// q->requeue_pi_key and return -EINVAL. If it does not,
// it likely gave the lock to the child, which is now hung
// in the kernel.
//
    ret = futex_cmp_requeue_pi(&f1, f1, &f2, 1, 0, FUTEX_PRIVATE_FLAG);
    if (ret < 0) {
    if (errno == EINVAL) {
//
// The kernel correctly detected the mismatched
// requeue_pi target and aborted. Wake the child with
// FUTEX_WAKE.
//
    ret = futex_wake(&f1, 1, FUTEX_PRIVATE_FLAG);
    if (ret == 1) {
    ret = 0;
    } else if (ret < 0) {
    ASSERT_GE(ret, 0)
    TH_LOG("futex_wake failed: %s", strerror(errno));
    } else {
    ASSERT_TRUE(0)
    TH_LOG("futex_wake did not wake the child");
    }
    } else {
    ASSERT_TRUE(0)
    TH_LOG("futex_cmp_requeue_pi failed with unexpected errno: %s", strerror(errno));
    }
    } else if (ret > 0) {
    EXPECT_EQ(ret, 0)
    TH_LOG("futex_cmp_requeue_pi failed to detect the mismatch");
    } else {
    ASSERT_TRUE(0)
    TH_LOG("futex_cmp_requeue_pi found no waiters");
    }
    pthread_join(child, core::ptr::null_mut());
    EXPECT_EQ(ret, 0)
    TH_LOG("Test failed: ret=%d", ret);
    EXPECT_EQ(child_ret, 0)
    TH_LOG("Child failed: child_ret=%d", child_ret);
    }
    TEST_HARNESS_MAIN
