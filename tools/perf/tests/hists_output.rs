//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/hists_output.c
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
    pub cpu: u32,
    pub pid: u32,
    pub ip: u64,
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
}

// For the numbers, see hists_common.c
    static struct sample fake_samples[] = {
// perf [kernel] schedule()
    { .cpu = 0, .pid = FAKE_PID_PERF1, .ip = FAKE_IP_KERNEL_SCHEDULE, },
// perf [perf]   main()
    { .cpu = 1, .pid = FAKE_PID_PERF1, .ip = FAKE_IP_PERF_MAIN, },
// perf [perf]   cmd_record()
    { .cpu = 1, .pid = FAKE_PID_PERF1, .ip = FAKE_IP_PERF_CMD_RECORD, },
// perf [libc]   malloc()
    { .cpu = 1, .pid = FAKE_PID_PERF1, .ip = FAKE_IP_LIBC_MALLOC, },
// perf [libc]   free()
    { .cpu = 2, .pid = FAKE_PID_PERF1, .ip = FAKE_IP_LIBC_FREE, },
// perf [perf]   main()
    { .cpu = 2, .pid = FAKE_PID_PERF2, .ip = FAKE_IP_PERF_MAIN, },
// perf [kernel] page_fault()
    { .cpu = 2, .pid = FAKE_PID_PERF2, .ip = FAKE_IP_KERNEL_PAGE_FAULT, },
// bash [bash]   main()
    { .cpu = 3, .pid = FAKE_PID_BASH,  .ip = FAKE_IP_BASH_MAIN, },
// bash [bash]   xmalloc()
    { .cpu = 0, .pid = FAKE_PID_BASH,  .ip = FAKE_IP_BASH_XMALLOC, },
// bash [kernel] page_fault()
    { .cpu = 1, .pid = FAKE_PID_BASH,  .ip = FAKE_IP_KERNEL_PAGE_FAULT, },
    };
#[no_mangle]
unsafe extern "C" fn add_hist_entries(hists: *mut hists, machine: *mut machine) -> c_int {
    static int add_hist_entries(struct hists *hists, struct machine *machine)
    {
    struct addr_location al;
    struct evsel *evsel = hists_to_evsel(hists);
    let mut sample: perf_sample = { .evsel = evsel, .period = 100, };
    size_t i;
    addr_location__init(&al);
    for (i = 0; i < ARRAY_SIZE(fake_samples); i++) {
    struct hist_entry_iter iter = {
    .sample = &sample,
    .ops = &hist_iter_normal,
    .hide_unresolved = false,
    };
    sample.cpumode = PERF_RECORD_MISC_USER;
    sample.cpu = fake_samples[i].cpu;
    sample.pid = fake_samples[i].pid;
    sample.tid = fake_samples[i].pid;
    sample.ip = fake_samples[i].ip;
    if (machine__resolve(machine, &al, &sample) < 0)
    goto out;
    if (hist_entry_iter__add(&iter, &al, sysctl_perf_event_max_stack,
    core::ptr::null_mut()) < 0) {
    goto out;
    }
    fake_samples[i].thread = al.thread;
    map__put(fake_samples[i].map);
    fake_samples[i].map = map__get(al.map);
    fake_samples[i].sym = al.sym;
    }
    addr_location__exit(&al);
    return TEST_OK;
    out:
    pr_debug("Not enough memory for adding a hist entry\n");
    addr_location__exit(&al);
    return TEST_FAIL;
    }
#[no_mangle]
unsafe extern "C" fn del_hist_entries(hists: *mut hists) {
    static void del_hist_entries(struct hists *hists)
    {
    struct hist_entry *he;
    struct rb_root_cached *root_in;
    struct rb_root_cached *root_out;
    struct rb_node *node;
    if (hists__has(hists, need_collapse))
    root_in = &hists.entries_collapsed;
    else
    root_in = hists.entries_in;
    root_out = &hists.entries;
    while (!RB_EMPTY_ROOT(&root_out.rb_root)) {
    node = rb_first_cached(root_out);
    he = rb_entry(node, struct hist_entry, rb_node);
    rb_erase_cached(node, root_out);
    rb_erase_cached(&he.rb_node_in, root_in);
    hist_entry__delete(he);
    }
    }
#[no_mangle]
unsafe extern "C" fn put_fake_samples() {
    static void put_fake_samples(void)
    {
    size_t i;
    for (i = 0; i < ARRAY_SIZE(fake_samples); i++) {
    map__put(fake_samples[i].map);
    fake_samples[i].map = core::ptr::null_mut();
    }
    }
    typedef int (*test_fn_t)(struct evsel *, struct machine *);

// default sort keys (no field)
#[no_mangle]
unsafe extern "C" fn test1(evsel: *mut evsel, machine: *mut machine) -> c_int {
    static int test1(struct evsel *evsel, struct machine *machine)
    {
    int err;
    struct hists *hists = evsel__hists(evsel);
    struct hist_entry *he;
    struct rb_root_cached *root;
    struct rb_node *node;
    field_order = core::ptr::null_mut();
    sort_order = core::ptr::null_mut(); /* equivalent to sort_order = "comm,dso,sym" */
    setup_sorting(/*evlist=*/core::ptr::null_mut(), machine.env);
//
// expected output:
//
// Overhead  Command  Shared Object          Symbol
// ========  =======  =============  ==============
// 20.00%     perf  perf           [.] main
// 10.00%     bash  [kernel]       [k] page_fault
// 10.00%     bash  bash           [.] main
// 10.00%     bash  bash           [.] xmalloc
// 10.00%     perf  [kernel]       [k] page_fault
// 10.00%     perf  [kernel]       [k] schedule
// 10.00%     perf  libc           [.] free
// 10.00%     perf  libc           [.] malloc
// 10.00%     perf  perf           [.] cmd_record
//
    err = add_hist_entries(hists, machine);
    if (err < 0)
    goto out;
    hists__collapse_resort(hists, core::ptr::null_mut());
    evsel__output_resort(evsel, core::ptr::null_mut());
    if (verbose > 2) {
    pr_info("[fields = %s, sort = %s]\n", field_order, sort_order);
    print_hists_out(hists);
    }
    root = &hists.entries;
    node = rb_first_cached(root);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "perf") &&
    !strcmp(SYM(he), "main") && he.stat.period == 200);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "bash") && !strcmp(DSO(he), "[kernel]") &&
    !strcmp(SYM(he), "page_fault") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "bash") && !strcmp(DSO(he), "bash") &&
    !strcmp(SYM(he), "main") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "bash") && !strcmp(DSO(he), "bash") &&
    !strcmp(SYM(he), "xmalloc") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "[kernel]") &&
    !strcmp(SYM(he), "page_fault") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "[kernel]") &&
    !strcmp(SYM(he), "schedule") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "libc") &&
    !strcmp(SYM(he), "free") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "libc") &&
    !strcmp(SYM(he), "malloc") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "perf") &&
    !strcmp(SYM(he), "cmd_record") && he.stat.period == 100);
    out:
    del_hist_entries(hists);
    reset_output_field();
    return err;
    }
// mixed fields and sort keys
#[no_mangle]
unsafe extern "C" fn test2(evsel: *mut evsel, machine: *mut machine) -> c_int {
    static int test2(struct evsel *evsel, struct machine *machine)
    {
    int err;
    struct hists *hists = evsel__hists(evsel);
    struct hist_entry *he;
    struct rb_root_cached *root;
    struct rb_node *node;
    field_order = "overhead,cpu";
    sort_order = "pid";
    setup_sorting(/*evlist=*/core::ptr::null_mut(), machine.env);
//
// expected output:
//
// Overhead  CPU  Command:  Pid
// ========  ===  =============
// 30.00%    1  perf   :  100
// 10.00%    0  perf   :  100
// 10.00%    2  perf   :  100
// 20.00%    2  perf   :  200
// 10.00%    0  bash   :  300
// 10.00%    1  bash   :  300
// 10.00%    3  bash   :  300
//
    err = add_hist_entries(hists, machine);
    if (err < 0)
    goto out;
    hists__collapse_resort(hists, core::ptr::null_mut());
    evsel__output_resort(evsel, core::ptr::null_mut());
    if (verbose > 2) {
    pr_info("[fields = %s, sort = %s]\n", field_order, sort_order);
    print_hists_out(hists);
    }
    root = &hists.entries;
    node = rb_first_cached(root);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 1 && PID(he) == 100 && he.stat.period == 300);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 0 && PID(he) == 100 && he.stat.period == 100);
    out:
    del_hist_entries(hists);
    reset_output_field();
    return err;
    }
// fields only (no sort key)
#[no_mangle]
unsafe extern "C" fn test3(evsel: *mut evsel, machine: *mut machine) -> c_int {
    static int test3(struct evsel *evsel, struct machine *machine)
    {
    int err;
    struct hists *hists = evsel__hists(evsel);
    struct hist_entry *he;
    struct rb_root_cached *root;
    struct rb_node *node;
    field_order = "comm,overhead,dso";
    sort_order = core::ptr::null_mut();
    setup_sorting(/*evlist=*/core::ptr::null_mut(), machine.env);
//
// expected output:
//
// Command  Overhead  Shared Object
// =======  ========  =============
// bash    20.00%  bash
// bash    10.00%  [kernel]
// perf    30.00%  perf
// perf    20.00%  [kernel]
// perf    20.00%  libc
//
    err = add_hist_entries(hists, machine);
    if (err < 0)
    goto out;
    hists__collapse_resort(hists, core::ptr::null_mut());
    evsel__output_resort(evsel, core::ptr::null_mut());
    if (verbose > 2) {
    pr_info("[fields = %s, sort = %s]\n", field_order, sort_order);
    print_hists_out(hists);
    }
    root = &hists.entries;
    node = rb_first_cached(root);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "bash") && !strcmp(DSO(he), "bash") &&
    he.stat.period == 200);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "bash") && !strcmp(DSO(he), "[kernel]") &&
    he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "perf") &&
    he.stat.period == 300);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "[kernel]") &&
    he.stat.period == 200);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "libc") &&
    he.stat.period == 200);
    out:
    del_hist_entries(hists);
    reset_output_field();
    return err;
    }
// handle duplicate 'dso' field
#[no_mangle]
unsafe extern "C" fn test4(evsel: *mut evsel, machine: *mut machine) -> c_int {
    static int test4(struct evsel *evsel, struct machine *machine)
    {
    int err;
    struct hists *hists = evsel__hists(evsel);
    struct hist_entry *he;
    struct rb_root_cached *root;
    struct rb_node *node;
    field_order = "dso,sym,comm,overhead,dso";
    sort_order = "sym";
    setup_sorting(/*evlist=*/core::ptr::null_mut(), machine.env);
//
// expected output:
//
// Shared Object          Symbol  Command  Overhead
// =============  ==============  =======  ========
// perf  [.] cmd_record     perf    10.00%
// libc  [.] free           perf    10.00%
// bash  [.] main           bash    10.00%
// perf  [.] main           perf    20.00%
// libc  [.] malloc         perf    10.00%
// [kernel]  [k] page_fault     bash    10.00%
// [kernel]  [k] page_fault     perf    10.00%
// [kernel]  [k] schedule       perf    10.00%
// bash  [.] xmalloc        bash    10.00%
//
    err = add_hist_entries(hists, machine);
    if (err < 0)
    goto out;
    hists__collapse_resort(hists, core::ptr::null_mut());
    evsel__output_resort(evsel, core::ptr::null_mut());
    if (verbose > 2) {
    pr_info("[fields = %s, sort = %s]\n", field_order, sort_order);
    print_hists_out(hists);
    }
    root = &hists.entries;
    node = rb_first_cached(root);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(DSO(he), "perf") && !strcmp(SYM(he), "cmd_record") &&
    !strcmp(COMM(he), "perf") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(DSO(he), "libc") && !strcmp(SYM(he), "free") &&
    !strcmp(COMM(he), "perf") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(DSO(he), "bash") && !strcmp(SYM(he), "main") &&
    !strcmp(COMM(he), "bash") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(DSO(he), "perf") && !strcmp(SYM(he), "main") &&
    !strcmp(COMM(he), "perf") && he.stat.period == 200);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(DSO(he), "libc") && !strcmp(SYM(he), "malloc") &&
    !strcmp(COMM(he), "perf") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(DSO(he), "[kernel]") && !strcmp(SYM(he), "page_fault") &&
    !strcmp(COMM(he), "bash") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(DSO(he), "[kernel]") && !strcmp(SYM(he), "page_fault") &&
    !strcmp(COMM(he), "perf") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(DSO(he), "[kernel]") && !strcmp(SYM(he), "schedule") &&
    !strcmp(COMM(he), "perf") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    !strcmp(DSO(he), "bash") && !strcmp(SYM(he), "xmalloc") &&
    !strcmp(COMM(he), "bash") && he.stat.period == 100);
    out:
    del_hist_entries(hists);
    reset_output_field();
    return err;
    }
// full sort keys w/o overhead field
#[no_mangle]
unsafe extern "C" fn test5(evsel: *mut evsel, machine: *mut machine) -> c_int {
    static int test5(struct evsel *evsel, struct machine *machine)
    {
    int err;
    struct hists *hists = evsel__hists(evsel);
    struct hist_entry *he;
    struct rb_root_cached *root;
    struct rb_node *node;
    field_order = "cpu,pid,comm,dso,sym";
    sort_order = "dso,pid";
    setup_sorting(/*evlist=*/core::ptr::null_mut(), machine.env);
//
// expected output:
//
// CPU  Command:  Pid  Command  Shared Object          Symbol
// ===  =============  =======  =============  ==============
// 0     perf:  100     perf       [kernel]  [k] schedule
// 2     perf:  200     perf       [kernel]  [k] page_fault
// 1     bash:  300     bash       [kernel]  [k] page_fault
// 0     bash:  300     bash           bash  [.] xmalloc
// 3     bash:  300     bash           bash  [.] main
// 1     perf:  100     perf           libc  [.] malloc
// 2     perf:  100     perf           libc  [.] free
// 1     perf:  100     perf           perf  [.] cmd_record
// 1     perf:  100     perf           perf  [.] main
// 2     perf:  200     perf           perf  [.] main
//
    err = add_hist_entries(hists, machine);
    if (err < 0)
    goto out;
    hists__collapse_resort(hists, core::ptr::null_mut());
    evsel__output_resort(evsel, core::ptr::null_mut());
    if (verbose > 2) {
    pr_info("[fields = %s, sort = %s]\n", field_order, sort_order);
    print_hists_out(hists);
    }
    root = &hists.entries;
    node = rb_first_cached(root);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 0 && PID(he) == 100 &&
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "[kernel]") &&
    !strcmp(SYM(he), "schedule") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 2 && PID(he) == 200 &&
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "[kernel]") &&
    !strcmp(SYM(he), "page_fault") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 1 && PID(he) == 300 &&
    !strcmp(COMM(he), "bash") && !strcmp(DSO(he), "[kernel]") &&
    !strcmp(SYM(he), "page_fault") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 0 && PID(he) == 300 &&
    !strcmp(COMM(he), "bash") && !strcmp(DSO(he), "bash") &&
    !strcmp(SYM(he), "xmalloc") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 3 && PID(he) == 300 &&
    !strcmp(COMM(he), "bash") && !strcmp(DSO(he), "bash") &&
    !strcmp(SYM(he), "main") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 1 && PID(he) == 100 &&
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "libc") &&
    !strcmp(SYM(he), "malloc") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 2 && PID(he) == 100 &&
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "libc") &&
    !strcmp(SYM(he), "free") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 1 && PID(he) == 100 &&
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "perf") &&
    !strcmp(SYM(he), "cmd_record") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 1 && PID(he) == 100 &&
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "perf") &&
    !strcmp(SYM(he), "main") && he.stat.period == 100);
    node = rb_next(node);
    he = rb_entry(node, struct hist_entry, rb_node);
    TEST_ASSERT_VAL("Invalid hist entry",
    CPU(he) == 2 && PID(he) == 200 &&
    !strcmp(COMM(he), "perf") && !strcmp(DSO(he), "perf") &&
    !strcmp(SYM(he), "main") && he.stat.period == 100);
    out:
    del_hist_entries(hists);
    reset_output_field();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test__hists_output(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__hists_output(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut err: c_int = TEST_FAIL;
    let mut machines: machines = { 0 };
    struct machine *machine;
    struct evsel *evsel;
    struct evlist *evlist = evlist__new();
    size_t i;
    test_fn_t testcases[] = {
    test1,
    test2,
    test3,
    test4,
    test5,
    };
    TEST_ASSERT_VAL("No memory", evlist);
    err = parse_event(evlist, "cpu-clock");
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
    evsel = evlist__first(evlist);
    for (i = 0; i < ARRAY_SIZE(testcases); i++) {
    err = testcases[i](evsel, machine);
    if (err < 0)
    break;
    }
    out:
// tear down everything
    evlist__put(evlist);
    machines__exit(&machines);
    put_fake_samples();
    return err;
    }
    DEFINE_SUITE("Sort output of hist entries", hists_output);
