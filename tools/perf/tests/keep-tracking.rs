//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/keep-tracking.c
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

    while ((x) < 0) {			\
    pr_debug(#x " failed!\n");	\
    goto out_err;			\
    }					\
    }

    while ((x) == core::ptr::null_mut()) {			\
    pr_debug(#x " failed!\n");	\
    goto out_err;			\
    }					\
    }
#[no_mangle]
unsafe extern "C" fn find_comm(evlist: *mut evlist, comm: *const c_char) -> c_int {
    static int find_comm(struct evlist *evlist, const char *comm)
    {
    union perf_event *event;
    struct mmap *md;
    int i, found;
    found = 0;
    for (i = 0; i < evlist__core(evlist).nr_mmaps; i++) {
    md = &evlist__mmap(evlist)[i];
    if (perf_mmap__read_init(&md.core) < 0)
    continue;
    while ((event = perf_mmap__read_event(&md.core)) != core::ptr::null_mut()) {
    if (event.header.type == PERF_RECORD_COMM &&
    (pid_t)event.comm.pid == getpid() &&
    (pid_t)event.comm.tid == getpid() &&
    strcmp(event.comm.comm, comm) == 0)
    found += 1;
    perf_mmap__consume(&md.core);
    }
    perf_mmap__read_done(&md.core);
    }
    return found;
    }
//
// test__keep_tracking - test using a dummy software event to keep tracking.
//
// This function implements a test that checks that tracking events continue
// when an event is disabled but a dummy software event is not disabled.  If the
// test passes %0 is returned, otherwise %-1 is returned.
//
#[no_mangle]
unsafe extern "C" fn test__keep_tracking(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__keep_tracking(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    struct record_opts opts = {
    .mmap_pages	     = UINT_MAX,
    .user_freq	     = UINT_MAX,
    .user_interval	     = ULLONG_MAX,
    .target		     = {
    .uses_mmap   = true,
    },
    };
    struct perf_thread_map *threads = core::ptr::null_mut();
    struct perf_cpu_map *cpus = core::ptr::null_mut();
    struct evlist *evlist = core::ptr::null_mut();
    struct evsel *evsel = core::ptr::null_mut();
    int found, err = -1;
    const char *comm;
    threads = thread_map__new_by_tid(getpid());
    CHECK_NOT_NULL__(threads);
    cpus = perf_cpu_map__new_online_cpus();
    CHECK_NOT_NULL__(cpus);
    evlist = evlist__new();
    CHECK_NOT_NULL__(evlist);
    perf_evlist__set_maps(evlist__core(evlist), cpus, threads);
    CHECK__(parse_event(evlist, "dummy:u"));
    CHECK__(parse_event(evlist, "cpu-cycles:u"));
    evlist__config(evlist, &opts, core::ptr::null_mut());
    evsel = evlist__first(evlist);
    evsel.core.attr.comm = 1;
    evsel.core.attr.disabled = 1;
    evsel.core.attr.enable_on_exec = 0;
    if (evlist__open(evlist) < 0) {
    pr_debug("Unable to open dummy and cycles event\n");
    err = TEST_SKIP;
    goto out_err;
    }
    CHECK__(evlist__do_mmap(evlist, UINT_MAX));
//
// First, test that a 'comm' event can be found when the event is
// enabled.
//
    evlist__enable(evlist);
    comm = "Test COMM 1";
    CHECK__(prctl(PR_SET_NAME, (unsigned long)comm, 0, 0, 0));
    evlist__disable(evlist);
    found = find_comm(evlist, comm);
    if (found != 1) {
    pr_debug("First time, failed to find tracking event.\n");
    goto out_err;
    }
//
// Secondly, test that a 'comm' event can be found when the event is
// disabled with the dummy event still enabled.
//
    evlist__enable(evlist);
    evsel = evlist__last(evlist);
    CHECK__(evsel__disable(evsel));
    comm = "Test COMM 2";
    CHECK__(prctl(PR_SET_NAME, (unsigned long)comm, 0, 0, 0));
    evlist__disable(evlist);
    found = find_comm(evlist, comm);
    if (found != 1) {
    pr_debug("Second time, failed to find tracking event.\n");
    goto out_err;
    }
    err = 0;
    out_err:
    if (evlist) {
    evlist__disable(evlist);
    evlist__put(evlist);
    }
    perf_cpu_map__put(cpus);
    perf_thread_map__put(threads);
    return err;
    }
    DEFINE_SUITE("Use a dummy software event to keep tracking", keep_tracking);
