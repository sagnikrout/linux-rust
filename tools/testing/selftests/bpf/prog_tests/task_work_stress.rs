//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/task_work_stress.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_data {
    pub prog_fd: c_int,
    pub exit: atomic_int,
}

    void *runner(void *test_data)
    {
    struct test_data *td = test_data;
    let mut err: c_int = 0;
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    while (!err && !atomic_load(&td.exit))
    err = bpf_prog_test_run_opts(td.prog_fd, &opts);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn get_env_int(str: *const c_char, def: c_int) -> c_int {
    static int get_env_int(const char *str, int def)
    {
    const char *s = getenv(str);
    char *end;
    int retval;
    if (!s || !*s)
    return def;
    errno = 0;
    retval = strtol(s, &end, 10);
    if (errno || *end || retval < 0)
    return def;
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn task_work_run(enable_delete: bool) {
    static void task_work_run(bool enable_delete)
    {
    struct task_work_stress *skel;
    struct bpf_program *scheduler, *deleter;
    let mut nthreads: c_int = 16;
    let mut test_time_s: c_int = get_env_int("BPF_TASK_WORK_TEST_TIME", 1);
    pthread_t tid[nthreads], tid_del;
    bool started[nthreads], started_del = false;
    let mut td_sched: test_data = { .exit = 0 }, td_del = { .exit = 1 };
    int i, err;
    skel = task_work_stress__open();
    if (!ASSERT_OK_PTR(skel, "task_work__open"))
    return;
    scheduler = bpf_object__find_program_by_name(skel.obj, "schedule_task_work");
    bpf_program__set_autoload(scheduler, true);
    deleter = bpf_object__find_program_by_name(skel.obj, "delete_task_work");
    bpf_program__set_autoload(deleter, true);
    err = task_work_stress__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto cleanup;
    for (i = 0; i < nthreads; ++i)
    started[i] = false;
    td_sched.prog_fd = bpf_program__fd(scheduler);
    for (i = 0; i < nthreads; ++i) {
    if (pthread_create(&tid[i], core::ptr::null_mut(), runner, &td_sched) != 0) {
    fprintf(stderr, "could not start thread");
    goto cancel;
    }
    started[i] = true;
    }
    if (enable_delete)
    atomic_store(&td_del.exit, 0);
    td_del.prog_fd = bpf_program__fd(deleter);
    if (pthread_create(&tid_del, core::ptr::null_mut(), runner, &td_del) != 0) {
    fprintf(stderr, "could not start thread");
    goto cancel;
    }
    started_del = true;
// Run stress test for some time
    sleep(test_time_s);
    cancel:
    atomic_store(&td_sched.exit, 1);
    atomic_store(&td_del.exit, 1);
    for (i = 0; i < nthreads; ++i) {
    if (started[i])
    pthread_join(tid[i], core::ptr::null_mut());
    }
    if (started_del)
    pthread_join(tid_del, core::ptr::null_mut());
    ASSERT_GT(skel.bss.callback_scheduled, 0, "work scheduled");
// Some scheduling attempts should have failed due to contention
    ASSERT_GT(skel.bss.schedule_error, 0, "schedule error");
    if (enable_delete) {
// If delete thread is enabled, it has cancelled some callbacks
    ASSERT_GT(skel.bss.delete_success, 0, "delete success");
    ASSERT_LT(skel.bss.callback_success, skel.bss.callback_scheduled, "callbacks");
    } else {
// Without delete thread number of scheduled callbacks is the same as fired
    ASSERT_EQ(skel.bss.callback_success, skel.bss.callback_scheduled, "callbacks");
    }
    cleanup:
    task_work_stress__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_task_work_stress() {
    void test_task_work_stress(void)
    {
    if (test__start_subtest("no_delete"))
    task_work_run(false);
    if (test__start_subtest("with_delete"))
    task_work_run(true);
    }
