//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/task_under_cgroup.c
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
// Copyright (c) 2023 Bytedance

#[no_mangle]
pub unsafe extern "C" fn test_task_under_cgroup() {
    void test_task_under_cgroup(void)
    {
    struct test_task_under_cgroup *skel;
    int ret, foo;
    pid_t pid;
    foo = test__join_cgroup(FOO);
    if (!ASSERT_OK(foo < 0, "cgroup_join_foo"))
    return;
    skel = test_task_under_cgroup__open();
    if (!ASSERT_OK_PTR(skel, "test_task_under_cgroup__open"))
    goto cleanup;
    skel.rodata.local_pid = getpid();
    skel.bss.remote_pid = getpid();
    skel.rodata.cgid = get_cgroup_id(FOO);
    ret = test_task_under_cgroup__load(skel);
    if (!ASSERT_OK(ret, "test_task_under_cgroup__load"))
    goto cleanup;
// First, attach the LSM program, and then it will be triggered when the
// TP_BTF program is attached.
//
    skel.links.lsm_run = bpf_program__attach_lsm(skel.progs.lsm_run);
    if (!ASSERT_OK_PTR(skel.links.lsm_run, "attach_lsm"))
    goto cleanup;
    skel.links.tp_btf_run = bpf_program__attach_trace(skel.progs.tp_btf_run);
    if (!ASSERT_OK_PTR(skel.links.tp_btf_run, "attach_tp_btf"))
    goto cleanup;
    pid = fork();
    if (pid == 0)
    exit(0);
    ret = (pid == -1);
    if (ASSERT_OK(ret, "fork process"))
    wait(core::ptr::null_mut());
    test_task_under_cgroup__detach(skel);
    ASSERT_NEQ(skel.bss.remote_pid, skel.rodata.local_pid,
    "test task_under_cgroup");
    cleanup:
    test_task_under_cgroup__destroy(skel);
    close(foo);
    }
