//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/ksock_wq.c
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
// Copyright (c) 2026 Isovalent

pub const CALLBACK_WAIT_RETRIES: c_int = 1000;
pub const CALLBACK_WAIT_US: c_int = 1000;
#[no_mangle]
pub unsafe extern "C" fn test_ksock_wq() {
    void test_ksock_wq(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    struct ksock_wq *skel;
    u32 callback_done;
    int err, i;
    skel = ksock_wq__open_and_load();
    if (!ASSERT_OK_PTR(skel, "ksock_wq open and load"))
    return;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.ksock_wq_start),
    &opts);
    if (!ASSERT_OK(err, "run ksock_wq_start"))
    goto out;
    if (!ASSERT_OK(opts.retval, "ksock_wq_start retval"))
    goto out;
    for (i = 0; i < CALLBACK_WAIT_RETRIES; i++) {
    if (__atomic_load_n(&skel.bss.callback_done, __ATOMIC_ACQUIRE))
    break;
    usleep(CALLBACK_WAIT_US);
    }
    callback_done = __atomic_load_n(&skel.bss.callback_done,
    __ATOMIC_ACQUIRE);
    if (!ASSERT_EQ(callback_done, 1, "workqueue callback completed"))
    goto out;
    ASSERT_EQ(skel.bss.create_err, -EOPNOTSUPP,
    "workqueue create rejected");
    out:
    ksock_wq__destroy(skel);
    }
