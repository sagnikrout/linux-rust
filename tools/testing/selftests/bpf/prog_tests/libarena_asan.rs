//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/libarena_asan.c
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

    static void run_libarena_asan_test(struct libarena_asan *skel,
    struct bpf_program *prog, const char *name)
    {
    int ret;
    if (strstr(name, "test_buddy")) {
// Buddy tests initialize the allocator directly.
    ret = libarena_run_prog(bpf_program__fd(skel.progs.arena_buddy_destroy));
    if (!ASSERT_OK(ret, "arena_buddy_destroy"))
    return;
    } else {
    ret = libarena_run_prog(bpf_program__fd(skel.progs.arena_buddy_reset));
    if (!ASSERT_OK(ret, "arena_buddy_reset"))
    return;
    }
    ret = libarena_run_prog(bpf_program__fd(prog));
    ASSERT_OK(ret, name);
    verify_test_stderr(skel.obj, prog);
    }
#[no_mangle]
unsafe extern "C" fn run_test() {
    static void run_test(void)
    {
    struct arena_alloc_reserve_args args;
    struct libarena_asan *skel;
    struct bpf_program *prog;
    int ret;
    skel = libarena_asan__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_and_load"))
    return;
    ret = libarena_asan__attach(skel);
    if (!ASSERT_OK(ret, "attach"))
    goto out;
    args.nr_pages = ARENA_RESERVE_PAGES_DFL;
    ret = libarena_run_prog_args(bpf_program__fd(skel.progs.arena_alloc_reserve),
    &args, sizeof(args));
    if (!ASSERT_OK(ret, "arena_alloc_reserve"))
    goto out;
    ret = libarena_asan_init(
    bpf_program__fd(skel.progs.arena_get_info),
    bpf_program__fd(skel.progs.asan_init),
    (1ULL << 32) / sysconf(_SC_PAGESIZE));
    if (!ASSERT_OK(ret, "libarena_asan_init"))
    goto out;
    bpf_object__for_each_program(prog, skel.obj) {
    const char *name = bpf_program__name(prog);
    if (!libarena_is_asan_test_prog(name))
    continue;
    if (!test__start_subtest(name))
    continue;
    run_libarena_asan_test(skel, prog, name);
    }
    out:
    libarena_asan__destroy(skel);
    }

//
// Run the test depending on whether LLVM can compile arena ASAN
// programs.
//
#[no_mangle]
pub unsafe extern "C" fn serial_test_libarena_asan() {
    void serial_test_libarena_asan(void)
    {

    run_test();

    test__skip();

    return;
    }
