//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/map_ops.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn map_update() {
    static void map_update(void)
    {
    (void)syscall(__NR_getpid);
    }
#[no_mangle]
unsafe extern "C" fn map_delete() {
    static void map_delete(void)
    {
    (void)syscall(__NR_getppid);
    }
#[no_mangle]
unsafe extern "C" fn map_push() {
    static void map_push(void)
    {
    (void)syscall(__NR_getuid);
    }
#[no_mangle]
unsafe extern "C" fn map_pop() {
    static void map_pop(void)
    {
    (void)syscall(__NR_geteuid);
    }
#[no_mangle]
unsafe extern "C" fn map_peek() {
    static void map_peek(void)
    {
    (void)syscall(__NR_getgid);
    }
#[no_mangle]
unsafe extern "C" fn map_for_each_pass() {
    static void map_for_each_pass(void)
    {
    (void)syscall(__NR_gettid);
    }
#[no_mangle]
unsafe extern "C" fn map_for_each_fail() {
    static void map_for_each_fail(void)
    {
    (void)syscall(__NR_getpgid);
    }
#[no_mangle]
unsafe extern "C" fn setup(skel: *mut test_map_ops) -> c_int {
    static int setup(struct test_map_ops **skel)
    {
    let mut err: c_int = 0;
    if (!skel)
    return -1;
// skel = test_map_ops__open();
    if (!ASSERT_OK_PTR(*skel, "test_map_ops__open"))
    return -1;
    (*skel).rodata.pid = getpid();
    err = test_map_ops__load(*skel);
    if (!ASSERT_OK(err, "test_map_ops__load"))
    return err;
    err = test_map_ops__attach(*skel);
    if (!ASSERT_OK(err, "test_map_ops__attach"))
    return err;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn teardown(skel: *mut test_map_ops) {
    static void teardown(struct test_map_ops **skel)
    {
    if (skel && *skel)
    test_map_ops__destroy(*skel);
    }
#[no_mangle]
unsafe extern "C" fn map_ops_update_delete_subtest() {
    static void map_ops_update_delete_subtest(void)
    {
    struct test_map_ops *skel;
    if (setup(&skel))
    goto teardown;
    map_update();
    ASSERT_OK(skel.bss.err, "map_update_initial");
    map_update();
    ASSERT_LT(skel.bss.err, 0, "map_update_existing");
    ASSERT_EQ(skel.bss.err, -EEXIST, "map_update_existing");
    map_delete();
    ASSERT_OK(skel.bss.err, "map_delete_existing");
    map_delete();
    ASSERT_LT(skel.bss.err, 0, "map_delete_non_existing");
    ASSERT_EQ(skel.bss.err, -ENOENT, "map_delete_non_existing");
    teardown:
    teardown(&skel);
    }
#[no_mangle]
unsafe extern "C" fn map_ops_push_peek_pop_subtest() {
    static void map_ops_push_peek_pop_subtest(void)
    {
    struct test_map_ops *skel;
    if (setup(&skel))
    goto teardown;
    map_push();
    ASSERT_OK(skel.bss.err, "map_push_initial");
    map_push();
    ASSERT_LT(skel.bss.err, 0, "map_push_when_full");
    ASSERT_EQ(skel.bss.err, -E2BIG, "map_push_when_full");
    map_peek();
    ASSERT_OK(skel.bss.err, "map_peek");
    map_pop();
    ASSERT_OK(skel.bss.err, "map_pop");
    map_peek();
    ASSERT_LT(skel.bss.err, 0, "map_peek_when_empty");
    ASSERT_EQ(skel.bss.err, -ENOENT, "map_peek_when_empty");
    map_pop();
    ASSERT_LT(skel.bss.err, 0, "map_pop_when_empty");
    ASSERT_EQ(skel.bss.err, -ENOENT, "map_pop_when_empty");
    teardown:
    teardown(&skel);
    }
#[no_mangle]
unsafe extern "C" fn map_ops_for_each_subtest() {
    static void map_ops_for_each_subtest(void)
    {
    struct test_map_ops *skel;
    if (setup(&skel))
    goto teardown;
    map_for_each_pass();
// expect to iterate over 1 element
    ASSERT_EQ(skel.bss.err, 1, "map_for_each_no_flags");
    map_for_each_fail();
    ASSERT_LT(skel.bss.err, 0, "map_for_each_with_flags");
    ASSERT_EQ(skel.bss.err, -EINVAL, "map_for_each_with_flags");
    teardown:
    teardown(&skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_map_ops() {
    void test_map_ops(void)
    {
    if (test__start_subtest("map_ops_update_delete"))
    map_ops_update_delete_subtest();
    if (test__start_subtest("map_ops_push_peek_pop"))
    map_ops_push_peek_pop_subtest();
    if (test__start_subtest("map_ops_for_each"))
    map_ops_for_each_subtest();
    }
