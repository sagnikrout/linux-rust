//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/openat-syscall-tp-fields.c
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

pub const O_DIRECTORY: c_int = 00200000;

    static int test__syscall_openat_tp_fields(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    struct record_opts opts = {
    .target = {
    .uses_mmap = true,
    },
    .no_buffering = true,
    .freq	      = 1,
    .mmap_pages   = 256,
    .raw_samples  = true,
    };
    const char *filename = "/etc/passwd";
    let mut flags: c_int = O_RDONLY | O_DIRECTORY;
    struct evlist *evlist = evlist__new();
    struct evsel *evsel;
    let mut ret: c_int = TEST_FAIL, err, i, nr_events = 0, nr_polls = 0;
    char sbuf[STRERR_BUFSIZE];
    if (evlist == core::ptr::null_mut()) {
    pr_debug("%s: evlist__new\n", __func__);
    goto out;
    }
    evsel = evsel__newtp("syscalls", "sys_enter_openat");
    if (IS_ERR(evsel)) {
    pr_debug("%s: evsel__newtp\n", __func__);
    ret = PTR_ERR(evsel) == -EACCES ? TEST_SKIP : TEST_FAIL;
    goto out_put_evlist;
    }
    evlist__add(evlist, evsel);
    err = evlist__create_maps(evlist, &opts.target);
    if (err < 0) {
    pr_debug("%s: evlist__create_maps\n", __func__);
    goto out_put_evlist;
    }
    evsel__config(evsel, &opts, core::ptr::null_mut());
    perf_thread_map__set_pid(evlist__core(evlist).threads, 0, getpid());
    err = evlist__open(evlist);
    if (err < 0) {
    pr_debug("perf_evlist__open: %s\n",
    str_error_r(errno, sbuf, sizeof(sbuf)));
    goto out_put_evlist;
    }
    err = evlist__do_mmap(evlist, UINT_MAX);
    if (err < 0) {
    pr_debug("evlist__mmap: %s\n",
    str_error_r(errno, sbuf, sizeof(sbuf)));
    goto out_put_evlist;
    }
    evlist__enable(evlist);
//
// Generate the event:
//
    openat(AT_FDCWD, filename, flags);
    while (1) {
    let mut before: c_int = nr_events;
    for (i = 0; i < evlist__core(evlist).nr_mmaps; i++) {
    union perf_event *event;
    struct mmap *md;
    md = &evlist__mmap(evlist)[i];
    if (perf_mmap__read_init(&md.core) < 0)
    continue;
    while ((event = perf_mmap__read_event(&md.core)) != core::ptr::null_mut()) {
    let mut type: u32 = event.header.type;
    int tp_flags;
    struct perf_sample sample;
    ++nr_events;
    if (type != PERF_RECORD_SAMPLE) {
    perf_mmap__consume(&md.core);
    continue;
    }
    perf_sample__init(&sample, /*all=*/false);
    err = evsel__parse_sample(evsel, event, &sample);
    if (err) {
    pr_debug("Can't parse sample, err = %d\n", err);
    perf_sample__exit(&sample);
    goto out_put_evlist;
    }
    tp_flags = perf_sample__intval(&sample, "flags");
    perf_sample__exit(&sample);
// C library wrapper may set additional flags,
    access mode must be unchanged */
    if ((tp_flags & O_ACCMODE) != (flags & O_ACCMODE) ||
    (tp_flags & flags) != flags) {
    pr_debug("%s: Expected flags=%#x, got %#x\n",
    __func__, flags, tp_flags);
    goto out_put_evlist;
    }
    goto out_ok;
    }
    perf_mmap__read_done(&md.core);
    }
    if (nr_events == before)
    evlist__poll(evlist, 10);
    if (++nr_polls > 5) {
    pr_debug("%s: no events!\n", __func__);
    goto out_put_evlist;
    }
    }
    out_ok:
    ret = TEST_OK;
    out_put_evlist:
    evlist__put(evlist);
    out:
    return ret;
    }
    static struct test_case tests__syscall_openat_tp_fields[] = {
    TEST_CASE_REASON("syscalls:sys_enter_openat event fields",
    syscall_openat_tp_fields,
    "permissions"),
    {	.name = core::ptr::null_mut(), }
    };
    struct test_suite suite__syscall_openat_tp_fields = {
    .desc = "syscalls:sys_enter_openat event fields",
    .test_cases = tests__syscall_openat_tp_fields,
    };
