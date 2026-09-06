//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/core_retro.c
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
// Copyright (c) 2020 Facebook
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn test_core_retro() {
    void test_core_retro(void)
    {
    int err, zero = 0, res, my_pid = getpid();
    struct test_core_retro *skel;
// load program
    skel = test_core_retro__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_load"))
    goto out_close;
    err = bpf_map__update_elem(skel.maps.exp_tgid_map, &zero, sizeof(zero),
    &my_pid, sizeof(my_pid), 0);
    if (!ASSERT_OK(err, "map_update"))
    goto out_close;
// attach probe
    err = test_core_retro__attach(skel);
    if (!ASSERT_OK(err, "attach_kprobe"))
    goto out_close;
// trigger
    usleep(1);
    err = bpf_map__lookup_elem(skel.maps.results, &zero, sizeof(zero), &res, sizeof(res), 0);
    if (!ASSERT_OK(err, "map_lookup"))
    goto out_close;
    ASSERT_EQ(res, my_pid, "pid_check");
    out_close:
    test_core_retro__destroy(skel);
    }
