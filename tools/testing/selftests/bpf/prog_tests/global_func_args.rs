//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/global_func_args.c
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

    static __u32 duration;
#[no_mangle]
unsafe extern "C" fn test_global_func_args0(obj: *mut bpf_object) {
    static void test_global_func_args0(struct bpf_object *obj)
    {
    int err, i, map_fd, actual_value;
    const char *map_name = "values";
    map_fd = bpf_find_map(__func__, obj, map_name);
    if (CHECK(map_fd < 0, "bpf_find_map", "cannot find BPF map %s: %s\n",
    map_name, strerror(errno)))
    return;
    struct {
    const char *descr;
    int expected_value;
    } tests[] = {
    {"passing core::ptr::null_mut() pointer", 0},
    {"returning value", 1},
    {"reading local variable", 100 },
    {"writing local variable", 101 },
    {"reading global variable", 42 },
    {"writing global variable", 43 },
    {"writing to pointer-to-pointer", 1 },
    };
    for (i = 0; i < ARRAY_SIZE(tests); ++i) {
    let mut expected_value: c_int = tests[i].expected_value;
    err = bpf_map_lookup_elem(map_fd, &i, &actual_value);
    CHECK(err || actual_value != expected_value, tests[i].descr,
    "err %d result %d expected %d\n", err, actual_value, expected_value);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_global_func_args() {
    void test_global_func_args(void)
    {
    const char *file = "./test_global_func_args.bpf.o";
    struct bpf_object *obj;
    int err, prog_fd;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = 1,
    );
    err = bpf_prog_test_load(file, BPF_PROG_TYPE_CGROUP_SKB, &obj, &prog_fd);
    if (CHECK(err, "load program", "error %d loading %s\n", err, file))
    return;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_OK(topts.retval, "test_run retval");
    test_global_func_args0(obj);
    bpf_object__close(obj);
    }
