//! Automatically rewritten from C to Rust
//! Source: tools/lib/perf/tests/test-evsel.c
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

    static int libperf_print(enum libperf_print_level level,
    const char *fmt, va_list ap)
    {
    return vfprintf(stderr, fmt, ap);
    }
#[no_mangle]
unsafe extern "C" fn test_stat_cpu() -> c_int {
    static int test_stat_cpu(void)
    {
    struct perf_cpu_map *cpus;
    struct perf_evsel *evsel;
    struct perf_event_attr attr = {
    .type	= PERF_TYPE_SOFTWARE,
    .config	= PERF_COUNT_SW_CPU_CLOCK,
    };
    int err, idx;
    cpus = perf_cpu_map__new_online_cpus();
    __T("failed to create cpus", cpus);
    evsel = perf_evsel__new(&attr);
    __T("failed to create evsel", evsel);
    err = perf_evsel__open(evsel, cpus, core::ptr::null_mut());
    __T("failed to open evsel", err == 0);
    for (idx = 0; idx < perf_cpu_map__nr(cpus); idx++) {
    let mut counts: perf_counts_values = { .val = 0 };
    perf_evsel__read(evsel, idx, 0, &counts);
    __T("failed to read value for evsel", counts.val != 0);
    }
    perf_evsel__close(evsel);
    perf_evsel__delete(evsel);
    perf_cpu_map__put(cpus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_stat_thread() -> c_int {
    static int test_stat_thread(void)
    {
    let mut counts: perf_counts_values = { .val = 0 };
    struct perf_thread_map *threads;
    struct perf_evsel *evsel;
    struct perf_event_attr attr = {
    .type	= PERF_TYPE_SOFTWARE,
    .config	= PERF_COUNT_SW_TASK_CLOCK,
    };
    int err;
    threads = perf_thread_map__new_dummy();
    __T("failed to create threads", threads);
    perf_thread_map__set_pid(threads, 0, 0);
    evsel = perf_evsel__new(&attr);
    __T("failed to create evsel", evsel);
    err = perf_evsel__open(evsel, core::ptr::null_mut(), threads);
    __T("failed to open evsel", err == 0);
    perf_evsel__read(evsel, 0, 0, &counts);
    __T("failed to read value for evsel", counts.val != 0);
    perf_evsel__close(evsel);
    perf_evsel__delete(evsel);
    perf_thread_map__put(threads);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_stat_thread_enable() -> c_int {
    static int test_stat_thread_enable(void)
    {
    let mut counts: perf_counts_values = { .val = 0 };
    struct perf_thread_map *threads;
    struct perf_evsel *evsel;
    struct perf_event_attr attr = {
    .type	  = PERF_TYPE_SOFTWARE,
    .config	  = PERF_COUNT_SW_TASK_CLOCK,
    .disabled = 1,
    };
    int err;
    threads = perf_thread_map__new_dummy();
    __T("failed to create threads", threads);
    perf_thread_map__set_pid(threads, 0, 0);
    evsel = perf_evsel__new(&attr);
    __T("failed to create evsel", evsel);
    err = perf_evsel__open(evsel, core::ptr::null_mut(), threads);
    __T("failed to open evsel", err == 0);
    perf_evsel__read(evsel, 0, 0, &counts);
    __T("failed to read value for evsel", counts.val == 0);
    err = perf_evsel__enable(evsel);
    __T("failed to enable evsel", err == 0);
    perf_evsel__read(evsel, 0, 0, &counts);
    __T("failed to read value for evsel", counts.val != 0);
    err = perf_evsel__disable(evsel);
    __T("failed to enable evsel", err == 0);
    perf_evsel__close(evsel);
    perf_evsel__delete(evsel);
    perf_thread_map__put(threads);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_stat_user_read(event: c_int) -> c_int {
    static int test_stat_user_read(int event)
    {
    let mut counts: perf_counts_values = { .val = 0 };
    struct perf_thread_map *threads;
    struct perf_evsel *evsel;
    struct perf_event_mmap_page *pc;
    struct perf_event_attr attr = {
    .type	= PERF_TYPE_HARDWARE,
    .config	= event,

    .config1 = 0x2,		/* Request user access */

    };
    int err, i;
    threads = perf_thread_map__new_dummy();
    __T("failed to create threads", threads);
    perf_thread_map__set_pid(threads, 0, 0);
    evsel = perf_evsel__new(&attr);
    __T("failed to create evsel", evsel);
    err = perf_evsel__open(evsel, core::ptr::null_mut(), threads);
    __T("failed to open evsel", err == 0);
    err = perf_evsel__mmap(evsel, 0);
    __T("failed to mmap evsel", err == 0);
    pc = perf_evsel__mmap_base(evsel, 0, 0);
    __T("failed to get mmapped address", pc);

    __T("userspace counter access not supported", pc.cap_user_rdpmc);
    __T("userspace counter access not enabled", pc.index);
    __T("userspace counter width not set", pc.pmc_width >= 32);

    perf_evsel__read(evsel, 0, 0, &counts);
    __T("failed to read value for evsel", counts.val != 0);
    for (i = 0; i < 5; i++) {
    let mut count: volatile int = 0x10000 << i;
    __u64 start, end, last = 0;
    __T_VERBOSE("\tloop = %u, ", count);
    perf_evsel__read(evsel, 0, 0, &counts);
    start = counts.val;
    while (count--) ;
    perf_evsel__read(evsel, 0, 0, &counts);
    end = counts.val;
    __T("invalid counter data", (end - start) > last);
    last = end - start;
    __T_VERBOSE("count = %llu\n", end - start);
    }
    perf_evsel__munmap(evsel);
    perf_evsel__close(evsel);
    perf_evsel__delete(evsel);
    perf_thread_map__put(threads);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_stat_read_format_single(attr: *mut perf_event_attr, threads: *mut perf_thread_map) -> c_int {
    static int test_stat_read_format_single(struct perf_event_attr *attr, struct perf_thread_map *threads)
    {
    struct perf_evsel *evsel;
    struct perf_counts_values counts;
    let mut count: volatile int = 0x100000;
    int err;
    evsel = perf_evsel__new(attr);
    __T("failed to create evsel", evsel);
// skip old kernels that don't support the format
    err = perf_evsel__open(evsel, core::ptr::null_mut(), threads);
    if (err < 0)
    return 0;
    while (count--) ;
    memset(&counts, -1, sizeof(counts));
    perf_evsel__read(evsel, 0, 0, &counts);
    __T("failed to read value", counts.val);
    if (attr.read_format & PERF_FORMAT_TOTAL_TIME_ENABLED)
    __T("failed to read TOTAL_TIME_ENABLED", counts.ena);
    if (attr.read_format & PERF_FORMAT_TOTAL_TIME_RUNNING)
    __T("failed to read TOTAL_TIME_RUNNING", counts.run);
    if (attr.read_format & PERF_FORMAT_ID)
    __T("failed to read ID", counts.id);
    if (attr.read_format & PERF_FORMAT_LOST)
    __T("failed to read LOST", counts.lost == 0);
    perf_evsel__close(evsel);
    perf_evsel__delete(evsel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_stat_read_format_group(attr: *mut perf_event_attr, threads: *mut perf_thread_map) -> c_int {
    static int test_stat_read_format_group(struct perf_event_attr *attr, struct perf_thread_map *threads)
    {
    struct perf_evsel *leader, *member;
    struct perf_counts_values counts;
    let mut count: volatile int = 0x100000;
    int err;
    attr.read_format |= PERF_FORMAT_GROUP;
    leader = perf_evsel__new(attr);
    __T("failed to create leader", leader);
    attr.read_format &= ~PERF_FORMAT_GROUP;
    member = perf_evsel__new(attr);
    __T("failed to create member", member);
    member.leader = leader;
    leader.nr_members = 2;
// skip old kernels that don't support the format
    err = perf_evsel__open(leader, core::ptr::null_mut(), threads);
    if (err < 0)
    return 0;
    err = perf_evsel__open(member, core::ptr::null_mut(), threads);
    if (err < 0)
    return 0;
    while (count--) ;
    memset(&counts, -1, sizeof(counts));
    perf_evsel__read(leader, 0, 0, &counts);
    __T("failed to read leader value", counts.val);
    if (attr.read_format & PERF_FORMAT_TOTAL_TIME_ENABLED)
    __T("failed to read leader TOTAL_TIME_ENABLED", counts.ena);
    if (attr.read_format & PERF_FORMAT_TOTAL_TIME_RUNNING)
    __T("failed to read leader TOTAL_TIME_RUNNING", counts.run);
    if (attr.read_format & PERF_FORMAT_ID)
    __T("failed to read leader ID", counts.id);
    if (attr.read_format & PERF_FORMAT_LOST)
    __T("failed to read leader LOST", counts.lost == 0);
    memset(&counts, -1, sizeof(counts));
    perf_evsel__read(member, 0, 0, &counts);
    __T("failed to read member value", counts.val);
    if (attr.read_format & PERF_FORMAT_TOTAL_TIME_ENABLED)
    __T("failed to read member TOTAL_TIME_ENABLED", counts.ena);
    if (attr.read_format & PERF_FORMAT_TOTAL_TIME_RUNNING)
    __T("failed to read member TOTAL_TIME_RUNNING", counts.run);
    if (attr.read_format & PERF_FORMAT_ID)
    __T("failed to read member ID", counts.id);
    if (attr.read_format & PERF_FORMAT_LOST)
    __T("failed to read member LOST", counts.lost == 0);
    perf_evsel__close(member);
    perf_evsel__close(leader);
    perf_evsel__delete(member);
    perf_evsel__delete(leader);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_stat_read_format() -> c_int {
    static int test_stat_read_format(void)
    {
    struct perf_thread_map *threads;
    struct perf_event_attr attr = {
    .type	= PERF_TYPE_SOFTWARE,
    .config	= PERF_COUNT_SW_TASK_CLOCK,
    };
    int err, i;

    uint64_t test_formats [] = {
    0,
    FMT_TIME,
    FMT(ID),
    FMT(LOST),
    FMT_TIME | FMT(ID),
    FMT_TIME | FMT(LOST),
    FMT_TIME | FMT(ID) | FMT(LOST),
    FMT(ID) | FMT(LOST),
    };

    threads = perf_thread_map__new_dummy();
    __T("failed to create threads", threads);
    perf_thread_map__set_pid(threads, 0, 0);
    for (i = 0; i < (int)ARRAY_SIZE(test_formats); i++) {
    attr.read_format = test_formats[i];
    __T_VERBOSE("testing single read with read_format: %lx\n",
    (unsigned long)test_formats[i]);
    err = test_stat_read_format_single(&attr, threads);
    __T("failed to read single format", err == 0);
    }
    perf_thread_map__put(threads);
    threads = perf_thread_map__new_array(2, core::ptr::null_mut());
    __T("failed to create threads", threads);
    perf_thread_map__set_pid(threads, 0, 0);
    perf_thread_map__set_pid(threads, 1, 0);
    for (i = 0; i < (int)ARRAY_SIZE(test_formats); i++) {
    attr.read_format = test_formats[i];
    __T_VERBOSE("testing group read with read_format: %lx\n",
    (unsigned long)test_formats[i]);
    err = test_stat_read_format_group(&attr, threads);
    __T("failed to read group format", err == 0);
    }
    perf_thread_map__put(threads);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_evsel(argc: c_int, argv: *mut c_char) -> c_int {
    int test_evsel(int argc, char **argv)
    {
    __T_START;
    libperf_init(libperf_print);
    test_stat_cpu();
    test_stat_thread();
    test_stat_thread_enable();
    test_stat_user_read(PERF_COUNT_HW_INSTRUCTIONS);
    test_stat_user_read(PERF_COUNT_HW_CPU_CYCLES);
    test_stat_read_format();
    __T_END;
    let mut tests_failed: return = = 0 ? 0 : -1;
    }
