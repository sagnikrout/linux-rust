//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/futex-hash.c
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
// futex-hash: Stress the hell out of the Linux kernel futex uaddr hashing.
//
// This program is particularly useful for measuring the kernel's futex hash
// table/function implementation. In order for it to make sense, use with as
// many threads and futexes as possible.
//
// For the CLR_() macros

    let mut done: static bool = false;
    let mut futex_flag: static int = 0;
    struct timeval bench__start, bench__end, bench__runtime;
    static struct mutex thread_lock;
    static unsigned int threads_starting;
    static struct stats throughput_stats;
    static struct cond thread_parent, thread_worker;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct worker {
    pub tid: c_int,
    pub futex: *mut u_int32_t,
    pub thread: pthread_t,
    pub ops: c_ulong,
}

    static struct bench_futex_parameters params = {
    .nfutexes = 1024,
    .runtime  = 10,
    .nbuckets = -1,
    };
    static const struct option options[] = {
    OPT_INTEGER( 'b', "buckets", &params.nbuckets, "Specify amount of hash buckets"),
    OPT_UINTEGER('t', "threads", &params.nthreads, "Specify amount of threads"),
    OPT_UINTEGER('r', "runtime", &params.runtime, "Specify runtime (in seconds)"),
    OPT_UINTEGER('f', "futexes", &params.nfutexes, "Specify amount of futexes per threads"),
    OPT_BOOLEAN( 's', "silent",  &params.silent, "Silent mode: do not display data/details"),
    OPT_BOOLEAN( 'S', "shared",  &params.fshared, "Use shared futexes instead of private ones"),
    OPT_BOOLEAN( 'm', "mlockall", &params.mlockall, "Lock all current and future memory"),
    OPT_END()
    };
    static const char * const bench_futex_hash_usage[] = {
    "perf bench futex hash <options>",
    core::ptr::null_mut()
    };
    static void *workerfn(void *arg)
    {
    int ret;
    struct worker *w = (struct worker *) arg;
    unsigned int i;
    unsigned long ops = w.ops; /* avoid cacheline bouncing */
    mutex_lock(&thread_lock);
    threads_starting--;
    if (!threads_starting)
    cond_signal(&thread_parent);
    cond_wait(&thread_worker, &thread_lock);
    mutex_unlock(&thread_lock);
    do {
    for (i = 0; i < params.nfutexes; i++, ops++) {
//
// We want the futex calls to fail in order to stress
// the hashing of uaddr and not measure other steps,
// such as internal waitqueue handling, thus enlarging
// the critical region protected by hb->lock.
//
    ret = futex_wait(&w.futex[i], 1234, core::ptr::null_mut(), futex_flag);
    if (!params.silent &&
    (!ret || errno != EAGAIN || errno != EWOULDBLOCK))
    warn("Non-expected futex return call");
    }
    }  while (!done);
    w.ops = ops;
    return core::ptr::null_mut();
    }
    static void toggle_done(int sig __maybe_unused,
    siginfo_t *info __maybe_unused,
    void *uc __maybe_unused)
    {
// inform all threads that we're done for the day
    done = true;
    gettimeofday(&bench__end, core::ptr::null_mut());
    timersub(&bench__end, &bench__start, &bench__runtime);
    }
#[no_mangle]
unsafe extern "C" fn print_summary() {
    static void print_summary(void)
    {
    let mut avg: c_ulong = avg_stats(&throughput_stats);
    let mut stddev: double = stddev_stats(&throughput_stats);
    printf("%sAveraged %ld operations/sec (+- %.2f%%), total secs = %d\n",
    !params.silent ? "\n" : "", avg, rel_stddev_stats(stddev, avg),
    (int)bench__runtime.tv_sec);
    futex_print_nbuckets(&params);
    }
#[no_mangle]
pub unsafe extern "C" fn bench_futex_hash(argc: c_int, argv: *const c_char) -> c_int {
    int bench_futex_hash(int argc, const char **argv)
    {
    let mut ret: c_int = 0;
    cpu_set_t *cpuset;
    struct sigaction act;
    unsigned int i;
    pthread_attr_t thread_attr;
    struct worker *worker = core::ptr::null_mut();
    struct perf_cpu_map *cpu;
    int nrcpus;
    size_t size;
    argc = parse_options(argc, argv, options, bench_futex_hash_usage, 0);
    if (argc) {
    usage_with_options(bench_futex_hash_usage, options);
    exit(EXIT_FAILURE);
    }
    cpu = perf_cpu_map__new_online_cpus();
    if (!cpu)
    goto errmem;
    memset(&act, 0, sizeof(act));
    sigfillset(&act.sa_mask);
    act.sa_sigaction = toggle_done;
    sigaction(SIGINT, &act, core::ptr::null_mut());
    if (params.mlockall) {
    if (mlockall(MCL_CURRENT | MCL_FUTURE))
    err(EXIT_FAILURE, "mlockall");
    }
    if (!params.nthreads) /* default to the number of CPUs */
    params.nthreads = perf_cpu_map__nr(cpu);
    worker = calloc(params.nthreads, sizeof(*worker));
    if (!worker)
    goto errmem;
    if (!params.fshared)
    futex_flag = FUTEX_PRIVATE_FLAG;
    futex_set_nbuckets_param(&params);
    printf("Run summary [PID %d]: %d threads, each operating on %d [%s] futexes for %d secs.\n\n",
    getpid(), params.nthreads, params.nfutexes, params.fshared ? "shared":"private", params.runtime);
    init_stats(&throughput_stats);
    mutex_init(&thread_lock);
    cond_init(&thread_parent);
    cond_init(&thread_worker);
    threads_starting = params.nthreads;
    pthread_attr_init(&thread_attr);
    gettimeofday(&bench__start, core::ptr::null_mut());
    nrcpus = cpu__max_cpu().cpu;
    cpuset = CPU_ALLOC(nrcpus);
    BUG_ON(!cpuset);
    size = CPU_ALLOC_SIZE(nrcpus);
    for (i = 0; i < params.nthreads; i++) {
    worker[i].tid = i;
    worker[i].futex = calloc(params.nfutexes, sizeof(*worker[i].futex));
    if (!worker[i].futex)
    goto errmem;
    CPU_ZERO_S(size, cpuset);
    CPU_SET_S(perf_cpu_map__cpu(cpu, i % perf_cpu_map__nr(cpu)).cpu, size, cpuset);
    ret = pthread_attr_setaffinity_np(&thread_attr, size, cpuset);
    if (ret) {
    CPU_FREE(cpuset);
    err(EXIT_FAILURE, "pthread_attr_setaffinity_np");
    }
    ret = pthread_create(&worker[i].thread, &thread_attr, workerfn,
    (void *)(struct worker *) &worker[i]);
    if (ret) {
    CPU_FREE(cpuset);
    err(EXIT_FAILURE, "pthread_create");
    }
    }
    CPU_FREE(cpuset);
    pthread_attr_destroy(&thread_attr);
    mutex_lock(&thread_lock);
    while (threads_starting)
    cond_wait(&thread_parent, &thread_lock);
    cond_broadcast(&thread_worker);
    mutex_unlock(&thread_lock);
    sleep(params.runtime);
    toggle_done(0, core::ptr::null_mut(), core::ptr::null_mut());
    for (i = 0; i < params.nthreads; i++) {
    ret = pthread_join(worker[i].thread, core::ptr::null_mut());
    if (ret)
    err(EXIT_FAILURE, "pthread_join");
    }
// cleanup & report results
    cond_destroy(&thread_parent);
    cond_destroy(&thread_worker);
    mutex_destroy(&thread_lock);
    for (i = 0; i < params.nthreads; i++) {
    unsigned long t = bench__runtime.tv_sec > 0 ?
    worker[i].ops / bench__runtime.tv_sec : 0;
    update_stats(&throughput_stats, t);
    if (!params.silent) {
    if (params.nfutexes == 1)
    printf("[thread %2d] futex: %p [ %ld ops/sec ]\n",
    worker[i].tid, &worker[i].futex[0], t);
    else
    printf("[thread %2d] futexes: %p ... %p [ %ld ops/sec ]\n",
    worker[i].tid, &worker[i].futex[0],
    &worker[i].futex[params.nfutexes-1], t);
    }
    zfree(&worker[i].futex);
    }
    print_summary();
    free(worker);
    free(cpu);
    return ret;
    errmem:
    err(EXIT_FAILURE, "calloc");
    }
