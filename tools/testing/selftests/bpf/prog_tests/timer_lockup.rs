//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/timer_lockup.c
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

    static long cpu;
    static int *timer1_err;
    static int *timer2_err;
    static bool skip;
    let mut k: volatile int = 0;
    static void *timer_lockup_thread(void *arg)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in	= &pkt_v4,
    .data_size_in	= sizeof(pkt_v4),
    .repeat		= 1000,
    );
    int i, prog_fd = *(int *)arg;
    cpu_set_t cpuset;
    CPU_ZERO(&cpuset);
    CPU_SET(__sync_fetch_and_add(&cpu, 1), &cpuset);
    ASSERT_OK(pthread_setaffinity_np(pthread_self(), sizeof(cpuset),
    &cpuset),
    "cpu affinity");
    for (i = 0; !READ_ONCE(*timer1_err) && !READ_ONCE(*timer2_err); i++) {
    bpf_prog_test_run_opts(prog_fd, &opts);
// Skip the test if we can't reproduce the race in a reasonable
// amount of time.
//
    if (i > 50) {
    WRITE_ONCE(skip, true);
    break;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn test_timer_lockup() {
    void test_timer_lockup(void)
    {
    int timer1_prog, timer2_prog;
    struct timer_lockup *skel;
    pthread_t thrds[2];
    void *ret;
    if (get_nprocs() < 2) {
    test__skip();
    return;
    }
    skel = timer_lockup__open_and_load();
    if (!skel && errno == EOPNOTSUPP) {
    test__skip();
    return;
    }
    if (!ASSERT_OK_PTR(skel, "timer_lockup__open_and_load"))
    return;
    timer1_prog = bpf_program__fd(skel.progs.timer1_prog);
    timer2_prog = bpf_program__fd(skel.progs.timer2_prog);
    timer1_err = &skel.bss.timer1_err;
    timer2_err = &skel.bss.timer2_err;
    if (!ASSERT_OK(pthread_create(&thrds[0], core::ptr::null_mut(), timer_lockup_thread,
    &timer1_prog),
    "pthread_create thread1"))
    goto out;
    if (!ASSERT_OK(pthread_create(&thrds[1], core::ptr::null_mut(), timer_lockup_thread,
    &timer2_prog),
    "pthread_create thread2")) {
    pthread_exit(&thrds[0]);
    goto out;
    }
    pthread_join(thrds[1], &ret);
    pthread_join(thrds[0], &ret);
    if (skip) {
    test__skip();
    goto out;
    }
    if (*timer1_err != -EDEADLK && *timer1_err != 0)
    ASSERT_FAIL("timer1_err bad value");
    if (*timer2_err != -EDEADLK && *timer2_err != 0)
    ASSERT_FAIL("timer2_err bad value");
    out:
    timer_lockup__destroy(skel);
    return;
    }
