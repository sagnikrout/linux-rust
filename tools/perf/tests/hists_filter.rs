//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/hists_filter.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sample {
    pub pid: u32,
    pub ip: u64,
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
    pub socket: c_int,
}

// For the numbers, see hists_common.c
    static struct sample fake_samples[] = {
// perf [kernel] schedule()
    { .pid = FAKE_PID_PERF1, .ip = FAKE_IP_KERNEL_SCHEDULE, .socket = 0 },
// perf [perf]   main()
    { .pid = FAKE_PID_PERF1, .ip = FAKE_IP_PERF_MAIN, .socket = 0 },
// perf [libc]   malloc()
    { .pid = FAKE_PID_PERF1, .ip = FAKE_IP_LIBC_MALLOC, .socket = 0 },
// perf [perf]   main()
    { .pid = FAKE_PID_PERF2, .ip = FAKE_IP_PERF_MAIN, .socket = 0 }, /* will be merged */
// perf [perf]   cmd_record()
    { .pid = FAKE_PID_PERF2, .ip = FAKE_IP_PERF_CMD_RECORD, .socket = 1 },
// perf [kernel] page_fault()
    { .pid = FAKE_PID_PERF2, .ip = FAKE_IP_KERNEL_PAGE_FAULT, .socket = 1 },
// bash [bash]   main()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_BASH_MAIN, .socket = 2 },
// bash [bash]   xmalloc()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_BASH_XMALLOC, .socket = 2 },
// bash [libc]   malloc()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_LIBC_MALLOC, .socket = 3 },
// bash [kernel] page_fault()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_KERNEL_PAGE_FAULT, .socket = 3 },
    };
    static int add_hist_entries(struct evlist *evlist,
    struct machine *machine)
    {
    struct evsel *evsel;
    struct addr_location al;
    let mut sample: perf_sample = { .period = 100, };
    size_t i;
    addr_location__init(&al);
//
// each evsel will have 10 samples but the 4th sample
// (perf [perf] main) will be collapsed to an existing entry
// so total 9 entries will be in the tree.
//
    evlist__for_each_entry(evlist, evsel) {
    for (i = 0; i < ARRAY_SIZE(fake_samples); i++) {
    struct hist_entry_iter iter = {
    .sample = &sample,
    .ops = &hist_iter_normal,
    .hide_unresolved = false,
    };
    struct hists *hists = evsel__hists(evsel);
    sample.evsel = evsel;
// make sure it has no filter at first
    hists.thread_filter = core::ptr::null_mut();
    hists.dso_filter = core::ptr::null_mut();
    hists.symbol_filter_str = core::ptr::null_mut();
    sample.cpumode = PERF_RECORD_MISC_USER;
    sample.pid = fake_samples[i].pid;
    sample.tid = fake_samples[i].pid;
    sample.ip = fake_samples[i].ip;
    if (machine__resolve(machine, &al, &sample) < 0)
    goto out;
    al.socket = fake_samples[i].socket;
    if (hist_entry_iter__add(&iter, &al,
    sysctl_perf_event_max_stack, core::ptr::null_mut()) < 0) {
    goto out;
    }
    thread__put(fake_samples[i].thread);
    fake_samples[i].thread = thread__get(al.thread);
    map__put(fake_samples[i].map);
    fake_samples[i].map = map__get(al.map);
    fake_samples[i].sym = al.sym;
    }
    }
    addr_location__exit(&al);
    return 0;
    out:
    pr_debug("Not enough memory for adding a hist entry\n");
    addr_location__exit(&al);
    return TEST_FAIL;
    }
#[no_mangle]
unsafe extern "C" fn put_fake_samples() {
    static void put_fake_samples(void)
    {
    size_t i;
    for (i = 0; i < ARRAY_SIZE(fake_samples); i++)
    map__put(fake_samples[i].map);
    }
#[no_mangle]
unsafe extern "C" fn test__hists_filter(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__hists_filter(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut err: c_int = TEST_FAIL;
    let mut machines: machines = { 0 };
    struct machine *machine;
    struct evsel *evsel;
    struct evlist *evlist = evlist__new();
    TEST_ASSERT_VAL("No memory", evlist);
    err = parse_event(evlist, "cpu-clock");
    if (err)
    goto out;
    err = parse_event(evlist, "task-clock");
    if (err)
    goto out;
    err = TEST_FAIL;
    if (machines__init(&machines))
    goto out;
// setup threads/dso/map/symbols also
    machine = setup_fake_machine(&machines);
    if (!machine)
    goto out;
    if (verbose > 1)
    machine__fprintf(machine, stderr);
// default sort order (comm,dso,sym) will be used
    if (setup_sorting(evlist, machine.env) < 0)
    goto out;
// process sample events
    err = add_hist_entries(evlist, machine);
    if (err < 0)
    goto out;
    evlist__for_each_entry(evlist, evsel) {
    struct hists *hists = evsel__hists(evsel);
    hists__collapse_resort(hists, core::ptr::null_mut());
    evsel__output_resort(evsel, core::ptr::null_mut());
    if (verbose > 2) {
    pr_info("Normal histogram\n");
    print_hists_out(hists);
    }
    TEST_ASSERT_VAL("Invalid nr samples",
    hists.stats.nr_samples == 10);
    TEST_ASSERT_VAL("Invalid nr hist entries",
    hists.nr_entries == 9);
    TEST_ASSERT_VAL("Invalid total period",
    hists.stats.total_period == 1000);
    TEST_ASSERT_VAL("Unmatched nr samples",
    hists.stats.nr_samples ==
    hists.stats.nr_non_filtered_samples);
    TEST_ASSERT_VAL("Unmatched nr hist entries",
    hists.nr_entries == hists.nr_non_filtered_entries);
    TEST_ASSERT_VAL("Unmatched total period",
    hists.stats.total_period ==
    hists.stats.total_non_filtered_period);
// now applying thread filter for 'bash'
    hists.thread_filter = fake_samples[9].thread;
    hists__filter_by_thread(hists);
    if (verbose > 2) {
    pr_info("Histogram for thread filter\n");
    print_hists_out(hists);
    }
// normal stats should be invariant
    TEST_ASSERT_VAL("Invalid nr samples",
    hists.stats.nr_samples == 10);
    TEST_ASSERT_VAL("Invalid nr hist entries",
    hists.nr_entries == 9);
    TEST_ASSERT_VAL("Invalid total period",
    hists.stats.total_period == 1000);
// but filter stats are changed
    TEST_ASSERT_VAL("Unmatched nr samples for thread filter",
    hists.stats.nr_non_filtered_samples == 4);
    TEST_ASSERT_VAL("Unmatched nr hist entries for thread filter",
    hists.nr_non_filtered_entries == 4);
    TEST_ASSERT_VAL("Unmatched total period for thread filter",
    hists.stats.total_non_filtered_period == 400);
// remove thread filter first
    hists.thread_filter = core::ptr::null_mut();
    hists__filter_by_thread(hists);
// now applying dso filter for 'kernel'
    hists.dso_filter = map__dso(fake_samples[0].map);
    hists__filter_by_dso(hists);
    if (verbose > 2) {
    pr_info("Histogram for dso filter\n");
    print_hists_out(hists);
    }
// normal stats should be invariant
    TEST_ASSERT_VAL("Invalid nr samples",
    hists.stats.nr_samples == 10);
    TEST_ASSERT_VAL("Invalid nr hist entries",
    hists.nr_entries == 9);
    TEST_ASSERT_VAL("Invalid total period",
    hists.stats.total_period == 1000);
// but filter stats are changed
    TEST_ASSERT_VAL("Unmatched nr samples for dso filter",
    hists.stats.nr_non_filtered_samples == 3);
    TEST_ASSERT_VAL("Unmatched nr hist entries for dso filter",
    hists.nr_non_filtered_entries == 3);
    TEST_ASSERT_VAL("Unmatched total period for dso filter",
    hists.stats.total_non_filtered_period == 300);
// remove dso filter first
    hists.dso_filter = core::ptr::null_mut();
    hists__filter_by_dso(hists);
//
// now applying symbol filter for 'main'.  Also note that
// there's 3 samples that have 'main' symbol but the 4th
// entry of fake_samples was collapsed already so it won't
// be counted as a separate entry but the sample count and
// total period will be remained.
//
    hists.symbol_filter_str = "main";
    hists__filter_by_symbol(hists);
    if (verbose > 2) {
    pr_info("Histogram for symbol filter\n");
    print_hists_out(hists);
    }
// normal stats should be invariant
    TEST_ASSERT_VAL("Invalid nr samples",
    hists.stats.nr_samples == 10);
    TEST_ASSERT_VAL("Invalid nr hist entries",
    hists.nr_entries == 9);
    TEST_ASSERT_VAL("Invalid total period",
    hists.stats.total_period == 1000);
// but filter stats are changed
    TEST_ASSERT_VAL("Unmatched nr samples for symbol filter",
    hists.stats.nr_non_filtered_samples == 3);
    TEST_ASSERT_VAL("Unmatched nr hist entries for symbol filter",
    hists.nr_non_filtered_entries == 2);
    TEST_ASSERT_VAL("Unmatched total period for symbol filter",
    hists.stats.total_non_filtered_period == 300);
// remove symbol filter first
    hists.symbol_filter_str = core::ptr::null_mut();
    hists__filter_by_symbol(hists);
// now applying socket filters
    hists.socket_filter = 2;
    hists__filter_by_socket(hists);
    if (verbose > 2) {
    pr_info("Histogram for socket filters\n");
    print_hists_out(hists);
    }
// normal stats should be invariant
    TEST_ASSERT_VAL("Invalid nr samples",
    hists.stats.nr_samples == 10);
    TEST_ASSERT_VAL("Invalid nr hist entries",
    hists.nr_entries == 9);
    TEST_ASSERT_VAL("Invalid total period",
    hists.stats.total_period == 1000);
// but filter stats are changed
    TEST_ASSERT_VAL("Unmatched nr samples for socket filter",
    hists.stats.nr_non_filtered_samples == 2);
    TEST_ASSERT_VAL("Unmatched nr hist entries for socket filter",
    hists.nr_non_filtered_entries == 2);
    TEST_ASSERT_VAL("Unmatched total period for socket filter",
    hists.stats.total_non_filtered_period == 200);
// remove socket filter first
    hists.socket_filter = -1;
    hists__filter_by_socket(hists);
// now applying all filters at once.
    hists.thread_filter = fake_samples[1].thread;
    hists.dso_filter = map__dso(fake_samples[1].map);
    hists__filter_by_thread(hists);
    hists__filter_by_dso(hists);
    if (verbose > 2) {
    pr_info("Histogram for all filters\n");
    print_hists_out(hists);
    }
// normal stats should be invariant
    TEST_ASSERT_VAL("Invalid nr samples",
    hists.stats.nr_samples == 10);
    TEST_ASSERT_VAL("Invalid nr hist entries",
    hists.nr_entries == 9);
    TEST_ASSERT_VAL("Invalid total period",
    hists.stats.total_period == 1000);
// but filter stats are changed
    TEST_ASSERT_VAL("Unmatched nr samples for all filter",
    hists.stats.nr_non_filtered_samples == 2);
    TEST_ASSERT_VAL("Unmatched nr hist entries for all filter",
    hists.nr_non_filtered_entries == 1);
    TEST_ASSERT_VAL("Unmatched total period for all filter",
    hists.stats.total_non_filtered_period == 200);
    }
    err = TEST_OK;
    out:
// tear down everything
    evlist__put(evlist);
    reset_output_field();
    machines__exit(&machines);
    put_fake_samples();
    return err;
    }
    DEFINE_SUITE("Filter hist entries", hists_filter);
