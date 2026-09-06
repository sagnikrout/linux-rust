//! Automatically rewritten from C to Rust
//! Source: tools/sched_ext/scx_central.c
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
// Macro flag: #define _GNU_SOURCE

    const char help_fmt[] =
    "A central FIFO sched_ext scheduler.\n"
    "\n"
    "See the top-level comment in .bpf.c for more details.\n"
    "\n"
    "Usage: %s [-s SLICE_US] [-c CPU] [-v]\n"
    "\n"
    "  -s SLICE_US   Override slice duration\n"
    "  -c CPU        Override the central CPU (default: 0)\n"
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
unsafe extern "C" fn sigint_handler(dummy: c_int) {
    static void sigint_handler(int dummy)
    {
    exit_req = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct scx_central *skel;
    struct bpf_link *link;
    let mut seq: __u64 = 0, ecode;
    __s32 opt;
    libbpf_set_print(libbpf_print_fn);
    signal(SIGINT, sigint_handler);
    signal(SIGTERM, sigint_handler);
    restart:
    optind = 1;
    skel = SCX_OPS_OPEN(central_ops, scx_central);
    skel.rodata.central_cpu = 0;
    skel.rodata.nr_cpu_ids = libbpf_num_possible_cpus();
    skel.rodata.slice_ns = __COMPAT_ENUM_OR_ZERO("scx_public_consts", "SCX_SLICE_DFL");
    assert(skel.rodata.nr_cpu_ids > 0);
    assert(skel.rodata.nr_cpu_ids <= INT32_MAX);
    while ((opt = getopt(argc, argv, "s:c:vh")) != -1) {
    switch (opt) {
    case 's':
    skel.rodata.slice_ns = strtoull(optarg, core::ptr::null_mut(), 0) * 1000;
    break;
    case 'c': {
    let mut central_cpu: u32 = strtoul(optarg, core::ptr::null_mut(), 0);
    if (central_cpu >= skel.rodata.nr_cpu_ids) {
    fprintf(stderr, "invalid central CPU id value, %u given (%u max)\n", central_cpu, skel.rodata.nr_cpu_ids);
    scx_central__destroy(skel);
    return -1;
    }
    skel.rodata.central_cpu = (s32)central_cpu;
    break;
    }
    case 'v':
    verbose = true;
    break;
    default:
    fprintf(stderr, help_fmt, basename(argv[0]));
    return opt != 'h';
    }
    }
// Resize arrays so their element count is equal to cpu count.
    RESIZE_ARRAY(skel, data, cpu_gimme_task, skel.rodata.nr_cpu_ids);
    RESIZE_ARRAY(skel, data, cpu_started_at, skel.rodata.nr_cpu_ids);
    SCX_OPS_LOAD(skel, central_ops, scx_central, uei);
    link = SCX_OPS_ATTACH(skel, central_ops, scx_central);
    if (!skel.data.timer_pinned)
    printf("WARNING : BPF_F_TIMER_CPU_PIN not available, timer not pinned to central\n");
    while (!exit_req && !UEI_EXITED(skel, uei)) {
    printf("[SEQ %llu]\n", seq++);
    printf("total   :%10" PRIu64 "    local:%10" PRIu64 "   queued:%10" PRIu64 "  lost:%10" PRIu64 "\n",
    skel.bss.nr_total,
    skel.bss.nr_locals,
    skel.bss.nr_queued,
    skel.bss.nr_lost_pids);
    printf("timer   :%10" PRIu64 " dispatch:%10" PRIu64 " mismatch:%10" PRIu64 " retry:%10" PRIu64 "\n",
    skel.bss.nr_timers,
    skel.bss.nr_dispatches,
    skel.bss.nr_mismatches,
    skel.bss.nr_retries);
    printf("overflow:%10" PRIu64 "\n",
    skel.bss.nr_overflows);
    fflush(stdout);
    sleep(1);
    }
    bpf_link__destroy(link);
    ecode = UEI_REPORT(skel, uei);
    scx_central__destroy(skel);
    if (!exit_req && UEI_ECODE_RESTART(ecode))
    goto restart;
    return 0;
    }
