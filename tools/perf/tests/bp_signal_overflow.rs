//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/bp_signal_overflow.c
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
// Originally done by Vince Weaver <vincent.weaver@maine.edu> for
// perf_event_tests (git://github.com/deater/perf_event_tests)
//
// Powerpc needs __SANE_USERSPACE_TYPES__ before <linux/types.h> to select
// 'int-ll64.h' and avoid compile warnings when printing __u64 with %llu.
//
// Macro flag: #define __SANE_USERSPACE_TYPES__

    static int overflows;
#[no_mangle]
unsafe extern "C" fn test_function() -> noinline int {
    static noinline int test_function(void)
    {
    return time(core::ptr::null_mut());
    }
    static void sig_handler(int signum __maybe_unused,
    siginfo_t *oh __maybe_unused,
    void *uc __maybe_unused)
    {
    overflows++;
    }
#[no_mangle]
unsafe extern "C" fn bp_count(fd: c_int) -> c_longlong {
    static long long bp_count(int fd)
    {
    long long count;
    int ret;
    ret = read(fd, &count, sizeof(long long));
    if (ret != sizeof(long long)) {
    pr_debug("failed to read: %d\n", ret);
    return TEST_FAIL;
    }
    return count;
    }
pub const EXECUTIONS: c_int = 10000;
pub const THRESHOLD: c_int = 100;
#[no_mangle]
unsafe extern "C" fn test__bp_signal_overflow(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__bp_signal_overflow(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    struct perf_event_attr pe;
    struct sigaction sa;
    long long count;
    int fd, i, fails = 0;
    if (!BP_SIGNAL_IS_SUPPORTED) {
    pr_debug("Test not supported on this architecture");
    return TEST_SKIP;
    }
// setup SIGIO signal handler
    memset(&sa, 0, sizeof(struct sigaction));
    sa.sa_sigaction = (void *) sig_handler;
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGIO, &sa, core::ptr::null_mut()) < 0) {
    pr_debug("failed setting up signal handler\n");
    return TEST_FAIL;
    }
    memset(&pe, 0, sizeof(struct perf_event_attr));
    pe.type = PERF_TYPE_BREAKPOINT;
    pe.size = sizeof(struct perf_event_attr);
    pe.config = 0;
    pe.bp_type = HW_BREAKPOINT_X;
    pe.bp_addr = (unsigned long) test_function;
    pe.bp_len = default_breakpoint_len();
    pe.sample_period = THRESHOLD;
    pe.sample_type = PERF_SAMPLE_IP;
    pe.wakeup_events = 1;
    pe.disabled = 1;
    pe.exclude_kernel = 1;
    pe.exclude_hv = 1;
    fd = sys_perf_event_open(&pe, 0, -1, -1,
    perf_event_open_cloexec_flag());
    if (fd < 0) {
    pr_debug("failed opening event %llx\n", pe.config);
    return TEST_FAIL;
    }
    fcntl(fd, F_SETFL, O_RDWR|O_NONBLOCK|O_ASYNC);
    fcntl(fd, F_SETSIG, SIGIO);
    fcntl(fd, F_SETOWN, getpid());
    ioctl(fd, PERF_EVENT_IOC_RESET, 0);
    ioctl(fd, PERF_EVENT_IOC_ENABLE, 0);
    for (i = 0; i < EXECUTIONS; i++)
    test_function();
    ioctl(fd, PERF_EVENT_IOC_DISABLE, 0);
    count = bp_count(fd);
    close(fd);
    pr_debug("count %lld, overflow %d\n",
    count, overflows);
    if (count != EXECUTIONS) {
    pr_debug("\tWrong number of executions %lld != %d\n",
    count, EXECUTIONS);
    fails++;
    }
    if (overflows != EXECUTIONS / THRESHOLD) {
    pr_debug("\tWrong number of overflows %d != %d\n",
    overflows, EXECUTIONS / THRESHOLD);
    fails++;
    }
    return fails ? TEST_FAIL : TEST_OK;
    }
    DEFINE_SUITE("Breakpoint overflow sampling", bp_signal_overflow);
