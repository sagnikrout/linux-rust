//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_xattr.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

    static const char xattr_value_a[] = "bpf_selftest_value_a";
    static const char xattr_value_b[] = "bpf_selftest_value_b";
    static const char xattr_name[] = "user.bpf_test";
#[no_mangle]
unsafe extern "C" fn test_read_cgroup_xattr() {
    static void test_read_cgroup_xattr(void)
    {
    int tmp_fd, parent_cgroup_fd = -1, child_cgroup_fd = -1;
    struct read_cgroupfs_xattr *skel = core::ptr::null_mut();
    parent_cgroup_fd = test__join_cgroup(CGROUP_FS_PARENT);
    if (!ASSERT_OK_FD(parent_cgroup_fd, "create parent cgroup"))
    return;
    if (!ASSERT_OK(set_cgroup_xattr(CGROUP_FS_PARENT, xattr_name, xattr_value_a),
    "set parent xattr"))
    goto out;
    child_cgroup_fd = test__join_cgroup(CGROUP_FS_CHILD);
    if (!ASSERT_OK_FD(child_cgroup_fd, "create child cgroup"))
    goto out;
    if (!ASSERT_OK(set_cgroup_xattr(CGROUP_FS_CHILD, xattr_name, xattr_value_b),
    "set child xattr"))
    goto out;
    skel = read_cgroupfs_xattr__open_and_load();
    if (!ASSERT_OK_PTR(skel, "read_cgroupfs_xattr__open_and_load"))
    goto out;
    skel.bss.target_pid = sys_gettid();
    if (!ASSERT_OK(read_cgroupfs_xattr__attach(skel), "read_cgroupfs_xattr__attach"))
    goto out;
    tmp_fd = open(TMP_FILE, O_RDONLY | O_CREAT);
    ASSERT_OK_FD(tmp_fd, "open tmp file");
    close(tmp_fd);
    ASSERT_TRUE(skel.bss.found_value_a, "found_value_a");
    ASSERT_TRUE(skel.bss.found_value_b, "found_value_b");
    out:
    close(child_cgroup_fd);
    close(parent_cgroup_fd);
    read_cgroupfs_xattr__destroy(skel);
    unlink(TMP_FILE);
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_xattr() {
    void test_cgroup_xattr(void)
    {
    RUN_TESTS(cgroup_read_xattr);
    if (test__start_subtest("read_cgroupfs_xattr"))
    test_read_cgroup_xattr();
    }
