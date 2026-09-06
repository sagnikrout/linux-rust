//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf-trace-summary.c
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

    static struct syscall_summary_bpf *skel;
    let mut cgroups: static struct rb_root = RB_ROOT;
#[no_mangle]
pub unsafe extern "C" fn trace_prepare_bpf_summary(mode: enum trace_summary_mode) -> c_int {
    int trace_prepare_bpf_summary(enum trace_summary_mode mode)
    {
    skel = syscall_summary_bpf__open();
    if (skel == core::ptr::null_mut()) {
    fprintf(stderr, "failed to open syscall summary bpf skeleton\n");
    return -1;
    }
    if (mode == SUMMARY__BY_THREAD)
    skel.rodata.aggr_mode = SYSCALL_AGGR_THREAD;
#[no_mangle]
pub unsafe extern "C" fn if(SUMMARY__BY_CGROUP: mode ==) -> else {
    else if (mode == SUMMARY__BY_CGROUP)
    skel.rodata.aggr_mode = SYSCALL_AGGR_CGROUP;
    else
    skel.rodata.aggr_mode = SYSCALL_AGGR_CPU;
    if (cgroup_is_v2("perf_event") > 0)
    skel.rodata.use_cgroup_v2 = 1;
    if (syscall_summary_bpf__load(skel) < 0) {
    fprintf(stderr, "failed to load syscall summary bpf skeleton\n");
    return -1;
    }
    if (syscall_summary_bpf__attach(skel) < 0) {
    fprintf(stderr, "failed to attach syscall summary bpf skeleton\n");
    return -1;
    }
    if (mode == SUMMARY__BY_CGROUP)
    read_all_cgroups(&cgroups);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_start_bpf_summary() {
    void trace_start_bpf_summary(void)
    {
    skel.bss.enabled = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_end_bpf_summary() {
    void trace_end_bpf_summary(void)
    {
    skel.bss.enabled = 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_node {
    pub syscall_nr: c_int,
    pub stats: syscall_stats,
}

#[no_mangle]
unsafe extern "C" fn rel_stddev(stat: *mut syscall_stats) -> double {
    static double rel_stddev(struct syscall_stats *stat)
    {
    double variance, average;
    if (stat.count < 2)
    return 0;
    average = (double)stat.total_time / stat.count;
    variance = stat.squared_sum;
    variance -= (stat.total_time * stat.total_time) / stat.count;
    variance /= stat.count - 1;
    return 100 * sqrt(variance / stat.count) / average;
    }
//
// The syscall_data is to maintain syscall stats ordered by total time.
// It supports different summary modes like per-thread or global.
//
// For per-thread stats, it uses two-level data strurcture -
// syscall_data is keyed by TID and has an array of nodes which
// represents each syscall for the thread.
//
// For global stats, it's still two-level technically but we don't need
// per-cpu analysis so it's keyed by the syscall number to combine stats
// from different CPUs.  And syscall_data always has a syscall_node so
// it can effectively work as flat hierarchy.
//
// For per-cgroup stats, it uses two-level data structure like thread
// syscall_data is keyed by CGROUP and has an array of node which
// represents each syscall for the cgroup.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_data {
    pub /: *mut *mut u64 key; / tid if AGGR_THREAD, syscall-nr if AGGR_CPU, cgroup if AGGR_CGROUP,
    pub nr_events: c_int,
    pub nr_nodes: c_int,
    pub total_time: u64,
    pub nodes: *mut syscall_node,
}

#[no_mangle]
unsafe extern "C" fn datacmp(a: *const c_void, b: *const c_void) -> c_int {
    static int datacmp(const void *a, const void *b)
    {
    const struct syscall_data * const *sa = a;
    const struct syscall_data * const *sb = b;
    return (*sa).total_time > (*sb).total_time ? -1 : 1;
    }
#[no_mangle]
unsafe extern "C" fn nodecmp(a: *const c_void, b: *const c_void) -> c_int {
    static int nodecmp(const void *a, const void *b)
    {
    const struct syscall_node *na = a;
    const struct syscall_node *nb = b;
    return na.stats.total_time > nb.stats.total_time ? -1 : 1;
    }
#[no_mangle]
unsafe extern "C" fn sc_node_hash(key: c_long, __maybe_unused: *mut *mut void ctx) -> usize {
    static size_t sc_node_hash(long key, void *ctx __maybe_unused)
    {
    return key;
    }
#[no_mangle]
unsafe extern "C" fn sc_node_equal(key1: c_long, key2: c_long, __maybe_unused: *mut *mut void ctx) -> bool {
    static bool sc_node_equal(long key1, long key2, void *ctx __maybe_unused)
    {
    let mut key1: return = = key2;
    }
#[no_mangle]
unsafe extern "C" fn print_common_stats(data: *mut syscall_data, max_summary: c_int, fp: *mut FILE) -> c_int {
    static int print_common_stats(struct syscall_data *data, int max_summary, FILE *fp)
    {
    let mut printed: c_int = 0;
    if (max_summary == 0 || max_summary > data.nr_nodes)
    max_summary = data.nr_nodes;
    for (int i = 0; i < max_summary; i++) {
    struct syscall_node *node = &data.nodes[i];
    struct syscall_stats *stat = &node.stats;
    let mut total: double = (double)(stat.total_time) / NSEC_PER_MSEC;
    let mut min: double = (double)(stat.min_time) / NSEC_PER_MSEC;
    let mut max: double = (double)(stat.max_time) / NSEC_PER_MSEC;
    let mut avg: double = total / stat.count;
    const char *name;
// TODO: support other ABIs
    name = syscalltbl__name(EM_HOST, node.syscall_nr);
    if (name)
    printed += fprintf(fp, "   %-15s", name);
    else
    printed += fprintf(fp, "   syscall:%-7d", node.syscall_nr);
    printed += fprintf(fp, " %8u %6u %9.3f %9.3f %9.3f %9.3f %9.2f%%\n",
    stat.count, stat.error, total, min, avg, max,
    rel_stddev(stat));
    }
    return printed;
    }
    static int update_thread_stats(struct hashmap *hash, struct syscall_key *map_key,
    struct syscall_stats *map_data)
    {
    struct syscall_data *data;
    struct syscall_node *nodes;
    if (!hashmap__find(hash, map_key.cpu_or_tid, &data)) {
    data = zalloc(sizeof(*data));
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    data.key = map_key.cpu_or_tid;
    if (hashmap__add(hash, data.key, data) < 0) {
    free(data);
    return -ENOMEM;
    }
    }
// update thread total stats
    data.nr_events += map_data.count;
    data.total_time += map_data.total_time;
    nodes = reallocarray(data.nodes, data.nr_nodes + 1, sizeof(*nodes));
    if (nodes == core::ptr::null_mut())
    return -ENOMEM;
    data.nodes = nodes;
    nodes = &data.nodes[data.nr_nodes++];
    nodes.syscall_nr = map_key.nr;
// each thread has an entry for each syscall, just use the stat
    memcpy(&nodes.stats, map_data, sizeof(*map_data));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn print_thread_stat(data: *mut syscall_data, max_summary: c_int, fp: *mut FILE) -> c_int {
    static int print_thread_stat(struct syscall_data *data, int max_summary, FILE *fp)
    {
    let mut printed: c_int = 0;
    qsort(data.nodes, data.nr_nodes, sizeof(*data.nodes), nodecmp);
    printed += fprintf(fp, " thread (%d), ", (int)data.key);
    printed += fprintf(fp, "%d events\n\n", data.nr_events);
    printed += fprintf(fp, "   syscall            calls  errors  total       min       avg       max       stddev\n");
    printed += fprintf(fp, "                                     (msec)    (msec)    (msec)    (msec)        (%%)\n");
    printed += fprintf(fp, "   --------------- --------  ------ -------- --------- --------- ---------     ------\n");
    printed += print_common_stats(data, max_summary, fp);
    printed += fprintf(fp, "\n\n");
    return printed;
    }
#[no_mangle]
unsafe extern "C" fn print_thread_stats(data: *mut syscall_data, nr_data: c_int, max_summary: c_int, fp: *mut FILE) -> c_int {
    static int print_thread_stats(struct syscall_data **data, int nr_data, int max_summary, FILE *fp)
    {
    let mut printed: c_int = 0;
    for (int i = 0; i < nr_data; i++)
    printed += print_thread_stat(data[i], max_summary, fp);
    return printed;
    }
    static int update_total_stats(struct hashmap *hash, struct syscall_key *map_key,
    struct syscall_stats *map_data)
    {
    struct syscall_data *data;
    struct syscall_stats *stat;
    if (!hashmap__find(hash, map_key.nr, &data)) {
    data = zalloc(sizeof(*data));
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    data.nodes = zalloc(sizeof(*data.nodes));
    if (data.nodes == core::ptr::null_mut()) {
    free(data);
    return -ENOMEM;
    }
    data.nr_nodes = 1;
    data.key = map_key.nr;
    data.nodes.syscall_nr = data.key;
    if (hashmap__add(hash, data.key, data) < 0) {
    free(data.nodes);
    free(data);
    return -ENOMEM;
    }
    }
// update total stats for this syscall
    data.nr_events += map_data.count;
    data.total_time += map_data.total_time;
// This is sum of the same syscall from different CPUs
    stat = &data.nodes.stats;
    stat.total_time += map_data.total_time;
    stat.squared_sum += map_data.squared_sum;
    stat.count += map_data.count;
    stat.error += map_data.error;
    if (stat.max_time < map_data.max_time)
    stat.max_time = map_data.max_time;
    if (stat.min_time > map_data.min_time || stat.min_time == 0)
    stat.min_time = map_data.min_time;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn print_total_stats(data: *mut syscall_data, nr_data: c_int, max_summary: c_int, fp: *mut FILE) -> c_int {
    static int print_total_stats(struct syscall_data **data, int nr_data, int max_summary, FILE *fp)
    {
    let mut printed: c_int = 0;
    let mut nr_events: c_int = 0;
    for (int i = 0; i < nr_data; i++)
    nr_events += data[i].nr_events;
    printed += fprintf(fp, " total, %d events\n\n", nr_events);
    printed += fprintf(fp, "   syscall            calls  errors  total       min       avg       max       stddev\n");
    printed += fprintf(fp, "                                     (msec)    (msec)    (msec)    (msec)        (%%)\n");
    printed += fprintf(fp, "   --------------- --------  ------ -------- --------- --------- ---------     ------\n");
    if (max_summary == 0 || max_summary > nr_data)
    max_summary = nr_data;
    for (int i = 0; i < max_summary; i++)
    printed += print_common_stats(data[i], max_summary, fp);
    printed += fprintf(fp, "\n\n");
    return printed;
    }
    static int update_cgroup_stats(struct hashmap *hash, struct syscall_key *map_key,
    struct syscall_stats *map_data)
    {
    struct syscall_data *data;
    struct syscall_node *nodes;
    if (!hashmap__find(hash, map_key.cgroup, &data)) {
    data = zalloc(sizeof(*data));
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    data.key = map_key.cgroup;
    if (hashmap__add(hash, data.key, data) < 0) {
    free(data);
    return -ENOMEM;
    }
    }
// update thread total stats
    data.nr_events += map_data.count;
    data.total_time += map_data.total_time;
    nodes = reallocarray(data.nodes, data.nr_nodes + 1, sizeof(*nodes));
    if (nodes == core::ptr::null_mut())
    return -ENOMEM;
    data.nodes = nodes;
    nodes = &data.nodes[data.nr_nodes++];
    nodes.syscall_nr = map_key.nr;
// each thread has an entry for each syscall, just use the stat
    memcpy(&nodes.stats, map_data, sizeof(*map_data));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn print_cgroup_stat(data: *mut syscall_data, max_summary: c_int, fp: *mut FILE) -> c_int {
    static int print_cgroup_stat(struct syscall_data *data, int max_summary, FILE *fp)
    {
    let mut printed: c_int = 0;
    struct cgroup *cgrp = __cgroup__find(&cgroups, data.key);
    qsort(data.nodes, data.nr_nodes, sizeof(*data.nodes), nodecmp);
    if (cgrp)
    printed += fprintf(fp, " cgroup %s,", cgrp.name);
    else
    printed += fprintf(fp, " cgroup id:%lu,", (unsigned long)data.key);
    printed += fprintf(fp, " %d events\n\n", data.nr_events);
    printed += fprintf(fp, "   syscall            calls  errors  total       min       avg       max       stddev\n");
    printed += fprintf(fp, "                                     (msec)    (msec)    (msec)    (msec)        (%%)\n");
    printed += fprintf(fp, "   --------------- --------  ------ -------- --------- --------- ---------     ------\n");
    printed += print_common_stats(data, max_summary, fp);
    printed += fprintf(fp, "\n\n");
    return printed;
    }
#[no_mangle]
unsafe extern "C" fn print_cgroup_stats(data: *mut syscall_data, nr_data: c_int, max_summary: c_int, fp: *mut FILE) -> c_int {
    static int print_cgroup_stats(struct syscall_data **data, int nr_data, int max_summary, FILE *fp)
    {
    let mut printed: c_int = 0;
    for (int i = 0; i < nr_data; i++)
    printed += print_cgroup_stat(data[i], max_summary, fp);
    return printed;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_print_bpf_summary(fp: *mut FILE, max_summary: c_int) -> c_int {
    int trace_print_bpf_summary(FILE *fp, int max_summary)
    {
    struct bpf_map *map = skel.maps.syscall_stats_map;
    struct syscall_key *prev_key, key;
    struct syscall_data **data = core::ptr::null_mut();
    struct hashmap schash;
    struct hashmap_entry *entry;
    let mut nr_data: c_int = 0;
    let mut printed: c_int = 0;
    int i;
    size_t bkt;
    hashmap__init(&schash, sc_node_hash, sc_node_equal, /*ctx=*/core::ptr::null_mut());
    printed = fprintf(fp, "\n Summary of events:\n\n");
// get stats from the bpf map
    prev_key = core::ptr::null_mut();
    while (!bpf_map__get_next_key(map, prev_key, &key, sizeof(key))) {
    struct syscall_stats stat;
    if (!bpf_map__lookup_elem(map, &key, sizeof(key), &stat, sizeof(stat), 0)) {
    switch (skel.rodata.aggr_mode) {
    case SYSCALL_AGGR_THREAD:
    update_thread_stats(&schash, &key, &stat);
    break;
    case SYSCALL_AGGR_CPU:
    update_total_stats(&schash, &key, &stat);
    break;
    case SYSCALL_AGGR_CGROUP:
    update_cgroup_stats(&schash, &key, &stat);
    break;
    default:
    break;
    }
    }
    prev_key = &key;
    }
    nr_data = hashmap__size(&schash);
    data = calloc(nr_data, sizeof(*data));
    if (data == core::ptr::null_mut())
    goto out;
    i = 0;
    hashmap__for_each_entry(&schash, entry, bkt)
    data[i++] = entry.pvalue;
    qsort(data, nr_data, sizeof(*data), datacmp);
    switch (skel.rodata.aggr_mode) {
    case SYSCALL_AGGR_THREAD:
    printed += print_thread_stats(data, nr_data, max_summary, fp);
    break;
    case SYSCALL_AGGR_CPU:
    printed += print_total_stats(data, nr_data, max_summary, fp);
    break;
    case SYSCALL_AGGR_CGROUP:
    printed += print_cgroup_stats(data, nr_data, max_summary, fp);
    break;
    default:
    break;
    }
    for (i = 0; i < nr_data && data; i++) {
    free(data[i].nodes);
    free(data[i]);
    }
    free(data);
    out:
    hashmap__clear(&schash);
    return printed;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_cleanup_bpf_summary() {
    void trace_cleanup_bpf_summary(void)
    {
    if (!RB_EMPTY_ROOT(&cgroups)) {
    struct cgroup *cgrp, *tmp;
    rbtree_postorder_for_each_entry_safe(cgrp, tmp, &cgroups, node)
    cgroup__put(cgrp);
    cgroups = RB_ROOT;
    }
    syscall_summary_bpf__destroy(skel);
    }
