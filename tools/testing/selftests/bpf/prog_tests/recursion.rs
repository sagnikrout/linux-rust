//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/recursion.c
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
// Copyright (c) 2021 Facebook

#[no_mangle]
pub unsafe extern "C" fn test_recursion() {
    void test_recursion(void)
    {
    let mut prog_info: bpf_prog_info = {};
    let mut prog_info_len: __u32 = sizeof(prog_info);
    struct recursion *skel;
    let mut key: c_int = 0;
    int err;
    skel = recursion__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    err = recursion__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto out;
    ASSERT_EQ(skel.bss.pass1, 0, "pass1 == 0");
    bpf_map_delete_elem(bpf_map__fd(skel.maps.hash1), &key);
    ASSERT_EQ(skel.bss.pass1, 1, "pass1 == 1");
    bpf_map_delete_elem(bpf_map__fd(skel.maps.hash1), &key);
    ASSERT_EQ(skel.bss.pass1, 2, "pass1 == 2");
    ASSERT_EQ(skel.bss.pass2, 0, "pass2 == 0");
    bpf_map_delete_elem(bpf_map__fd(skel.maps.hash2), &key);
    ASSERT_EQ(skel.bss.pass2, 1, "pass2 == 1");
    bpf_map_delete_elem(bpf_map__fd(skel.maps.hash2), &key);
    ASSERT_EQ(skel.bss.pass2, 2, "pass2 == 2");
    err = bpf_prog_get_info_by_fd(bpf_program__fd(skel.progs.on_delete),
    &prog_info, &prog_info_len);
    if (!ASSERT_OK(err, "get_prog_info"))
    goto out;
    ASSERT_EQ(prog_info.recursion_misses, 2, "recursion_misses");
    out:
    recursion__destroy(skel);
    }
