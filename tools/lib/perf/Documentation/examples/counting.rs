//! Automatically rewritten from C to Rust
//! Source: tools/lib/perf/Documentation/examples/counting.c
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


    static int libperf_print(enum libperf_print_level level,
    const char *fmt, va_list ap)
    {
    return vfprintf(stderr, fmt, ap);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut count: c_int = 100000, err = 0;
    struct perf_evlist *evlist;
    struct perf_evsel *evsel;
    struct perf_thread_map *threads;
    struct perf_counts_values counts;
    struct perf_event_attr attr1 = {
    .type        = PERF_TYPE_SOFTWARE,
    .config      = PERF_COUNT_SW_CPU_CLOCK,
    .read_format = PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING,
    .disabled    = 1,
    };
    struct perf_event_attr attr2 = {
    .type        = PERF_TYPE_SOFTWARE,
    .config      = PERF_COUNT_SW_TASK_CLOCK,
    .read_format = PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING,
    .disabled    = 1,
    };
    libperf_init(libperf_print);
    threads = perf_thread_map__new_dummy();
    if (!threads) {
    fprintf(stderr, "failed to create threads\n");
    return -1;
    }
    perf_thread_map__set_pid(threads, 0, 0);
    evlist = perf_evlist__new();
    if (!evlist) {
    fprintf(stderr, "failed to create evlist\n");
    goto out_threads;
    }
    evsel = perf_evsel__new(&attr1);
    if (!evsel) {
    fprintf(stderr, "failed to create evsel1\n");
    goto out_evlist;
    }
    perf_evlist__add(evlist, evsel);
    evsel = perf_evsel__new(&attr2);
    if (!evsel) {
    fprintf(stderr, "failed to create evsel2\n");
    goto out_evlist;
    }
    perf_evlist__add(evlist, evsel);
    perf_evlist__set_maps(evlist, core::ptr::null_mut(), threads);
    err = perf_evlist__open(evlist);
    if (err) {
    fprintf(stderr, "failed to open evsel\n");
    goto out_evlist;
    }
    perf_evlist__enable(evlist);
    while (count--);
    perf_evlist__disable(evlist);
    perf_evlist__for_each_evsel(evlist, evsel) {
    perf_evsel__read(evsel, 0, 0, &counts);
    fprintf(stdout, "count %llu, enabled %llu, run %llu\n",
    counts.val, counts.ena, counts.run);
    }
    perf_evlist__close(evlist);
    out_evlist:
    perf_evlist__delete(evlist);
    out_threads:
    perf_thread_map__put(threads);
    return err;
    }
