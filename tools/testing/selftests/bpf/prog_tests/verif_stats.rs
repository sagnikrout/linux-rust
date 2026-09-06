//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/verif_stats.c
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
pub unsafe extern "C" fn test_verif_stats() {
    void test_verif_stats(void)
    {
    let mut len: __u32 = sizeof(struct bpf_prog_info);
    struct trace_vprintk_lskel *skel;
    let mut info: bpf_prog_info = {};
    int err;
    skel = trace_vprintk_lskel__open_and_load();
    if (!ASSERT_OK_PTR(skel, "trace_vprintk__open_and_load"))
    goto cleanup;
    err = bpf_prog_get_info_by_fd(skel.progs.sys_enter.prog_fd,
    &info, &len);
    if (!ASSERT_OK(err, "bpf_prog_get_info_by_fd"))
    goto cleanup;
    if (!ASSERT_GT(info.verified_insns, 0, "verified_insns"))
    goto cleanup;
    cleanup:
    trace_vprintk_lskel__destroy(skel);
    }
