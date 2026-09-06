//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/epoll-ctl.c
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
// Copyright (C) 2018 Davidlohr Bueso.
//
// Benchmark the various operations allowed for epoll_ctl(2).
// The idea is to concurrently stress a single epoll instance
//

// For the CLR_() macros

    do { if (__verbose) printf(fmt, ## arg); } while (0)
    let mut nthreads: static unsigned int = 0;
    let mut nsecs: static unsigned int = 8;
    static bool done, __verbose, randomize;
//
// epoll related shared variables.
//
// Maximum number of nesting allowed inside epoll sets
pub const EPOLL_MAXNESTS: c_int = 4;
    enum {
    OP_EPOLL_ADD,
    OP_EPOLL_MOD,
    OP_EPOLL_DEL,
    EPOLL_NR_OPS,
    };
    static int epollfd;
    static int *epollfdp;
    static bool noaffinity;
    let mut nested: static unsigned int = 0;
// amount of fds to monitor, per thread
    let mut nfds: static unsigned int = 64;
    static struct mutex thread_lock;
    static unsigned int threads_starting;
    static struct stats all_stats[EPOLL_NR_OPS];
    static struct cond thread_parent, thread_worker;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct worker {
    pub tid: c_int,
    pub thread: pthread_t,
    pub ops: [c_ulong; EPOLL_NR_OPS],
    pub fdmap: *mut c_int,
}

    static const struct option options[] = {
    OPT_UINTEGER('t', "threads", &nthreads, "Specify amount of threads"),
    OPT_UINTEGER('r', "runtime", &nsecs,    "Specify runtime (in seconds)"),
    OPT_UINTEGER('f', "nfds", &nfds, "Specify amount of file descriptors to monitor for each thread"),
    OPT_BOOLEAN( 'n', "noaffinity",  &noaffinity,   "Disables CPU affinity"),
    OPT_UINTEGER( 'N', "nested",  &nested,   "Nesting level epoll hierarchy (default is 0, no nesting)"),
    OPT_BOOLEAN( 'R', "randomize", &randomize,   "Perform random operations on random fds"),
    OPT_BOOLEAN( 'v', "verbose",  &__verbose,   "Verbose mode"),
    OPT_END()
    };
    static const char * const bench_epoll_ctl_usage[] = {
    "perf bench epoll ctl <options>",
    core::ptr::null_mut()
    };
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
unsafe extern "C" fn nest_epollfd() {
    static void nest_epollfd(void)
    {
    unsigned int i;
    struct epoll_event ev;
    if (nested > EPOLL_MAXNESTS)
    nested = EPOLL_MAXNESTS;
    printinfo("Nesting level(s): %d\n", nested);
    epollfdp = calloc(nested, sizeof(int));
    if (!epollfdp)
    err(EXIT_FAILURE, "calloc");
    for (i = 0; i < nested; i++) {
    epollfdp[i] = epoll_create(1);
    if (epollfd < 0)
    err(EXIT_FAILURE, "epoll_create");
    }
    ev.events = EPOLLHUP; /* anything */
    ev.data.u64 = i; /* any number */
    for (i = nested - 1; i; i--) {
    if (epoll_ctl(epollfdp[i - 1], EPOLL_CTL_ADD,
    epollfdp[i], &ev) < 0)
    err(EXIT_FAILURE, "epoll_ctl");
    }
    if (epoll_ctl(epollfd, EPOLL_CTL_ADD, *epollfdp, &ev) < 0)
    err(EXIT_FAILURE, "epoll_ctl");
    }
#[no_mangle]
pub unsafe extern "C" fn do_epoll_op(w: *mut worker, op: c_int, fd: c_int) {
    static inline void do_epoll_op(struct worker *w, int op, int fd)
    {
    int error;
    struct epoll_event ev;
    ev.events = EPOLLIN;
    ev.data.u64 = fd;
    switch (op) {
    case OP_EPOLL_ADD:
    error = epoll_ctl(epollfd, EPOLL_CTL_ADD, fd, &ev);
    break;
    case OP_EPOLL_MOD:
    ev.events = EPOLLOUT;
    error = epoll_ctl(epollfd, EPOLL_CTL_MOD, fd, &ev);
    break;
    case OP_EPOLL_DEL:
    error = epoll_ctl(epollfd, EPOLL_CTL_DEL, fd, core::ptr::null_mut());
    break;
    default:
    error = 1;
    break;
    }
    if (!error)
    w.ops[op]++;
    }
#[no_mangle]
pub unsafe extern "C" fn do_random_epoll_op(w: *mut worker) {
    static inline void do_random_epoll_op(struct worker *w)
    {
    let mut rnd1: c_ulong = random(), rnd2 = random();
    int op, fd;
    fd = w.fdmap[rnd1 % nfds];
    op = rnd2 % EPOLL_NR_OPS;
    do_epoll_op(w, op, fd);
    }
    static void *workerfn(void *arg)
    {
    unsigned int i;
    struct worker *w = (struct worker *) arg;
    struct timespec ts = { .tv_sec = 0,
    .tv_nsec = 250 };
    mutex_lock(&thread_lock);
    threads_starting--;
    if (!threads_starting)
    cond_signal(&thread_parent);
    cond_wait(&thread_worker, &thread_lock);
    mutex_unlock(&thread_lock);
// Let 'em loose
    do {
// random
    if (randomize) {
    do_random_epoll_op(w);
    } else {
    for (i = 0; i < nfds; i++) {
    do_epoll_op(w, OP_EPOLL_ADD, w.fdmap[i]);
    do_epoll_op(w, OP_EPOLL_MOD, w.fdmap[i]);
    do_epoll_op(w, OP_EPOLL_DEL, w.fdmap[i]);
    }
    }
    nanosleep(&ts, core::ptr::null_mut());
    }  while (!done);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn init_fdmaps(w: *mut worker, pct: c_int) {
    static void init_fdmaps(struct worker *w, int pct)
    {
    unsigned int i;
    int inc;
    struct epoll_event ev;
    if (!pct)
    return;
    inc = 100/pct;
    for (i = 0; i < nfds; i+=inc) {
    ev.data.fd = w.fdmap[i];
    ev.events = EPOLLIN;
    if (epoll_ctl(epollfd, EPOLL_CTL_ADD, w.fdmap[i], &ev) < 0)
    err(EXIT_FAILURE, "epoll_ct");
    }
    }
#[no_mangle]
unsafe extern "C" fn do_threads(worker: *mut worker, cpu: *mut perf_cpu_map) -> c_int {
    static int do_threads(struct worker *worker, struct perf_cpu_map *cpu)
    {
    pthread_attr_t thread_attr, *attrp = core::ptr::null_mut();
    cpu_set_t *cpuset;
    unsigned int i, j;
    let mut ret: c_int = 0;
    int nrcpus;
    size_t size;
    if (!noaffinity)
    pthread_attr_init(&thread_attr);
    nrcpus = cpu__max_cpu().cpu;
    cpuset = CPU_ALLOC(nrcpus);
    BUG_ON(!cpuset);
    size = CPU_ALLOC_SIZE(nrcpus);
    for (i = 0; i < nthreads; i++) {
    struct worker *w = &worker[i];
    w.tid = i;
    w.fdmap = calloc(nfds, sizeof(int));
    if (!w.fdmap)
    return 1;
    for (j = 0; j < nfds; j++) {
    w.fdmap[j] = eventfd(0, EFD_NONBLOCK);
    if (w.fdmap[j] < 0)
    err(EXIT_FAILURE, "eventfd");
    }
//
// Lets add 50% of the fdmap to the epoll instance, and
// do it before any threads are started; otherwise there is
// an initial bias of the call failing  (mod and del ops).
//
    if (randomize)
    init_fdmaps(w, 50);
    if (!noaffinity) {
    CPU_ZERO_S(size, cpuset);
    CPU_SET_S(perf_cpu_map__cpu(cpu, i % perf_cpu_map__nr(cpu)).cpu,
    size, cpuset);
    ret = pthread_attr_setaffinity_np(&thread_attr, size, cpuset);
    if (ret) {
    CPU_FREE(cpuset);
    err(EXIT_FAILURE, "pthread_attr_setaffinity_np");
    }
    attrp = &thread_attr;
    }
    ret = pthread_create(&w.thread, attrp, workerfn,
    (void *)(struct worker *) w);
    if (ret) {
    CPU_FREE(cpuset);
    err(EXIT_FAILURE, "pthread_create");
    }
    }
    CPU_FREE(cpuset);
    if (!noaffinity)
    pthread_attr_destroy(&thread_attr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn print_summary() {
    static void print_summary(void)
    {
    int i;
    unsigned long avg[EPOLL_NR_OPS];
    double stddev[EPOLL_NR_OPS];
    for (i = 0; i < EPOLL_NR_OPS; i++) {
    avg[i] = avg_stats(&all_stats[i]);
    stddev[i] = stddev_stats(&all_stats[i]);
    }
    printf("\nAveraged %ld ADD operations (+- %.2f%%)\n",
    avg[OP_EPOLL_ADD], rel_stddev_stats(stddev[OP_EPOLL_ADD],
    avg[OP_EPOLL_ADD]));
    printf("Averaged %ld MOD operations (+- %.2f%%)\n",
    avg[OP_EPOLL_MOD], rel_stddev_stats(stddev[OP_EPOLL_MOD],
    avg[OP_EPOLL_MOD]));
    printf("Averaged %ld DEL operations (+- %.2f%%)\n",
    avg[OP_EPOLL_DEL], rel_stddev_stats(stddev[OP_EPOLL_DEL],
    avg[OP_EPOLL_DEL]));
    }
#[no_mangle]
pub unsafe extern "C" fn bench_epoll_ctl(argc: c_int, argv: *const c_char) -> c_int {
    int bench_epoll_ctl(int argc, const char **argv)
    {
    int j, ret = 0;
    struct sigaction act;
    struct worker *worker = core::ptr::null_mut();
    struct perf_cpu_map *cpu;
    struct rlimit rl, prevrl;
    unsigned int i;
    argc = parse_options(argc, argv, options, bench_epoll_ctl_usage, 0);
    if (argc) {
    usage_with_options(bench_epoll_ctl_usage, options);
    exit(EXIT_FAILURE);
    }
    memset(&act, 0, sizeof(act));
    sigfillset(&act.sa_mask);
    act.sa_sigaction = toggle_done;
    sigaction(SIGINT, &act, core::ptr::null_mut());
    cpu = perf_cpu_map__new_online_cpus();
    if (!cpu)
    goto errmem;
// a single, main epoll instance
    epollfd = epoll_create(1);
    if (epollfd < 0)
    err(EXIT_FAILURE, "epoll_create");
//
// Deal with nested epolls, if any.
//
    if (nested)
    nest_epollfd();
// default to the number of CPUs
    if (!nthreads)
    nthreads = perf_cpu_map__nr(cpu);
    worker = calloc(nthreads, sizeof(*worker));
    if (!worker)
    goto errmem;
    if (getrlimit(RLIMIT_NOFILE, &prevrl))
    err(EXIT_FAILURE, "getrlimit");
    rl.rlim_cur = rl.rlim_max = nfds * nthreads * 2 + 50;
    printinfo("Setting RLIMIT_NOFILE rlimit from %" PRIu64 " to: %" PRIu64 "\n",
    (uint64_t)prevrl.rlim_max, (uint64_t)rl.rlim_max);
    if (setrlimit(RLIMIT_NOFILE, &rl) < 0)
    err(EXIT_FAILURE, "setrlimit");
    printf("Run summary [PID %d]: %d threads doing epoll_ctl ops "
    "%d file-descriptors for %d secs.\n\n",
    getpid(), nthreads, nfds, nsecs);
    for (i = 0; i < EPOLL_NR_OPS; i++)
    init_stats(&all_stats[i]);
    mutex_init(&thread_lock);
    cond_init(&thread_parent);
    cond_init(&thread_worker);
    threads_starting = nthreads;
    gettimeofday(&bench__start, core::ptr::null_mut());
    do_threads(worker, cpu);
    mutex_lock(&thread_lock);
    while (threads_starting)
    cond_wait(&thread_parent, &thread_lock);
    cond_broadcast(&thread_worker);
    mutex_unlock(&thread_lock);
    sleep(nsecs);
    toggle_done(0, core::ptr::null_mut(), core::ptr::null_mut());
    printinfo("main thread: toggling done\n");
    for (i = 0; i < nthreads; i++) {
    ret = pthread_join(worker[i].thread, core::ptr::null_mut());
    if (ret)
    err(EXIT_FAILURE, "pthread_join");
    }
// cleanup & report results
    cond_destroy(&thread_parent);
    cond_destroy(&thread_worker);
    mutex_destroy(&thread_lock);
    for (i = 0; i < nthreads; i++) {
    unsigned long t[EPOLL_NR_OPS];
    for (j = 0; j < EPOLL_NR_OPS; j++) {
    t[j] = worker[i].ops[j];
    update_stats(&all_stats[j], t[j]);
    }
    if (nfds == 1)
    printf("[thread %2d] fdmap: %p [ add: %04ld; mod: %04ld; del: %04lds ops ]\n",
    worker[i].tid, &worker[i].fdmap[0],
    t[OP_EPOLL_ADD], t[OP_EPOLL_MOD], t[OP_EPOLL_DEL]);
    else
    printf("[thread %2d] fdmap: %p ... %p [ add: %04ld ops; mod: %04ld ops; del: %04ld ops ]\n",
    worker[i].tid, &worker[i].fdmap[0],
    &worker[i].fdmap[nfds-1],
    t[OP_EPOLL_ADD], t[OP_EPOLL_MOD], t[OP_EPOLL_DEL]);
    }
    print_summary();
    close(epollfd);
    perf_cpu_map__put(cpu);
    for (i = 0; i < nthreads; i++)
    free(worker[i].fdmap);
    free(worker);
    return ret;
    errmem:
    err(EXIT_FAILURE, "calloc");
    }
