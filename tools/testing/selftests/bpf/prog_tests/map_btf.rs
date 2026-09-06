//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/map_btf.c
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

#[no_mangle]
unsafe extern "C" fn do_test_normal_map_btf() {
    static void do_test_normal_map_btf(void)
    {
    struct normal_map_btf *skel;
    int i, err, new_fd = -1;
    int map_fd_arr[64];
    skel = normal_map_btf__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_load"))
    return;
    err = normal_map_btf__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    goto out;
    skel.bss.pid = getpid();
    usleep(1);
    ASSERT_TRUE(skel.bss.done, "done");
// Use percpu_array to slow bpf_map_free_deferred() down.
// The memory allocation may fail, so doesn't check the returned fd.
//
    for (i = 0; i < ARRAY_SIZE(map_fd_arr); i++)
    map_fd_arr[i] = bpf_map_create(BPF_MAP_TYPE_PERCPU_ARRAY, core::ptr::null_mut(), 4, 4, 256, core::ptr::null_mut());
// Close array fd later
    new_fd = dup(bpf_map__fd(skel.maps.array));
    out:
    normal_map_btf__destroy(skel);
    if (new_fd < 0)
    return;
// Use kern_sync_rcu() to wait for the start of the free of the bpf
// program and use an assumed delay to wait for the release of the map
// btf which is held by other maps (e.g, bss). After that, array map
// holds the last reference of map btf.
//
    kern_sync_rcu();
    usleep(4000);
// Spawn multiple kworkers to delay the invocation of
// bpf_map_free_deferred() for array map.
//
    for (i = 0; i < ARRAY_SIZE(map_fd_arr); i++) {
    if (map_fd_arr[i] < 0)
    continue;
    close(map_fd_arr[i]);
    }
    close(new_fd);
    }
#[no_mangle]
unsafe extern "C" fn do_test_map_in_map_btf() {
    static void do_test_map_in_map_btf(void)
    {
    int err, zero = 0, new_fd = -1;
    struct map_in_map_btf *skel;
    skel = map_in_map_btf__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_load"))
    return;
    err = map_in_map_btf__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    goto out;
    skel.bss.pid = getpid();
    usleep(1);
    ASSERT_TRUE(skel.bss.done, "done");
// Close inner_array fd later
    new_fd = dup(bpf_map__fd(skel.maps.inner_array));
// Defer the free of inner_array
    err = bpf_map__delete_elem(skel.maps.outer_array, &zero, sizeof(zero), 0);
    ASSERT_OK(err, "delete inner map");
    out:
    map_in_map_btf__destroy(skel);
    if (new_fd < 0)
    return;
// Use kern_sync_rcu() to wait for the start of the free of the bpf
// program and use an assumed delay to wait for the free of the outer
// map and the release of map btf. After that, inner map holds the last
// reference of map btf.
//
    kern_sync_rcu();
    usleep(10000);
    close(new_fd);
    }
#[no_mangle]
pub unsafe extern "C" fn test_map_btf() {
    void test_map_btf(void)
    {
    if (test__start_subtest("array_btf"))
    do_test_normal_map_btf();
    if (test__start_subtest("inner_array_btf"))
    do_test_map_in_map_btf();
    }
