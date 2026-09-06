//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/metadata.c
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
//
// Copyright 2020 Google LLC.
//

    static int duration;
#[no_mangle]
unsafe extern "C" fn prog_holds_map(prog_fd: c_int, map_fd: c_int) -> c_int {
    static int prog_holds_map(int prog_fd, int map_fd)
    {
    let mut prog_info: bpf_prog_info = {};
    let mut map_info: bpf_map_info = {};
    __u32 prog_info_len;
    __u32 map_info_len;
    __u32 *map_ids;
    int nr_maps;
    int ret;
    int i;
    map_info_len = sizeof(map_info);
    ret = bpf_map_get_info_by_fd(map_fd, &map_info, &map_info_len);
    if (ret)
    return -errno;
    prog_info_len = sizeof(prog_info);
    ret = bpf_prog_get_info_by_fd(prog_fd, &prog_info, &prog_info_len);
    if (ret)
    return -errno;
    map_ids = calloc(prog_info.nr_map_ids, sizeof(__u32));
    if (!map_ids)
    return -ENOMEM;
    nr_maps = prog_info.nr_map_ids;
    memset(&prog_info, 0, sizeof(prog_info));
    prog_info.nr_map_ids = nr_maps;
    prog_info.map_ids = ptr_to_u64(map_ids);
    prog_info_len = sizeof(prog_info);
    ret = bpf_prog_get_info_by_fd(prog_fd, &prog_info, &prog_info_len);
    if (ret) {
    ret = -errno;
    goto free_map_ids;
    }
    ret = -ENOENT;
    for (i = 0; i < prog_info.nr_map_ids; i++) {
    if (map_ids[i] == map_info.id) {
    ret = 0;
    break;
    }
    }
    free_map_ids:
    free(map_ids);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_metadata_unused() {
    static void test_metadata_unused(void)
    {
    struct metadata_unused *obj;
    int err;
    obj = metadata_unused__open_and_load();
    if (CHECK(!obj, "skel-load", "errno %d", errno))
    return;
    err = prog_holds_map(bpf_program__fd(obj.progs.prog),
    bpf_map__fd(obj.maps.rodata));
    if (CHECK(err, "prog-holds-rodata", "errno: %d", err))
    return;
// Assert that we can access the metadata in skel and the values are
// what we expect.
//
    if (CHECK(strncmp(obj.rodata.bpf_metadata_a, "foo",
    sizeof(obj.rodata.bpf_metadata_a)),
    "bpf_metadata_a", "expected \"foo\", value differ"))
    goto close_bpf_object;
    if (CHECK(obj.rodata.bpf_metadata_b != 1, "bpf_metadata_b",
    "expected 1, got %d", obj.rodata.bpf_metadata_b))
    goto close_bpf_object;
// Assert that binding metadata map to prog again succeeds.
    err = bpf_prog_bind_map(bpf_program__fd(obj.progs.prog),
    bpf_map__fd(obj.maps.rodata), core::ptr::null_mut());
    CHECK(err, "rebind_map", "errno %d, expected 0", errno);
    close_bpf_object:
    metadata_unused__destroy(obj);
    }
#[no_mangle]
unsafe extern "C" fn test_metadata_used() {
    static void test_metadata_used(void)
    {
    struct metadata_used *obj;
    int err;
    obj = metadata_used__open_and_load();
    if (CHECK(!obj, "skel-load", "errno %d", errno))
    return;
    err = prog_holds_map(bpf_program__fd(obj.progs.prog),
    bpf_map__fd(obj.maps.rodata));
    if (CHECK(err, "prog-holds-rodata", "errno: %d", err))
    return;
// Assert that we can access the metadata in skel and the values are
// what we expect.
//
    if (CHECK(strncmp(obj.rodata.bpf_metadata_a, "bar",
    sizeof(obj.rodata.bpf_metadata_a)),
    "metadata_a", "expected \"bar\", value differ"))
    goto close_bpf_object;
    if (CHECK(obj.rodata.bpf_metadata_b != 2, "metadata_b",
    "expected 2, got %d", obj.rodata.bpf_metadata_b))
    goto close_bpf_object;
// Assert that binding metadata map to prog again succeeds.
    err = bpf_prog_bind_map(bpf_program__fd(obj.progs.prog),
    bpf_map__fd(obj.maps.rodata), core::ptr::null_mut());
    CHECK(err, "rebind_map", "errno %d, expected 0", errno);
    close_bpf_object:
    metadata_used__destroy(obj);
    }
#[no_mangle]
pub unsafe extern "C" fn test_metadata() {
    void test_metadata(void)
    {
    if (test__start_subtest("unused"))
    test_metadata_unused();
    if (test__start_subtest("used"))
    test_metadata_used();
    }
