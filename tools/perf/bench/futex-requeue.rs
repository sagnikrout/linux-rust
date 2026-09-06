//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/futex-requeue.c
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
// futex-requeue: Block a bunch of threads on futex1 and requeue them
// on futex2, N at a time.
//
// This program is particularly useful to measure the latency of nthread
// requeues without waking up any tasks (in the non-pi case) -- thus
// mimicking a regular futex_wait.
//
// For the CLR_() macros

    let mut futex1: static u_int32_t = 0, futex2 = 0;
    static pthread_t *worker;
    let mut done: static bool = false;
    static struct mutex thread_lock;
    static struct cond thread_parent, thread_worker;
    static struct stats requeuetime_stats, requeued_stats;
    static unsigned int threads_starting;
    let mut futex_flag: static int = 0;
    static struct bench_futex_parameters params = {
    .nbuckets = -1,
//
// How many tasks to requeue at a time.
// Default to 1 in order to make the kernel work more.
//
    .nrequeue = 1,
    };
    static const struct option options[] = {
    OPT_INTEGER( 'b', "buckets", &params.nbuckets, "Specify amount of hash buckets"),
    OPT_UINTEGER('t', "threads",  &params.nthreads, "Specify amount of threads"),
    OPT_UINTEGER('q', "nrequeue", &params.nrequeue, "Specify amount of threads to requeue at once"),
    OPT_BOOLEAN( 's', "silent",   &params.silent, "Silent mode: do not display data/details"),
    OPT_BOOLEAN( 'S', "shared",   &params.fshared, "Use shared futexes instead of private ones"),
    OPT_BOOLEAN( 'm', "mlockall", &params.mlockall, "Lock all current and future memory"),
    OPT_BOOLEAN( 'B', "broadcast", &params.broadcast, "Requeue all threads at once"),
    OPT_BOOLEAN( 'p', "pi", &params.pi, "Use PI-aware variants of FUTEX_CMP_REQUEUE"),
    OPT_END()
    };
    static const char * const bench_futex_requeue_usage[] = {
    "perf bench futex requeue <options>",
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn print_summary() {
    static void print_summary(void)
    {
    let mut requeuetime_avg: double = avg_stats(&requeuetime_stats);
    let mut requeuetime_stddev: double = stddev_stats(&requeuetime_stats);
    let mut requeued_avg: c_uint = avg_stats(&requeued_stats);
    printf("Requeued %d of %d threads in %.4f ms (+-%.2f%%)\n",
    requeued_avg,
    params.nthreads,
    requeuetime_avg / USEC_PER_MSEC,
    rel_stddev_stats(requeuetime_stddev, requeuetime_avg));
    futex_print_nbuckets(&params);
    }
    static void *workerfn(void *arg __maybe_unused)
    {
    int ret;
    mutex_lock(&thread_lock);
    threads_starting--;
    if (!threads_starting)
    cond_signal(&thread_parent);
    cond_wait(&thread_worker, &thread_lock);
    mutex_unlock(&thread_lock);
    while (1) {
    if (!params.pi) {
    ret = futex_wait(&futex1, 0, core::ptr::null_mut(), futex_flag);
    if (!ret)
    break;
    if (ret && errno != EAGAIN) {
    if (!params.silent)
    warnx("futex_wait");
    break;
    }
    } else {
    ret = futex_wait_requeue_pi(&futex1, 0, &futex2,
    core::ptr::null_mut(), futex_flag);
    if (!ret) {
// got the lock at futex2
    futex_unlock_pi(&futex2, futex_flag);
    break;
    }
    if (ret && errno != EAGAIN) {
    if (!params.silent)
    warnx("futex_wait_requeue_pi");
    break;
    }
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn block_threads(w: *mut pthread_t, cpu: *mut perf_cpu_map) {
    static void block_threads(pthread_t *w, struct perf_cpu_map *cpu)
    {
    cpu_set_t *cpuset;
    unsigned int i;
    let mut nrcpus: c_int = cpu__max_cpu().cpu;
    size_t size;
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
pub unsafe extern "C" fn bench_futex_requeue(argc: c_int, argv: *const c_char) -> c_int {
    int bench_futex_requeue(int argc, const char **argv)
    {
    let mut ret: c_int = 0;
    unsigned int i, j;
    struct sigaction act;
    struct perf_cpu_map *cpu;
    argc = parse_options(argc, argv, options, bench_futex_requeue_usage, 0);
    if (argc)
    goto err;
    cpu = perf_cpu_map__new_online_cpus();
    if (!cpu)
    err(EXIT_FAILURE, "cpu_map__new");
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
    if (params.nrequeue > params.nthreads)
    params.nrequeue = params.nthreads;
    if (params.broadcast)
    params.nrequeue = params.nthreads;
    futex_set_nbuckets_param(&params);
    printf("Run summary [PID %d]: Requeuing %d threads (from [%s] %p to %s%p), "
    "%d at a time.\n\n",  getpid(), params.nthreads,
    params.fshared ? "shared":"private", &futex1,
    params.pi ? "PI ": "", &futex2, params.nrequeue);
    init_stats(&requeued_stats);
    init_stats(&requeuetime_stats);
    mutex_init(&thread_lock);
    cond_init(&thread_parent);
    cond_init(&thread_worker);
    for (j = 0; j < bench_repeat && !done; j++) {
    let mut nrequeued: c_uint = 0, wakeups = 0;
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
// Ok, all threads are patiently blocked, start requeueing
    gettimeofday(&start, core::ptr::null_mut());
    while (nrequeued < params.nthreads) {
    int r;
//
// For the regular non-pi case, do not wakeup any tasks
// blocked on futex1, allowing us to really measure
// futex_wait functionality. For the PI case the first
// waiter is always awoken.
//
    if (!params.pi) {
    r = futex_cmp_requeue(&futex1, 0, &futex2, 0,
    params.nrequeue,
    futex_flag);
    } else {
    r = futex_cmp_requeue_pi(&futex1, 0, &futex2,
    params.nrequeue,
    futex_flag);
    wakeups++; /* assume no error */
    }
    if (r < 0)
    err(EXIT_FAILURE, "couldn't requeue from %p to %p",
    &futex1, &futex2);
    nrequeued += r;
    }
    gettimeofday(&end, core::ptr::null_mut());
    timersub(&end, &start, &runtime);
    update_stats(&requeued_stats, nrequeued);
    update_stats(&requeuetime_stats, runtime.tv_usec);
    if (!params.silent) {
    if (!params.pi)
    printf("[Run %d]: Requeued %d of %d threads in "
    "%.4f ms\n", j + 1, nrequeued,
    params.nthreads,
    runtime.tv_usec / (double)USEC_PER_MSEC);
    else {
    nrequeued -= wakeups;
    printf("[Run %d]: Awoke and Requeued (%d+%d) of "
    "%d threads in %.4f ms\n",
    j + 1, wakeups, nrequeued,
    params.nthreads,
    runtime.tv_usec / (double)USEC_PER_MSEC);
    }
    }
    if (!params.pi) {
// everybody should be blocked on futex2, wake'em up
    nrequeued = futex_wake(&futex2, nrequeued, futex_flag);
    if (params.nthreads != nrequeued)
    warnx("couldn't wakeup all tasks (%d/%d)",
    nrequeued, params.nthreads);
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
    err:
    usage_with_options(bench_futex_requeue_usage, options);
    exit(EXIT_FAILURE);
    }
