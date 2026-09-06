//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/futex/functional/futex_wait_wouldblock.c
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
// Test if FUTEX_WAIT op returns -EWOULDBLOCK if the futex value differs
// from the expected one.
//
// AUTHOR
// Gowrishankar <gowrishankar.m@in.ibm.com>
//
// HISTORY
// 2009-Nov-14: Initial version by Gowrishankar <gowrishankar.m@in.ibm.com>
//

pub const timeout_ns: c_int = 100000;
    TEST(futex_wait_wouldblock)
    {
    let mut to: timespec = {.tv_sec = 0, .tv_nsec = timeout_ns};
    let mut f1: futex_t = FUTEX_INITIALIZER;
    int res;
    TH_LOG("Calling futex_wait on f1: %u @ %p with val=%u", f1, &f1, f1+1);
    res = futex_wait(&f1, f1+1, &to, FUTEX_PRIVATE_FLAG);
    EXPECT_EQ(res, -1)
    TH_LOG("futex_wait returned unexpected result: %d", res);
    if (res == -1) {
    EXPECT_EQ(errno, EWOULDBLOCK)
    TH_LOG("futex_wait returned unexpected errno: %d", errno);
    }
    }
    TEST(futex_waitv_wouldblock)
    {
    let mut to: timespec = {.tv_sec = 0, .tv_nsec = timeout_ns};
    let mut f1: futex_t = FUTEX_INITIALIZER;
    struct futex_waitv waitv = {
    .uaddr		= (uintptr_t)&f1,
    .val		= f1 + 1,
    .flags		= FUTEX_32,
    .__reserved	= 0,
    };
    int res;
    if (!is_futex_waitv_supported())
    SKIP(return, "futex_waitv syscall not supported");
    ASSERT_EQ(clock_gettime(CLOCK_MONOTONIC, &to), 0)
    TH_LOG("clock_gettime failed");
    to.tv_nsec += timeout_ns;
    if (to.tv_nsec >= 1000000000) {
    to.tv_sec++;
    to.tv_nsec -= 1000000000;
    }
    TH_LOG("Calling futex_waitv on f1: %u @ %p with val=%u", f1, &f1, f1+1);
    res = futex_waitv(&waitv, 1, 0, &to, CLOCK_MONOTONIC);
    EXPECT_EQ(res, -1)
    TH_LOG("futex_waitv returned unexpected result: %d", res);
    if (res == -1) {
    EXPECT_EQ(errno, EWOULDBLOCK)
    TH_LOG("futex_waitv returned unexpected errno: %d", errno);
    }
    }
    TEST_HARNESS_MAIN
