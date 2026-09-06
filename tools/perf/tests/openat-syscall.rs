//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/openat-syscall.c
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

    static int test__openat_syscall_event(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    let mut err: c_int = TEST_FAIL, fd;
    struct evsel *evsel;
    let mut nr_openat_calls: c_uint = 111, i;
    struct perf_thread_map *threads = thread_map__new_by_tid(getpid());
    char sbuf[STRERR_BUFSIZE];
    char errbuf[BUFSIZ];
    if (threads == core::ptr::null_mut()) {
    pr_debug("thread_map__new\n");
    return TEST_FAIL;
    }
    evsel = evsel__newtp("syscalls", "sys_enter_openat");
    if (IS_ERR(evsel)) {
    tracing_path__strerror_open_tp(errno, errbuf, sizeof(errbuf), "syscalls", "sys_enter_openat");
    pr_debug("%s\n", errbuf);
    err = TEST_SKIP;
    goto out_thread_map_delete;
    }
    if (evsel__open_per_thread(evsel, threads) < 0) {
    pr_debug("failed to open counter: %s, "
    "tweak /proc/sys/kernel/perf_event_paranoid?\n",
    str_error_r(errno, sbuf, sizeof(sbuf)));
    err = TEST_SKIP;
    goto out_evsel_put;
    }
    for (i = 0; i < nr_openat_calls; ++i) {
    fd = openat(0, "/etc/passwd", O_RDONLY);
    close(fd);
    }
    if (evsel__read_on_cpu(evsel, 0, 0) < 0) {
    pr_debug("evsel__read_on_cpu\n");
    goto out_close_fd;
    }
    if (perf_counts(evsel.counts, 0, 0).val != nr_openat_calls) {
    pr_debug("evsel__read_on_cpu: expected to intercept %d calls, got %" PRIu64 "\n",
    nr_openat_calls, perf_counts(evsel.counts, 0, 0).val);
    goto out_close_fd;
    }
    err = TEST_OK;
    out_close_fd:
    perf_evsel__close_fd(&evsel.core);
    out_evsel_put:
    evsel__put(evsel);
    out_thread_map_delete:
    perf_thread_map__put(threads);
    return err;
    }
    static struct test_case tests__openat_syscall_event[] = {
    TEST_CASE_REASON("Detect openat syscall event",
    openat_syscall_event,
    "permissions"),
    {	.name = core::ptr::null_mut(), }
    };
    struct test_suite suite__openat_syscall_event = {
    .desc = "Detect openat syscall event",
    .test_cases = tests__openat_syscall_event,
    };
