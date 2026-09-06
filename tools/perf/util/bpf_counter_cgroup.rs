//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_counter_cgroup.c
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
// Copyright (c) 2021 Facebook
// Copyright (c) 2021 Google

    static struct perf_event_attr cgrp_switch_attr = {
    .type = PERF_TYPE_SOFTWARE,
    .config = PERF_COUNT_SW_CGROUP_SWITCHES,
    .size = sizeof(cgrp_switch_attr),
    .sample_period = 1,
    .disabled = 1,
    };
    static struct evsel *cgrp_switch;
    static struct bperf_cgroup_bpf *skel;

#[no_mangle]
unsafe extern "C" fn setup_rodata(sk: *mut bperf_cgroup_bpf, evlist_size: c_int) {
    static void setup_rodata(struct bperf_cgroup_bpf *sk, int evlist_size)
    {
    int map_size, total_cpus = cpu__max_cpu().cpu;
    sk.rodata.num_cpus = total_cpus;
    sk.rodata.num_events = evlist_size / nr_cgroups;
    if (cgroup_is_v2("perf_event") > 0)
    sk.rodata.use_cgroup_v2 = 1;
    BUG_ON(evlist_size % nr_cgroups != 0);
// we need one copy of events per cpu for reading
    map_size = total_cpus * evlist_size / nr_cgroups;
    bpf_map__set_max_entries(sk.maps.events, map_size);
    bpf_map__set_max_entries(sk.maps.cgrp_idx, nr_cgroups);
// previous result is saved in a per-cpu array
    map_size = evlist_size / nr_cgroups;
    bpf_map__set_max_entries(sk.maps.prev_readings, map_size);
// cgroup result needs all events (per-cpu)
    map_size = evlist_size;
    bpf_map__set_max_entries(sk.maps.cgrp_readings, map_size);
    }
#[no_mangle]
unsafe extern "C" fn test_max_events_program_load() {
    static void test_max_events_program_load(void)
    {

//
// Test that the program verifies with the maximum number of events. If
// this test fails unfortunately perf needs recompiling with a lower
// BPERF_CGROUP__MAX_EVENTS to avoid BPF verifier issues.
//
    int err, max_events = BPERF_CGROUP__MAX_EVENTS * nr_cgroups;
    struct bperf_cgroup_bpf *test_skel = bperf_cgroup_bpf__open();
    if (!test_skel) {
    pr_err("Failed to open cgroup skeleton\n");
    return;
    }
    setup_rodata(test_skel, max_events);
    err = bperf_cgroup_bpf__load(test_skel);
    if (err) {
    pr_err("Failed to load cgroup skeleton with max events %d.\n",
    BPERF_CGROUP__MAX_EVENTS);
    }
    bperf_cgroup_bpf__destroy(test_skel);

    }
#[no_mangle]
unsafe extern "C" fn bperf_load_program(evlist: *mut evlist) -> c_int {
    static int bperf_load_program(struct evlist *evlist)
    {
    struct bpf_link *link;
    struct evsel *evsel;
    struct cgroup *cgrp, *leader_cgrp;
    unsigned int i;
    struct perf_cpu cpu;
    let mut total_cpus: c_int = cpu__max_cpu().cpu;
    int map_fd, prog_fd, err;
    set_max_rlimit();
    if (nr_cgroups == 0 || evlist__nr_entries(evlist) % nr_cgroups != 0) {
    pr_err("Invalid cgroup or event count\n");
    return -EINVAL;
    }
    test_max_events_program_load();
    skel = bperf_cgroup_bpf__open();
    if (!skel) {
    pr_err("Failed to open cgroup skeleton\n");
    return -1;
    }
    setup_rodata(skel, evlist__nr_entries(evlist));
    err = bperf_cgroup_bpf__load(skel);
    if (err) {
    pr_err("Failed to load cgroup skeleton\n");
    goto out;
    }
    err = -1;
    cgrp_switch = evsel__new(&cgrp_switch_attr);
    if (evsel__open_per_cpu(cgrp_switch, evlist__core(evlist).all_cpus, -1) < 0) {
    pr_err("Failed to open cgroup switches event\n");
    goto out;
    }
    perf_cpu_map__for_each_cpu(cpu, i, evlist__core(evlist).all_cpus) {
    link = bpf_program__attach_perf_event(skel.progs.on_cgrp_switch,
    FD(cgrp_switch, i));
    if (IS_ERR(link)) {
    pr_err("Failed to attach cgroup program\n");
    err = PTR_ERR(link);
    goto out;
    }
    }
//
// Update cgrp_idx map from cgroup-id to event index.
//
    cgrp = core::ptr::null_mut();
    i = 0;
    evlist__for_each_entry(evlist, evsel) {
    if (cgrp == core::ptr::null_mut() || evsel.cgrp == leader_cgrp) {
    unsigned int j;
    leader_cgrp = evsel.cgrp;
    evsel.cgrp = core::ptr::null_mut();
// open single copy of the events w/o cgroup
    err = evsel__open_per_cpu(evsel, evsel.core.cpus, -1);
    if (err == 0)
    evsel.supported = true;
    map_fd = bpf_map__fd(skel.maps.events);
    perf_cpu_map__for_each_cpu(cpu, j, evsel.core.cpus) {
    let mut fd: c_int = FD(evsel, j);
    let mut idx: __u32 = evsel.core.idx * total_cpus + cpu.cpu;
    bpf_map_update_elem(map_fd, &idx, &fd, BPF_ANY);
    }
    evsel.cgrp = leader_cgrp;
    }
    if (evsel.cgrp == cgrp)
    continue;
    cgrp = evsel.cgrp;
    if (read_cgroup_id(cgrp) < 0) {
    pr_debug("Failed to get cgroup id for %s\n", cgrp.name);
    cgrp.id = 0;
    }
    map_fd = bpf_map__fd(skel.maps.cgrp_idx);
    err = bpf_map_update_elem(map_fd, &cgrp.id, &i, BPF_ANY);
    if (err < 0) {
    pr_err("Failed to update cgroup index map\n");
    goto out;
    }
    i++;
    }
//
// Propagate supported flag from leaders to followers. Follower events
// are not opened, so their supported flag remains false.
//
    {
    struct evsel *leader;
    let mut num_events: c_int = evlist__nr_entries(evlist) / nr_cgroups;
    evlist__for_each_entry(evlist, evsel) {
    leader = evlist__find_evsel(evlist, evsel.core.idx % num_events);
    if (leader)
    evsel.supported = leader.supported;
    }
    }
//
// bperf uses BPF_PROG_TEST_RUN to get accurate reading. Check
// whether the kernel support it
//
    prog_fd = bpf_program__fd(skel.progs.trigger_read);
    err = bperf_trigger_reading(prog_fd, 0);
    if (err) {
    pr_warning("The kernel does not support test_run for raw_tp BPF programs.\n"
    "Therefore, --for-each-cgroup might show inaccurate readings\n");
    err = 0;
    }
    out:
    return err;
    }
    static int bperf_cgrp__load(struct evsel *evsel,
    struct target *target __maybe_unused)
    {
    let mut bperf_loaded: static bool = false;
    evsel.bperf_leader_prog_fd = -1;
    evsel.bperf_leader_link_fd = -1;
    if (!bperf_loaded && bperf_load_program(evsel.evlist))
    return -1;
    bperf_loaded = true;
// just to bypass bpf_counter_skip()
    evsel.follower_skel = (struct bperf_follower_bpf *)skel;
    return 0;
    }
    static int bperf_cgrp__install_pe(struct evsel *evsel __maybe_unused,
    int cpu_map_idx __maybe_unused,
    int fd __maybe_unused)
    {
// nothing to do
    return 0;
    }
//
// trigger the leader prog on each cpu, so the cgrp_reading map could get
// the latest results.
//
#[no_mangle]
unsafe extern "C" fn bperf_cgrp__sync_counters(evlist: *mut evlist) -> c_int {
    static int bperf_cgrp__sync_counters(struct evlist *evlist)
    {
    struct perf_cpu cpu;
    unsigned int idx;
    let mut prog_fd: c_int = bpf_program__fd(skel.progs.trigger_read);
    perf_cpu_map__for_each_cpu(cpu, idx, evlist__core(evlist).all_cpus)
    bperf_trigger_reading(prog_fd, cpu.cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bperf_cgrp__enable(evsel: *mut evsel) -> c_int {
    static int bperf_cgrp__enable(struct evsel *evsel)
    {
    if (evsel.core.idx)
    return 0;
    bperf_cgrp__sync_counters(evsel.evlist);
    skel.bss.enabled = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bperf_cgrp__disable(evsel: *mut evsel) -> c_int {
    static int bperf_cgrp__disable(struct evsel *evsel)
    {
    if (evsel.core.idx)
    return 0;
    bperf_cgrp__sync_counters(evsel.evlist);
    skel.bss.enabled = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bperf_cgrp__read(evsel: *mut evsel) -> c_int {
    static int bperf_cgrp__read(struct evsel *evsel)
    {
    struct evlist *evlist = evsel.evlist;
    let mut total_cpus: c_int = cpu__max_cpu().cpu;
    struct perf_counts_values *counts;
    struct bpf_perf_event_value *values;
    int reading_map_fd, err = 0;
    if (evsel.core.idx)
    return 0;
    bperf_cgrp__sync_counters(evsel.evlist);
    values = calloc(total_cpus, sizeof(*values));
    if (values == core::ptr::null_mut())
    return -ENOMEM;
    reading_map_fd = bpf_map__fd(skel.maps.cgrp_readings);
    evlist__for_each_entry(evlist, evsel) {
    let mut idx: __u32 = evsel.core.idx;
    unsigned int i;
    struct perf_cpu cpu;
    err = bpf_map_lookup_elem(reading_map_fd, &idx, values);
    if (err) {
    pr_err("bpf map lookup failed: idx=%u, event=%s, cgrp=%s\n",
    idx, evsel__name(evsel), evsel.cgrp.name);
    goto out;
    }
    perf_cpu_map__for_each_cpu(cpu, i, evsel.core.cpus) {
    counts = perf_counts(evsel.counts, i, 0);
    counts.val = values[cpu.cpu].counter;
    counts.ena = values[cpu.cpu].enabled;
    counts.run = values[cpu.cpu].running;
    }
    }
    out:
    free(values);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bperf_cgrp__destroy(evsel: *mut evsel) -> c_int {
    static int bperf_cgrp__destroy(struct evsel *evsel)
    {
    if (evsel.core.idx)
    return 0;
    bperf_cgroup_bpf__destroy(skel);
    evsel__put(cgrp_switch);  // it'll destroy on_switch progs too
    return 0;
    }
    struct bpf_counter_ops bperf_cgrp_ops = {
    .load       = bperf_cgrp__load,
    .enable     = bperf_cgrp__enable,
    .disable    = bperf_cgrp__disable,
    .read       = bperf_cgrp__read,
    .install_pe = bperf_cgrp__install_pe,
    .destroy    = bperf_cgrp__destroy,
    };
