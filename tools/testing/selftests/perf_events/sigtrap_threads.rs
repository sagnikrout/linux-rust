//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/perf_events/sigtrap_threads.c
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
// Test for perf events with SIGTRAP across all threads.
//
// Copyright (C) 2021, Google LLC.
//
// Macro flag: #define _GNU_SOURCE
// We need the latest siginfo from the kernel repo.

pub const __have_siginfo_t: c_int = 1;
pub const __have_sigval_t: c_int = 1;
pub const __have_sigevent_t: c_int = 1;
// Macro flag: #define __siginfo_t_defined
// Macro flag: #define __sigval_t_defined
// Macro flag: #define __sigevent_t_defined
pub const _BITS_SIGINFO_CONSTS_H: c_int = 1;
pub const _BITS_SIGEVENT_CONSTS_H: c_int = 1;

pub const NUM_THREADS: c_int = 5;
// Data shared between test body, threads, and signal handler.
    static struct {
    int tids_want_signal;		/* Which threads still want a signal. */
    int signal_count;		/* Sanity check number of signals received. */
    volatile int iterate_on;	/* Variable to set breakpoint on. */
    siginfo_t first_siginfo;	/* First observed siginfo_t. */
    } ctx;
// Unique value to check si_perf_data is correctly set from perf_event_attr::sig_data.

    static struct perf_event_attr make_event_attr(bool enabled, volatile void *addr,
    unsigned long id)
    {
    struct perf_event_attr attr = {
    .type		= PERF_TYPE_BREAKPOINT,
    .size		= sizeof(attr),
    .sample_period	= 1,
    .disabled	= !enabled,
    .bp_addr	= (unsigned long)addr,
    .bp_type	= HW_BREAKPOINT_RW,
    .bp_len		= HW_BREAKPOINT_LEN_1,
    .inherit	= 1, /* Children inherit events ... */
    .inherit_thread = 1, /* ... but only cloned with CLONE_THREAD. */
    .remove_on_exec = 1, /* Required by sigtrap. */
    .sigtrap	= 1, /* Request synchronous SIGTRAP on event. */
    .sig_data	= TEST_SIG_DATA(addr, id),
    .exclude_kernel = 1, /* To allow */
    .exclude_hv     = 1, /* running as !root */
    };
    return attr;
    }
#[no_mangle]
unsafe extern "C" fn sigtrap_handler(signum: c_int, info: *mut siginfo_t, ucontext: *mut c_void) {
    static void sigtrap_handler(int signum, siginfo_t *info, void *ucontext)
    {
    if (info.si_code != TRAP_PERF) {
    fprintf(stderr, "%s: unexpected si_code %d\n", __func__, info.si_code);
    return;
    }
//
// The data in siginfo_t we're interested in should all be the same
// across threads.
//
    if (!__atomic_fetch_add(&ctx.signal_count, 1, __ATOMIC_RELAXED))
    ctx.first_siginfo = *info;
    __atomic_fetch_sub(&ctx.tids_want_signal, syscall(__NR_gettid), __ATOMIC_RELAXED);
    }
    static void *test_thread(void *arg)
    {
    pthread_barrier_t *barrier = (pthread_barrier_t *)arg;
    let mut tid: pid_t = syscall(__NR_gettid);
    int iter;
    int i;
    pthread_barrier_wait(barrier);
    __atomic_fetch_add(&ctx.tids_want_signal, tid, __ATOMIC_RELAXED);
    iter = ctx.iterate_on; /* read */
    if (iter >= 0) {
    for (i = 0; i < iter - 1; i++) {
    __atomic_fetch_add(&ctx.tids_want_signal, tid, __ATOMIC_RELAXED);
    ctx.iterate_on = iter; /* idempotent write */
    }
    } else {
    while (ctx.iterate_on);
    }
    return core::ptr::null_mut();
    }
    FIXTURE(sigtrap_threads)
    {
    struct sigaction oldact;
    pthread_t threads[NUM_THREADS];
    pthread_barrier_t barrier;
    int fd;
    };
    FIXTURE_SETUP(sigtrap_threads)
    {
    let mut attr: perf_event_attr = make_event_attr(false, &ctx.iterate_on, 0);
    let mut action: sigaction = {};
    int i;
    memset(&ctx, 0, sizeof(ctx));
// Initialize sigtrap handler.
    action.sa_flags = SA_SIGINFO | SA_NODEFER;
    action.sa_sigaction = sigtrap_handler;
    sigemptyset(&action.sa_mask);
    ASSERT_EQ(sigaction(SIGTRAP, &action, &self.oldact), 0);
// Initialize perf event.
    self.fd = syscall(__NR_perf_event_open, &attr, 0, -1, -1, PERF_FLAG_FD_CLOEXEC);
    ASSERT_NE(self.fd, -1);
// Spawn threads inheriting perf event.
    pthread_barrier_init(&self.barrier, core::ptr::null_mut(), NUM_THREADS + 1);
    for (i = 0; i < NUM_THREADS; i++)
    ASSERT_EQ(pthread_create(&self.threads[i], core::ptr::null_mut(), test_thread, &self.barrier), 0);
    }
    FIXTURE_TEARDOWN(sigtrap_threads)
    {
    pthread_barrier_destroy(&self.barrier);
    close(self.fd);
    sigaction(SIGTRAP, &self.oldact, core::ptr::null_mut());
    }
    static void run_test_threads(struct __test_metadata *_metadata,
    FIXTURE_DATA(sigtrap_threads) *self)
    {
    int i;
    pthread_barrier_wait(&self.barrier);
    for (i = 0; i < NUM_THREADS; i++)
    ASSERT_EQ(pthread_join(self.threads[i], core::ptr::null_mut()), 0);
    }
    TEST_F(sigtrap_threads, remain_disabled)
    {
    run_test_threads(_metadata, self);
    EXPECT_EQ(ctx.signal_count, 0);
    EXPECT_NE(ctx.tids_want_signal, 0);
    }
    TEST_F(sigtrap_threads, enable_event)
    {
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    run_test_threads(_metadata, self);
    EXPECT_EQ(ctx.signal_count, NUM_THREADS);
    EXPECT_EQ(ctx.tids_want_signal, 0);
    EXPECT_EQ(ctx.first_siginfo.si_addr, &ctx.iterate_on);
    EXPECT_EQ(ctx.first_siginfo.si_perf_type, PERF_TYPE_BREAKPOINT);
    EXPECT_EQ(ctx.first_siginfo.si_perf_data, TEST_SIG_DATA(&ctx.iterate_on, 0));
// Check enabled for parent.
    ctx.iterate_on = 0;
    EXPECT_EQ(ctx.signal_count, NUM_THREADS + 1);
    }
// Test that modification propagates to all inherited events.
    TEST_F(sigtrap_threads, modify_and_enable_event)
    {
    let mut new_attr: perf_event_attr = make_event_attr(true, &ctx.iterate_on, 42);
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_MODIFY_ATTRIBUTES, &new_attr), 0);
    run_test_threads(_metadata, self);
    EXPECT_EQ(ctx.signal_count, NUM_THREADS);
    EXPECT_EQ(ctx.tids_want_signal, 0);
    EXPECT_EQ(ctx.first_siginfo.si_addr, &ctx.iterate_on);
    EXPECT_EQ(ctx.first_siginfo.si_perf_type, PERF_TYPE_BREAKPOINT);
    EXPECT_EQ(ctx.first_siginfo.si_perf_data, TEST_SIG_DATA(&ctx.iterate_on, 42));
// Check enabled for parent.
    ctx.iterate_on = 0;
    EXPECT_EQ(ctx.signal_count, NUM_THREADS + 1);
    }
// Stress test event + signal handling.
    TEST_F(sigtrap_threads, signal_stress)
    {
    ctx.iterate_on = 3000;
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    run_test_threads(_metadata, self);
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_DISABLE, 0), 0);
    EXPECT_EQ(ctx.signal_count, NUM_THREADS * ctx.iterate_on);
    EXPECT_EQ(ctx.tids_want_signal, 0);
    EXPECT_EQ(ctx.first_siginfo.si_addr, &ctx.iterate_on);
    EXPECT_EQ(ctx.first_siginfo.si_perf_type, PERF_TYPE_BREAKPOINT);
    EXPECT_EQ(ctx.first_siginfo.si_perf_data, TEST_SIG_DATA(&ctx.iterate_on, 0));
    }
    TEST_F(sigtrap_threads, signal_stress_with_disable)
    {
    let mut target_count: c_int = NUM_THREADS * 3000;
    int i;
    ctx.iterate_on = -1;
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    pthread_barrier_wait(&self.barrier);
    while (__atomic_load_n(&ctx.signal_count, __ATOMIC_RELAXED) < target_count) {
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_DISABLE, 0), 0);
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    }
    ctx.iterate_on = 0;
    for (i = 0; i < NUM_THREADS; i++)
    ASSERT_EQ(pthread_join(self.threads[i], core::ptr::null_mut()), 0);
    EXPECT_EQ(ioctl(self.fd, PERF_EVENT_IOC_DISABLE, 0), 0);
    EXPECT_EQ(ctx.first_siginfo.si_addr, &ctx.iterate_on);
    EXPECT_EQ(ctx.first_siginfo.si_perf_type, PERF_TYPE_BREAKPOINT);
    EXPECT_EQ(ctx.first_siginfo.si_perf_data, TEST_SIG_DATA(&ctx.iterate_on, 0));
    }
    TEST_HARNESS_MAIN
