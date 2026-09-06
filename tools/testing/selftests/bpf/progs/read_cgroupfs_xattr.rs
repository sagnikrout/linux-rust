//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/read_cgroupfs_xattr.c
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

    char _license[] SEC("license") = "GPL";
    let mut target_pid: pid_t = 0;
    char xattr_value[64];
    static const char expected_value_a[] = "bpf_selftest_value_a";
    static const char expected_value_b[] = "bpf_selftest_value_b";
    bool found_value_a;
    bool found_value_b;
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_file_open) -> c_int {
    int BPF_PROG(test_file_open)
    {
    let mut cgrp_id: u64 = bpf_get_current_cgroup_id();
    struct cgroup_subsys_state *css, *tmp;
    struct bpf_dynptr value_ptr;
    struct cgroup *cgrp;
    if ((bpf_get_current_pid_tgid() >> 32) != target_pid)
    return 0;
    bpf_rcu_read_lock();
    cgrp = bpf_cgroup_from_id(cgrp_id);
    if (!cgrp) {
    bpf_rcu_read_unlock();
    return 0;
    }
    css = &cgrp.self;
    bpf_dynptr_from_mem(xattr_value, sizeof(xattr_value), 0, &value_ptr);
    bpf_for_each(css, tmp, css, BPF_CGROUP_ITER_ANCESTORS_UP) {
    int ret;
    ret = bpf_cgroup_read_xattr(tmp.cgroup, "user.bpf_test",
    &value_ptr);
    if (ret < 0)
    continue;
    if (ret == sizeof(expected_value_a) &&
    !bpf_strncmp(xattr_value, sizeof(expected_value_a), expected_value_a))
    found_value_a = true;
    if (ret == sizeof(expected_value_b) &&
    !bpf_strncmp(xattr_value, sizeof(expected_value_b), expected_value_b))
    found_value_b = true;
    }
    bpf_rcu_read_unlock();
    bpf_cgroup_release(cgrp);
    return 0;
    }
