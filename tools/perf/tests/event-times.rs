//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/event-times.c
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

#[no_mangle]
unsafe extern "C" fn attach__enable_on_exec(evlist: *mut evlist) -> c_int {
    static int attach__enable_on_exec(struct evlist *evlist)
    {
    struct evsel *evsel = evlist__last(evlist);
    let mut target: target = {};
    const char *argv[] = { "true", core::ptr::null_mut(), };
    char sbuf[STRERR_BUFSIZE];
    int err;
    pr_debug("attaching to spawned child, enable on exec\n");
    err = evlist__create_maps(evlist, &target);
    if (err < 0) {
    pr_debug("Not enough memory to create thread/cpu maps\n");
    return err;
    }
    err = evlist__prepare_workload(evlist, &target, argv, false, core::ptr::null_mut());
    if (err < 0) {
    pr_debug("Couldn't run the workload!\n");
    return err;
    }
    evsel.core.attr.enable_on_exec = 1;
    err = evlist__open(evlist);
    if (err < 0) {
    pr_debug("perf_evlist__open: %s\n",
    str_error_r(errno, sbuf, sizeof(sbuf)));
    return err;
    }
    return evlist__start_workload(evlist) == 1 ? TEST_OK : TEST_FAIL;
    }
#[no_mangle]
unsafe extern "C" fn detach__enable_on_exec(evlist: *mut evlist) -> c_int {
    static int detach__enable_on_exec(struct evlist *evlist)
    {
    waitpid(evlist__workload_pid(evlist), core::ptr::null_mut(), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn attach__current_disabled(evlist: *mut evlist) -> c_int {
    static int attach__current_disabled(struct evlist *evlist)
    {
    struct evsel *evsel = evlist__last(evlist);
    struct perf_thread_map *threads;
    int err;
    pr_debug("attaching to current thread as disabled\n");
    threads = thread_map__new_by_tid(getpid());
    if (threads == core::ptr::null_mut()) {
    pr_debug("thread_map__new\n");
    return -1;
    }
    evsel.core.attr.disabled = 1;
    err = evsel__open_per_thread(evsel, threads);
    if (err) {
    pr_debug("Failed to open event cpu-clock:u\n");
    return err;
    }
    perf_thread_map__put(threads);
    return evsel__enable(evsel) == 0 ? TEST_OK : TEST_FAIL;
    }
#[no_mangle]
unsafe extern "C" fn attach__current_enabled(evlist: *mut evlist) -> c_int {
    static int attach__current_enabled(struct evlist *evlist)
    {
    struct evsel *evsel = evlist__last(evlist);
    struct perf_thread_map *threads;
    int err;
    pr_debug("attaching to current thread as enabled\n");
    threads = thread_map__new_by_tid(getpid());
    if (threads == core::ptr::null_mut()) {
    pr_debug("failed to call thread_map__new\n");
    return -1;
    }
    err = evsel__open_per_thread(evsel, threads);
    perf_thread_map__put(threads);
    let mut err: return = = 0 ? TEST_OK : TEST_FAIL;
    }
#[no_mangle]
unsafe extern "C" fn detach__disable(evlist: *mut evlist) -> c_int {
    static int detach__disable(struct evlist *evlist)
    {
    struct evsel *evsel = evlist__last(evlist);
    return evsel__enable(evsel);
    }
#[no_mangle]
unsafe extern "C" fn attach__cpu_disabled(evlist: *mut evlist) -> c_int {
    static int attach__cpu_disabled(struct evlist *evlist)
    {
    struct evsel *evsel = evlist__last(evlist);
    struct perf_cpu_map *cpus;
    int err;
    pr_debug("attaching to CPU 0 as enabled\n");
    cpus = perf_cpu_map__new("0");
    if (cpus == core::ptr::null_mut()) {
    pr_debug("failed to call perf_cpu_map__new\n");
    return -1;
    }
    evsel.core.attr.disabled = 1;
    err = evsel__open_per_cpu(evsel, cpus, -1);
    perf_cpu_map__put(cpus);
    if (err) {
    if (err == -EACCES)
    return TEST_SKIP;
    pr_debug("Failed to open event cpu-clock:u\n");
    return err;
    }
    return evsel__enable(evsel);
    }
#[no_mangle]
unsafe extern "C" fn attach__cpu_enabled(evlist: *mut evlist) -> c_int {
    static int attach__cpu_enabled(struct evlist *evlist)
    {
    struct evsel *evsel = evlist__last(evlist);
    struct perf_cpu_map *cpus;
    int err;
    pr_debug("attaching to CPU 0 as enabled\n");
    cpus = perf_cpu_map__new("0");
    if (cpus == core::ptr::null_mut()) {
    pr_debug("failed to call perf_cpu_map__new\n");
    return -1;
    }
    err = evsel__open_per_cpu(evsel, cpus, -1);
    perf_cpu_map__put(cpus);
    if (err == -EACCES)
    return TEST_SKIP;
    return err ? TEST_FAIL : TEST_OK;
    }
    static int test_times(int (attach)(struct evlist *),
    int (detach)(struct evlist *))
    {
    struct perf_counts_values count;
    struct evlist *evlist = core::ptr::null_mut();
    struct evsel *evsel;
    let mut err: c_int = -1, i;
    evlist = evlist__new();
    if (!evlist) {
    pr_debug("failed to create event list\n");
    goto out_err;
    }
    err = parse_event(evlist, "cpu-clock:u");
    if (err) {
    pr_debug("failed to parse event cpu-clock:u\n");
    goto out_err;
    }
    evsel = evlist__last(evlist);
    evsel.core.attr.read_format |=
    PERF_FORMAT_TOTAL_TIME_ENABLED |
    PERF_FORMAT_TOTAL_TIME_RUNNING;
    err = attach(evlist);
    if (err == TEST_SKIP) {
    pr_debug("  SKIP  : not enough rights\n");
    evlist__put(evlist);
    return err;
    }
    TEST_ASSERT_VAL("failed to attach", !err);
    for (i = 0; i < 100000000; i++) { }
    TEST_ASSERT_VAL("failed to detach", !detach(evlist));
    perf_evsel__read(&evsel.core, 0, 0, &count);
    err = !(count.ena == count.run);
    pr_debug("  %s: ena %" PRIu64", run %" PRIu64"\n",
    !err ? "OK    " : "FAILED",
    count.ena, count.run);
    out_err:
    evlist__put(evlist);
    return !err ? TEST_OK : TEST_FAIL;
    }
//
// This test creates software event 'cpu-clock'
// attaches it in several ways (explained below)
// and checks that enabled and running times
// match.
//
#[no_mangle]
unsafe extern "C" fn test__event_times(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__event_times(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    int err, ret = 0;

    err = test_times(attach, detach);	\
    if (err && (ret == TEST_OK || ret == TEST_SKIP))	\
    ret = err;
// attach on newly spawned process after exec
    _T(attach__enable_on_exec,   detach__enable_on_exec)
// attach on current process as enabled
    _T(attach__current_enabled,  detach__disable)
// attach on current process as disabled
    _T(attach__current_disabled, detach__disable)
// attach on cpu as disabled
    _T(attach__cpu_disabled,     detach__disable)
// attach on cpu as enabled
    _T(attach__cpu_enabled,      detach__disable)

    return ret;
    }
    DEFINE_SUITE("Event times", event_times);
