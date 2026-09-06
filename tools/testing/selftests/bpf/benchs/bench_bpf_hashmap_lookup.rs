//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/benchs/bench_bpf_hashmap_lookup.c
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
// Copyright (c) 2023 Isovalent

// BPF triggering benchmarks
    static struct ctx {
    struct bpf_hashmap_lookup *skel;
    } ctx;
// only available to kernel, so define it here

    static struct {
    __u32 key_size;
    __u32 map_flags;
    __u32 max_entries;
    __u32 nr_entries;
    __u32 nr_loops;
    } args = {
    .key_size = 4,
    .map_flags = 0,
    .max_entries = 1000,
    .nr_entries = 500,
    .nr_loops = 1000000,
    };
    enum {
    ARG_KEY_SIZE = 8001,
    ARG_MAP_FLAGS,
    ARG_MAX_ENTRIES,
    ARG_NR_ENTRIES,
    ARG_NR_LOOPS,
    };
    static const struct argp_option opts[] = {
    { "key_size", ARG_KEY_SIZE, "KEY_SIZE", 0,
    "The hashmap key size (max 1024)"},
    { "map_flags", ARG_MAP_FLAGS, "MAP_FLAGS", 0,
    "The hashmap flags passed to BPF_MAP_CREATE"},
    { "max_entries", ARG_MAX_ENTRIES, "MAX_ENTRIES", 0,
    "The hashmap max entries"},
    { "nr_entries", ARG_NR_ENTRIES, "NR_ENTRIES", 0,
    "The number of entries to insert/lookup"},
    { "nr_loops", ARG_NR_LOOPS, "NR_LOOPS", 0,
    "The number of loops for the benchmark"},
    {},
    };
#[no_mangle]
unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    static error_t parse_arg(int key, char *arg, struct argp_state *state)
    {
    long ret;
    switch (key) {
    case ARG_KEY_SIZE:
    ret = strtol(arg, core::ptr::null_mut(), 10);
    if (ret < 1 || ret > MAX_KEY_SIZE) {
    fprintf(stderr, "invalid key_size");
    argp_usage(state);
    }
    args.key_size = ret;
    break;
    case ARG_MAP_FLAGS:
    ret = strtol(arg, core::ptr::null_mut(), 0);
    if (ret < 0 || ret > UINT_MAX) {
    fprintf(stderr, "invalid map_flags");
    argp_usage(state);
    }
    args.map_flags = ret;
    break;
    case ARG_MAX_ENTRIES:
    ret = strtol(arg, core::ptr::null_mut(), 10);
    if (ret < 1 || ret > UINT_MAX) {
    fprintf(stderr, "invalid max_entries");
    argp_usage(state);
    }
    args.max_entries = ret;
    break;
    case ARG_NR_ENTRIES:
    ret = strtol(arg, core::ptr::null_mut(), 10);
    if (ret < 1 || ret > UINT_MAX) {
    fprintf(stderr, "invalid nr_entries");
    argp_usage(state);
    }
    args.nr_entries = ret;
    break;
    case ARG_NR_LOOPS:
    ret = strtol(arg, core::ptr::null_mut(), 10);
    if (ret < 1 || ret > BPF_MAX_LOOPS) {
    fprintf(stderr, "invalid nr_loops: %ld (min=1 max=%u)\n",
    ret, BPF_MAX_LOOPS);
    argp_usage(state);
    }
    args.nr_loops = ret;
    break;
    default:
    return ARGP_ERR_UNKNOWN;
    }
    return 0;
    }
    const struct argp bench_hashmap_lookup_argp = {
    .options = opts,
    .parser = parse_arg,
    };
#[no_mangle]
unsafe extern "C" fn validate() {
    static void validate(void)
    {
    if (env.consumer_cnt != 0) {
    fprintf(stderr, "benchmark doesn't support consumer!\n");
    exit(1);
    }
    if (args.nr_entries > args.max_entries) {
    fprintf(stderr, "args.nr_entries is too big! (max %u, got %u)\n",
    args.max_entries, args.nr_entries);
    exit(1);
    }
    }
    static void *producer(void *input)
    {
    while (true) {
// trigger the bpf program
    syscall(__NR_getpgid);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn measure(res: *mut bench_res) {
    static void measure(struct bench_res *res)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn patch_key(i: u32, key: *mut u32) {
    static inline void patch_key(u32 i, u32 *key)
    {

// key = i + 1;

// key = __builtin_bswap32(i + 1);

// the rest of key is random
    }
#[no_mangle]
unsafe extern "C" fn hashmap_lookup_setup(map_type: enum bpf_map_type) {
    static void hashmap_lookup_setup(enum bpf_map_type map_type)
    {
    struct bpf_link *link;
    __u32 map_flags;
    int map_fd;
    int ret;
    int i;
    setup_libbpf();
    ctx.skel = bpf_hashmap_lookup__open();
    if (!ctx.skel) {
    fprintf(stderr, "failed to open skeleton\n");
    exit(1);
    }
    map_flags = args.map_flags;
    if (map_type == BPF_MAP_TYPE_RHASH)
    map_flags |= BPF_F_NO_PREALLOC;
    bpf_map__set_type(ctx.skel.maps.hash_map_bench, map_type);
    bpf_map__set_max_entries(ctx.skel.maps.hash_map_bench, args.max_entries);
    bpf_map__set_key_size(ctx.skel.maps.hash_map_bench, args.key_size);
    bpf_map__set_value_size(ctx.skel.maps.hash_map_bench, 8);
    bpf_map__set_map_flags(ctx.skel.maps.hash_map_bench, map_flags);
    ctx.skel.bss.nr_entries = args.nr_entries;
    ctx.skel.bss.nr_loops = args.nr_loops / args.nr_entries;
    if (args.key_size > 4) {
    for (i = 1; i < args.key_size/4; i++)
    ctx.skel.bss.key[i] = 2654435761 * i;
    }
    ret = bpf_hashmap_lookup__load(ctx.skel);
    if (ret) {
    bpf_hashmap_lookup__destroy(ctx.skel);
    fprintf(stderr, "failed to load map: %s", strerror(-ret));
    exit(1);
    }
// fill in the hash_map
    map_fd = bpf_map__fd(ctx.skel.maps.hash_map_bench);
    for (u64 i = 0; i < args.nr_entries; i++) {
    patch_key(i, ctx.skel.bss.key);
    bpf_map_update_elem(map_fd, ctx.skel.bss.key, &i, BPF_ANY);
    }
    link = bpf_program__attach(ctx.skel.progs.benchmark);
    if (!link) {
    fprintf(stderr, "failed to attach program!\n");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn setup() {
    static void setup(void)
    {
    hashmap_lookup_setup(BPF_MAP_TYPE_HASH);
    }
#[no_mangle]
unsafe extern "C" fn rhash_setup() {
    static void rhash_setup(void)
    {
    hashmap_lookup_setup(BPF_MAP_TYPE_RHASH);
    }
#[no_mangle]
pub unsafe extern "C" fn events_from_time(time: u64) -> double {
    static inline double events_from_time(u64 time)
    {
    if (time)
    return args.nr_loops * 1000000000llu / time / 1000000.0L;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn compute_events(times: *mut u64, events_mean: *mut double, events_stddev: *mut double, mean_time: *mut u64) -> c_int {
    static int compute_events(u64 *times, double *events_mean, double *events_stddev, u64 *mean_time)
    {
    int i, n = 0;
// events_mean = 0;
// events_stddev = 0;
// mean_time = 0;
    for (i = 0; i < 32; i++) {
    if (!times[i])
    break;
// mean_time += times[i];
// events_mean += events_from_time(times[i]);
    n += 1;
    }
    if (!n)
    return 0;
// mean_time /= n;
// events_mean /= n;
    if (n > 1) {
    for (i = 0; i < n; i++) {
    let mut events_i: double = *events_mean - events_from_time(times[i]);
// events_stddev += events_i * events_i / (n - 1);
    }
// events_stddev = sqrt(*events_stddev);
    }
    return n;
    }
#[no_mangle]
unsafe extern "C" fn hashmap_report_final(res[]: bench_res, res_cnt: c_int) {
    static void hashmap_report_final(struct bench_res res[], int res_cnt)
    {
    let mut nr_cpus: c_uint = bpf_num_possible_cpus();
    double events_mean, events_stddev;
    u64 mean_time;
    int i, n;
    for (i = 0; i < nr_cpus; i++) {
    n = compute_events(ctx.skel.bss.percpu_times[i], &events_mean,
    &events_stddev, &mean_time);
    if (n == 0)
    continue;
    if (env.quiet) {
// we expect only one cpu to be present
    if (env.affinity)
    printf("%.3lf\n", events_mean);
    else
    printf("cpu%02d %.3lf\n", i, events_mean);
    } else {
    printf("cpu%02d: lookup %.3lfM ± %.3lfM events/sec"
    " (approximated from %d samples of ~%lums)\n",
    i, events_mean, 2*events_stddev,
    n, mean_time / 1000000);
    }
    }
    }
    const struct bench bench_bpf_hashmap_lookup = {
    .name = "bpf-hashmap-lookup",
    .argp = &bench_hashmap_lookup_argp,
    .validate = validate,
    .setup = setup,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = core::ptr::null_mut(),
    .report_final = hashmap_report_final,
    };
    const struct bench bench_bpf_rhashmap_lookup = {
    .name = "bpf-rhashmap-lookup",
    .argp = &bench_hashmap_lookup_argp,
    .validate = validate,
    .setup = rhash_setup,
    .producer_thread = producer,
    .measure = measure,
    .report_progress = core::ptr::null_mut(),
    .report_final = hashmap_report_final,
    };
