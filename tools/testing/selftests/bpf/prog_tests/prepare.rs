//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/prepare.c
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
// Copyright (c) 2025 Meta

#[no_mangle]
unsafe extern "C" fn check_prepared(obj: *mut bpf_object) -> bool {
    static bool check_prepared(struct bpf_object *obj)
    {
    let mut is_prepared: bool = true;
    const struct bpf_map *map;
    bpf_object__for_each_map(map, obj) {
    if (bpf_map__fd(map) < 0)
    is_prepared = false;
    }
    return is_prepared;
    }
#[no_mangle]
unsafe extern "C" fn test_prepare_no_load() {
    static void test_prepare_no_load(void)
    {
    struct prepare *skel;
    int err;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    );
    skel = prepare__open();
    if (!ASSERT_OK_PTR(skel, "prepare__open"))
    return;
    if (!ASSERT_FALSE(check_prepared(skel.obj), "not check_prepared"))
    goto cleanup;
    err = bpf_object__prepare(skel.obj);
    if (!ASSERT_TRUE(check_prepared(skel.obj), "check_prepared"))
    goto cleanup;
    if (!ASSERT_OK(err, "bpf_object__prepare"))
    goto cleanup;
    cleanup:
    prepare__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_prepare_load() {
    static void test_prepare_load(void)
    {
    struct prepare *skel;
    int err, prog_fd;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    );
    skel = prepare__open();
    if (!ASSERT_OK_PTR(skel, "prepare__open"))
    return;
    if (!ASSERT_FALSE(check_prepared(skel.obj), "not check_prepared"))
    goto cleanup;
    err = bpf_object__prepare(skel.obj);
    if (!ASSERT_OK(err, "bpf_object__prepare"))
    goto cleanup;
    err = prepare__load(skel);
    if (!ASSERT_OK(err, "prepare__load"))
    goto cleanup;
    if (!ASSERT_TRUE(check_prepared(skel.obj), "check_prepared"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.program);
    if (!ASSERT_GE(prog_fd, 0, "prog_fd"))
    goto cleanup;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    if (!ASSERT_OK(err, "test_run_opts err"))
    goto cleanup;
    if (!ASSERT_OK(topts.retval, "test_run_opts retval"))
    goto cleanup;
    ASSERT_EQ(skel.bss.err, 0, "err");
    cleanup:
    prepare__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_prepare() {
    void test_prepare(void)
    {
    if (test__start_subtest("prepare_load"))
    test_prepare_load();
    if (test__start_subtest("prepare_no_load"))
    test_prepare_no_load();
    }
