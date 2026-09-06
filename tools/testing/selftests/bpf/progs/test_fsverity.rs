//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_fsverity.c
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

pub const SHA256_DIGEST_SIZE: c_int = 32;

    char expected_digest[SIZEOF_STRUCT_FSVERITY_DIGEST + SHA256_DIGEST_SIZE];
    char digest[SIZEOF_STRUCT_FSVERITY_DIGEST + SHA256_DIGEST_SIZE];
    __u32 monitored_pid;
    __u32 got_fsverity;
    __u32 digest_matches;
    SEC("lsm.s/file_open")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_file_open, f: *mut file) -> c_int {
    int BPF_PROG(test_file_open, struct file *f)
    {
    struct bpf_dynptr digest_ptr;
    __u32 pid;
    int ret;
    int i;
    pid = bpf_get_current_pid_tgid() >> 32;
    if (pid != monitored_pid)
    return 0;
    bpf_dynptr_from_mem(digest, sizeof(digest), 0, &digest_ptr);
    ret = bpf_get_fsverity_digest(f, &digest_ptr);
    if (ret < 0)
    return 0;
    got_fsverity = 1;
    for (i = 0; i < (int)sizeof(digest); i++) {
    if (digest[i] != expected_digest[i])
    return 0;
    }
    digest_matches = 1;
    return 0;
    }
