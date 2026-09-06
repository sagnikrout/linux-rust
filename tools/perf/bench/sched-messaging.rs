//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/sched-messaging.c
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
// sched-messaging.c
//
// messaging: Benchmark for scheduler and IPC mechanisms
//
// Based on hackbench by Rusty Russell <rusty@rustcorp.com.au>
// Ported to perf by Hitoshi Mitake <mitake@dcl.info.waseda.ac.jp>
//

// Test groups of 20 processes spraying to 20 receivers

pub const DATASIZE: c_int = 100;
    let mut use_pipes: static bool = false;
    let mut nr_loops: static unsigned int = 100;
    let mut thread_mode: static bool = false;
    let mut num_groups: static unsigned int = 10;
    let mut total_children: static unsigned int = 0;
    let mut sender_contexts: static struct list_head = LIST_HEAD_INIT(sender_contexts);
    let mut receiver_contexts: static struct list_head = LIST_HEAD_INIT(receiver_contexts);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sender_context {
    pub list: list_head,
    pub num_fds: c_uint,
    pub ready_out: c_int,
    pub wakefd: c_int,
    pub out_fds: [c_int; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct receiver_context {
    pub list: list_head,
    pub num_packets: c_uint,
    pub in_fds: [c_int; 2],
    pub ready_out: c_int,
    pub wakefd: c_int,
}

    union messaging_worker {
    pthread_t thread;
    pid_t pid;
    };
    static union messaging_worker *worker_tab;
#[no_mangle]
unsafe extern "C" fn fdpair(fds[2]: c_int) {
    static void fdpair(int fds[2])
    {
    if (use_pipes) {
    if (pipe(fds) == 0)
    return;
    } else {
    if (socketpair(AF_UNIX, SOCK_STREAM, 0, fds) == 0)
    return;
    }
    err(EXIT_FAILURE, use_pipes ? "pipe()" : "socketpair()");
    }
// Block until we're ready to go
#[no_mangle]
unsafe extern "C" fn ready(ready_out: c_int, wakefd: c_int) {
    static void ready(int ready_out, int wakefd)
    {
    let mut pollfd: pollfd = { .fd = wakefd, .events = POLLIN };
// Tell them we're ready.
    if (write(ready_out, "R", 1) != 1)
    err(EXIT_FAILURE, "CLIENT: ready write");
// Wait for "GO" signal
    if (poll(&pollfd, 1, -1) != 1)
    err(EXIT_FAILURE, "poll");
    }
// Sender sprays nr_loops messages down each file descriptor
    static void *sender(struct sender_context *ctx)
    {
    char data[DATASIZE];
    unsigned int i, j;
    ready(ctx.ready_out, ctx.wakefd);
    memset(data, 'S', sizeof(data));
// Now pump to every receiver.
    for (i = 0; i < nr_loops; i++) {
    for (j = 0; j < ctx.num_fds; j++) {
    int ret, done = 0;
    again:
    ret = write(ctx.out_fds[j], data + done,
    sizeof(data) - done);
    if (ret < 0)
    err(EXIT_FAILURE, "SENDER: write");
    done += ret;
    if (done < DATASIZE)
    goto again;
    }
    }
    return core::ptr::null_mut();
    }
// One receiver per fd
    static void *receiver(struct receiver_context* ctx)
    {
    unsigned int i;
    if (!thread_mode)
    close(ctx.in_fds[1]);
// Wait for start...
    ready(ctx.ready_out, ctx.wakefd);
// Receive them all
    for (i = 0; i < ctx.num_packets; i++) {
    char data[DATASIZE];
    int ret, done = 0;
    again:
    ret = read(ctx.in_fds[0], data + done, DATASIZE - done);
    if (ret < 0)
    err(EXIT_FAILURE, "SERVER: read");
    done += ret;
    if (done < DATASIZE)
    goto again;
    }
    return core::ptr::null_mut();
    }
    static void create_thread_worker(union messaging_worker *worker,
    void *ctx, void *(*func)(void *))
    {
    pthread_attr_t attr;
    int ret;
    if (pthread_attr_init(&attr) != 0)
    err(EXIT_FAILURE, "pthread_attr_init:");

    if (pthread_attr_setstacksize(&attr, PTHREAD_STACK_MIN) != 0)
    err(EXIT_FAILURE, "pthread_attr_setstacksize");

    ret = pthread_create(&worker.thread, &attr, func, ctx);
    if (ret != 0)
    err(EXIT_FAILURE, "pthread_create failed");
    pthread_attr_destroy(&attr);
    }
    static void create_process_worker(union messaging_worker *worker,
    void *ctx, void *(*func)(void *))
    {
// Fork the receiver.
    worker.pid = fork();
    if (worker.pid == -1) {
    err(EXIT_FAILURE, "fork()");
    } else if (worker.pid == 0) {
    (*func) (ctx);
    exit(0);
    }
    }
    static void create_worker(union messaging_worker *worker,
    void *ctx, void *(*func)(void *))
    {
    if (!thread_mode)
    return create_process_worker(worker, ctx, func);
    else
    return create_thread_worker(worker, ctx, func);
    }
#[no_mangle]
unsafe extern "C" fn reap_worker(worker: *mut union messaging_worker) {
    static void reap_worker(union messaging_worker *worker)
    {
    int proc_status;
    void *thread_status;
    if (!thread_mode) {
// process mode
    wait(&proc_status);
    if (!WIFEXITED(proc_status))
    exit(1);
    } else {
    pthread_join(worker.thread, &thread_status);
    }
    }
// One group of senders and receivers
    static unsigned int group(union messaging_worker *worker,
    unsigned int num_fds,
    int ready_out,
    int wakefd)
    {
    unsigned int i;
    struct sender_context *snd_ctx = malloc(sizeof(struct sender_context) +
    num_fds * sizeof(int));
    if (!snd_ctx)
    err(EXIT_FAILURE, "malloc()");
    list_add(&snd_ctx.list, &sender_contexts);
    for (i = 0; i < num_fds; i++) {
    int fds[2];
    struct receiver_context *ctx = malloc(sizeof(*ctx));
    if (!ctx)
    err(EXIT_FAILURE, "malloc()");
    list_add(&ctx.list, &receiver_contexts);
// Create the pipe between client and server
    fdpair(fds);
    ctx.num_packets = num_fds * nr_loops;
    ctx.in_fds[0] = fds[0];
    ctx.in_fds[1] = fds[1];
    ctx.ready_out = ready_out;
    ctx.wakefd = wakefd;
    create_worker(worker + i, ctx, (void *)receiver);
    snd_ctx.out_fds[i] = fds[1];
    if (!thread_mode)
    close(fds[0]);
    }
// Now we have all the fds, fork the senders
    for (i = 0; i < num_fds; i++) {
    snd_ctx.ready_out = ready_out;
    snd_ctx.wakefd = wakefd;
    snd_ctx.num_fds = num_fds;
    create_worker(worker + num_fds + i, snd_ctx, (void *)sender);
    }
// Close the fds we have left
    if (!thread_mode)
    for (i = 0; i < num_fds; i++)
    close(snd_ctx.out_fds[i]);
// Return number of children to reap
    return num_fds * 2;
    }
#[no_mangle]
unsafe extern "C" fn sig_handler(__maybe_unused: int sig) {
    static void sig_handler(int sig __maybe_unused)
    {
    unsigned int i;
//
// When exit abnormally, kill all forked child processes.
//
    for (i = 0; i < total_children; i++)
    kill(worker_tab[i].pid, SIGKILL);
    }
    static const struct option options[] = {
    OPT_BOOLEAN('p', "pipe", &use_pipes,
    "Use pipe() instead of socketpair()"),
    OPT_BOOLEAN('t', "thread", &thread_mode,
    "Be multi thread instead of multi process"),
    OPT_UINTEGER('g', "group", &num_groups, "Specify number of groups"),
    OPT_UINTEGER('l', "nr_loops", &nr_loops, "Specify the number of loops to run (default: 100)"),
    OPT_END()
    };
    static const char * const bench_sched_message_usage[] = {
    "perf bench sched messaging <options>",
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn bench_sched_messaging(argc: c_int, argv: *const c_char) -> c_int {
    int bench_sched_messaging(int argc, const char **argv)
    {
    unsigned int i;
    struct timeval start, stop, diff;
    let mut num_fds: c_uint = 20;
    int readyfds[2], wakefds[2];
    char dummy;
    struct sender_context *pos, *n;
    argc = parse_options(argc, argv, options,
    bench_sched_message_usage, 0);
    worker_tab = calloc(num_fds * 2 * num_groups, sizeof(union messaging_worker));
    if (!worker_tab)
    err(EXIT_FAILURE, "main:malloc()");
    fdpair(readyfds);
    fdpair(wakefds);
    if (!thread_mode) {
    signal(SIGINT, sig_handler);
    signal(SIGTERM, sig_handler);
    }
    for (i = 0; i < num_groups; i++)
    total_children += group(worker_tab + total_children, num_fds,
    readyfds[1], wakefds[0]);
// Wait for everyone to be ready
    for (i = 0; i < total_children; i++)
    if (read(readyfds[0], &dummy, 1) != 1)
    err(EXIT_FAILURE, "Reading for readyfds");
    gettimeofday(&start, core::ptr::null_mut());
// Kick them off
    if (write(wakefds[1], &dummy, 1) != 1)
    err(EXIT_FAILURE, "Writing to start them");
// Reap them all
    for (i = 0; i < total_children; i++)
    reap_worker(worker_tab + i);
    gettimeofday(&stop, core::ptr::null_mut());
    timersub(&stop, &start, &diff);
    switch (bench_format) {
    case BENCH_FORMAT_DEFAULT:
    printf("# %d sender and receiver %s per group\n",
    num_fds, thread_mode ? "threads" : "processes");
    printf("# %d groups == %d %s run\n\n",
    num_groups, num_groups * 2 * num_fds,
    thread_mode ? "threads" : "processes");
    printf(" %14s: %lu.%03lu [sec]\n", "Total time",
    (unsigned long) diff.tv_sec,
    (unsigned long) (diff.tv_usec / USEC_PER_MSEC));
    break;
    case BENCH_FORMAT_SIMPLE:
    printf("%lu.%03lu\n", (unsigned long) diff.tv_sec,
    (unsigned long) (diff.tv_usec / USEC_PER_MSEC));
    break;
    default:
// reaching here is something disaster
    fprintf(stderr, "Unknown format:%d\n", bench_format);
    exit(1);
    break;
    }
    free(worker_tab);
    list_for_each_entry_safe(pos, n, &sender_contexts, list) {
    list_del_init(&pos.list);
    free(pos);
    }
    list_for_each_entry_safe(pos, n, &receiver_contexts, list) {
    list_del_init(&pos.list);
    free(pos);
    }
    return 0;
    }
