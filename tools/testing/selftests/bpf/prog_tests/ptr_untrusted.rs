//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/ptr_untrusted.c
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
// Copyright (C) 2023 Yafang Shao <laoar.shao@gmail.com>

#[no_mangle]
pub unsafe extern "C" fn serial_test_ptr_untrusted() {
    void serial_test_ptr_untrusted(void)
    {
    struct test_ptr_untrusted *skel;
    int err;
    skel = test_ptr_untrusted__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    goto cleanup;
// First, attach lsm prog
    skel.links.lsm_run = bpf_program__attach_lsm(skel.progs.lsm_run);
    if (!ASSERT_OK_PTR(skel.links.lsm_run, "lsm_attach"))
    goto cleanup;
// Second, attach raw_tp prog. The lsm prog will be triggered.
    skel.links.raw_tp_run = bpf_program__attach_raw_tracepoint(skel.progs.raw_tp_run,
    TP_NAME);
    if (!ASSERT_OK_PTR(skel.links.raw_tp_run, "raw_tp_attach"))
    goto cleanup;
    err = strncmp(skel.bss.tp_name, TP_NAME, strlen(TP_NAME));
    ASSERT_EQ(err, 0, "cmp_tp_name");
    cleanup:
    test_ptr_untrusted__destroy(skel);
    }
