//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/local_kptr_stash.c
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
unsafe extern "C" fn test_local_kptr_stash_simple() {
    static void test_local_kptr_stash_simple(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct local_kptr_stash *skel;
    int ret;
    skel = local_kptr_stash__open_and_load();
    if (!ASSERT_OK_PTR(skel, "local_kptr_stash__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.stash_rb_nodes), &opts);
    ASSERT_OK(ret, "local_kptr_stash_add_nodes run");
    ASSERT_OK(opts.retval, "local_kptr_stash_add_nodes retval");
    local_kptr_stash__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_local_kptr_stash_plain() {
    static void test_local_kptr_stash_plain(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct local_kptr_stash *skel;
    int ret;
    skel = local_kptr_stash__open_and_load();
    if (!ASSERT_OK_PTR(skel, "local_kptr_stash__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.stash_plain), &opts);
    ASSERT_OK(ret, "local_kptr_stash_add_plain run");
    ASSERT_OK(opts.retval, "local_kptr_stash_add_plain retval");
    local_kptr_stash__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_local_kptr_stash_local_with_root() {
    static void test_local_kptr_stash_local_with_root(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct local_kptr_stash *skel;
    int ret;
    skel = local_kptr_stash__open_and_load();
    if (!ASSERT_OK_PTR(skel, "local_kptr_stash__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.stash_local_with_root), &opts);
    ASSERT_OK(ret, "local_kptr_stash_add_local_with_root run");
    ASSERT_OK(opts.retval, "local_kptr_stash_add_local_with_root retval");
    local_kptr_stash__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_local_kptr_stash_unstash() {
    static void test_local_kptr_stash_unstash(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct local_kptr_stash *skel;
    int ret;
    skel = local_kptr_stash__open_and_load();
    if (!ASSERT_OK_PTR(skel, "local_kptr_stash__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.stash_rb_nodes), &opts);
    ASSERT_OK(ret, "local_kptr_stash_add_nodes run");
    ASSERT_OK(opts.retval, "local_kptr_stash_add_nodes retval");
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.unstash_rb_node), &opts);
    ASSERT_OK(ret, "local_kptr_stash_add_nodes run");
    ASSERT_EQ(opts.retval, 42, "local_kptr_stash_add_nodes retval");
    local_kptr_stash__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_refcount_acquire_without_unstash() {
    static void test_refcount_acquire_without_unstash(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct local_kptr_stash *skel;
    int ret;
    skel = local_kptr_stash__open_and_load();
    if (!ASSERT_OK_PTR(skel, "local_kptr_stash__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.refcount_acquire_without_unstash),
    &opts);
    ASSERT_OK(ret, "refcount_acquire_without_unstash run");
    ASSERT_EQ(opts.retval, 2, "refcount_acquire_without_unstash retval");
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.stash_refcounted_node), &opts);
    ASSERT_OK(ret, "stash_refcounted_node run");
    ASSERT_OK(opts.retval, "stash_refcounted_node retval");
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.refcount_acquire_without_unstash),
    &opts);
    ASSERT_OK(ret, "refcount_acquire_without_unstash (2) run");
    ASSERT_EQ(opts.retval, 42, "refcount_acquire_without_unstash (2) retval");
    local_kptr_stash__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_local_kptr_stash_fail() {
    static void test_local_kptr_stash_fail(void)
    {
    RUN_TESTS(local_kptr_stash_fail);
    }
#[no_mangle]
pub unsafe extern "C" fn test_local_kptr_stash() {
    void test_local_kptr_stash(void)
    {
    if (test__start_subtest("local_kptr_stash_simple"))
    test_local_kptr_stash_simple();
    if (test__start_subtest("local_kptr_stash_plain"))
    test_local_kptr_stash_plain();
    if (test__start_subtest("local_kptr_stash_local_with_root"))
    test_local_kptr_stash_local_with_root();
    if (test__start_subtest("local_kptr_stash_unstash"))
    test_local_kptr_stash_unstash();
    if (test__start_subtest("refcount_acquire_without_unstash"))
    test_refcount_acquire_without_unstash();
    if (test__start_subtest("local_kptr_stash_fail"))
    test_local_kptr_stash_fail();
    }
