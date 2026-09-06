//! Automatically rewritten from C to Rust
//! Source: tools/sched_ext/scx_simple.c
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
//
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2022 Tejun Heo <tj@kernel.org>
// Copyright (c) 2022 David Vernet <dvernet@meta.com>
//

    const char help_fmt[] =
    "A simple sched_ext scheduler.\n"
    "\n"
    "See the top-level comment in .bpf.c for more details.\n"
    "\n"
    "Usage: %s [-f] [-v]\n"
    "\n"
    "  -f            Use FIFO scheduling instead of weighted vtime scheduling\n"
    "  -v            Print libbpf debug messages\n"
    "  -h            Display this help and exit\n";
    static bool verbose;
    static volatile int exit_req;
#[no_mangle]
unsafe extern "C" fn libbpf_print_fn(level: enum libbpf_print_level, format: *const c_char, args: va_list) -> c_int {
    static int libbpf_print_fn(enum libbpf_print_level level, const char *format, va_list args)
    {
    if (level == LIBBPF_DEBUG && !verbose)
    return 0;
    return vfprintf(stderr, format, args);
    }
#[no_mangle]
unsafe extern "C" fn sigint_handler(simple: c_int) {
    static void sigint_handler(int simple)
    {
    exit_req = 1;
    }
#[no_mangle]
unsafe extern "C" fn read_stats(skel: *mut scx_simple, stats: *mut __u64) {
    static void read_stats(struct scx_simple *skel, __u64 *stats)
    {
    let mut nr_cpus: c_int = libbpf_num_possible_cpus();
    assert(nr_cpus > 0);
    __u64 cnts[2][nr_cpus];
    __u32 idx;
    memset(stats, 0, sizeof(stats[0]) * 2);
    for (idx = 0; idx < 2; idx++) {
    int ret, cpu;
    ret = bpf_map_lookup_elem(bpf_map__fd(skel.maps.stats),
    &idx, cnts[idx]);
    if (ret < 0)
    continue;
    for (cpu = 0; cpu < nr_cpus; cpu++)
    stats[idx] += cnts[idx][cpu];
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct scx_simple *skel;
    struct bpf_link *link;
    __s32 opt;
    __u64 ecode;
    libbpf_set_print(libbpf_print_fn);
    signal(SIGINT, sigint_handler);
    signal(SIGTERM, sigint_handler);
    restart:
    optind = 1;
    skel = SCX_OPS_OPEN(simple_ops, scx_simple);
    while ((opt = getopt(argc, argv, "fvh")) != -1) {
    switch (opt) {
    case 'f':
    skel.rodata.fifo_sched = true;
    break;
    case 'v':
    verbose = true;
    break;
    default:
    fprintf(stderr, help_fmt, basename(argv[0]));
    return opt != 'h';
    }
    }
    SCX_OPS_LOAD(skel, simple_ops, scx_simple, uei);
    link = SCX_OPS_ATTACH(skel, simple_ops, scx_simple);
    while (!exit_req && !UEI_EXITED(skel, uei)) {
    __u64 stats[2];
    read_stats(skel, stats);
    printf("local=%llu global=%llu\n", stats[0], stats[1]);
    fflush(stdout);
    sleep(1);
    }
    bpf_link__destroy(link);
    ecode = UEI_REPORT(skel, uei);
    scx_simple__destroy(skel);
    if (!exit_req && UEI_ECODE_RESTART(ecode))
    goto restart;
    return 0;
    }
