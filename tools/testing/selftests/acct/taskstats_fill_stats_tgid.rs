//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/acct/taskstats_fill_stats_tgid.c
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
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct worker_ctx {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub ready: bool,
    pub release: bool,
}

    static unsigned long busy_sink;
    static uint64_t timespec_diff_ns(const struct timespec *start,
    const struct timespec *end)
    {
    return (uint64_t)(end.tv_sec - start.tv_sec) * 1000000000ULL +
    (uint64_t)(end.tv_nsec - start.tv_nsec);
    }
#[no_mangle]
unsafe extern "C" fn burn_cpu_for_ns(runtime_ns: u64) {
    static void burn_cpu_for_ns(uint64_t runtime_ns)
    {
    struct timespec start, now;
    let mut acc: c_ulong = 0;
    if (clock_gettime(CLOCK_MONOTONIC, &start)) {
    perror("clock_gettime");
    exit(EXIT_FAILURE);
    }
    do {
    for (int i = 0; i < 100000; i++)
    acc += i;
    if (clock_gettime(CLOCK_MONOTONIC, &now)) {
    perror("clock_gettime");
    exit(EXIT_FAILURE);
    }
    } while (timespec_diff_ns(&start, &now) < runtime_ns);
    busy_sink = acc;
    }
    static int get_taskstats(int fd, int family_id, uint16_t attr_type, uint32_t id,
    struct taskstats *stats)
    {
    struct {
    struct nlmsghdr nlh;
    struct genlmsghdr genl;
    char buf[256];
    } req = { 0 };
    char resp[16384];
    struct nlmsghdr *nlh;
    struct genlmsghdr *genl;
    struct nlattr *na;
    struct nlattr *nested;
    int len;
    int rem;
    int nrem;
    int ret;
    memset(stats, 0, sizeof(*stats));
    req.nlh.nlmsg_len = NLMSG_LENGTH(GENL_HDRLEN);
    req.nlh.nlmsg_type = family_id;
    req.nlh.nlmsg_flags = NLM_F_REQUEST;
    req.nlh.nlmsg_seq = 2;
    req.nlh.nlmsg_pid = getpid();
    req.genl.cmd = TASKSTATS_CMD_GET;
    req.genl.version = 1;
    na = (struct nlattr *)((char *)&req + NLMSG_ALIGN(req.nlh.nlmsg_len));
    na.nla_type = attr_type;
    na.nla_len = NLA_HDRLEN + sizeof(id);
    memcpy(nla_data(na), &id, sizeof(id));
    req.nlh.nlmsg_len = NLMSG_ALIGN(req.nlh.nlmsg_len) + NLA_ALIGN(na.nla_len);
    ret = send_request(fd, &req, req.nlh.nlmsg_len);
    if (ret)
    return ret;
    len = recv(fd, resp, sizeof(resp), 0);
    if (len < 0)
    return -errno;
    for (nlh = (struct nlmsghdr *)resp; NLMSG_OK(nlh, len);
    nlh = NLMSG_NEXT(nlh, len)) {
    if (nlh.nlmsg_type == NLMSG_ERROR) {
    struct nlmsgerr *err = NLMSG_DATA(nlh);
    return err.error ? err.error : -ENOENT;
    }
    genl = (struct genlmsghdr *)NLMSG_DATA(nlh);
    rem = nlh.nlmsg_len - NLMSG_HDRLEN - GENL_HDRLEN;
    na = (struct nlattr *)((char *)genl + GENL_HDRLEN);
    while (nla_ok(na, rem)) {
    if (na.nla_type == TASKSTATS_TYPE_AGGR_PID ||
    na.nla_type == TASKSTATS_TYPE_AGGR_TGID) {
    nested = (struct nlattr *)nla_data(na);
    nrem = na.nla_len - NLA_HDRLEN;
    while (nla_ok(nested, nrem)) {
    if (nested.nla_type == TASKSTATS_TYPE_STATS) {
    memcpy(stats, nla_data(nested),
    sizeof(*stats));
    return 0;
    }
    nested = nla_next(nested, &nrem);
    }
    }
    na = nla_next(na, &rem);
    }
    }
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn cpu_total(stats: *const taskstats) -> u64 {
    static uint64_t cpu_total(const struct taskstats *stats)
    {
    return (uint64_t)stats.ac_utime + (uint64_t)stats.ac_stime;
    }
#[no_mangle]
unsafe extern "C" fn print_stats(label: *const c_char, stats: *const taskstats) {
    static void print_stats(const char *label, const struct taskstats *stats)
    {
    ksft_print_msg("%s: cpu_total=%llu nvcsw=%llu nivcsw=%llu\n",
    label, (unsigned long long)cpu_total(stats),
    (unsigned long long)stats.nvcsw,
    (unsigned long long)stats.nivcsw);
    }
    static void *worker_thread(void *arg)
    {
    struct worker_ctx *ctx = arg;
    burn_cpu_for_ns(BUSY_NS);
    pthread_mutex_lock(&ctx.lock);
    ctx.ready = true;
    pthread_cond_broadcast(&ctx.cond);
    while (!ctx.release)
    pthread_cond_wait(&ctx.cond, &ctx.lock);
    pthread_mutex_unlock(&ctx.lock);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct worker_ctx ctx = {
    .lock = PTHREAD_MUTEX_INITIALIZER,
    .cond = PTHREAD_COND_INITIALIZER,
    };
    struct taskstats before, after;
    pthread_t thread;
    let mut tgid: pid_t = getpid();
    int family_id;
    int fd;
    int ret;
    ksft_print_header();
    ksft_set_plan(1);
    if (geteuid())
    ksft_exit_skip("taskstats_fill_stats_tgid needs root\n");
    fd = netlink_open();
    if (fd < 0)
    ksft_exit_skip("failed to open generic netlink socket: %s\n",
    strerror(-fd));
    family_id = get_family_id(fd, TASKSTATS_GENL_NAME);
    if (family_id < 0)
    ksft_exit_skip("taskstats generic netlink family unavailable: %s\n",
    strerror(-family_id));
// Create worker thread that burns 200ms of CPU
    if (pthread_create(&thread, core::ptr::null_mut(), worker_thread, &ctx) != 0)
    ksft_exit_fail_msg("pthread_create failed: %s\n", strerror(errno));
// Wait for worker to finish generating activity
    pthread_mutex_lock(&ctx.lock);
    while (!ctx.ready)
    pthread_cond_wait(&ctx.cond, &ctx.lock);
    pthread_mutex_unlock(&ctx.lock);
//
// Snapshot A: TGID stats while worker is alive and sleeping.
// Contains main thread + worker contributions.
//
    ret = get_taskstats(fd, family_id, TASKSTATS_CMD_ATTR_TGID, tgid, &before);
    if (ret)
    ksft_exit_fail_msg("TGID query before exit failed: %s\n",
    strerror(-ret));
// Release worker so it can exit, then join (deterministic wait).
//
// Kernel exit path ordering guarantees:
// do_exit()
// taskstats_exit() -> fill_tgid_exit()  (accumulates worker into signal->stats)
// exit_notify()                         (releases the thread)
// do_task_dead() -> __schedule()        (wakes joiner)
//
// So pthread_join() returns only after fill_tgid_exit() has completed.
//
    pthread_mutex_lock(&ctx.lock);
    ctx.release = true;
    pthread_cond_broadcast(&ctx.cond);
    pthread_mutex_unlock(&ctx.lock);
    pthread_join(thread, core::ptr::null_mut());
//
// Snapshot B: TGID stats after worker has exited.
// fill_stats_for_tgid() does:
// memcpy(signal->stats)   <- includes fill_tgid_exit accumulation
// + scan live threads      <- only main thread now
//
    ret = get_taskstats(fd, family_id, TASKSTATS_CMD_ATTR_TGID, tgid, &after);
    if (ret)
    ksft_exit_fail_msg("TGID query after exit failed: %s\n",
    strerror(-ret));
    print_stats("TGID before worker exit", &before);
    print_stats("TGID after  worker exit", &after);
//
// The worker burned 200ms of CPU before the first snapshot.
// If the kernel correctly retained its contribution via
// fill_tgid_exit(), then the TGID CPU total after exit must be at
// least as large as the TGID CPU total before exit.
//
    ksft_test_result(cpu_total(&after) >= cpu_total(&before),
    "TGID CPU stats should not regress after thread exit\n");
    close(fd);
    ksft_finished();
    return ksft_get_fail_cnt() ? KSFT_FAIL : KSFT_PASS;
    }
