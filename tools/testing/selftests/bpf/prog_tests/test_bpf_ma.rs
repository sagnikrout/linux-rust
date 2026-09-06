//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_bpf_ma.c
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
// Copyright (C) 2023. Huawei Technologies Co., Ltd
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn do_bpf_ma_test(name: *const c_char) {
    static void do_bpf_ma_test(const char *name)
    {
    struct test_bpf_ma *skel;
    struct bpf_program *prog;
    struct btf *btf;
    int i, err, id;
    char tname[32];
    skel = test_bpf_ma__open();
    if (!ASSERT_OK_PTR(skel, "open"))
    return;
    btf = bpf_object__btf(skel.obj);
    if (!ASSERT_OK_PTR(btf, "btf"))
    goto out;
    for (i = 0; i < ARRAY_SIZE(skel.rodata.data_sizes); i++) {
    snprintf(tname, sizeof(tname), "bin_data_%u", skel.rodata.data_sizes[i]);
    id = btf__find_by_name_kind(btf, tname, BTF_KIND_STRUCT);
    if (!ASSERT_GT(id, 0, tname))
    goto out;
    skel.rodata.data_btf_ids[i] = id;
    }
    for (i = 0; i < ARRAY_SIZE(skel.rodata.percpu_data_sizes); i++) {
    snprintf(tname, sizeof(tname), "percpu_bin_data_%u", skel.rodata.percpu_data_sizes[i]);
    id = btf__find_by_name_kind(btf, tname, BTF_KIND_STRUCT);
    if (!ASSERT_GT(id, 0, tname))
    goto out;
    skel.rodata.percpu_data_btf_ids[i] = id;
    }
    prog = bpf_object__find_program_by_name(skel.obj, name);
    if (!ASSERT_OK_PTR(prog, "invalid prog name"))
    goto out;
    bpf_program__set_autoload(prog, true);
    err = test_bpf_ma__load(skel);
    if (!ASSERT_OK(err, "load"))
    goto out;
    err = test_bpf_ma__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    goto out;
    skel.bss.pid = getpid();
    usleep(1);
    ASSERT_OK(skel.bss.err, "test error");
    out:
    test_bpf_ma__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_test_bpf_ma() {
    void test_test_bpf_ma(void)
    {
    if (test__start_subtest("batch_alloc_free"))
    do_bpf_ma_test("test_batch_alloc_free");
    if (test__start_subtest("free_through_map_free"))
    do_bpf_ma_test("test_free_through_map_free");
    if (test__start_subtest("batch_percpu_alloc_free"))
    do_bpf_ma_test("test_batch_percpu_alloc_free");
    if (test__start_subtest("percpu_free_through_map_free"))
    do_bpf_ma_test("test_percpu_free_through_map_free");
    }
