//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_struct_ops_assoc.c
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

#[no_mangle]
unsafe extern "C" fn test_st_ops_assoc() {
    static void test_st_ops_assoc(void)
    {
    struct struct_ops_assoc *skel = core::ptr::null_mut();
    int err, pid;
    skel = struct_ops_assoc__open_and_load();
    if (!ASSERT_OK_PTR(skel, "struct_ops_assoc__open"))
    goto out;
// cannot explicitly associate struct_ops program
    err = bpf_program__assoc_struct_ops(skel.progs.test_1_a,
    skel.maps.st_ops_map_a, core::ptr::null_mut());
    ASSERT_ERR(err, "bpf_program__assoc_struct_ops(test_1_a, st_ops_map_a)");
    err = bpf_program__assoc_struct_ops(skel.progs.syscall_prog_a,
    skel.maps.st_ops_map_a, core::ptr::null_mut());
    ASSERT_OK(err, "bpf_program__assoc_struct_ops(syscall_prog_a, st_ops_map_a)");
    err = bpf_program__assoc_struct_ops(skel.progs.sys_enter_prog_a,
    skel.maps.st_ops_map_a, core::ptr::null_mut());
    ASSERT_OK(err, "bpf_program__assoc_struct_ops(sys_enter_prog_a, st_ops_map_a)");
    err = bpf_program__assoc_struct_ops(skel.progs.syscall_prog_b,
    skel.maps.st_ops_map_b, core::ptr::null_mut());
    ASSERT_OK(err, "bpf_program__assoc_struct_ops(syscall_prog_b, st_ops_map_b)");
    err = bpf_program__assoc_struct_ops(skel.progs.sys_enter_prog_b,
    skel.maps.st_ops_map_b, core::ptr::null_mut());
    ASSERT_OK(err, "bpf_program__assoc_struct_ops(sys_enter_prog_b, st_ops_map_b)");
// sys_enter_prog_a already associated with map_a
    err = bpf_program__assoc_struct_ops(skel.progs.sys_enter_prog_a,
    skel.maps.st_ops_map_b, core::ptr::null_mut());
    ASSERT_ERR(err, "bpf_program__assoc_struct_ops(sys_enter_prog_a, st_ops_map_b)");
    err = struct_ops_assoc__attach(skel);
    if (!ASSERT_OK(err, "struct_ops_assoc__attach"))
    goto out;
// run tracing prog that calls .test_1 and checks return
    pid = getpid();
    skel.bss.test_pid = pid;
    sys_gettid();
    skel.bss.test_pid = 0;
    ASSERT_EQ(skel.bss.test_err_a, 0, "skel.bss.test_err_a");
    ASSERT_EQ(skel.bss.test_err_b, 0, "skel.bss.test_err_b");
// run syscall_prog that calls .test_1 and checks return
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.syscall_prog_a), core::ptr::null_mut());
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.syscall_prog_b), core::ptr::null_mut());
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    ASSERT_EQ(skel.bss.test_err_a, 0, "skel.bss.test_err_a");
    ASSERT_EQ(skel.bss.test_err_b, 0, "skel.bss.test_err_b");
    out:
    struct_ops_assoc__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_st_ops_assoc_reuse() {
    static void test_st_ops_assoc_reuse(void)
    {
    struct struct_ops_assoc_reuse *skel = core::ptr::null_mut();
    int err;
    skel = struct_ops_assoc_reuse__open_and_load();
    if (!ASSERT_OK_PTR(skel, "struct_ops_assoc_reuse__open"))
    goto out;
    err = bpf_program__assoc_struct_ops(skel.progs.syscall_prog_a,
    skel.maps.st_ops_map_a, core::ptr::null_mut());
    ASSERT_OK(err, "bpf_program__assoc_struct_ops(syscall_prog_a, st_ops_map_a)");
    err = bpf_program__assoc_struct_ops(skel.progs.syscall_prog_b,
    skel.maps.st_ops_map_b, core::ptr::null_mut());
    ASSERT_OK(err, "bpf_program__assoc_struct_ops(syscall_prog_b, st_ops_map_b)");
    err = struct_ops_assoc_reuse__attach(skel);
    if (!ASSERT_OK(err, "struct_ops_assoc__attach"))
    goto out;
// run syscall_prog that calls .test_1 and checks return
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.syscall_prog_a), core::ptr::null_mut());
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.syscall_prog_b), core::ptr::null_mut());
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    ASSERT_EQ(skel.bss.test_err_a, 0, "skel.bss.test_err_a");
    ASSERT_EQ(skel.bss.test_err_b, 0, "skel.bss.test_err_b");
    out:
    struct_ops_assoc_reuse__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_st_ops_assoc_in_timer() {
    static void test_st_ops_assoc_in_timer(void)
    {
    struct struct_ops_assoc_in_timer *skel = core::ptr::null_mut();
    int err;
    skel = struct_ops_assoc_in_timer__open_and_load();
    if (!ASSERT_OK_PTR(skel, "struct_ops_assoc_in_timer__open"))
    goto out;
    err = bpf_program__assoc_struct_ops(skel.progs.syscall_prog,
    skel.maps.st_ops_map, core::ptr::null_mut());
    ASSERT_OK(err, "bpf_program__assoc_struct_ops");
    err = struct_ops_assoc_in_timer__attach(skel);
    if (!ASSERT_OK(err, "struct_ops_assoc__attach"))
    goto out;
//
// Run .test_1 by calling kfunc bpf_kfunc_multi_st_ops_test_1_prog_arg() and checks
// the return value. .test_1 will also schedule timer_cb that runs .test_1 again
// immediately.
//
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.syscall_prog), core::ptr::null_mut());
    ASSERT_OK(err, "bpf_prog_test_run_opts");
// Check the return of the kfunc after timer_cb runs
    while (!READ_ONCE(skel.bss.timer_cb_run))
    sched_yield();
    ASSERT_EQ(skel.bss.timer_test_1_ret, 1234, "skel.bss.timer_test_1_ret");
    ASSERT_EQ(skel.bss.test_err, 0, "skel.bss.test_err_a");
    out:
    struct_ops_assoc_in_timer__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_st_ops_assoc_in_timer_no_uref() {
    static void test_st_ops_assoc_in_timer_no_uref(void)
    {
    struct struct_ops_assoc_in_timer *skel = core::ptr::null_mut();
    struct bpf_link *link;
    int err;
    skel = struct_ops_assoc_in_timer__open_and_load();
    if (!ASSERT_OK_PTR(skel, "struct_ops_assoc_in_timer__open"))
    goto out;
    err = bpf_program__assoc_struct_ops(skel.progs.syscall_prog,
    skel.maps.st_ops_map, core::ptr::null_mut());
    ASSERT_OK(err, "bpf_program__assoc_struct_ops");
    link = bpf_map__attach_struct_ops(skel.maps.st_ops_map);
    if (!ASSERT_OK_PTR(link, "bpf_map__attach_struct_ops"))
    goto out;
//
// Run .test_1 by calling kfunc bpf_kfunc_multi_st_ops_test_1_prog_arg() and checks
// the return value. .test_1 will also schedule timer_cb that runs .test_1 again.
// timer_cb will run 500ms after syscall_prog runs, when the user space no longer
// holds a reference to st_ops_map.
//
    skel.bss.timer_ns = 500000000;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.syscall_prog), core::ptr::null_mut());
    ASSERT_OK(err, "bpf_prog_test_run_opts");
// Detach and close struct_ops map to cause it to be freed
    bpf_link__destroy(link);
    close(bpf_program__fd(skel.progs.syscall_prog));
    close(bpf_map__fd(skel.maps.st_ops_map));
// Check the return of the kfunc after timer_cb runs
    while (!READ_ONCE(skel.bss.timer_cb_run))
    sched_yield();
    ASSERT_EQ(skel.bss.timer_test_1_ret, -1, "skel.bss.timer_test_1_ret");
    ASSERT_EQ(skel.bss.test_err, 0, "skel.bss.test_err_a");
    out:
    struct_ops_assoc_in_timer__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_struct_ops_assoc() {
    void test_struct_ops_assoc(void)
    {
    if (test__start_subtest("st_ops_assoc"))
    test_st_ops_assoc();
    if (test__start_subtest("st_ops_assoc_reuse"))
    test_st_ops_assoc_reuse();
    if (test__start_subtest("st_ops_assoc_in_timer"))
    test_st_ops_assoc_in_timer();
    if (test__start_subtest("st_ops_assoc_in_timer_no_uref"))
    test_st_ops_assoc_in_timer_no_uref();
    }
