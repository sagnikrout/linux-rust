//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/exe_ctx.c
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
// Copyright (c) 2026 Valve Corporation.
// Author: Changwoo Min <changwoo@igalia.com>
//

#[no_mangle]
pub unsafe extern "C" fn test_exe_ctx() {
    void test_exe_ctx(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    cpu_set_t old_cpuset, target_cpuset;
    struct test_ctx *skel;
    int err, prog_fd;
// 1. Pin the current process to CPU 0.
    if (sched_getaffinity(0, sizeof(old_cpuset), &old_cpuset) == 0) {
    CPU_ZERO(&target_cpuset);
    CPU_SET(0, &target_cpuset);
    ASSERT_OK(sched_setaffinity(0, sizeof(target_cpuset),
    &target_cpuset), "setaffinity");
    }
    skel = test_ctx__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_load"))
    goto restore_affinity;
    err = test_ctx__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto cleanup;
// 2. When we run this, the kernel will execute the BPF prog on CPU 0.
    prog_fd = bpf_program__fd(skel.progs.trigger_all_contexts);
    err = bpf_prog_test_run_opts(prog_fd, &opts);
    ASSERT_OK(err, "test_run_trigger");
// 3. Wait for the local CPU's softirq/tasklet to finish.
    for (int i = 0; i < 1000; i++) {
    if (skel.bss.count_task > 0 &&
    skel.bss.count_hardirq > 0 &&
    skel.bss.count_softirq > 0)
    break;
    usleep(1000); /* Wait 1ms per iteration, up to 1 sec total */
    }
// On CPU 0, these should now all be non-zero.
    ASSERT_GT(skel.bss.count_task, 0, "task_ok");
    ASSERT_GT(skel.bss.count_hardirq, 0, "hardirq_ok");
    ASSERT_GT(skel.bss.count_softirq, 0, "softirq_ok");
    cleanup:
    test_ctx__destroy(skel);
    restore_affinity:
    ASSERT_OK(sched_setaffinity(0, sizeof(old_cpuset), &old_cpuset),
    "restore_affinity");
    }
