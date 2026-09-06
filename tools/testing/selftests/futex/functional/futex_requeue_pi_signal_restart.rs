//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/futex/functional/futex_requeue_pi_signal_restart.c
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
// Copyright © International Business Machines  Corp., 2006-2008
//
// DESCRIPTION
// This test exercises the futex_wait_requeue_pi() signal handling both
// before and after the requeue. The first should be restarted by the
// kernel. The latter should return EWOULDBLOCK to the waiter.
//
// AUTHORS
// Darren Hart <dvhart@linux.intel.com>
//
// HISTORY
// 2008-May-5: Initial version by Darren Hart <dvhart@linux.intel.com>
//

pub const DELAY_US: c_int = 100;
    let mut f1: futex_t = FUTEX_INITIALIZER;
    let mut f2: futex_t = FUTEX_INITIALIZER;
    let mut requeued: core::sync::atomic::AtomicI32 = ATOMIC_INITIALIZER;
    int waiter_ret;
    int create_rt_thread(struct __test_metadata *_metadata, pthread_t *pth, void*(*func)(void *),
    void *arg, int policy, int prio)
    {
    struct sched_param schedp;
    pthread_attr_t attr;
    int ret;
    pthread_attr_init(&attr);
    memset(&schedp, 0, sizeof(schedp));
    ret = pthread_attr_setinheritsched(&attr, PTHREAD_EXPLICIT_SCHED);
    ASSERT_EQ(ret, 0)
    TH_LOG("pthread_attr_setinheritsched failed");
    ret = pthread_attr_setschedpolicy(&attr, policy);
    ASSERT_EQ(ret, 0)
    TH_LOG("pthread_attr_setschedpolicy failed");
    schedp.sched_priority = prio;
    ret = pthread_attr_setschedparam(&attr, &schedp);
    ASSERT_EQ(ret, 0)
    TH_LOG("pthread_attr_setschedparam failed");
    ret = pthread_create(pth, &attr, func, arg);
    ASSERT_EQ(ret, 0)
    TH_LOG("pthread_create failed");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn handle_signal(signo: c_int) {
    void handle_signal(int signo)
    {
    printf("INFO: signal received %s requeue\n", requeued.val ? "after" : "prior to");
    }
    void *waiterfn(void *arg)
    {
    struct __test_metadata *_metadata = (struct __test_metadata *)arg;
    unsigned int old_val;
    int res;
    TH_LOG("Waiter running");
    TH_LOG("Calling FUTEX_LOCK_PI on f2=%x @ %p", f2, &f2);
    old_val = f1;
    res = futex_wait_requeue_pi(&f1, old_val, &(f2), core::ptr::null_mut(),
    FUTEX_PRIVATE_FLAG);
    if (!requeued.val || errno != EWOULDBLOCK) {
    EXPECT_TRUE(0) {
    TH_LOG("unexpected return from futex_wait_requeue_pi: %d (%s)",
    res, strerror(errno));
    }
    TH_LOG("w2:futex: %x", f2);
    if (!res)
    futex_unlock_pi(&f2, FUTEX_PRIVATE_FLAG);
    }
    pthread_exit(core::ptr::null_mut());
    }
    TEST(futex_requeue_pi_signal_restart)
    {
    unsigned int old_val;
    struct sigaction sa;
    pthread_t waiter;
    int res;
    sa.sa_handler = handle_signal;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = 0;
    ASSERT_EQ(sigaction(SIGUSR1, &sa, core::ptr::null_mut()), 0)
    TH_LOG("sigaction failed");
    TH_LOG("m1:f2: %x", f2);
    TH_LOG("Creating waiter");
    create_rt_thread(_metadata, &waiter, waiterfn, _metadata, SCHED_FIFO, 1);
    TH_LOG("Calling FUTEX_LOCK_PI on f2=%x @ %p", f2, &f2);
    TH_LOG("m2:f2: %x", f2);
    futex_lock_pi(&f2, 0, 0, FUTEX_PRIVATE_FLAG);
    TH_LOG("m3:f2: %x", f2);
    while (1) {
//
// signal the waiter before requeue, waiter should automatically
// restart futex_wait_requeue_pi() in the kernel. Wait for the
// waiter to block on f1 again.
//
    TH_LOG("Issuing SIGUSR1 to waiter");
    pthread_kill(waiter, SIGUSR1);
    usleep(DELAY_US);
    TH_LOG("Requeueing waiter via FUTEX_CMP_REQUEUE_PI");
    old_val = f1;
    res = futex_cmp_requeue_pi(&f1, old_val, &(f2), 1, 0,
    FUTEX_PRIVATE_FLAG);
//
// If res is non-zero, we either requeued the waiter or hit an
// error, break out and handle it. If it is zero, then the
// signal may have hit before the waiter was blocked on f1.
// Try again.
//
    if (res > 0) {
    atomic_set(&requeued, 1);
    break;
    } else if (res < 0) {
    ASSERT_GE(res, 0)
    TH_LOG("FUTEX_CMP_REQUEUE_PI failed: %s", strerror(errno));
    }
    }
    TH_LOG("m4:f2: %x", f2);
//
// Signal the waiter after requeue, waiter should return from
// futex_wait_requeue_pi() with EWOULDBLOCK. Join the thread here so the
// futex_unlock_pi() can't happen before the signal wakeup is detected
// in the kernel.
//
    TH_LOG("Issuing SIGUSR1 to waiter");
    pthread_kill(waiter, SIGUSR1);
    TH_LOG("Waiting for waiter to return");
    pthread_join(waiter, core::ptr::null_mut());
    TH_LOG("Calling FUTEX_UNLOCK_PI on mutex=%x @ %p", f2, &f2);
    futex_unlock_pi(&f2, FUTEX_PRIVATE_FLAG);
    TH_LOG("m5:f2: %x", f2);
    }
    TEST_HARNESS_MAIN
