//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/task-exit.c
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

    static int exited;
    static int nr_exit;
#[no_mangle]
unsafe extern "C" fn sig_handler(__maybe_unused: int sig) {
    static void sig_handler(int sig __maybe_unused)
    {
    exited = 1;
    }
//
// evlist__prepare_workload will send a SIGUSR1 if the fork fails, since
// we asked by setting its exec_error to this handler.
//
    static void workload_exec_failed_signal(int signo __maybe_unused,
    siginfo_t *info __maybe_unused,
    void *ucontext __maybe_unused)
    {
    exited	= 1;
    nr_exit = -1;
    }
//
// This test will start a workload that does nothing then it checks
// if the number of exit event reported by the kernel is 1 or not
// in order to check the kernel returns correct number of event.
//
#[no_mangle]
unsafe extern "C" fn test__task_exit(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__task_exit(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut err: c_int = -1;
    union perf_event *event;
    struct evsel *evsel;
    struct evlist *evlist;
    struct target target = {
    .uses_mmap	= true,
    };
    const char *argv[] = { "true", core::ptr::null_mut() };
    char sbuf[STRERR_BUFSIZE];
    struct perf_cpu_map *cpus;
    struct perf_thread_map *threads;
    struct mmap *md;
    let mut retry_count: c_int = 0;
    signal(SIGCHLD, sig_handler);
    evlist = evlist__new_dummy();
    if (evlist == core::ptr::null_mut()) {
    pr_debug("evlist__new_dummy\n");
    return -1;
    }
//
// Create maps of threads and cpus to monitor. In this case
// we start with all threads and cpus (-1, -1) but then in
// evlist__prepare_workload we'll fill in the only thread
// we're monitoring, the one forked there.
//
    cpus = perf_cpu_map__new_any_cpu();
    threads = thread_map__new_by_tid(-1);
    if (!cpus || !threads) {
    err = -ENOMEM;
    pr_debug("Not enough memory to create thread/cpu maps\n");
    goto out_put_evlist;
    }
    perf_evlist__set_maps(evlist__core(evlist), cpus, threads);
    err = evlist__prepare_workload(evlist, &target, argv, false, workload_exec_failed_signal);
    if (err < 0) {
    pr_debug("Couldn't run the workload!\n");
    goto out_put_evlist;
    }
    evsel = evlist__first(evlist);
    evsel.core.attr.task = 1;

    evsel.core.attr.sample_freq = 1000000;

    evsel.core.attr.sample_freq = 1;

    evsel.core.attr.inherit = 0;
    evsel.core.attr.watermark = 0;
    evsel.core.attr.wakeup_events = 1;
    evsel.core.attr.exclude_kernel = 1;
    err = evlist__open(evlist);
    if (err < 0) {
    pr_debug("Couldn't open the evlist: %s\n",
    str_error_r(-err, sbuf, sizeof(sbuf)));
    goto out_put_evlist;
    }
    if (evlist__do_mmap(evlist, 128) < 0) {
    pr_debug("failed to mmap events: %d (%s)\n", errno,
    str_error_r(errno, sbuf, sizeof(sbuf)));
    err = -1;
    goto out_put_evlist;
    }
    evlist__start_workload(evlist);
    retry:
    md = &evlist__mmap(evlist)[0];
    if (perf_mmap__read_init(&md.core) < 0)
    goto out_init;
    while ((event = perf_mmap__read_event(&md.core)) != core::ptr::null_mut()) {
    if (event.header.type == PERF_RECORD_EXIT)
    nr_exit++;
    perf_mmap__consume(&md.core);
    }
    perf_mmap__read_done(&md.core);
    out_init:
    if (!exited || !nr_exit) {
    evlist__poll(evlist, -1);
    if (retry_count++ > 1000) {
    pr_debug("Failed after retrying 1000 times\n");
    err = -1;
    goto out_put_evlist;
    }
    goto retry;
    }
    if (nr_exit != 1) {
    pr_debug("received %d EXIT records\n", nr_exit);
    err = -1;
    }
    out_put_evlist:
    perf_cpu_map__put(cpus);
    perf_thread_map__put(threads);
    evlist__put(evlist);
    return err;
    }
    struct test_case tests__task_exit[] = {
    TEST_CASE_EXCLUSIVE("Number of exit events of a simple workload", task_exit),
    {	.name = core::ptr::null_mut(), }
    };
    struct test_suite suite__task_exit = {
    .desc = "Number of exit events of a simple workload",
    .test_cases = tests__task_exit,
    };
