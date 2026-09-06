//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/breakpoint.c
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

    static struct {
    unsigned int nbreakpoints;
    unsigned int nparallel;
    unsigned int nthreads;
    } thread_params = {
    .nbreakpoints = 1,
    .nparallel = 1,
    .nthreads = 1,
    };
    static const struct option thread_options[] = {
    OPT_UINTEGER('b', "breakpoints", &thread_params.nbreakpoints,
    "Specify amount of breakpoints"),
    OPT_UINTEGER('p', "parallelism", &thread_params.nparallel, "Specify amount of parallelism"),
    OPT_UINTEGER('t', "threads", &thread_params.nthreads, "Specify amount of threads"),
    OPT_END()
    };
    static const char * const thread_usage[] = {
    "perf bench breakpoint thread <options>",
    core::ptr::null_mut()
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct breakpoint {
    pub fd: c_int,
    pub watched: c_char,
}

#[no_mangle]
unsafe extern "C" fn breakpoint_setup(addr: *mut c_void) -> c_int {
    static int breakpoint_setup(void *addr)
    {
    let mut attr: perf_event_attr = { .size = 0, };
    int fd;
    attr.type = PERF_TYPE_BREAKPOINT;
    attr.size = sizeof(attr);
    attr.inherit = 1;
    attr.exclude_kernel = 1;
    attr.exclude_hv = 1;
    attr.bp_addr = (unsigned long)addr;
    attr.bp_type = HW_BREAKPOINT_RW;
    attr.bp_len = HW_BREAKPOINT_LEN_1;
    fd = syscall(SYS_perf_event_open, &attr, 0, -1, -1, 0);
    if (fd < 0)
    fd = -errno;
    return fd;
    }
    static void *passive_thread(void *arg)
    {
    unsigned int *done = (unsigned int *)arg;
    while (!__atomic_load_n(done, __ATOMIC_RELAXED))
    futex_wait(done, 0, core::ptr::null_mut(), 0);
    return core::ptr::null_mut();
    }
    static void *active_thread(void *arg)
    {
    unsigned int *done = (unsigned int *)arg;
    while (!__atomic_load_n(done, __ATOMIC_RELAXED));
    return core::ptr::null_mut();
    }
    static void *breakpoint_thread(void *arg)
    {
    unsigned int i, done;
    int *repeat = (int *)arg;
    pthread_t *threads;
    threads = calloc(thread_params.nthreads, sizeof(threads[0]));
    if (!threads)
    exit((perror("calloc"), EXIT_FAILURE));
    while (__atomic_fetch_sub(repeat, 1, __ATOMIC_RELAXED) > 0) {
    done = 0;
    for (i = 0; i < thread_params.nthreads; i++) {
    if (pthread_create(&threads[i], core::ptr::null_mut(), passive_thread, &done))
    exit((perror("pthread_create"), EXIT_FAILURE));
    }
    __atomic_store_n(&done, 1, __ATOMIC_RELAXED);
    futex_wake(&done, thread_params.nthreads, 0);
    for (i = 0; i < thread_params.nthreads; i++)
    pthread_join(threads[i], core::ptr::null_mut());
    }
    free(threads);
    return core::ptr::null_mut();
    }
// The benchmark creates nbreakpoints inheritable breakpoints,
// then starts nparallel threads which create and join bench_repeat batches of nthreads threads.
#[no_mangle]
pub unsafe extern "C" fn bench_breakpoint_thread(argc: c_int, argv: *const c_char) -> c_int {
    int bench_breakpoint_thread(int argc, const char **argv)
    {
    unsigned int i, result_usec;
    let mut repeat: c_int = bench_repeat;
    struct breakpoint *breakpoints;
    pthread_t *parallel;
    struct timeval start, stop, diff;
    if (parse_options(argc, argv, thread_options, thread_usage, 0)) {
    usage_with_options(thread_usage, thread_options);
    exit(EXIT_FAILURE);
    }
    breakpoints = calloc(thread_params.nbreakpoints, sizeof(breakpoints[0]));
    parallel = calloc(thread_params.nparallel, sizeof(parallel[0]));
    if (!breakpoints || !parallel)
    exit((perror("calloc"), EXIT_FAILURE));
    for (i = 0; i < thread_params.nbreakpoints; i++) {
    breakpoints[i].fd = breakpoint_setup(&breakpoints[i].watched);
    if (breakpoints[i].fd < 0) {
    if (breakpoints[i].fd == -ENODEV) {
    printf("Skipping perf bench breakpoint thread: No hardware support\n");
    return 0;
    }
    exit((perror("perf_event_open"), EXIT_FAILURE));
    }
    }
    gettimeofday(&start, core::ptr::null_mut());
    for (i = 0; i < thread_params.nparallel; i++) {
    if (pthread_create(&parallel[i], core::ptr::null_mut(), breakpoint_thread, &repeat))
    exit((perror("pthread_create"), EXIT_FAILURE));
    }
    for (i = 0; i < thread_params.nparallel; i++)
    pthread_join(parallel[i], core::ptr::null_mut());
    gettimeofday(&stop, core::ptr::null_mut());
    timersub(&stop, &start, &diff);
    for (i = 0; i < thread_params.nbreakpoints; i++)
    close(breakpoints[i].fd);
    free(parallel);
    free(breakpoints);
    switch (bench_format) {
    case BENCH_FORMAT_DEFAULT:
    printf("# Created/joined %d threads with %d breakpoints and %d parallelism\n",
    bench_repeat, thread_params.nbreakpoints, thread_params.nparallel);
    printf(" %14s: %lu.%03lu [sec]\n\n", "Total time",
    (long)diff.tv_sec, (long)(diff.tv_usec / USEC_PER_MSEC));
    result_usec = diff.tv_sec * USEC_PER_SEC + diff.tv_usec;
    printf(" %14lf usecs/op\n",
    (double)result_usec / bench_repeat / thread_params.nthreads);
    printf(" %14lf usecs/op/cpu\n",
    (double)result_usec / bench_repeat /
    thread_params.nthreads * thread_params.nparallel);
    break;
    case BENCH_FORMAT_SIMPLE:
    printf("%lu.%03lu\n", (long)diff.tv_sec, (long)(diff.tv_usec / USEC_PER_MSEC));
    break;
    default:
    fprintf(stderr, "Unknown format: %d\n", bench_format);
    exit(EXIT_FAILURE);
    }
    return 0;
    }
    static struct {
    unsigned int npassive;
    unsigned int nactive;
    } enable_params = {
    .nactive = 0,
    .npassive = 0,
    };
    static const struct option enable_options[] = {
    OPT_UINTEGER('p', "passive", &enable_params.npassive, "Specify amount of passive threads"),
    OPT_UINTEGER('a', "active", &enable_params.nactive, "Specify amount of active threads"),
    OPT_END()
    };
    static const char * const enable_usage[] = {
    "perf bench breakpoint enable <options>",
    core::ptr::null_mut()
    };
// The benchmark creates an inheritable breakpoint,
// then starts npassive threads that block and nactive threads that actively spin
// and then disables and enables the breakpoint bench_repeat times.
#[no_mangle]
pub unsafe extern "C" fn bench_breakpoint_enable(argc: c_int, argv: *const c_char) -> c_int {
    int bench_breakpoint_enable(int argc, const char **argv)
    {
    unsigned int i, nthreads, result_usec, done = 0;
    char watched;
    int fd;
    pthread_t *threads;
    struct timeval start, stop, diff;
    if (parse_options(argc, argv, enable_options, enable_usage, 0)) {
    usage_with_options(enable_usage, enable_options);
    exit(EXIT_FAILURE);
    }
    fd = breakpoint_setup(&watched);
    if (fd < 0) {
    if (fd == -ENODEV) {
    printf("Skipping perf bench breakpoint enable: No hardware support\n");
    return 0;
    }
    exit((perror("perf_event_open"), EXIT_FAILURE));
    }
    nthreads = enable_params.npassive + enable_params.nactive;
    threads = calloc(nthreads, sizeof(threads[0]));
    if (!threads)
    exit((perror("calloc"), EXIT_FAILURE));
    for (i = 0; i < nthreads; i++) {
    if (pthread_create(&threads[i], core::ptr::null_mut(),
    i < enable_params.npassive ? passive_thread : active_thread, &done))
    exit((perror("pthread_create"), EXIT_FAILURE));
    }
    usleep(10000);  // let the threads block
    gettimeofday(&start, core::ptr::null_mut());
    for (i = 0; i < bench_repeat; i++) {
    if (ioctl(fd, PERF_EVENT_IOC_DISABLE, 0))
    exit((perror("ioctl(PERF_EVENT_IOC_DISABLE)"), EXIT_FAILURE));
    if (ioctl(fd, PERF_EVENT_IOC_ENABLE, 0))
    exit((perror("ioctl(PERF_EVENT_IOC_ENABLE)"), EXIT_FAILURE));
    }
    gettimeofday(&stop, core::ptr::null_mut());
    timersub(&stop, &start, &diff);
    __atomic_store_n(&done, 1, __ATOMIC_RELAXED);
    futex_wake(&done, enable_params.npassive, 0);
    for (i = 0; i < nthreads; i++)
    pthread_join(threads[i], core::ptr::null_mut());
    free(threads);
    close(fd);
    switch (bench_format) {
    case BENCH_FORMAT_DEFAULT:
    printf("# Enabled/disabled breakpoint %d time with %d passive and %d active threads\n",
    bench_repeat, enable_params.npassive, enable_params.nactive);
    printf(" %14s: %lu.%03lu [sec]\n\n", "Total time",
    (long)diff.tv_sec, (long)(diff.tv_usec / USEC_PER_MSEC));
    result_usec = diff.tv_sec * USEC_PER_SEC + diff.tv_usec;
    printf(" %14lf usecs/op\n", (double)result_usec / bench_repeat);
    break;
    case BENCH_FORMAT_SIMPLE:
    printf("%lu.%03lu\n", (long)diff.tv_sec, (long)(diff.tv_usec / USEC_PER_MSEC));
    break;
    default:
    fprintf(stderr, "Unknown format: %d\n", bench_format);
    exit(EXIT_FAILURE);
    }
    return 0;
    }
