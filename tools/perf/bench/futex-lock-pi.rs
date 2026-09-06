//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/futex-lock-pi.c
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
// For the CLR_() macros

#[repr(C)]
#[derive(Copy, Clone)]
pub struct worker {
    pub tid: c_int,
    pub futex: *mut u_int32_t,
    pub thread: pthread_t,
    pub ops: c_ulong,
}

    let mut global_futex: static u_int32_t = 0;
    static struct worker *worker;
    let mut done: static bool = false;
    let mut futex_flag: static int = 0;
    static struct mutex thread_lock;
    static unsigned int threads_starting;
    static struct stats throughput_stats;
    static struct cond thread_parent, thread_worker;
    static struct bench_futex_parameters params = {
    .nbuckets = -1,
    .runtime  = 10,
    };
    static const struct option options[] = {
    OPT_INTEGER( 'b', "buckets", &params.nbuckets, "Specify amount of hash buckets"),
    OPT_UINTEGER('t', "threads", &params.nthreads, "Specify amount of threads"),
    OPT_UINTEGER('r', "runtime", &params.runtime, "Specify runtime (in seconds)"),
    OPT_BOOLEAN( 'M', "multi",   &params.multi, "Use multiple futexes"),
    OPT_BOOLEAN( 's', "silent",  &params.silent, "Silent mode: do not display data/details"),
    OPT_BOOLEAN( 'S', "shared",  &params.fshared, "Use shared futexes instead of private ones"),
    OPT_BOOLEAN( 'm', "mlockall", &params.mlockall, "Lock all current and future memory"),
    OPT_END()
    };
    static const char * const bench_futex_lock_pi_usage[] = {
    "perf bench futex lock-pi <options>",
    core::ptr::null_mut()
    };
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
    static void toggle_done(int sig __maybe_unused,
    siginfo_t *info __maybe_unused,
    void *uc __maybe_unused)
    {
// inform all threads that we're done for the day
    done = true;
    gettimeofday(&bench__end, core::ptr::null_mut());
    timersub(&bench__end, &bench__start, &bench__runtime);
    }
    static void *workerfn(void *arg)
    {
    struct worker *w = (struct worker *) arg;
    let mut ops: c_ulong = w.ops;
    mutex_lock(&thread_lock);
    threads_starting--;
    if (!threads_starting)
    cond_signal(&thread_parent);
    cond_wait(&thread_worker, &thread_lock);
    mutex_unlock(&thread_lock);
    do {
    int ret;
    again:
    ret = futex_lock_pi(w.futex, core::ptr::null_mut(), futex_flag);
    if (ret) { /* handle lock acquisition */
    if (!params.silent)
    warn("thread %d: Could not lock pi-lock for %p (%d)",
    w.tid, w.futex, ret);
    if (done)
    break;
    goto again;
    }
    usleep(1);
    ret = futex_unlock_pi(w.futex, futex_flag);
    if (ret && !params.silent)
    warn("thread %d: Could not unlock pi-lock for %p (%d)",
    w.tid, w.futex, ret);
    ops++; /* account for thread's share of work */
    }  while (!done);
    w.ops = ops;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn create_threads(w: *mut worker, cpu: *mut perf_cpu_map) {
    static void create_threads(struct worker *w, struct perf_cpu_map *cpu)
    {
    cpu_set_t *cpuset;
    unsigned int i;
    let mut nrcpus: c_int = cpu__max_cpu().cpu;
    size_t size;
    threads_starting = params.nthreads;
    cpuset = CPU_ALLOC(nrcpus);
    BUG_ON(!cpuset);
    size = CPU_ALLOC_SIZE(nrcpus);
    for (i = 0; i < params.nthreads; i++) {
    pthread_attr_t thread_attr;
    pthread_attr_init(&thread_attr);
    worker[i].tid = i;
    if (params.multi) {
    worker[i].futex = calloc(1, sizeof(u_int32_t));
    if (!worker[i].futex)
    err(EXIT_FAILURE, "calloc");
    } else
    worker[i].futex = &global_futex;
    CPU_ZERO_S(size, cpuset);
    CPU_SET_S(perf_cpu_map__cpu(cpu, i % perf_cpu_map__nr(cpu)).cpu, size, cpuset);
    if (pthread_attr_setaffinity_np(&thread_attr, size, cpuset)) {
    CPU_FREE(cpuset);
    err(EXIT_FAILURE, "pthread_attr_setaffinity_np");
    }
    if (pthread_create(&w[i].thread, &thread_attr, workerfn, &worker[i])) {
    CPU_FREE(cpuset);
    err(EXIT_FAILURE, "pthread_create");
    }
    pthread_attr_destroy(&thread_attr);
    }
    CPU_FREE(cpuset);
    }
#[no_mangle]
pub unsafe extern "C" fn bench_futex_lock_pi(argc: c_int, argv: *const c_char) -> c_int {
    int bench_futex_lock_pi(int argc, const char **argv)
    {
    let mut ret: c_int = 0;
    unsigned int i;
    struct sigaction act;
    struct perf_cpu_map *cpu;
    argc = parse_options(argc, argv, options, bench_futex_lock_pi_usage, 0);
    if (argc)
    goto err;
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
    printf("Run summary [PID %d]: %d threads doing pi lock/unlock pairing for %d secs.\n\n",
    getpid(), params.nthreads, params.runtime);
    init_stats(&throughput_stats);
    mutex_init(&thread_lock);
    cond_init(&thread_parent);
    cond_init(&thread_worker);
    futex_set_nbuckets_param(&params);
    threads_starting = params.nthreads;
    gettimeofday(&bench__start, core::ptr::null_mut());
    create_threads(worker, cpu);
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
    if (!params.silent)
    printf("[thread %3d] futex: %p [ %ld ops/sec ]\n",
    worker[i].tid, worker[i].futex, t);
    if (params.multi)
    zfree(&worker[i].futex);
    }
    print_summary();
    free(worker);
    perf_cpu_map__put(cpu);
    return ret;
    err:
    usage_with_options(bench_futex_lock_pi_usage, options);
    exit(EXIT_FAILURE);
    }
