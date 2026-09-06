//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_get_xattr.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    __u32 monitored_pid;
    __u32 found_xattr_from_file;
    __u32 found_xattr_from_dentry;
    static const char expected_value[] = "hello";
    char value1[32];
    char value2[32];
// Matches caller of test_get_xattr() in prog_tests/fs_kfuncs.c
    static const char xattr_names[][64] = {
// The following work.
    "user.kfuncs",
    "security.bpf.xxx",
// The following do not work.
    "security.bpf",
    "security.selinux"
    };
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_file_open, f: *mut file) -> c_int {
    int BPF_PROG(test_file_open, struct file *f)
    {
    struct bpf_dynptr value_ptr;
    __u32 pid;
    int ret, i;
    pid = bpf_get_current_pid_tgid() >> 32;
    if (pid != monitored_pid)
    return 0;
    bpf_dynptr_from_mem(value1, sizeof(value1), 0, &value_ptr);
    for (i = 0; i < ARRAY_SIZE(xattr_names); i++) {
    ret = bpf_get_file_xattr(f, xattr_names[i], &value_ptr);
    if (ret == sizeof(expected_value))
    break;
    }
    if (ret != sizeof(expected_value))
    return 0;
    if (bpf_strncmp(value1, ret, expected_value))
    return 0;
    found_xattr_from_file = 1;
    return 0;
    }
    SEC("lsm.s/inode_getxattr")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_inode_getxattr, dentry: *mut dentry, name: *mut c_char) -> c_int {
    int BPF_PROG(test_inode_getxattr, struct dentry *dentry, char *name)
    {
    struct bpf_dynptr value_ptr;
    __u32 pid;
    int ret, i;
    pid = bpf_get_current_pid_tgid() >> 32;
    if (pid != monitored_pid)
    return 0;
    bpf_dynptr_from_mem(value2, sizeof(value2), 0, &value_ptr);
    for (i = 0; i < ARRAY_SIZE(xattr_names); i++) {
    ret = bpf_get_dentry_xattr(dentry, xattr_names[i], &value_ptr);
    if (ret == sizeof(expected_value))
    break;
    }
    if (ret != sizeof(expected_value))
    return 0;
    if (bpf_strncmp(value2, ret, expected_value))
    return 0;
    found_xattr_from_dentry = 1;
// return non-zero to fail getxattr from user space
    return -EINVAL;
    }
