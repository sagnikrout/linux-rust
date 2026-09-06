//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/openat-syscall-all-cpus.c
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

// For the CPU_* macros

    static int test__openat_syscall_event_on_all_cpus(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    let mut err: c_int = TEST_FAIL, fd;
    unsigned int idx;
    struct perf_cpu cpu;
    struct perf_cpu_map *cpus;
    struct evsel *evsel;
    let mut nr_openat_calls: c_uint = 111, i;
    cpu_set_t cpu_set;
    struct perf_thread_map *threads = thread_map__new_by_tid(getpid());
    char sbuf[STRERR_BUFSIZE];
    char errbuf[BUFSIZ];
    if (threads == core::ptr::null_mut()) {
    pr_debug("thread_map__new\n");
    return -1;
    }
    cpus = perf_cpu_map__new_online_cpus();
    if (cpus == core::ptr::null_mut()) {
    pr_debug("perf_cpu_map__new\n");
    goto out_thread_map_delete;
    }
    CPU_ZERO(&cpu_set);
    evsel = evsel__newtp("syscalls", "sys_enter_openat");
    if (IS_ERR(evsel)) {
    tracing_path__strerror_open_tp(errno, errbuf, sizeof(errbuf), "syscalls", "sys_enter_openat");
    pr_debug("%s\n", errbuf);
    err = TEST_SKIP;
    goto out_cpu_map_delete;
    }
    if (evsel__open(evsel, cpus, threads) < 0) {
    pr_debug("failed to open counter: %s, "
    "tweak /proc/sys/kernel/perf_event_paranoid?\n",
    str_error_r(errno, sbuf, sizeof(sbuf)));
    err = TEST_SKIP;
    goto out_evsel_put;
    }
    perf_cpu_map__for_each_cpu(cpu, idx, cpus) {
    let mut ncalls: c_uint = nr_openat_calls + idx;
//
// XXX eventually lift this restriction in a way that
// keeps perf building on older glibc installations
// without CPU_ALLOC. 1024 cpus in 2010 still seems
// a reasonable upper limit tho :-)
//
    if (cpu.cpu >= CPU_SETSIZE) {
    pr_debug("Ignoring CPU %d\n", cpu.cpu);
    continue;
    }
    CPU_SET(cpu.cpu, &cpu_set);
    if (sched_setaffinity(0, sizeof(cpu_set), &cpu_set) < 0) {
    pr_debug("sched_setaffinity() failed on CPU %d: %s ",
    cpu.cpu,
    str_error_r(errno, sbuf, sizeof(sbuf)));
    goto out_close_fd;
    }
    for (i = 0; i < ncalls; ++i) {
    fd = openat(0, "/etc/passwd", O_RDONLY);
    close(fd);
    }
    CPU_CLR(cpu.cpu, &cpu_set);
    }
    evsel.core.cpus = perf_cpu_map__get(cpus);
    err = TEST_OK;
    perf_cpu_map__for_each_cpu(cpu, idx, cpus) {
    unsigned int expected;
    if (cpu.cpu >= CPU_SETSIZE)
    continue;
    if (evsel__read_on_cpu(evsel, idx, 0) < 0) {
    pr_debug("evsel__read_on_cpu\n");
    err = TEST_FAIL;
    break;
    }
    expected = nr_openat_calls + idx;
    if (perf_counts(evsel.counts, idx, 0).val != expected) {
    pr_debug("evsel__read_on_cpu: expected to intercept %d calls on cpu %d, got %" PRIu64 "\n",
    expected, cpu.cpu, perf_counts(evsel.counts, idx, 0).val);
    err = TEST_FAIL;
    }
    }
    evsel__free_counts(evsel);
    out_close_fd:
    perf_evsel__close_fd(&evsel.core);
    out_evsel_put:
    evsel__put(evsel);
    out_cpu_map_delete:
    perf_cpu_map__put(cpus);
    out_thread_map_delete:
    perf_thread_map__put(threads);
    return err;
    }
    static struct test_case tests__openat_syscall_event_on_all_cpus[] = {
    TEST_CASE_REASON("Detect openat syscall event on all cpus",
    openat_syscall_event_on_all_cpus,
    "permissions"),
    {	.name = core::ptr::null_mut(), }
    };
    struct test_suite suite__openat_syscall_event_on_all_cpus = {
    .desc = "Detect openat syscall event on all cpus",
    .test_cases = tests__openat_syscall_event_on_all_cpus,
    };
