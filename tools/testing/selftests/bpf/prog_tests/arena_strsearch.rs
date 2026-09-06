//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/arena_strsearch.c
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

#[no_mangle]
unsafe extern "C" fn test_arena_str() {
    static void test_arena_str(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    struct arena_strsearch *skel;
    int ret;
    skel = arena_strsearch__open_and_load();
    if (!ASSERT_OK_PTR(skel, "arena_strsearch__open_and_load"))
    return;
    ret = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.arena_strsearch), &opts);
    ASSERT_OK(ret, "ret_add");
    ASSERT_OK(opts.retval, "retval");
    if (skel.bss.skip) {
    printf("%s:SKIP:compiler doesn't support arena_cast\n", __func__);
    test__skip();
    }
    arena_strsearch__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_arena_strsearch() {
    void serial_test_arena_strsearch(void)
    {
    if (test__start_subtest("arena_strsearch"))
    test_arena_str();
    }
