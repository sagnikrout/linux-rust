//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/global_func_dead_code.c
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

#[no_mangle]
pub unsafe extern "C" fn test_global_func_dead_code() {
    void test_global_func_dead_code(void)
    {
    struct verifier_global_subprogs *tgt_skel = core::ptr::null_mut();
    struct freplace_dead_global_func *skel = core::ptr::null_mut();
    char log_buf[4096];
    int err, tgt_fd;
// first, try to load target with good global subprog
    tgt_skel = verifier_global_subprogs__open();
    if (!ASSERT_OK_PTR(tgt_skel, "tgt_skel_good_open"))
    return;
    bpf_program__set_autoload(tgt_skel.progs.chained_global_func_calls_success, true);
    err = verifier_global_subprogs__load(tgt_skel);
    if (!ASSERT_OK(err, "tgt_skel_good_load"))
    goto out;
    tgt_fd = bpf_program__fd(tgt_skel.progs.chained_global_func_calls_success);
// Attach to good non-eliminated subprog
    skel = freplace_dead_global_func__open();
    if (!ASSERT_OK_PTR(skel, "skel_good_open"))
    goto out;
    err = bpf_program__set_attach_target(skel.progs.freplace_prog, tgt_fd, "global_good");
    ASSERT_OK(err, "attach_target_good");
    err = freplace_dead_global_func__load(skel);
    if (!ASSERT_OK(err, "skel_good_load"))
    goto out;
    freplace_dead_global_func__destroy(skel);
// Try attaching to dead code-eliminated subprog
    skel = freplace_dead_global_func__open();
    if (!ASSERT_OK_PTR(skel, "skel_dead_open"))
    goto out;
    bpf_program__set_log_buf(skel.progs.freplace_prog, log_buf, sizeof(log_buf));
    err = bpf_program__set_attach_target(skel.progs.freplace_prog, tgt_fd, "global_dead");
    ASSERT_OK(err, "attach_target_dead");
    err = freplace_dead_global_func__load(skel);
    if (!ASSERT_ERR(err, "skel_dead_load"))
    goto out;
    ASSERT_HAS_SUBSTR(log_buf, "Subprog global_dead doesn't exist", "dead_subprog_missing_msg");
    out:
    verifier_global_subprogs__destroy(tgt_skel);
    freplace_dead_global_func__destroy(skel);
    }
