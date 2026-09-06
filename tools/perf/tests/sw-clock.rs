//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/sw-clock.c
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

pub const NR_LOOPS: c_int = 10000000;
//
// This test will open software clock events (cpu-clock, task-clock)
// then check their frequency -> period conversion has no artifact of
// setting period to 1 forcefully.
//
#[no_mangle]
unsafe extern "C" fn __test__sw_clock_freq(clock_id: enum perf_sw_ids) -> c_int {
    static int __test__sw_clock_freq(enum perf_sw_ids clock_id)
    {
    int i, err = -1;
    let mut __maybe_unused: volatile int tmp = 0;
    let mut total_periods: u64 = 0;
    let mut nr_samples: c_int = 0;
    char sbuf[STRERR_BUFSIZE];
    union perf_event *event;
    struct evsel *evsel;
    struct evlist *evlist;
    struct perf_event_attr attr = {
    .type = PERF_TYPE_SOFTWARE,
    .config = clock_id,
    .sample_type = PERF_SAMPLE_PERIOD,
    .exclude_kernel = 1,
    .disabled = 1,
    .freq = 1,
    };
    struct perf_cpu_map *cpus = core::ptr::null_mut();
    struct perf_thread_map *threads = core::ptr::null_mut();
    struct mmap *md;
    attr.sample_freq = 500;
    evlist = evlist__new();
    if (evlist == core::ptr::null_mut()) {
    pr_debug("evlist__new\n");
    return -1;
    }
    evsel = evsel__new(&attr);
    if (evsel == core::ptr::null_mut()) {
    pr_debug("evsel__new\n");
    goto out_put_evlist;
    }
    evlist__add(evlist, evsel);
    cpus = perf_cpu_map__new_any_cpu();
    threads = thread_map__new_by_tid(getpid());
    if (!cpus || !threads) {
    err = -ENOMEM;
    pr_debug("Not enough memory to create thread/cpu maps\n");
    goto out_put_evlist;
    }
    perf_evlist__set_maps(evlist__core(evlist), cpus, threads);
    if (evlist__open(evlist)) {
    const char *knob = "/proc/sys/kernel/perf_event_max_sample_rate";
    err = -errno;
    pr_debug("Couldn't open evlist: %s\nHint: check %s, using %" PRIu64 " in this test.\n",
    str_error_r(errno, sbuf, sizeof(sbuf)),
    knob, (u64)attr.sample_freq);
    goto out_put_evlist;
    }
    err = evlist__do_mmap(evlist, 128);
    if (err < 0) {
    pr_debug("failed to mmap event: %d (%s)\n", errno,
    str_error_r(errno, sbuf, sizeof(sbuf)));
    goto out_put_evlist;
    }
    evlist__enable(evlist);
// collect samples
    for (i = 0; i < NR_LOOPS; i++)
    tmp++;
    evlist__disable(evlist);
    md = &evlist__mmap(evlist)[0];
    if (perf_mmap__read_init(&md.core) < 0)
    goto out_init;
    while ((event = perf_mmap__read_event(&md.core)) != core::ptr::null_mut()) {
    struct perf_sample sample;
    perf_sample__init(&sample, /*all=*/false);
    if (event.header.type != PERF_RECORD_SAMPLE)
    goto next_event;
    err = evlist__parse_sample(evlist, event, &sample);
    if (err < 0) {
    pr_debug("Error during parse sample\n");
    perf_sample__exit(&sample);
    goto out_put_evlist;
    }
    total_periods += sample.period;
    nr_samples++;
    next_event:
    perf_mmap__consume(&md.core);
    perf_sample__exit(&sample);
    }
    perf_mmap__read_done(&md.core);
    out_init:
    if ((u64) nr_samples == total_periods) {
    pr_debug("All (%d) samples have period value of 1!\n",
    nr_samples);
    err = -1;
    }
    out_put_evlist:
    perf_cpu_map__put(cpus);
    perf_thread_map__put(threads);
    evlist__put(evlist);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test__sw_clock_freq(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__sw_clock_freq(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    int ret;
    ret = __test__sw_clock_freq(PERF_COUNT_SW_CPU_CLOCK);
    if (!ret)
    ret = __test__sw_clock_freq(PERF_COUNT_SW_TASK_CLOCK);
    return ret;
    }
    DEFINE_SUITE("Software clock events period values", sw_clock_freq);
