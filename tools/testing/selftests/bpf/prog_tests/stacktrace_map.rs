//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/stacktrace_map.c
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

#[no_mangle]
pub unsafe extern "C" fn test_stacktrace_map() {
    void test_stacktrace_map(void)
    {
    struct stacktrace_map *skel;
    int control_map_fd, stackid_hmap_fd, stackmap_fd, stack_amap_fd;
    int err, stack_trace_len;
    __u32 key, val, stack_id, duration = 0;
    __u64 stack[PERF_MAX_STACK_DEPTH];
    skel = stacktrace_map__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    control_map_fd = bpf_map__fd(skel.maps.control_map);
    stackid_hmap_fd = bpf_map__fd(skel.maps.stackid_hmap);
    stackmap_fd = bpf_map__fd(skel.maps.stackmap);
    stack_amap_fd = bpf_map__fd(skel.maps.stack_amap);
    err = stacktrace_map__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto out;
// give some time for bpf program run
    sleep(1);
// disable stack trace collection
    key = 0;
    val = 1;
    bpf_map_update_elem(control_map_fd, &key, &val, 0);
// for every element in stackid_hmap, we can find a corresponding one
// in stackmap, and vice versa.
//
    err = compare_map_keys(stackid_hmap_fd, stackmap_fd);
    if (CHECK(err, "compare_map_keys stackid_hmap vs. stackmap",
    "err %d errno %d\n", err, errno))
    goto out;
    err = compare_map_keys(stackmap_fd, stackid_hmap_fd);
    if (CHECK(err, "compare_map_keys stackmap vs. stackid_hmap",
    "err %d errno %d\n", err, errno))
    goto out;
    stack_trace_len = PERF_MAX_STACK_DEPTH * sizeof(__u64);
    err = compare_stack_ips(stackmap_fd, stack_amap_fd, stack_trace_len);
    if (CHECK(err, "compare_stack_ips stackmap vs. stack_amap",
    "err %d errno %d\n", err, errno))
    goto out;
    stack_id = skel.bss.stack_id;
    err = bpf_map_lookup_and_delete_elem(stackmap_fd, &stack_id,  stack);
    if (!ASSERT_OK(err, "lookup and delete target stack_id"))
    goto out;
    err = bpf_map_lookup_elem(stackmap_fd, &stack_id, stack);
    if (!ASSERT_EQ(err, -ENOENT, "lookup deleted stack_id"))
    goto out;
    out:
    stacktrace_map__destroy(skel);
    }
