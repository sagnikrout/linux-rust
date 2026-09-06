//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/futex-wake-parallel.c
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
// Copyright (C) 2015 Davidlohr Bueso.
//
// Block a bunch of threads and let parallel waker threads wakeup an
// equal amount of them. The program output reflects the avg latency
// for each individual thread to service its share of work. Ultimately
// it can be used to measure futex_wake() changes.
//

#[no_mangle]
pub unsafe extern "C" fn bench_futex_wake_parallel(__maybe_unused: int argc, __maybe_unused: *const *const *const char argv) -> c_int {
    int bench_futex_wake_parallel(int argc __maybe_unused, const char **argv __maybe_unused)
    {
    pr_err("%s: pthread_barrier_t unavailable, disabling this test...\n", __func__);
    return 0;
    }

// For the CLR_() macros

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_data {
    pub worker: pthread_t,
    pub nwoken: c_uint,
    pub runtime: timeval,
}

    let mut nwakes: static unsigned int = 1;
// all threads will block on the same futex -- hash bucket chaos ;)
    let mut futex: static u_int32_t = 0;
    static pthread_t *blocked_worker;
    let mut done: static bool = false;
    static struct mutex thread_lock;
    static struct cond thread_parent, thread_worker;
    static pthread_barrier_t barrier;
    static struct stats waketime_stats, wakeup_stats;
    static unsigned int threads_starting;
    let mut futex_flag: static int = 0;
    static struct bench_futex_parameters params = {
    .nbuckets = -1,
    };
    static const struct option options[] = {
    OPT_INTEGER( 'b', "buckets", &params.nbuckets, "Specify amount of hash buckets"),
    OPT_UINTEGER('t', "threads", &params.nthreads, "Specify amount of threads"),
    OPT_UINTEGER('w', "nwakers", &params.nwakes, "Specify amount of waking threads"),
    OPT_BOOLEAN( 's', "silent",  &params.silent, "Silent mode: do not display data/details"),
    OPT_BOOLEAN( 'S', "shared",  &params.fshared, "Use shared futexes instead of private ones"),
    OPT_BOOLEAN( 'm', "mlockall", &params.mlockall, "Lock all current and future memory"),
    OPT_END()
    };
    static const char * const bench_futex_wake_parallel_usage[] = {
    "perf bench futex wake-parallel <options>",
    core::ptr::null_mut()
    };
    static void *waking_workerfn(void *arg)
    {
    struct thread_data *waker = (struct thread_data *) arg;
    struct timeval start, end;
    pthread_barrier_wait(&barrier);
    gettimeofday(&start, core::ptr::null_mut());
    waker.nwoken = futex_wake(&futex, nwakes, futex_flag);
    if (waker.nwoken != nwakes)
    warnx("couldn't wakeup all tasks (%d/%d)",
    waker.nwoken, nwakes);
    gettimeofday(&end, core::ptr::null_mut());
    timersub(&end, &start, &waker.runtime);
    pthread_exit(core::ptr::null_mut());
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn wakeup_threads(td: *mut thread_data) {
    static void wakeup_threads(struct thread_data *td)
    {
    unsigned int i;
    pthread_attr_t thread_attr;
    pthread_attr_init(&thread_attr);
    pthread_attr_setdetachstate(&thread_attr, PTHREAD_CREATE_JOINABLE);
    pthread_barrier_init(&barrier, core::ptr::null_mut(), params.nwakes + 1);
// create and block all threads
    for (i = 0; i < params.nwakes; i++) {
//
// Thread creation order will impact per-thread latency
// as it will affect the order to acquire the hb spinlock.
// For now let the scheduler decide.
//
    if (pthread_create(&td[i].worker, &thread_attr,
    waking_workerfn, (void *)&td[i]))
    err(EXIT_FAILURE, "pthread_create");
    }
    pthread_barrier_wait(&barrier);
    for (i = 0; i < params.nwakes; i++)
    if (pthread_join(td[i].worker, core::ptr::null_mut()))
    err(EXIT_FAILURE, "pthread_join");
    pthread_barrier_destroy(&barrier);
    pthread_attr_destroy(&thread_attr);
    }
    static void *blocked_workerfn(void *arg __maybe_unused)
    {
    mutex_lock(&thread_lock);
    threads_starting--;
    if (!threads_starting)
    cond_signal(&thread_parent);
    cond_wait(&thread_worker, &thread_lock);
    mutex_unlock(&thread_lock);
    while (1) { /* handle spurious wakeups */
    if (futex_wait(&futex, 0, core::ptr::null_mut(), futex_flag) != EINTR)
    break;
    }
    pthread_exit(core::ptr::null_mut());
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
    if (pthread_create(&w[i], &thread_attr, blocked_workerfn, core::ptr::null_mut())) {
    CPU_FREE(cpuset);
    err(EXIT_FAILURE, "pthread_create");
    }
    pthread_attr_destroy(&thread_attr);
    }
    CPU_FREE(cpuset);
    }
#[no_mangle]
unsafe extern "C" fn print_run(waking_worker: *mut thread_data, run_num: c_uint) {
    static void print_run(struct thread_data *waking_worker, unsigned int run_num)
    {
    unsigned int i, wakeup_avg;
    double waketime_avg, waketime_stddev;
    struct stats __waketime_stats, __wakeup_stats;
    init_stats(&__wakeup_stats);
    init_stats(&__waketime_stats);
    for (i = 0; i < params.nwakes; i++) {
    update_stats(&__waketime_stats, waking_worker[i].runtime.tv_usec);
    update_stats(&__wakeup_stats, waking_worker[i].nwoken);
    }
    waketime_avg = avg_stats(&__waketime_stats);
    waketime_stddev = stddev_stats(&__waketime_stats);
    wakeup_avg = avg_stats(&__wakeup_stats);
    printf("[Run %d]: Avg per-thread latency (waking %d/%d threads) "
    "in %.4f ms (+-%.2f%%)\n", run_num + 1, wakeup_avg,
    params.nthreads, waketime_avg / USEC_PER_MSEC,
    rel_stddev_stats(waketime_stddev, waketime_avg));
    }
#[no_mangle]
unsafe extern "C" fn print_summary() {
    static void print_summary(void)
    {
    unsigned int wakeup_avg;
    double waketime_avg, waketime_stddev;
    waketime_avg = avg_stats(&waketime_stats);
    waketime_stddev = stddev_stats(&waketime_stats);
    wakeup_avg = avg_stats(&wakeup_stats);
    printf("Avg per-thread latency (waking %d/%d threads) in %.4f ms (+-%.2f%%)\n",
    wakeup_avg,
    params.nthreads,
    waketime_avg / USEC_PER_MSEC,
    rel_stddev_stats(waketime_stddev, waketime_avg));
    futex_print_nbuckets(&params);
    }
#[no_mangle]
unsafe extern "C" fn do_run_stats(waking_worker: *mut thread_data) {
    static void do_run_stats(struct thread_data *waking_worker)
    {
    unsigned int i;
    for (i = 0; i < params.nwakes; i++) {
    update_stats(&waketime_stats, waking_worker[i].runtime.tv_usec);
    update_stats(&wakeup_stats, waking_worker[i].nwoken);
    }
    }
    static void toggle_done(int sig __maybe_unused,
    siginfo_t *info __maybe_unused,
    void *uc __maybe_unused)
    {
    done = true;
    }
#[no_mangle]
pub unsafe extern "C" fn bench_futex_wake_parallel(argc: c_int, argv: *const c_char) -> c_int {
    int bench_futex_wake_parallel(int argc, const char **argv)
    {
    let mut ret: c_int = 0;
    unsigned int i, j;
    struct sigaction act;
    struct thread_data *waking_worker;
    struct perf_cpu_map *cpu;
    argc = parse_options(argc, argv, options,
    bench_futex_wake_parallel_usage, 0);
    if (argc) {
    usage_with_options(bench_futex_wake_parallel_usage, options);
    exit(EXIT_FAILURE);
    }
    memset(&act, 0, sizeof(act));
    sigfillset(&act.sa_mask);
    act.sa_sigaction = toggle_done;
    sigaction(SIGINT, &act, core::ptr::null_mut());
    if (params.mlockall) {
    if (mlockall(MCL_CURRENT | MCL_FUTURE))
    err(EXIT_FAILURE, "mlockall");
    }
    cpu = perf_cpu_map__new_online_cpus();
    if (!cpu)
    err(EXIT_FAILURE, "calloc");
    if (!params.nthreads)
    params.nthreads = perf_cpu_map__nr(cpu);
// some sanity checks
    if (params.nwakes > params.nthreads ||
    !params.nwakes)
    params.nwakes = params.nthreads;
    if (params.nthreads % params.nwakes)
    errx(EXIT_FAILURE, "Must be perfectly divisible");
//
// Each thread will wakeup nwakes tasks in
// a single futex_wait call.
//
    nwakes = params.nthreads/params.nwakes;
    blocked_worker = calloc(params.nthreads, sizeof(*blocked_worker));
    if (!blocked_worker)
    err(EXIT_FAILURE, "calloc");
    if (!params.fshared)
    futex_flag = FUTEX_PRIVATE_FLAG;
    futex_set_nbuckets_param(&params);
    printf("Run summary [PID %d]: blocking on %d threads (at [%s] "
    "futex %p), %d threads waking up %d at a time.\n\n",
    getpid(), params.nthreads, params.fshared ? "shared":"private",
    &futex, params.nwakes, nwakes);
    init_stats(&wakeup_stats);
    init_stats(&waketime_stats);
    mutex_init(&thread_lock);
    cond_init(&thread_parent);
    cond_init(&thread_worker);
    for (j = 0; j < bench_repeat && !done; j++) {
    waking_worker = calloc(params.nwakes, sizeof(*waking_worker));
    if (!waking_worker)
    err(EXIT_FAILURE, "calloc");
// create, launch & block all threads
    block_threads(blocked_worker, cpu);
// make sure all threads are already blocked
    mutex_lock(&thread_lock);
    while (threads_starting)
    cond_wait(&thread_parent, &thread_lock);
    cond_broadcast(&thread_worker);
    mutex_unlock(&thread_lock);
    usleep(200000);
// Ok, all threads are patiently blocked, start waking folks up
    wakeup_threads(waking_worker);
    for (i = 0; i < params.nthreads; i++) {
    ret = pthread_join(blocked_worker[i], core::ptr::null_mut());
    if (ret)
    err(EXIT_FAILURE, "pthread_join");
    }
    do_run_stats(waking_worker);
    if (!params.silent)
    print_run(waking_worker, j);
    free(waking_worker);
    }
// cleanup & report results
    cond_destroy(&thread_parent);
    cond_destroy(&thread_worker);
    mutex_destroy(&thread_lock);
    print_summary();
    free(blocked_worker);
    perf_cpu_map__put(cpu);
    return ret;
    }
