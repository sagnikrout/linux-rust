//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/epoll-wait.c
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
// This program benchmarks concurrent epoll_wait(2) monitoring multiple
// file descriptors under one or two load balancing models. The first,
// and default, is the single/combined queueing (which refers to a single
// epoll instance for N worker threads):
//
// |---> [worker A]
// |---> [worker B]
// [combined queue]  .---> [worker C]
// |---> [worker D]
// |---> [worker E]
//
// While the second model, enabled via --multiq option, uses multiple
// queueing (which refers to one epoll instance per worker). For example,
// short lived tcp connections in a high throughput httpd server will
// distribute the accept()'ing  connections across CPUs. In this case each
// worker does a limited  amount of processing.
//
// [queue A]  ---> [worker]
// [queue B]  ---> [worker]
// [queue C]  ---> [worker]
// [queue D]  ---> [worker]
// [queue E]  ---> [worker]
//
// Naturally, the single queue will enforce more concurrency on the epoll
// instance, and can therefore scale poorly compared to multiple queues.
// However, this is a benchmark raw data and must be taken with a grain of
// salt when choosing how to make use of sys_epoll.
// Each thread has a number of private, nonblocking file descriptors,
// referred to as fdmap. A writer thread will constantly be writing to
// the fdmaps of all threads, minimizing each threads's chances of
// epoll_wait not finding any ready read events and blocking as this
// is not what we want to stress. The size of the fdmap can be adjusted
// by the user; enlarging the value will increase the chances of
// epoll_wait(2) blocking as the lineal writer thread will take "longer",
// at least at a high level.
//
// Note that because fds are private to each thread, this workload does
// not stress scenarios where multiple tasks are awoken per ready IO; ie:
// EPOLLEXCLUSIVE semantics.
//
// The end result/metric is throughput: number of ops/second where an
// operation consists of:
//
// epoll_wait(2) + [others]
//
// ... where [others] is the cost of re-adding the fd (EPOLLET),
// or rearming it (EPOLLONESHOT).
//
// The purpose of this is program is that it be useful for measuring
// kernel related changes to the sys_epoll, and not comparing different
// IO polling methods, for example. Hence everything is very adhoc and
// outputs raw microbenchmark numbers. Also this uses eventfd, similar
// tools tend to use pipes or sockets, but the result is the same.
//
// For the CLR_() macros

    do { if (__verbose) { printf(fmt, ## arg); fflush(stdout); } } while (0)
    let mut nthreads: static unsigned int = 0;
    let mut nsecs: static unsigned int = 8;
    static bool wdone, done, __verbose, randomize, nonblocking;
//
// epoll related shared variables.
//
// Maximum number of nesting allowed inside epoll sets
pub const EPOLL_MAXNESTS: c_int = 4;
    static int epollfd;
    static int *epollfdp;
    static bool noaffinity;
    let mut nested: static unsigned int = 0;
    static bool et; /* edge-trigger */
    static bool oneshot;
    static bool multiq; /* use an epoll instance per thread */
// amount of fds to monitor, per thread
    let mut nfds: static unsigned int = 64;
    static struct mutex thread_lock;
    static unsigned int threads_starting;
    static struct stats throughput_stats;
    static struct cond thread_parent, thread_worker;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct worker {
    pub tid: c_int,
    pub /: *mut *mut int epollfd; / for --multiq,
    pub thread: pthread_t,
    pub ops: c_ulong,
    pub fdmap: *mut c_int,
}

    static const struct option options[] = {
// general benchmark options
    OPT_UINTEGER('t', "threads", &nthreads, "Specify amount of threads"),
    OPT_UINTEGER('r', "runtime", &nsecs, "Specify runtime (in seconds)"),
    OPT_UINTEGER('f', "nfds",    &nfds,  "Specify amount of file descriptors to monitor for each thread"),
    OPT_BOOLEAN( 'n', "noaffinity",  &noaffinity,   "Disables CPU affinity"),
    OPT_BOOLEAN('R', "randomize", &randomize,   "Enable random write behaviour (default is lineal)"),
    OPT_BOOLEAN( 'v', "verbose", &__verbose, "Verbose mode"),
// epoll specific options
    OPT_BOOLEAN( 'm', "multiq",  &multiq,   "Use multiple epoll instances (one per thread)"),
    OPT_BOOLEAN( 'B', "nonblocking", &nonblocking, "Nonblocking epoll_wait(2) behaviour"),
    OPT_UINTEGER( 'N', "nested",  &nested,   "Nesting level epoll hierarchy (default is 0, no nesting)"),
    OPT_BOOLEAN( 'S', "oneshot",  &oneshot,   "Use EPOLLONESHOT semantics"),
    OPT_BOOLEAN( 'E', "edge",  &et,   "Use Edge-triggered interface (default is LT)"),
    OPT_END()
    };
    static const char * const bench_epoll_wait_usage[] = {
    "perf bench epoll wait <options>",
    core::ptr::null_mut()
    };
//
// Arrange the N elements of ARRAY in random order.
// Only effective if N is much smaller than RAND_MAX;
// if this may not be the case, use a better random
// number generator. -- Ben Pfaff.
//
#[no_mangle]
unsafe extern "C" fn shuffle(array: *mut c_void, n: usize, size: usize) {
    static void shuffle(void *array, size_t n, size_t size)
    {
    char *carray = array;
    void *aux;
    size_t i;
    if (n <= 1)
    return;
    aux = calloc(1, size);
    if (!aux)
    err(EXIT_FAILURE, "calloc");
    for (i = 1; i < n; ++i) {
    let mut j: usize = i + rand() / (RAND_MAX / (n - i) + 1);
    j *= size;
    memcpy(aux, &carray[j], size);
    memcpy(&carray[j], &carray[i*size], size);
    memcpy(&carray[i*size], aux, size);
    }
    free(aux);
    }
    static void *workerfn(void *arg)
    {
    int fd, ret, r;
    struct worker *w = (struct worker *) arg;
    let mut ops: c_ulong = w.ops;
    struct epoll_event ev;
    uint64_t val;
    let mut to: c_int = nonblocking? 0 : -1;
    let mut efd: c_int = multiq ? w.epollfd : epollfd;
    mutex_lock(&thread_lock);
    threads_starting--;
    if (!threads_starting)
    cond_signal(&thread_parent);
    cond_wait(&thread_worker, &thread_lock);
    mutex_unlock(&thread_lock);
    do {
//
// Block indefinitely waiting for the IN event.
// In order to stress the epoll_wait(2) syscall,
// call it event per event, instead of a larger
// batch (max)limit.
//
    do {
    ret = epoll_wait(efd, &ev, 1, to);
    } while (ret < 0 && errno == EINTR);
    if (ret < 0)
    err(EXIT_FAILURE, "epoll_wait");
    fd = ev.data.fd;
    do {
    r = read(fd, &val, sizeof(val));
    } while (!done && (r < 0 && errno == EAGAIN));
    if (et) {
    ev.events = EPOLLIN | EPOLLET;
    ret = epoll_ctl(efd, EPOLL_CTL_ADD, fd, &ev);
    }
    if (oneshot) {
// rearm the file descriptor with a new event mask
    ev.events |= EPOLLIN | EPOLLONESHOT;
    ret = epoll_ctl(efd, EPOLL_CTL_MOD, fd, &ev);
    }
    ops++;
    }  while (!done);
    if (multiq)
    close(w.epollfd);
    w.ops = ops;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn nest_epollfd(w: *mut worker) {
    static void nest_epollfd(struct worker *w)
    {
    unsigned int i;
    struct epoll_event ev;
    let mut efd: c_int = multiq ? w.epollfd : epollfd;
    if (nested > EPOLL_MAXNESTS)
    nested = EPOLL_MAXNESTS;
    epollfdp = calloc(nested, sizeof(*epollfdp));
    if (!epollfdp)
    err(EXIT_FAILURE, "calloc");
    for (i = 0; i < nested; i++) {
    epollfdp[i] = epoll_create(1);
    if (epollfdp[i] < 0)
    err(EXIT_FAILURE, "epoll_create");
    }
    ev.events = EPOLLHUP; /* anything */
    ev.data.u64 = i; /* any number */
    for (i = nested - 1; i; i--) {
    if (epoll_ctl(epollfdp[i - 1], EPOLL_CTL_ADD,
    epollfdp[i], &ev) < 0)
    err(EXIT_FAILURE, "epoll_ctl");
    }
    if (epoll_ctl(efd, EPOLL_CTL_ADD, *epollfdp, &ev) < 0)
    err(EXIT_FAILURE, "epoll_ctl");
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
    printf("\nAveraged %ld operations/sec (+- %.2f%%), total secs = %d\n",
    avg, rel_stddev_stats(stddev, avg),
    (int)bench__runtime.tv_sec);
    }
#[no_mangle]
unsafe extern "C" fn do_threads(worker: *mut worker, cpu: *mut perf_cpu_map) -> c_int {
    static int do_threads(struct worker *worker, struct perf_cpu_map *cpu)
    {
    pthread_attr_t thread_attr, *attrp = core::ptr::null_mut();
    cpu_set_t *cpuset;
    unsigned int i, j;
    let mut ret: c_int = 0, events = EPOLLIN;
    int nrcpus;
    size_t size;
    if (oneshot)
    events |= EPOLLONESHOT;
    if (et)
    events |= EPOLLET;
    printinfo("starting worker/consumer %sthreads%s\n",
    noaffinity ?  "":"CPU affinity ",
    nonblocking ? " (nonblocking)":"");
    if (!noaffinity)
    pthread_attr_init(&thread_attr);
    nrcpus = cpu__max_cpu().cpu;
    cpuset = CPU_ALLOC(nrcpus);
    BUG_ON(!cpuset);
    size = CPU_ALLOC_SIZE(nrcpus);
    for (i = 0; i < nthreads; i++) {
    struct worker *w = &worker[i];
    if (multiq) {
    w.epollfd = epoll_create(1);
    if (w.epollfd < 0)
    err(EXIT_FAILURE, "epoll_create");
    if (nested)
    nest_epollfd(w);
    }
    w.tid = i;
    w.fdmap = calloc(nfds, sizeof(int));
    if (!w.fdmap)
    return 1;
    for (j = 0; j < nfds; j++) {
    let mut efd: c_int = multiq ? w.epollfd : epollfd;
    struct epoll_event ev;
    w.fdmap[j] = eventfd(0, EFD_NONBLOCK);
    if (w.fdmap[j] < 0)
    err(EXIT_FAILURE, "eventfd");
    ev.data.fd = w.fdmap[j];
    ev.events = events;
    ret = epoll_ctl(efd, EPOLL_CTL_ADD,
    w.fdmap[j], &ev);
    if (ret < 0)
    err(EXIT_FAILURE, "epoll_ctl");
    }
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
    static void *writerfn(void *p)
    {
    struct worker *worker = p;
    size_t i, j, iter;
    let mut val: u64 = 1;
    ssize_t sz;
    struct timespec ts = { .tv_sec = 0,
    .tv_nsec = 500 };
    printinfo("starting writer-thread: doing %s writes ...\n",
    randomize? "random":"lineal");
    for (iter = 0; !wdone; iter++) {
    if (randomize) {
    shuffle((void *)worker, nthreads, sizeof(*worker));
    }
    for (i = 0; i < nthreads; i++) {
    struct worker *w = &worker[i];
    if (randomize) {
    shuffle((void *)w.fdmap, nfds, sizeof(int));
    }
    for (j = 0; j < nfds; j++) {
    do {
    sz = write(w.fdmap[j], &val, sizeof(val));
    } while (!wdone && (sz < 0 && errno == EAGAIN));
    }
    }
    nanosleep(&ts, core::ptr::null_mut());
    }
    printinfo("exiting writer-thread (total full-loops: %zd)\n", iter);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cmpworker(p1: *const c_void, p2: *const c_void) -> c_int {
    static int cmpworker(const void *p1, const void *p2)
    {
    struct worker *w1 = (struct worker *) p1;
    struct worker *w2 = (struct worker *) p2;
    if (w1.tid > w2.tid)
    return 1;
    if (w1.tid < w2.tid)
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bench_epoll_wait(argc: c_int, argv: *const c_char) -> c_int {
    int bench_epoll_wait(int argc, const char **argv)
    {
    let mut ret: c_int = 0;
    struct sigaction act;
    unsigned int i;
    struct worker *worker = core::ptr::null_mut();
    struct perf_cpu_map *cpu;
    pthread_t wthread;
    struct rlimit rl, prevrl;
    argc = parse_options(argc, argv, options, bench_epoll_wait_usage, 0);
    if (argc) {
    usage_with_options(bench_epoll_wait_usage, options);
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
    if (!multiq) {
    epollfd = epoll_create(1);
    if (epollfd < 0)
    err(EXIT_FAILURE, "epoll_create");
//
// Deal with nested epolls, if any.
//
    if (nested)
    nest_epollfd(core::ptr::null_mut());
    }
    printinfo("Using %s queue model\n", multiq ? "multi" : "single");
    printinfo("Nesting level(s): %d\n", nested);
// default to the number of CPUs and leave one for the writer pthread
    if (!nthreads)
    nthreads = perf_cpu_map__nr(cpu) - 1;
    worker = calloc(nthreads, sizeof(*worker));
    if (!worker) {
    goto errmem;
    }
    if (getrlimit(RLIMIT_NOFILE, &prevrl))
    err(EXIT_FAILURE, "getrlimit");
    rl.rlim_cur = rl.rlim_max = nfds * nthreads * 2 + 50;
    printinfo("Setting RLIMIT_NOFILE rlimit from %" PRIu64 " to: %" PRIu64 "\n",
    (uint64_t)prevrl.rlim_max, (uint64_t)rl.rlim_max);
    if (setrlimit(RLIMIT_NOFILE, &rl) < 0)
    err(EXIT_FAILURE, "setrlimit");
    printf("Run summary [PID %d]: %d threads monitoring%s on "
    "%d file-descriptors for %d secs.\n\n",
    getpid(), nthreads, oneshot ? " (EPOLLONESHOT semantics)": "", nfds, nsecs);
    init_stats(&throughput_stats);
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
//
// At this point the workers should be blocked waiting for read events
// to become ready. Launch the writer which will constantly be writing
// to each thread's fdmap.
//
    ret = pthread_create(&wthread, core::ptr::null_mut(), writerfn,
    (void *)(struct worker *) worker);
    if (ret)
    err(EXIT_FAILURE, "pthread_create");
    sleep(nsecs);
    toggle_done(0, core::ptr::null_mut(), core::ptr::null_mut());
    printinfo("main thread: toggling done\n");
    sleep(1); /* meh */
    wdone = true;
    ret = pthread_join(wthread, core::ptr::null_mut());
    if (ret)
    err(EXIT_FAILURE, "pthread_join");
// cleanup & report results
    cond_destroy(&thread_parent);
    cond_destroy(&thread_worker);
    mutex_destroy(&thread_lock);
// sort the array back before reporting
    if (randomize)
    qsort(worker, nthreads, sizeof(struct worker), cmpworker);
    for (i = 0; i < nthreads; i++) {
    unsigned long t = bench__runtime.tv_sec > 0 ?
    worker[i].ops / bench__runtime.tv_sec : 0;
    update_stats(&throughput_stats, t);
    if (nfds == 1)
    printf("[thread %2d] fdmap: %p [ %04ld ops/sec ]\n",
    worker[i].tid, &worker[i].fdmap[0], t);
    else
    printf("[thread %2d] fdmap: %p ... %p [ %04ld ops/sec ]\n",
    worker[i].tid, &worker[i].fdmap[0],
    &worker[i].fdmap[nfds-1], t);
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
