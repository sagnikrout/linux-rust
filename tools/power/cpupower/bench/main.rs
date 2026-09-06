//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/bench/main.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// cpufreq-bench CPUFreq microbenchmark
//
// Copyright (C) 2008 Christian Kornacker <ckornacker@suse.de>
//

    static struct option long_options[] = {
    {"output",	1,	0,	'o'},
    {"sleep",	1,	0,	's'},
    {"load",	1,	0,	'l'},
    {"verbose",	0,	0,	'v'},
    {"cpu",		1,	0,	'c'},
    {"governor",	1,	0,	'g'},
    {"prio",	1,	0,	'p'},
    {"file",	1,	0,	'f'},
    {"cycles",	1,	0,	'n'},
    {"rounds",	1,	0,	'r'},
    {"load-step",	1,	0,	'x'},
    {"sleep-step",	1,	0,	'y'},
    {"help",	0,	0,	'h'},
    {0, 0, 0, 0}
    };
//
    usage
//
#[no_mangle]
pub unsafe extern "C" fn usage() {
    void usage()
    {
    printf("usage: ./bench\n");
    printf("Options:\n");
    printf(" -l, --load=<long int>\t\tinitial load time in us\n");
    printf(" -s, --sleep=<long int>\t\tinitial sleep time in us\n");
    printf(" -x, --load-step=<long int>\ttime to be added to load time, in us\n");
    printf(" -y, --sleep-step=<long int>\ttime to be added to sleep time, in us\n");
    printf(" -c, --cpu=<cpu #>\t\t\tCPU Nr. to use, starting at 0\n");
    printf(" -p, --prio=<priority>\t\t\tscheduler priority, HIGH, LOW or DEFAULT\n");
    printf(" -g, --governor=<governor>\t\tcpufreq governor to test\n");
    printf(" -n, --cycles=<int>\t\t\tload/sleep cycles\n");
    printf(" -r, --rounds<int>\t\t\tload/sleep rounds\n");
    printf(" -f, --file=<configfile>\t\tconfig file to use\n");
    printf(" -o, --output=<dir>\t\t\toutput path. Filename will be OUTPUTPATH/benchmark_TIMESTAMP.log\n");
    printf(" -v, --verbose\t\t\t\tverbose output on/off\n");
    printf(" -h, --help\t\t\t\tPrint this help screen\n");
    exit(1);
    }
//
    main
//
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int c;
    let mut option_index: c_int = 0;
    struct config *config = core::ptr::null_mut();
    config = prepare_default_config();
    if (config == core::ptr::null_mut())
    return EXIT_FAILURE;
    while (1) {
    c = getopt_long (argc, argv, "hg:o:s:l:vc:p:f:n:r:x:y:",
    long_options, &option_index);
    if (c == -1)
    break;
    switch (c) {
    case 'o':
    if (config.output != core::ptr::null_mut())
    fclose(config.output);
    config.output = prepare_output(optarg);
    if (config.output == core::ptr::null_mut())
    return EXIT_FAILURE;
    dprintf("user output path . %s\n", optarg);
    break;
    case 's':
    sscanf(optarg, "%li", &config.sleep);
    dprintf("user sleep time . %s\n", optarg);
    break;
    case 'l':
    sscanf(optarg, "%li", &config.load);
    dprintf("user load time . %s\n", optarg);
    break;
    case 'c':
    sscanf(optarg, "%u", &config.cpu);
    dprintf("user cpu . %s\n", optarg);
    break;
    case 'g':
    strncpy(config.governor, optarg, 14);
    dprintf("user governor . %s\n", optarg);
    break;
    case 'p':
    if (string_to_prio(optarg) != SCHED_ERR) {
    config.prio = string_to_prio(optarg);
    dprintf("user prio . %s\n", optarg);
    } else {
    if (config != core::ptr::null_mut()) {
    if (config.output != core::ptr::null_mut())
    fclose(config.output);
    free(config);
    }
    usage();
    }
    break;
    case 'n':
    sscanf(optarg, "%u", &config.cycles);
    dprintf("user cycles . %s\n", optarg);
    break;
    case 'r':
    sscanf(optarg, "%u", &config.rounds);
    dprintf("user rounds . %s\n", optarg);
    break;
    case 'x':
    sscanf(optarg, "%li", &config.load_step);
    dprintf("user load_step . %s\n", optarg);
    break;
    case 'y':
    sscanf(optarg, "%li", &config.sleep_step);
    dprintf("user sleep_step . %s\n", optarg);
    break;
    case 'f':
    if (prepare_config(optarg, config))
    return EXIT_FAILURE;
    break;
    case 'v':
    config.verbose = 1;
    dprintf("verbose output enabled\n");
    break;
    case 'h':
    case '?':
    default:
    if (config != core::ptr::null_mut()) {
    if (config.output != core::ptr::null_mut())
    fclose(config.output);
    free(config);
    }
    usage();
    }
    }
    if (config.verbose) {
    printf("starting benchmark with parameters:\n");
    printf("config:\n\t"
    "sleep=%li\n\t"
    "load=%li\n\t"
    "sleep_step=%li\n\t"
    "load_step=%li\n\t"
    "cpu=%u\n\t"
    "cycles=%u\n\t"
    "rounds=%u\n\t"
    "governor=%s\n\n",
    config.sleep,
    config.load,
    config.sleep_step,
    config.load_step,
    config.cpu,
    config.cycles,
    config.rounds,
    config.governor);
    }
    prepare_user(config);
    prepare_system(config);
    start_benchmark(config);
    if (config.output != stdout)
    fclose(config.output);
    free(config);
    return EXIT_SUCCESS;
    }
