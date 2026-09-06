//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/bpf_loop.c
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
// Copyright (c) 2021 Facebook

#[no_mangle]
unsafe extern "C" fn check_nr_loops(skel: *mut bpf_loop) {
    static void check_nr_loops(struct bpf_loop *skel)
    {
    struct bpf_link *link;
    link = bpf_program__attach(skel.progs.test_prog);
    if (!ASSERT_OK_PTR(link, "link"))
    return;
// test 0 loops
    skel.bss.nr_loops = 0;
    usleep(1);
    ASSERT_EQ(skel.bss.nr_loops_returned, skel.bss.nr_loops,
    "0 loops");
// test 500 loops
    skel.bss.nr_loops = 500;
    usleep(1);
    ASSERT_EQ(skel.bss.nr_loops_returned, skel.bss.nr_loops,
    "500 loops");
    ASSERT_EQ(skel.bss.g_output, (500 * 499) / 2, "g_output");
// test exceeding the max limit
    skel.bss.nr_loops = -1;
    usleep(1);
    ASSERT_EQ(skel.bss.err, -E2BIG, "over max limit");
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn check_callback_fn_stop(skel: *mut bpf_loop) {
    static void check_callback_fn_stop(struct bpf_loop *skel)
    {
    struct bpf_link *link;
    link = bpf_program__attach(skel.progs.test_prog);
    if (!ASSERT_OK_PTR(link, "link"))
    return;
// testing that loop is stopped when callback_fn returns 1
    skel.bss.nr_loops = 400;
    skel.data.stop_index = 50;
    usleep(1);
    ASSERT_EQ(skel.bss.nr_loops_returned, skel.data.stop_index + 1,
    "nr_loops_returned");
    ASSERT_EQ(skel.bss.g_output, (50 * 49) / 2,
    "g_output");
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn check_null_callback_ctx(skel: *mut bpf_loop) {
    static void check_null_callback_ctx(struct bpf_loop *skel)
    {
    struct bpf_link *link;
// check that user is able to pass in a null callback_ctx
    link = bpf_program__attach(skel.progs.prog_null_ctx);
    if (!ASSERT_OK_PTR(link, "link"))
    return;
    skel.bss.nr_loops = 10;
    usleep(1);
    ASSERT_EQ(skel.bss.nr_loops_returned, skel.bss.nr_loops,
    "nr_loops_returned");
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn check_invalid_flags(skel: *mut bpf_loop) {
    static void check_invalid_flags(struct bpf_loop *skel)
    {
    struct bpf_link *link;
// check that passing in non-zero flags returns -EINVAL
    link = bpf_program__attach(skel.progs.prog_invalid_flags);
    if (!ASSERT_OK_PTR(link, "link"))
    return;
    usleep(1);
    ASSERT_EQ(skel.bss.err, -EINVAL, "err");
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn check_nested_calls(skel: *mut bpf_loop) {
    static void check_nested_calls(struct bpf_loop *skel)
    {
    let mut nr_loops: __u32 = 100, nested_callback_nr_loops = 4;
    struct bpf_link *link;
// check that nested calls are supported
    link = bpf_program__attach(skel.progs.prog_nested_calls);
    if (!ASSERT_OK_PTR(link, "link"))
    return;
    skel.bss.nr_loops = nr_loops;
    skel.bss.nested_callback_nr_loops = nested_callback_nr_loops;
    usleep(1);
    ASSERT_EQ(skel.bss.nr_loops_returned, nr_loops * nested_callback_nr_loops
// nested_callback_nr_loops, "nr_loops_returned");
    ASSERT_EQ(skel.bss.g_output, (4 * 3) / 2 * nested_callback_nr_loops
// nr_loops, "g_output");
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn check_non_constant_callback(skel: *mut bpf_loop) {
    static void check_non_constant_callback(struct bpf_loop *skel)
    {
    struct bpf_link *link =
    bpf_program__attach(skel.progs.prog_non_constant_callback);
    if (!ASSERT_OK_PTR(link, "link"))
    return;
    skel.bss.callback_selector = 0x0F;
    usleep(1);
    ASSERT_EQ(skel.bss.g_output, 0x0F, "g_output #1");
    skel.bss.callback_selector = 0xF0;
    usleep(1);
    ASSERT_EQ(skel.bss.g_output, 0xF0, "g_output #2");
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn check_stack(skel: *mut bpf_loop) {
    static void check_stack(struct bpf_loop *skel)
    {
    struct bpf_link *link = bpf_program__attach(skel.progs.stack_check);
    let mut max_key: c_int = 12;
    int key;
    int map_fd;
    if (!ASSERT_OK_PTR(link, "link"))
    return;
    map_fd = bpf_map__fd(skel.maps.map1);
    if (!ASSERT_GE(map_fd, 0, "bpf_map__fd"))
    goto out;
    for (key = 1; key <= max_key; ++key) {
    let mut val: c_int = key;
    let mut err: c_int = bpf_map_update_elem(map_fd, &key, &val, BPF_NOEXIST);
    if (!ASSERT_OK(err, "bpf_map_update_elem"))
    goto out;
    }
    usleep(1);
    for (key = 1; key <= max_key; ++key) {
    int val;
    let mut err: c_int = bpf_map_lookup_elem(map_fd, &key, &val);
    if (!ASSERT_OK(err, "bpf_map_lookup_elem"))
    goto out;
    if (!ASSERT_EQ(val, key + 1, "bad value in the map"))
    goto out;
    }
    out:
    bpf_link__destroy(link);
    }
#[no_mangle]
pub unsafe extern "C" fn test_bpf_loop() {
    void test_bpf_loop(void)
    {
    struct bpf_loop *skel;
    skel = bpf_loop__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_loop__open_and_load"))
    return;
    skel.bss.pid = getpid();
    if (test__start_subtest("check_nr_loops"))
    check_nr_loops(skel);
    if (test__start_subtest("check_callback_fn_stop"))
    check_callback_fn_stop(skel);
    if (test__start_subtest("check_null_callback_ctx"))
    check_null_callback_ctx(skel);
    if (test__start_subtest("check_invalid_flags"))
    check_invalid_flags(skel);
    if (test__start_subtest("check_nested_calls"))
    check_nested_calls(skel);
    if (test__start_subtest("check_non_constant_callback"))
    check_non_constant_callback(skel);
    if (test__start_subtest("check_stack"))
    check_stack(skel);
    bpf_loop__destroy(skel);
    }
