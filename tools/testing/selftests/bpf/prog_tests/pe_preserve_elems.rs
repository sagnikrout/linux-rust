//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/pe_preserve_elems.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2019 Facebook

    static int duration;
    static void test_one_map(struct bpf_map *map, struct bpf_program *prog,
    bool has_share_pe)
    {
    int err, key = 0, pfd = -1, mfd = bpf_map__fd(map);
    DECLARE_LIBBPF_OPTS(bpf_test_run_opts, opts);
    struct perf_event_attr attr = {
    .size = sizeof(struct perf_event_attr),
    .type = PERF_TYPE_SOFTWARE,
    .config = PERF_COUNT_SW_CPU_CLOCK,
    };
    pfd = syscall(__NR_perf_event_open, &attr, 0 /* pid */,
    -1 /* cpu 0 */, -1 /* group id */, 0 /* flags */);
    if (CHECK(pfd < 0, "perf_event_open", "failed\n"))
    return;
    err = bpf_map_update_elem(mfd, &key, &pfd, BPF_ANY);
    close(pfd);
    if (CHECK(err < 0, "bpf_map_update_elem", "failed\n"))
    return;
    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &opts);
    if (CHECK(err < 0, "bpf_prog_test_run_opts", "failed\n"))
    return;
    if (CHECK(opts.retval != 0, "bpf_perf_event_read_value",
    "failed with %d\n", opts.retval))
    return;
// closing mfd, prog still holds a reference on map
    close(mfd);
    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &opts);
    if (CHECK(err < 0, "bpf_prog_test_run_opts", "failed\n"))
    return;
    if (has_share_pe) {
    CHECK(opts.retval != 0, "bpf_perf_event_read_value",
    "failed with %d\n", opts.retval);
    } else {
    CHECK(opts.retval != -ENOENT, "bpf_perf_event_read_value",
    "should have failed with %d, but got %d\n", -ENOENT,
    opts.retval);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_pe_preserve_elems() {
    void test_pe_preserve_elems(void)
    {
    struct test_pe_preserve_elems *skel;
    skel = test_pe_preserve_elems__open_and_load();
    if (CHECK(!skel, "skel_open", "failed to open skeleton\n"))
    return;
    test_one_map(skel.maps.array_1, skel.progs.read_array_1, false);
    test_one_map(skel.maps.array_2, skel.progs.read_array_2, true);
    test_pe_preserve_elems__destroy(skel);
    }
