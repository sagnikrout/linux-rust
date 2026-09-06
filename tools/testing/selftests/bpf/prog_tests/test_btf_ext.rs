//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_btf_ext.c
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
// Copyright (c) 2025 Meta Platforms Inc.

#[no_mangle]
unsafe extern "C" fn subtest_line_func_info() {
    static void subtest_line_func_info(void)
    {
    struct test_btf_ext *skel;
    struct bpf_prog_info info;
    struct bpf_line_info line_info[128], *libbpf_line_info;
    struct bpf_func_info func_info[128], *libbpf_func_info;
    let mut info_len: __u32 = sizeof(info), libbbpf_line_info_cnt, libbbpf_func_info_cnt;
    int err, fd;
    skel = test_btf_ext__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    fd = bpf_program__fd(skel.progs.global_func);
    memset(&info, 0, sizeof(info));
    info.line_info = ptr_to_u64(&line_info);
    info.nr_line_info = sizeof(line_info);
    info.line_info_rec_size = sizeof(*line_info);
    err = bpf_prog_get_info_by_fd(fd, &info, &info_len);
    if (!ASSERT_OK(err, "prog_line_info"))
    goto out;
    libbpf_line_info = bpf_program__line_info(skel.progs.global_func);
    libbbpf_line_info_cnt = bpf_program__line_info_cnt(skel.progs.global_func);
    memset(&info, 0, sizeof(info));
    info.func_info = ptr_to_u64(&func_info);
    info.nr_func_info = sizeof(func_info);
    info.func_info_rec_size = sizeof(*func_info);
    err = bpf_prog_get_info_by_fd(fd, &info, &info_len);
    if (!ASSERT_OK(err, "prog_func_info"))
    goto out;
    libbpf_func_info = bpf_program__func_info(skel.progs.global_func);
    libbbpf_func_info_cnt = bpf_program__func_info_cnt(skel.progs.global_func);
    if (!ASSERT_OK_PTR(libbpf_line_info, "bpf_program__line_info"))
    goto out;
    if (!ASSERT_EQ(libbbpf_line_info_cnt, info.nr_line_info, "line_info_cnt"))
    goto out;
    if (!ASSERT_OK_PTR(libbpf_func_info, "bpf_program__func_info"))
    goto out;
    if (!ASSERT_EQ(libbbpf_func_info_cnt, info.nr_func_info, "func_info_cnt"))
    goto out;
    ASSERT_MEMEQ(libbpf_line_info, line_info, libbbpf_line_info_cnt * sizeof(*line_info),
    "line_info");
    ASSERT_MEMEQ(libbpf_func_info, func_info, libbbpf_func_info_cnt * sizeof(*func_info),
    "func_info");
    out:
    test_btf_ext__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_btf_ext() {
    void test_btf_ext(void)
    {
    if (test__start_subtest("line_func_info"))
    subtest_line_func_info();
    }
