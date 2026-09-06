//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/futex-wake.c
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
// Copyright (C) 2013  Davidlohr Bueso <davidlohr@hp.com>
//
// futex-wake: Block a bunch of threads on a futex and wake'em up, N at a time.
//
// This program is particularly useful to measure the latency of nthread wakeups
// in non-error situations:  all waiters are queued and all wake calls wakeup
// one or more tasks, and thus the waitqueue is never empty.
//
// For the CLR_() macros

// all threads will block on the same futex
    let mut futex1: static u_int32_t = 0;
    static pthread_t *worker;
    let mut done: static bool = false;
    static struct mutex thread_lock;
    static struct cond thread_parent, thread_worker;
    static struct stats waketime_stats, wakeup_stats;
    static unsigned int threads_starting;
    let mut futex_flag: static int = 0;
    static struct bench_futex_parameters params = {
    .nbuckets = -1,
//
// How many wakeups to do at a time.
// Default to 1 in order to make the kernel work more.
//
    .nwakes  = 1,
    };
    static const struct option options[] = {
    OPT_INTEGER( 'b', "buckets", &params.nbuckets, "Specify amount of hash buckets"),
    OPT_UINTEGER('t', "threads", &params.nthreads, "Specify amount of threads"),
    OPT_UINTEGER('w', "nwakes",  &params.nwakes, "Specify amount of threads to wake at once"),
    OPT_BOOLEAN( 's', "silent",  &params.silent, "Silent mode: do not display data/details"),
    OPT_BOOLEAN( 'S', "shared",  &params.fshared, "Use shared futexes instead of private ones"),
    OPT_BOOLEAN( 'm', "mlockall", &params.mlockall, "Lock all current and future memory"),
    OPT_END()
    };
    static const char * const bench_futex_wake_usage[] = {
    "perf bench futex wake <options>",
    core::ptr::null_mut()
    };
    static void *workerfn(void *arg __maybe_unused)
    {
    mutex_lock(&thread_lock);
    threads_starting--;
    if (!threads_starting)
    cond_signal(&thread_parent);
    cond_wait(&thread_worker, &thread_lock);
    mutex_unlock(&thread_lock);
    while (1) {
    if (futex_wait(&futex1, 0, core::ptr::null_mut(), futex_flag) != EINTR)
    break;
    }
    pthread_exit(core::ptr::null_mut());
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn print_summary() {
    static void print_summary(void)
    {
    let mut waketime_avg: double = avg_stats(&waketime_stats);
    let mut waketime_stddev: double = stddev_stats(&waketime_stats);
    let mut wakeup_avg: c_uint = avg_stats(&wakeup_stats);
    printf("Wokeup %d of %d threads in %.4f ms (+-%.2f%%)\n",
    wakeup_avg,
    params.nthreads,
    waketime_avg / USEC_PER_MSEC,
    rel_stddev_stats(waketime_stddev, waketime_avg));
    futex_print_nbuckets(&params);
    }
#[no_mangle]
unsafe extern "C" fn block_threads(w: *mut pthread_t, cpu: *mut perf_cpu_map) {
    static void block_threads(pthread_t *w, struct perf_cpu_map *cpu)
    {
    cpu_set_t *cpuset;
    unsigned int i;
    size_t size;
    let mut nrcpus: c_int = cpu__max_cpu().cpu;
    threads_starting = params.nthreads;
    cpuset = CPU_ALLOC(nrcpus);
    BUG_ON(!cpuset);
    size = CPU_ALLOC_SIZE(nrcpus);
// create and block all threads
    for (i = 0; i < params.nthreads; i++) {
    pthread_attr_t thread_attr;
    pthread_attr_init(&thread_attr);
    CPU_ZERO_S(size, cpuset);
    CPU_SET_S(perf_cpu_map__cpu(cpu, i % perf_cpu_map__nr(cpu)).cpu, size, cpuset);
    if (pthread_attr_setaffinity_np(&thread_attr, size, cpuset)) {
    CPU_FREE(cpuset);
    err(EXIT_FAILURE, "pthread_attr_setaffinity_np");
    }
    if (pthread_create(&w[i], &thread_attr, workerfn, core::ptr::null_mut())) {
    CPU_FREE(cpuset);
    err(EXIT_FAILURE, "pthread_create");
    }
    pthread_attr_destroy(&thread_attr);
    }
    CPU_FREE(cpuset);
    }
    static void toggle_done(int sig __maybe_unused,
    siginfo_t *info __maybe_unused,
    void *uc __maybe_unused)
    {
    done = true;
    }
#[no_mangle]
pub unsafe extern "C" fn bench_futex_wake(argc: c_int, argv: *const c_char) -> c_int {
    int bench_futex_wake(int argc, const char **argv)
    {
    let mut ret: c_int = 0;
    unsigned int i, j;
    struct sigaction act;
    struct perf_cpu_map *cpu;
    argc = parse_options(argc, argv, options, bench_futex_wake_usage, 0);
    if (argc) {
    usage_with_options(bench_futex_wake_usage, options);
    exit(EXIT_FAILURE);
    }
    cpu = perf_cpu_map__new_online_cpus();
    if (!cpu)
    err(EXIT_FAILURE, "calloc");
    memset(&act, 0, sizeof(act));
    sigfillset(&act.sa_mask);
    act.sa_sigaction = toggle_done;
    sigaction(SIGINT, &act, core::ptr::null_mut());
    if (params.mlockall) {
    if (mlockall(MCL_CURRENT | MCL_FUTURE))
    err(EXIT_FAILURE, "mlockall");
    }
    if (!params.nthreads)
    params.nthreads = perf_cpu_map__nr(cpu);
    worker = calloc(params.nthreads, sizeof(*worker));
    if (!worker)
    err(EXIT_FAILURE, "calloc");
    if (!params.fshared)
    futex_flag = FUTEX_PRIVATE_FLAG;
    printf("Run summary [PID %d]: blocking on %d threads (at [%s] futex %p), "
    "waking up %d at a time.\n\n",
    getpid(), params.nthreads, params.fshared ? "shared":"private",
    &futex1, params.nwakes);
    init_stats(&wakeup_stats);
    init_stats(&waketime_stats);
    mutex_init(&thread_lock);
    cond_init(&thread_parent);
    cond_init(&thread_worker);
    for (j = 0; j < bench_repeat && !done; j++) {
    let mut nwoken: c_uint = 0;
    struct timeval start, end, runtime;
// create, launch & block all threads
    block_threads(worker, cpu);
// make sure all threads are already blocked
    mutex_lock(&thread_lock);
    while (threads_starting)
    cond_wait(&thread_parent, &thread_lock);
    cond_broadcast(&thread_worker);
    mutex_unlock(&thread_lock);
    usleep(100000);
// Ok, all threads are patiently blocked, start waking folks up
    gettimeofday(&start, core::ptr::null_mut());
    while (nwoken != params.nthreads)
    nwoken += futex_wake(&futex1,
    params.nwakes, futex_flag);
    gettimeofday(&end, core::ptr::null_mut());
    timersub(&end, &start, &runtime);
    update_stats(&wakeup_stats, nwoken);
    update_stats(&waketime_stats, runtime.tv_usec);
    if (!params.silent) {
    printf("[Run %d]: Wokeup %d of %d threads in %.4f ms\n",
    j + 1, nwoken, params.nthreads,
    runtime.tv_usec / (double)USEC_PER_MSEC);
    }
    for (i = 0; i < params.nthreads; i++) {
    ret = pthread_join(worker[i], core::ptr::null_mut());
    if (ret)
    err(EXIT_FAILURE, "pthread_join");
    }
    }
// cleanup & report results
    cond_destroy(&thread_parent);
    cond_destroy(&thread_worker);
    mutex_destroy(&thread_lock);
    print_summary();
    free(worker);
    perf_cpu_map__put(cpu);
    return ret;
    }
