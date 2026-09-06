//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/load_bytes_relative.c
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

#[no_mangle]
pub unsafe extern "C" fn test_load_bytes_relative() {
    void test_load_bytes_relative(void)
    {
    int server_fd, cgroup_fd, prog_fd, map_fd, client_fd;
    int err;
    struct bpf_object *obj;
    struct bpf_program *prog;
    struct bpf_map *test_result;
    let mut duration: __u32 = 0;
    let mut map_key: __u32 = 0;
    let mut map_value: __u32 = 0;
    cgroup_fd = test__join_cgroup("/load_bytes_relative");
    if (CHECK_FAIL(cgroup_fd < 0))
    return;
    server_fd = start_server(AF_INET, SOCK_STREAM, core::ptr::null_mut(), 0, 0);
    if (CHECK_FAIL(server_fd < 0))
    goto close_cgroup_fd;
    err = bpf_prog_test_load("./load_bytes_relative.bpf.o", BPF_PROG_TYPE_CGROUP_SKB,
    &obj, &prog_fd);
    if (CHECK_FAIL(err))
    goto close_server_fd;
    test_result = bpf_object__find_map_by_name(obj, "test_result");
    if (CHECK_FAIL(!test_result))
    goto close_bpf_object;
    map_fd = bpf_map__fd(test_result);
    if (map_fd < 0)
    goto close_bpf_object;
    prog = bpf_object__find_program_by_name(obj, "load_bytes_relative");
    if (CHECK_FAIL(!prog))
    goto close_bpf_object;
    err = bpf_prog_attach(prog_fd, cgroup_fd, BPF_CGROUP_INET_EGRESS,
    BPF_F_ALLOW_MULTI);
    if (CHECK_FAIL(err))
    goto close_bpf_object;
    client_fd = connect_to_fd(server_fd, 0);
    if (CHECK_FAIL(client_fd < 0))
    goto close_bpf_object;
    close(client_fd);
    err = bpf_map_lookup_elem(map_fd, &map_key, &map_value);
    if (CHECK_FAIL(err))
    goto close_bpf_object;
    CHECK(map_value != 1, "bpf", "bpf program returned failure");
    close_bpf_object:
    bpf_object__close(obj);
    close_server_fd:
    close(server_fd);
    close_cgroup_fd:
    close(cgroup_fd);
    }
