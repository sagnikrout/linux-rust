//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/refcounted_kptr.c
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
pub unsafe extern "C" fn test_refcounted_kptr() {
    void test_refcounted_kptr(void)
    {
    RUN_TESTS(refcounted_kptr);
    }
#[no_mangle]
pub unsafe extern "C" fn test_refcounted_kptr_fail() {
    void test_refcounted_kptr_fail(void)
    {
    RUN_TESTS(refcounted_kptr_fail);
    }
#[no_mangle]
pub unsafe extern "C" fn test_refcounted_kptr_wrong_owner() {
    void test_refcounted_kptr_wrong_owner(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    struct refcounted_kptr *skel;
    int ret;
    skel = refcounted_kptr__open_and_load();
    if (!ASSERT_OK_PTR(skel, "refcounted_kptr__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.rbtree_wrong_owner_remove_fail_a1), &opts);
    ASSERT_OK(ret, "rbtree_wrong_owner_remove_fail_a1");
    ASSERT_OK(opts.retval, "rbtree_wrong_owner_remove_fail_a1 retval");
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.rbtree_wrong_owner_remove_fail_b), &opts);
    ASSERT_OK(ret, "rbtree_wrong_owner_remove_fail_b");
    ASSERT_OK(opts.retval, "rbtree_wrong_owner_remove_fail_b retval");
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.rbtree_wrong_owner_remove_fail_a2), &opts);
    ASSERT_OK(ret, "rbtree_wrong_owner_remove_fail_a2");
    ASSERT_OK(opts.retval, "rbtree_wrong_owner_remove_fail_a2 retval");
    refcounted_kptr__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_percpu_hash_refcounted_kptr_refcount_leak() {
    void test_percpu_hash_refcounted_kptr_refcount_leak(void)
    {
    struct refcounted_kptr *skel;
    int cpu_nr, fd, err, key = 0;
    struct bpf_map *map;
    size_t values_sz;
    u64 *values;
    LIBBPF_OPTS(bpf_test_run_opts, opts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    LIBBPF_OPTS(bpf_test_run_opts, syscall_opts);
    cpu_nr = libbpf_num_possible_cpus();
    if (!ASSERT_GT(cpu_nr, 0, "libbpf_num_possible_cpus"))
    return;
    values = calloc(cpu_nr, sizeof(u64));
    if (!ASSERT_OK_PTR(values, "calloc values"))
    return;
    skel = refcounted_kptr__open_and_load();
    if (!ASSERT_OK_PTR(skel, "refcounted_kptr__open_and_load")) {
    free(values);
    return;
    }
    values_sz = cpu_nr * sizeof(u64);
    memset(values, 0, values_sz);
    map = skel.maps.percpu_hash;
    err = bpf_map__update_elem(map, &key, sizeof(key), values, values_sz, 0);
    if (!ASSERT_OK(err, "bpf_map__update_elem"))
    goto out;
    fd = bpf_program__fd(skel.progs.percpu_hash_refcount_leak);
    err = bpf_prog_test_run_opts(fd, &opts);
    if (!ASSERT_OK(err, "bpf_prog_test_run_opts"))
    goto out;
    if (!ASSERT_EQ(opts.retval, 2, "opts.retval"))
    goto out;
    fd = bpf_program__fd(skel.progs.clear_percpu_hash_kptr);
    err = bpf_prog_test_run_opts(fd, &syscall_opts);
    if (!ASSERT_OK(err, "bpf_prog_test_run_opts"))
    goto out;
    if (!ASSERT_EQ(syscall_opts.retval, 1, "syscall_opts.retval"))
    goto out;
    fd = bpf_program__fd(skel.progs.check_percpu_hash_refcount);
    err = bpf_prog_test_run_opts(fd, &opts);
    ASSERT_OK(err, "bpf_prog_test_run_opts");
    ASSERT_EQ(opts.retval, 1, "opts.retval");
    out:
    refcounted_kptr__destroy(skel);
    free(values);
    }
