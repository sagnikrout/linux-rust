//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/ptrace/perf-hwbreak.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// perf events self profiling example test case for hw breakpoints.
//
// This tests perf PERF_TYPE_BREAKPOINT parameters
// 1) tests all variants of the break on read/write flags
// 2) tests exclude_user == 0 and 1
// 3) test array matches (if DAWR is supported))
// 4) test different numbers of breakpoints matches
//
// Configure this breakpoint, then read and write the data a number of
// times. Then check the output count from perf is as expected.
//
// Based on:
// http://ozlabs.org/~anton/junkcode/perf_events_example1.c
//
// Copyright (C) 2018 Michael Neuling, IBM Corporation.
//
// Macro flag: #define _GNU_SOURCE

pub const PPC_DEBUG_FEATURE_DATA_BP_ARCH_31: c_uint = 0x20;

pub const MAX_LOOPS: c_int = 10000;

    int nprocs;
    let mut a: static volatile int = 10;
    let mut b: static volatile int = 10;
    static volatile char c[512 + 8] __attribute__((aligned(512)));
    static void perf_event_attr_set(struct perf_event_attr *attr,
    __u32 type, __u64 addr, __u64 len,
    bool exclude_user)
    {
    memset(attr, 0, sizeof(struct perf_event_attr));
    attr.type           = PERF_TYPE_BREAKPOINT;
    attr.size           = sizeof(struct perf_event_attr);
    attr.bp_type        = type;
    attr.bp_addr        = addr;
    attr.bp_len         = len;
    attr.exclude_kernel = 1;
    attr.exclude_hv     = 1;
    attr.exclude_guest  = 1;
    attr.exclude_user   = exclude_user;
    attr.disabled       = 1;
    }
    static int
    perf_process_event_open_exclude_user(__u32 type, __u64 addr, __u64 len, bool exclude_user)
    {
    struct perf_event_attr attr;
    perf_event_attr_set(&attr, type, addr, len, exclude_user);
    return syscall(__NR_perf_event_open, &attr, getpid(), -1, -1, 0);
    }
#[no_mangle]
unsafe extern "C" fn perf_process_event_open(type: __u32, addr: __u64, len: __u64) -> c_int {
    static int perf_process_event_open(__u32 type, __u64 addr, __u64 len)
    {
    struct perf_event_attr attr;
    perf_event_attr_set(&attr, type, addr, len, 0);
    return syscall(__NR_perf_event_open, &attr, getpid(), -1, -1, 0);
    }
#[no_mangle]
unsafe extern "C" fn perf_cpu_event_open(cpu: c_long, type: __u32, addr: __u64, len: __u64) -> c_int {
    static int perf_cpu_event_open(long cpu, __u32 type, __u64 addr, __u64 len)
    {
    struct perf_event_attr attr;
    perf_event_attr_set(&attr, type, addr, len, 0);
    return syscall(__NR_perf_event_open, &attr, -1, cpu, -1, 0);
    }
#[no_mangle]
unsafe extern "C" fn close_fds(fd: *mut c_int, n: c_int) {
    static void close_fds(int *fd, int n)
    {
    int i;
    for (i = 0; i < n; i++)
    close(fd[i]);
    }
#[no_mangle]
unsafe extern "C" fn read_fds(fd: *mut c_int, n: c_int) -> c_ulong {
    static unsigned long read_fds(int *fd, int n)
    {
    int i;
    let mut c: c_ulong = 0;
    let mut count: c_ulong = 0;
    size_t res;
    for (i = 0; i < n; i++) {
    res = read(fd[i], &c, sizeof(c));
    assert(res == sizeof(unsigned long long));
    count += c;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn reset_fds(fd: *mut c_int, n: c_int) {
    static void reset_fds(int *fd, int n)
    {
    int i;
    for (i = 0; i < n; i++)
    ioctl(fd[i], PERF_EVENT_IOC_RESET);
    }
#[no_mangle]
unsafe extern "C" fn enable_fds(fd: *mut c_int, n: c_int) {
    static void enable_fds(int *fd, int n)
    {
    int i;
    for (i = 0; i < n; i++)
    ioctl(fd[i], PERF_EVENT_IOC_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn disable_fds(fd: *mut c_int, n: c_int) {
    static void disable_fds(int *fd, int n)
    {
    int i;
    for (i = 0; i < n; i++)
    ioctl(fd[i], PERF_EVENT_IOC_DISABLE);
    }
#[no_mangle]
unsafe extern "C" fn perf_systemwide_event_open(fd: *mut c_int, type: __u32, addr: __u64, len: __u64) -> c_int {
    static int perf_systemwide_event_open(int *fd, __u32 type, __u64 addr, __u64 len)
    {
    int i, ncpus, cpu, ret = 0;
    struct rlimit rlim;
    cpu_set_t *mask;
    size_t size;
    if (getrlimit(RLIMIT_NOFILE, &rlim)) {
    perror("getrlimit");
    return -1;
    }
    rlim.rlim_cur = 65536;
    if (setrlimit(RLIMIT_NOFILE, &rlim)) {
    perror("setrlimit");
    return -1;
    }
    ncpus = get_nprocs_conf();
    size = CPU_ALLOC_SIZE(ncpus);
    mask = CPU_ALLOC(ncpus);
    if (!mask) {
    perror("malloc");
    return -1;
    }
    CPU_ZERO_S(size, mask);
    if (sched_getaffinity(0, size, mask)) {
    perror("sched_getaffinity");
    ret = -1;
    goto done;
    }
    for (i = 0, cpu = 0; i < nprocs && cpu < ncpus; cpu++) {
    if (!CPU_ISSET_S(cpu, size, mask))
    continue;
    fd[i] = perf_cpu_event_open(cpu, type, addr, len);
    if (fd[i] < 0) {
    perror("perf_systemwide_event_open");
    close_fds(fd, i);
    ret = fd[i];
    goto done;
    }
    i++;
    }
    if (i < nprocs) {
    printf("Error: Number of online cpus reduced since start of test: %d < %d\n", i, nprocs);
    close_fds(fd, i);
    ret = -1;
    }
    done:
    CPU_FREE(mask);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn breakpoint_test(len: c_int) -> bool {
    static inline bool breakpoint_test(int len)
    {
    int fd;
// bp_addr can point anywhere but needs to be aligned
    fd = perf_process_event_open(HW_BREAKPOINT_R, (__u64)(&fd) & 0xfffffffffffff800, len);
    if (fd < 0)
    return false;
    close(fd);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_breakpoint_supported() -> bool {
    static inline bool perf_breakpoint_supported(void)
    {
    return breakpoint_test(4);
    }
#[no_mangle]
pub unsafe extern "C" fn dawr_supported() -> bool {
    static inline bool dawr_supported(void)
    {
    return breakpoint_test(DAWR_LENGTH_MAX);
    }
#[no_mangle]
unsafe extern "C" fn runtestsingle(readwriteflag: c_int, exclude_user: c_int, arraytest: c_int) -> c_int {
    static int runtestsingle(int readwriteflag, int exclude_user, int arraytest)
    {
    int i,j;
    size_t res;
    unsigned long long breaks, needed;
    int readint;
    int readintarraybig[2*DAWR_LENGTH_MAX/sizeof(int)];
    int *readintalign;
    volatile int *ptr;
    int break_fd;
    int loop_num = MAX_LOOPS - (rand() % 100); /* provide some variability */
    volatile int *k;
    __u64 len;
// align to 0x400 boundary as required by DAWR
    readintalign = (int *)(((unsigned long)readintarraybig + 0x7ff) &
    0xfffffffffffff800);
    ptr = &readint;
    if (arraytest)
    ptr = &readintalign[0];
    len = arraytest ? DAWR_LENGTH_MAX : sizeof(int);
    break_fd = perf_process_event_open_exclude_user(readwriteflag, (__u64)ptr,
    len, exclude_user);
    if (break_fd < 0) {
    perror("perf_process_event_open_exclude_user");
    exit(1);
    }
// start counters
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
// Test a bunch of reads and writes
    k = &readint;
    for (i = 0; i < loop_num; i++) {
    if (arraytest)
    k = &(readintalign[i % (DAWR_LENGTH_MAX/sizeof(int))]);
    j = *k;
// k = j;
    }
// stop counters
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
// read and check counters
    res = read(break_fd, &breaks, sizeof(unsigned long long));
    assert(res == sizeof(unsigned long long));
// we read and write each loop, so subtract the ones we are counting
    needed = 0;
    if (readwriteflag & HW_BREAKPOINT_R)
    needed += loop_num;
    if (readwriteflag & HW_BREAKPOINT_W)
    needed += loop_num;
    needed = needed * (1 - exclude_user);
    printf("TESTED: addr:0x%lx brks:% 8lld loops:% 8i rw:%i !user:%i array:%i\n",
    (unsigned long int)ptr, breaks, loop_num, readwriteflag, exclude_user, arraytest);
    if (breaks != needed) {
    printf("FAILED: 0x%lx brks:%lld needed:%lli %i %i %i\n\n",
    (unsigned long int)ptr, breaks, needed, loop_num, readwriteflag, exclude_user);
    return 1;
    }
    close(break_fd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn runtest_dar_outside() -> c_int {
    static int runtest_dar_outside(void)
    {
    void *target;
    volatile __u16 temp16;
    volatile __u64 temp64;
    int break_fd;
    unsigned long long breaks;
    let mut fail: c_int = 0;
    size_t res;
    target = malloc(8);
    if (!target) {
    perror("malloc failed");
    exit(EXIT_FAILURE);
    }
// watch middle half of target array
    break_fd = perf_process_event_open(HW_BREAKPOINT_RW, (__u64)(target + 2), 4);
    if (break_fd < 0) {
    free(target);
    perror("perf_process_event_open");
    exit(EXIT_FAILURE);
    }
// Shouldn't hit.
    ioctl(break_fd, PERF_EVENT_IOC_RESET);
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
    temp16 = *((__u16 *)target);
// ((__u16 *)target) = temp16;
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
    res = read(break_fd, &breaks, sizeof(unsigned long long));
    assert(res == sizeof(unsigned long long));
    if (breaks == 0) {
    printf("TESTED: No overlap\n");
    } else {
    printf("FAILED: No overlap: %lld != 0\n", breaks);
    fail = 1;
    }
// Hit
    ioctl(break_fd, PERF_EVENT_IOC_RESET);
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
    temp16 = *((__u16 *)(target + 1));
// ((__u16 *)(target + 1)) = temp16;
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
    res = read(break_fd, &breaks, sizeof(unsigned long long));
    assert(res == sizeof(unsigned long long));
    if (breaks == 2) {
    printf("TESTED: Partial overlap\n");
    } else {
    printf("FAILED: Partial overlap: %lld != 2\n", breaks);
    fail = 1;
    }
// Hit
    ioctl(break_fd, PERF_EVENT_IOC_RESET);
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
    temp16 = *((__u16 *)(target + 5));
// ((__u16 *)(target + 5)) = temp16;
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
    res = read(break_fd, &breaks, sizeof(unsigned long long));
    assert(res == sizeof(unsigned long long));
    if (breaks == 2) {
    printf("TESTED: Partial overlap\n");
    } else {
    printf("FAILED: Partial overlap: %lld != 2\n", breaks);
    fail = 1;
    }
// Shouldn't Hit
    ioctl(break_fd, PERF_EVENT_IOC_RESET);
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
    temp16 = *((__u16 *)(target + 6));
// ((__u16 *)(target + 6)) = temp16;
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
    res = read(break_fd, &breaks, sizeof(unsigned long long));
    assert(res == sizeof(unsigned long long));
    if (breaks == 0) {
    printf("TESTED: No overlap\n");
    } else {
    printf("FAILED: No overlap: %lld != 0\n", breaks);
    fail = 1;
    }
// Hit
    ioctl(break_fd, PERF_EVENT_IOC_RESET);
    ioctl(break_fd, PERF_EVENT_IOC_ENABLE);
    temp64 = *((__u64 *)target);
// ((__u64 *)target) = temp64;
    ioctl(break_fd, PERF_EVENT_IOC_DISABLE);
    res = read(break_fd, &breaks, sizeof(unsigned long long));
    assert(res == sizeof(unsigned long long));
    if (breaks == 2) {
    printf("TESTED: Full overlap\n");
    } else {
    printf("FAILED: Full overlap: %lld != 2\n", breaks);
    fail = 1;
    }
    free(target);
    close(break_fd);
    return fail;
    }
#[no_mangle]
unsafe extern "C" fn multi_dawr_workload() {
    static void multi_dawr_workload(void)
    {
    a += 10;
    b += 10;
    c[512 + 1] += 'a';
    }
#[no_mangle]
unsafe extern "C" fn test_process_multi_diff_addr() -> c_int {
    static int test_process_multi_diff_addr(void)
    {
    let mut breaks1: c_ulonglong = 0, breaks2 = 0;
    int fd1, fd2;
    char *desc = "Process specific, Two events, diff addr";
    size_t res;
    fd1 = perf_process_event_open(HW_BREAKPOINT_RW, (__u64)&a, (__u64)sizeof(a));
    if (fd1 < 0) {
    perror("perf_process_event_open");
    exit(EXIT_FAILURE);
    }
    fd2 = perf_process_event_open(HW_BREAKPOINT_RW, (__u64)&b, (__u64)sizeof(b));
    if (fd2 < 0) {
    close(fd1);
    perror("perf_process_event_open");
    exit(EXIT_FAILURE);
    }
    ioctl(fd1, PERF_EVENT_IOC_RESET);
    ioctl(fd2, PERF_EVENT_IOC_RESET);
    ioctl(fd1, PERF_EVENT_IOC_ENABLE);
    ioctl(fd2, PERF_EVENT_IOC_ENABLE);
    multi_dawr_workload();
    ioctl(fd1, PERF_EVENT_IOC_DISABLE);
    ioctl(fd2, PERF_EVENT_IOC_DISABLE);
    res = read(fd1, &breaks1, sizeof(breaks1));
    assert(res == sizeof(unsigned long long));
    res = read(fd2, &breaks2, sizeof(breaks2));
    assert(res == sizeof(unsigned long long));
    close(fd1);
    close(fd2);
    if (breaks1 != 2 || breaks2 != 2) {
    printf("FAILED: %s: %lld != 2 || %lld != 2\n", desc, breaks1, breaks2);
    return 1;
    }
    printf("TESTED: %s\n", desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_process_multi_same_addr() -> c_int {
    static int test_process_multi_same_addr(void)
    {
    let mut breaks1: c_ulonglong = 0, breaks2 = 0;
    int fd1, fd2;
    char *desc = "Process specific, Two events, same addr";
    size_t res;
    fd1 = perf_process_event_open(HW_BREAKPOINT_RW, (__u64)&a, (__u64)sizeof(a));
    if (fd1 < 0) {
    perror("perf_process_event_open");
    exit(EXIT_FAILURE);
    }
    fd2 = perf_process_event_open(HW_BREAKPOINT_RW, (__u64)&a, (__u64)sizeof(a));
    if (fd2 < 0) {
    close(fd1);
    perror("perf_process_event_open");
    exit(EXIT_FAILURE);
    }
    ioctl(fd1, PERF_EVENT_IOC_RESET);
    ioctl(fd2, PERF_EVENT_IOC_RESET);
    ioctl(fd1, PERF_EVENT_IOC_ENABLE);
    ioctl(fd2, PERF_EVENT_IOC_ENABLE);
    multi_dawr_workload();
    ioctl(fd1, PERF_EVENT_IOC_DISABLE);
    ioctl(fd2, PERF_EVENT_IOC_DISABLE);
    res = read(fd1, &breaks1, sizeof(breaks1));
    assert(res == sizeof(unsigned long long));
    res = read(fd2, &breaks2, sizeof(breaks2));
    assert(res == sizeof(unsigned long long));
    close(fd1);
    close(fd2);
    if (breaks1 != 2 || breaks2 != 2) {
    printf("FAILED: %s: %lld != 2 || %lld != 2\n", desc, breaks1, breaks2);
    return 1;
    }
    printf("TESTED: %s\n", desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_process_multi_diff_addr_ro_wo() -> c_int {
    static int test_process_multi_diff_addr_ro_wo(void)
    {
    let mut breaks1: c_ulonglong = 0, breaks2 = 0;
    int fd1, fd2;
    char *desc = "Process specific, Two events, diff addr, one is RO, other is WO";
    size_t res;
    fd1 = perf_process_event_open(HW_BREAKPOINT_W, (__u64)&a, (__u64)sizeof(a));
    if (fd1 < 0) {
    perror("perf_process_event_open");
    exit(EXIT_FAILURE);
    }
    fd2 = perf_process_event_open(HW_BREAKPOINT_R, (__u64)&b, (__u64)sizeof(b));
    if (fd2 < 0) {
    close(fd1);
    perror("perf_process_event_open");
    exit(EXIT_FAILURE);
    }
    ioctl(fd1, PERF_EVENT_IOC_RESET);
    ioctl(fd2, PERF_EVENT_IOC_RESET);
    ioctl(fd1, PERF_EVENT_IOC_ENABLE);
    ioctl(fd2, PERF_EVENT_IOC_ENABLE);
    multi_dawr_workload();
    ioctl(fd1, PERF_EVENT_IOC_DISABLE);
    ioctl(fd2, PERF_EVENT_IOC_DISABLE);
    res = read(fd1, &breaks1, sizeof(breaks1));
    assert(res == sizeof(unsigned long long));
    res = read(fd2, &breaks2, sizeof(breaks2));
    assert(res == sizeof(unsigned long long));
    close(fd1);
    close(fd2);
    if (breaks1 != 1 || breaks2 != 1) {
    printf("FAILED: %s: %lld != 1 || %lld != 1\n", desc, breaks1, breaks2);
    return 1;
    }
    printf("TESTED: %s\n", desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_process_multi_same_addr_ro_wo() -> c_int {
    static int test_process_multi_same_addr_ro_wo(void)
    {
    let mut breaks1: c_ulonglong = 0, breaks2 = 0;
    int fd1, fd2;
    char *desc = "Process specific, Two events, same addr, one is RO, other is WO";
    size_t res;
    fd1 = perf_process_event_open(HW_BREAKPOINT_R, (__u64)&a, (__u64)sizeof(a));
    if (fd1 < 0) {
    perror("perf_process_event_open");
    exit(EXIT_FAILURE);
    }
    fd2 = perf_process_event_open(HW_BREAKPOINT_W, (__u64)&a, (__u64)sizeof(a));
    if (fd2 < 0) {
    close(fd1);
    perror("perf_process_event_open");
    exit(EXIT_FAILURE);
    }
    ioctl(fd1, PERF_EVENT_IOC_RESET);
    ioctl(fd2, PERF_EVENT_IOC_RESET);
    ioctl(fd1, PERF_EVENT_IOC_ENABLE);
    ioctl(fd2, PERF_EVENT_IOC_ENABLE);
    multi_dawr_workload();
    ioctl(fd1, PERF_EVENT_IOC_DISABLE);
    ioctl(fd2, PERF_EVENT_IOC_DISABLE);
    res = read(fd1, &breaks1, sizeof(breaks1));
    assert(res == sizeof(unsigned long long));
    res = read(fd2, &breaks2, sizeof(breaks2));
    assert(res == sizeof(unsigned long long));
    close(fd1);
    close(fd2);
    if (breaks1 != 1 || breaks2 != 1) {
    printf("FAILED: %s: %lld != 1 || %lld != 1\n", desc, breaks1, breaks2);
    return 1;
    }
    printf("TESTED: %s\n", desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_syswide_multi_diff_addr() -> c_int {
    static int test_syswide_multi_diff_addr(void)
    {
    let mut breaks1: c_ulonglong = 0, breaks2 = 0;
    int *fd1 = malloc(nprocs * sizeof(int));
    int *fd2 = malloc(nprocs * sizeof(int));
    char *desc = "Systemwide, Two events, diff addr";
    int ret;
    ret = perf_systemwide_event_open(fd1, HW_BREAKPOINT_RW, (__u64)&a, (__u64)sizeof(a));
    if (ret)
    exit(EXIT_FAILURE);
    ret = perf_systemwide_event_open(fd2, HW_BREAKPOINT_RW, (__u64)&b, (__u64)sizeof(b));
    if (ret) {
    close_fds(fd1, nprocs);
    exit(EXIT_FAILURE);
    }
    reset_fds(fd1, nprocs);
    reset_fds(fd2, nprocs);
    enable_fds(fd1, nprocs);
    enable_fds(fd2, nprocs);
    multi_dawr_workload();
    disable_fds(fd1, nprocs);
    disable_fds(fd2, nprocs);
    breaks1 = read_fds(fd1, nprocs);
    breaks2 = read_fds(fd2, nprocs);
    close_fds(fd1, nprocs);
    close_fds(fd2, nprocs);
    free(fd1);
    free(fd2);
    if (breaks1 != 2 || breaks2 != 2) {
    printf("FAILED: %s: %lld != 2 || %lld != 2\n", desc, breaks1, breaks2);
    return 1;
    }
    printf("TESTED: %s\n", desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_syswide_multi_same_addr() -> c_int {
    static int test_syswide_multi_same_addr(void)
    {
    let mut breaks1: c_ulonglong = 0, breaks2 = 0;
    int *fd1 = malloc(nprocs * sizeof(int));
    int *fd2 = malloc(nprocs * sizeof(int));
    char *desc = "Systemwide, Two events, same addr";
    int ret;
    ret = perf_systemwide_event_open(fd1, HW_BREAKPOINT_RW, (__u64)&a, (__u64)sizeof(a));
    if (ret)
    exit(EXIT_FAILURE);
    ret = perf_systemwide_event_open(fd2, HW_BREAKPOINT_RW, (__u64)&a, (__u64)sizeof(a));
    if (ret) {
    close_fds(fd1, nprocs);
    exit(EXIT_FAILURE);
    }
    reset_fds(fd1, nprocs);
    reset_fds(fd2, nprocs);
    enable_fds(fd1, nprocs);
    enable_fds(fd2, nprocs);
    multi_dawr_workload();
    disable_fds(fd1, nprocs);
    disable_fds(fd2, nprocs);
    breaks1 = read_fds(fd1, nprocs);
    breaks2 = read_fds(fd2, nprocs);
    close_fds(fd1, nprocs);
    close_fds(fd2, nprocs);
    free(fd1);
    free(fd2);
    if (breaks1 != 2 || breaks2 != 2) {
    printf("FAILED: %s: %lld != 2 || %lld != 2\n", desc, breaks1, breaks2);
    return 1;
    }
    printf("TESTED: %s\n", desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_syswide_multi_diff_addr_ro_wo() -> c_int {
    static int test_syswide_multi_diff_addr_ro_wo(void)
    {
    let mut breaks1: c_ulonglong = 0, breaks2 = 0;
    int *fd1 = malloc(nprocs * sizeof(int));
    int *fd2 = malloc(nprocs * sizeof(int));
    char *desc = "Systemwide, Two events, diff addr, one is RO, other is WO";
    int ret;
    ret = perf_systemwide_event_open(fd1, HW_BREAKPOINT_W, (__u64)&a, (__u64)sizeof(a));
    if (ret)
    exit(EXIT_FAILURE);
    ret = perf_systemwide_event_open(fd2, HW_BREAKPOINT_R, (__u64)&b, (__u64)sizeof(b));
    if (ret) {
    close_fds(fd1, nprocs);
    exit(EXIT_FAILURE);
    }
    reset_fds(fd1, nprocs);
    reset_fds(fd2, nprocs);
    enable_fds(fd1, nprocs);
    enable_fds(fd2, nprocs);
    multi_dawr_workload();
    disable_fds(fd1, nprocs);
    disable_fds(fd2, nprocs);
    breaks1 = read_fds(fd1, nprocs);
    breaks2 = read_fds(fd2, nprocs);
    close_fds(fd1, nprocs);
    close_fds(fd2, nprocs);
    free(fd1);
    free(fd2);
    if (breaks1 != 1 || breaks2 != 1) {
    printf("FAILED: %s: %lld != 1 || %lld != 1\n", desc, breaks1, breaks2);
    return 1;
    }
    printf("TESTED: %s\n", desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_syswide_multi_same_addr_ro_wo() -> c_int {
    static int test_syswide_multi_same_addr_ro_wo(void)
    {
    let mut breaks1: c_ulonglong = 0, breaks2 = 0;
    int *fd1 = malloc(nprocs * sizeof(int));
    int *fd2 = malloc(nprocs * sizeof(int));
    char *desc = "Systemwide, Two events, same addr, one is RO, other is WO";
    int ret;
    ret = perf_systemwide_event_open(fd1, HW_BREAKPOINT_W, (__u64)&a, (__u64)sizeof(a));
    if (ret)
    exit(EXIT_FAILURE);
    ret = perf_systemwide_event_open(fd2, HW_BREAKPOINT_R, (__u64)&a, (__u64)sizeof(a));
    if (ret) {
    close_fds(fd1, nprocs);
    exit(EXIT_FAILURE);
    }
    reset_fds(fd1, nprocs);
    reset_fds(fd2, nprocs);
    enable_fds(fd1, nprocs);
    enable_fds(fd2, nprocs);
    multi_dawr_workload();
    disable_fds(fd1, nprocs);
    disable_fds(fd2, nprocs);
    breaks1 = read_fds(fd1, nprocs);
    breaks2 = read_fds(fd2, nprocs);
    close_fds(fd1, nprocs);
    close_fds(fd2, nprocs);
    free(fd1);
    free(fd2);
    if (breaks1 != 1 || breaks2 != 1) {
    printf("FAILED: %s: %lld != 1 || %lld != 1\n", desc, breaks1, breaks2);
    return 1;
    }
    printf("TESTED: %s\n", desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn runtest_multi_dawr() -> c_int {
    static int runtest_multi_dawr(void)
    {
    let mut ret: c_int = 0;
    ret |= test_process_multi_diff_addr();
    ret |= test_process_multi_same_addr();
    ret |= test_process_multi_diff_addr_ro_wo();
    ret |= test_process_multi_same_addr_ro_wo();
    ret |= test_syswide_multi_diff_addr();
    ret |= test_syswide_multi_same_addr();
    ret |= test_syswide_multi_diff_addr_ro_wo();
    ret |= test_syswide_multi_same_addr_ro_wo();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn runtest_unaligned_512bytes() -> c_int {
    static int runtest_unaligned_512bytes(void)
    {
    let mut breaks: c_ulonglong = 0;
    int fd;
    char *desc = "Process specific, 512 bytes, unaligned";
    let mut addr: __u64 = (__u64)&c + 8;
    size_t res;
    fd = perf_process_event_open(HW_BREAKPOINT_RW, addr, 512);
    if (fd < 0) {
    perror("perf_process_event_open");
    exit(EXIT_FAILURE);
    }
    ioctl(fd, PERF_EVENT_IOC_RESET);
    ioctl(fd, PERF_EVENT_IOC_ENABLE);
    multi_dawr_workload();
    ioctl(fd, PERF_EVENT_IOC_DISABLE);
    res = read(fd, &breaks, sizeof(breaks));
    assert(res == sizeof(unsigned long long));
    close(fd);
    if (breaks != 2) {
    printf("FAILED: %s: %lld != 2\n", desc, breaks);
    return 1;
    }
    printf("TESTED: %s\n", desc);
    return 0;
    }
// There is no perf api to find number of available watchpoints. Use ptrace.
#[no_mangle]
unsafe extern "C" fn get_nr_wps(arch_31: *mut bool) -> c_int {
    static int get_nr_wps(bool *arch_31)
    {
    struct ppc_debug_info dbginfo;
    int child_pid;
    child_pid = fork();
    if (!child_pid) {
    let mut ret: c_int = ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut(), 0);
    if (ret) {
    perror("PTRACE_TRACEME failed\n");
    exit(EXIT_FAILURE);
    }
    kill(getpid(), SIGUSR1);
    sleep(1);
    exit(EXIT_SUCCESS);
    }
    wait(core::ptr::null_mut());
    if (ptrace(PPC_PTRACE_GETHWDBGINFO, child_pid, core::ptr::null_mut(), &dbginfo)) {
    perror("Can't get breakpoint info");
    exit(EXIT_FAILURE);
    }
// arch_31 = !!(dbginfo.features & PPC_DEBUG_FEATURE_DATA_BP_ARCH_31);
    return dbginfo.num_data_bps;
    }
#[no_mangle]
unsafe extern "C" fn runtest() -> c_int {
    static int runtest(void)
    {
    int rwflag;
    int exclude_user;
    int ret;
    let mut dawr: bool = dawr_supported();
    let mut arch_31: bool = false;
    let mut nr_wps: c_int = get_nr_wps(&arch_31);
//
// perf defines rwflag as two bits read and write and at least
// one must be set.  So range 1-3.
//
    for (rwflag = 1 ; rwflag < 4; rwflag++) {
    for (exclude_user = 0 ; exclude_user < 2; exclude_user++) {
    ret = runtestsingle(rwflag, exclude_user, 0);
    if (ret)
    return ret;
// if we have the dawr, we can do an array test
    if (!dawr)
    continue;
    ret = runtestsingle(rwflag, exclude_user, 1);
    if (ret)
    return ret;
    }
    }
    ret = runtest_dar_outside();
    if (ret)
    return ret;
    if (dawr && nr_wps > 1) {
    nprocs = get_nprocs();
    ret = runtest_multi_dawr();
    if (ret)
    return ret;
    }
    if (dawr && arch_31)
    ret = runtest_unaligned_512bytes();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn perf_hwbreak() -> c_int {
    static int perf_hwbreak(void)
    {
    srand ( time(core::ptr::null_mut()) );
    SKIP_IF_MSG(!perf_breakpoint_supported(), "Perf breakpoints not supported");
    return runtest();
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char, envp: *mut c_char) -> c_int {
    int main(int argc, char *argv[], char **envp)
    {
    return test_harness(perf_hwbreak, "perf_hwbreak");
    }
