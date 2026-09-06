//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/rbtree.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn test_rbtree_add_nodes() {
    static void test_rbtree_add_nodes(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct rbtree *skel;
    int ret;
    skel = rbtree__open_and_load();
    if (!ASSERT_OK_PTR(skel, "rbtree__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.rbtree_add_nodes), &opts);
    ASSERT_OK(ret, "rbtree_add_nodes run");
    ASSERT_OK(opts.retval, "rbtree_add_nodes retval");
    ASSERT_EQ(skel.data.less_callback_ran, 1, "rbtree_add_nodes less_callback_ran");
    rbtree__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_rbtree_add_nodes_nested() {
    static void test_rbtree_add_nodes_nested(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct rbtree *skel;
    int ret;
    skel = rbtree__open_and_load();
    if (!ASSERT_OK_PTR(skel, "rbtree__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.rbtree_add_nodes_nested), &opts);
    ASSERT_OK(ret, "rbtree_add_nodes_nested run");
    ASSERT_OK(opts.retval, "rbtree_add_nodes_nested retval");
    ASSERT_EQ(skel.data.less_callback_ran, 1, "rbtree_add_nodes_nested less_callback_ran");
    rbtree__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_rbtree_add_and_remove() {
    static void test_rbtree_add_and_remove(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct rbtree *skel;
    int ret;
    skel = rbtree__open_and_load();
    if (!ASSERT_OK_PTR(skel, "rbtree__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.rbtree_add_and_remove), &opts);
    ASSERT_OK(ret, "rbtree_add_and_remove");
    ASSERT_OK(opts.retval, "rbtree_add_and_remove retval");
    ASSERT_EQ(skel.data.removed_key, 5, "rbtree_add_and_remove first removed key");
    rbtree__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_rbtree_add_and_remove_array() {
    static void test_rbtree_add_and_remove_array(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct rbtree *skel;
    int ret;
    skel = rbtree__open_and_load();
    if (!ASSERT_OK_PTR(skel, "rbtree__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.rbtree_add_and_remove_array), &opts);
    ASSERT_OK(ret, "rbtree_add_and_remove_array");
    ASSERT_OK(opts.retval, "rbtree_add_and_remove_array retval");
    rbtree__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_rbtree_first_and_remove() {
    static void test_rbtree_first_and_remove(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct rbtree *skel;
    int ret;
    skel = rbtree__open_and_load();
    if (!ASSERT_OK_PTR(skel, "rbtree__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.rbtree_first_and_remove), &opts);
    ASSERT_OK(ret, "rbtree_first_and_remove");
    ASSERT_OK(opts.retval, "rbtree_first_and_remove retval");
    ASSERT_EQ(skel.data.first_data[0], 2, "rbtree_first_and_remove first rbtree_first()");
    ASSERT_EQ(skel.data.removed_key, 1, "rbtree_first_and_remove first removed key");
    ASSERT_EQ(skel.data.first_data[1], 4, "rbtree_first_and_remove second rbtree_first()");
    rbtree__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_rbtree_api_release_aliasing() {
    static void test_rbtree_api_release_aliasing(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct rbtree *skel;
    int ret;
    skel = rbtree__open_and_load();
    if (!ASSERT_OK_PTR(skel, "rbtree__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.rbtree_api_release_aliasing), &opts);
    ASSERT_OK(ret, "rbtree_api_release_aliasing");
    ASSERT_OK(opts.retval, "rbtree_api_release_aliasing retval");
    ASSERT_EQ(skel.data.first_data[0], 42, "rbtree_api_release_aliasing first rbtree_remove()");
    ASSERT_EQ(skel.data.first_data[1], -1, "rbtree_api_release_aliasing second rbtree_remove()");
    rbtree__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_success() {
    void test_rbtree_success(void)
    {
    if (test__start_subtest("rbtree_add_nodes"))
    test_rbtree_add_nodes();
    if (test__start_subtest("rbtree_add_nodes_nested"))
    test_rbtree_add_nodes_nested();
    if (test__start_subtest("rbtree_add_and_remove"))
    test_rbtree_add_and_remove();
    if (test__start_subtest("rbtree_add_and_remove_array"))
    test_rbtree_add_and_remove_array();
    if (test__start_subtest("rbtree_first_and_remove"))
    test_rbtree_first_and_remove();
    if (test__start_subtest("rbtree_api_release_aliasing"))
    test_rbtree_api_release_aliasing();
    }

    void test_rbtree_btf_fail__##suffix(void)							\
    {												\
    struct rbtree_btf_fail__##suffix *skel;							\
    \
    skel = rbtree_btf_fail__##suffix##__open_and_load();					\
    if (!ASSERT_ERR_PTR(skel,								\
    "rbtree_btf_fail__" #suffix "__open_and_load unexpected success"))	\
    rbtree_btf_fail__##suffix##__destroy(skel);					\
    }

    if (test__start_subtest("rbtree_btf_fail__" #suffix))	\
    test_rbtree_btf_fail__##suffix();
    BTF_FAIL_TEST(wrong_node_type);
    BTF_FAIL_TEST(add_wrong_type);
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_btf_fail() {
    void test_rbtree_btf_fail(void)
    {
    RUN_BTF_FAIL_TEST(wrong_node_type);
    RUN_BTF_FAIL_TEST(add_wrong_type);
    }
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_fail() {
    void test_rbtree_fail(void)
    {
    RUN_TESTS(rbtree_fail);
    }
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_search() {
    void test_rbtree_search(void)
    {
    RUN_TESTS(rbtree_search);
    }
#[no_mangle]
pub unsafe extern "C" fn test_rbtree_search_kptr() {
    void test_rbtree_search_kptr(void)
    {
    RUN_TESTS(rbtree_search_kptr);
    }
