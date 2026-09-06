//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/hists_link.c
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
}

// For the numbers, see hists_common.c
    static struct sample fake_common_samples[] = {
// perf [kernel] schedule()
    { .pid = FAKE_PID_PERF1, .ip = FAKE_IP_KERNEL_SCHEDULE, },
// perf [perf]   main()
    { .pid = FAKE_PID_PERF2, .ip = FAKE_IP_PERF_MAIN, },
// perf [perf]   cmd_record()
    { .pid = FAKE_PID_PERF2, .ip = FAKE_IP_PERF_CMD_RECORD, },
// bash [bash]   xmalloc()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_BASH_XMALLOC, },
// bash [libc]   malloc()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_LIBC_MALLOC, },
    };
    static struct sample fake_samples[][5] = {
    {
// perf [perf]   run_command()
    { .pid = FAKE_PID_PERF1, .ip = FAKE_IP_PERF_RUN_COMMAND, },
// perf [libc]   malloc()
    { .pid = FAKE_PID_PERF1, .ip = FAKE_IP_LIBC_MALLOC, },
// perf [kernel] page_fault()
    { .pid = FAKE_PID_PERF1, .ip = FAKE_IP_KERNEL_PAGE_FAULT, },
// perf [kernel] sys_perf_event_open()
    { .pid = FAKE_PID_PERF2, .ip = FAKE_IP_KERNEL_SYS_PERF_EVENT_OPEN, },
// bash [libc]   free()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_LIBC_FREE, },
    },
    {
// perf [libc]   free()
    { .pid = FAKE_PID_PERF2, .ip = FAKE_IP_LIBC_FREE, },
// bash [libc]   malloc()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_LIBC_MALLOC, }, /* will be merged */
// bash [bash]   xfee()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_BASH_XFREE, },
// bash [libc]   realloc()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_LIBC_REALLOC, },
// bash [kernel] page_fault()
    { .pid = FAKE_PID_BASH,  .ip = FAKE_IP_KERNEL_PAGE_FAULT, },
    },
    };
#[no_mangle]
unsafe extern "C" fn add_hist_entries(evlist: *mut evlist, machine: *mut machine) -> c_int {
    static int add_hist_entries(struct evlist *evlist, struct machine *machine)
    {
    struct evsel *evsel;
    struct addr_location al;
    struct hist_entry *he;
    let mut sample: perf_sample = { .period = 1, .weight = 1, };
    let mut i: usize = 0, k;
    addr_location__init(&al);
//
// each evsel will have 10 samples - 5 common and 5 distinct.
// However the second evsel also has a collapsed entry for
// "bash [libc] malloc" so total 9 entries will be in the tree.
//
    evlist__for_each_entry(evlist, evsel) {
    struct hists *hists = evsel__hists(evsel);
    for (k = 0; k < ARRAY_SIZE(fake_common_samples); k++) {
    sample.cpumode = PERF_RECORD_MISC_USER;
    sample.pid = fake_common_samples[k].pid;
    sample.tid = fake_common_samples[k].pid;
    sample.ip = fake_common_samples[k].ip;
    if (machine__resolve(machine, &al, &sample) < 0)
    goto out;
    he = hists__add_entry(hists, &al, core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), &sample, true);
    if (he == core::ptr::null_mut()) {
    goto out;
    }
    thread__put(fake_common_samples[k].thread);
    fake_common_samples[k].thread = thread__get(al.thread);
    map__put(fake_common_samples[k].map);
    fake_common_samples[k].map = map__get(al.map);
    fake_common_samples[k].sym = al.sym;
    }
    for (k = 0; k < ARRAY_SIZE(fake_samples[i]); k++) {
    sample.pid = fake_samples[i][k].pid;
    sample.tid = fake_samples[i][k].pid;
    sample.ip = fake_samples[i][k].ip;
    if (machine__resolve(machine, &al, &sample) < 0)
    goto out;
    he = hists__add_entry(hists, &al, core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), &sample, true);
    if (he == core::ptr::null_mut()) {
    goto out;
    }
    thread__put(fake_samples[i][k].thread);
    fake_samples[i][k].thread = thread__get(al.thread);
    map__put(fake_samples[i][k].map);
    fake_samples[i][k].map = map__get(al.map);
    fake_samples[i][k].sym = al.sym;
    }
    i++;
    }
    addr_location__exit(&al);
    return 0;
    out:
    addr_location__exit(&al);
    pr_debug("Not enough memory for adding a hist entry\n");
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn put_fake_samples() {
    static void put_fake_samples(void)
    {
    size_t i, j;
    for (i = 0; i < ARRAY_SIZE(fake_common_samples); i++)
    map__put(fake_common_samples[i].map);
    for (i = 0; i < ARRAY_SIZE(fake_samples); i++) {
    for (j = 0; j < ARRAY_SIZE(fake_samples[0]); j++)
    map__put(fake_samples[i][j].map);
    }
    }
    static int find_sample(struct sample *samples, size_t nr_samples,
    struct thread *t, struct map *m, struct symbol *s)
    {
    while (nr_samples--) {
    if (RC_CHK_EQUAL(samples.thread, t) &&
    RC_CHK_EQUAL(samples.map, m) &&
    samples.sym == s)
    return 1;
    samples++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __validate_match(hists: *mut hists) -> c_int {
    static int __validate_match(struct hists *hists)
    {
    let mut count: usize = 0;
    struct rb_root_cached *root;
    struct rb_node *node;
//
// Only entries from fake_common_samples should have a pair.
//
    if (hists__has(hists, need_collapse))
    root = &hists.entries_collapsed;
    else
    root = hists.entries_in;
    node = rb_first_cached(root);
    while (node) {
    struct hist_entry *he;
    he = rb_entry(node, struct hist_entry, rb_node_in);
    if (hist_entry__has_pairs(he)) {
    if (find_sample(fake_common_samples,
    ARRAY_SIZE(fake_common_samples),
    he.thread, he.ms.map, he.ms.sym)) {
    count++;
    } else {
    pr_debug("Can't find the matched entry\n");
    return -1;
    }
    }
    node = rb_next(node);
    }
    if (count != ARRAY_SIZE(fake_common_samples)) {
    pr_debug("Invalid count for matched entries: %zd of %zd\n",
    count, ARRAY_SIZE(fake_common_samples));
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn validate_match(leader: *mut hists, other: *mut hists) -> c_int {
    static int validate_match(struct hists *leader, struct hists *other)
    {
    return __validate_match(leader) || __validate_match(other);
    }
#[no_mangle]
unsafe extern "C" fn __validate_link(hists: *mut hists, idx: c_int) -> c_int {
    static int __validate_link(struct hists *hists, int idx)
    {
    let mut count: usize = 0;
    let mut count_pair: usize = 0;
    let mut count_dummy: usize = 0;
    struct rb_root_cached *root;
    struct rb_node *node;
//
// Leader hists (idx = 0) will have dummy entries from other,
// and some entries will have no pair.  However every entry
// in other hists should have (dummy) pair.
//
    if (hists__has(hists, need_collapse))
    root = &hists.entries_collapsed;
    else
    root = hists.entries_in;
    node = rb_first_cached(root);
    while (node) {
    struct hist_entry *he;
    he = rb_entry(node, struct hist_entry, rb_node_in);
    if (hist_entry__has_pairs(he)) {
    if (!find_sample(fake_common_samples,
    ARRAY_SIZE(fake_common_samples),
    he.thread, he.ms.map, he.ms.sym) &&
    !find_sample(fake_samples[idx],
    ARRAY_SIZE(fake_samples[idx]),
    he.thread, he.ms.map, he.ms.sym)) {
    count_dummy++;
    }
    count_pair++;
    } else if (idx) {
    pr_debug("A entry from the other hists should have pair\n");
    return -1;
    }
    count++;
    node = rb_next(node);
    }
//
// Note that we have a entry collapsed in the other (idx = 1) hists.
//
    if (idx == 0) {
    if (count_dummy != ARRAY_SIZE(fake_samples[1]) - 1) {
    pr_debug("Invalid count of dummy entries: %zd of %zd\n",
    count_dummy, ARRAY_SIZE(fake_samples[1]) - 1);
    return -1;
    }
    if (count != count_pair + ARRAY_SIZE(fake_samples[0])) {
    pr_debug("Invalid count of total leader entries: %zd of %zd\n",
    count, count_pair + ARRAY_SIZE(fake_samples[0]));
    return -1;
    }
    } else {
    if (count != count_pair) {
    pr_debug("Invalid count of total other entries: %zd of %zd\n",
    count, count_pair);
    return -1;
    }
    if (count_dummy > 0) {
    pr_debug("Other hists should not have dummy entries: %zd\n",
    count_dummy);
    return -1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn validate_link(leader: *mut hists, other: *mut hists) -> c_int {
    static int validate_link(struct hists *leader, struct hists *other)
    {
    return __validate_link(leader, 0) || __validate_link(other, 1);
    }
#[no_mangle]
unsafe extern "C" fn test__hists_link(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__hists_link(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut err: c_int = -1;
    struct hists *hists, *first_hists;
    let mut machines: machines = { 0 };
    struct machine *machine = core::ptr::null_mut();
    struct evsel *evsel, *first;
    struct evlist *evlist = evlist__new();
    if (evlist == core::ptr::null_mut())
    return -ENOMEM;
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
    hists = evsel__hists(evsel);
    hists__collapse_resort(hists, core::ptr::null_mut());
    if (verbose > 2)
    print_hists_in(hists);
    }
    first = evlist__first(evlist);
    evsel = evlist__last(evlist);
    first_hists = evsel__hists(first);
    hists = evsel__hists(evsel);
// match common entries
    hists__match(first_hists, hists);
    err = validate_match(first_hists, hists);
    if (err)
    goto out;
// link common and/or dummy entries
    hists__link(first_hists, hists);
    err = validate_link(first_hists, hists);
    if (err)
    goto out;
    err = 0;
    out:
// tear down everything
    evlist__put(evlist);
    reset_output_field();
    machines__exit(&machines);
    put_fake_samples();
    return err;
    }
    DEFINE_SUITE("Match and link multiple hists", hists_link);
