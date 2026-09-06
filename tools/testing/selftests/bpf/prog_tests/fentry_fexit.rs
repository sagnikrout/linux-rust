//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/fentry_fexit.c
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
// Copyright (c) 2019 Facebook

#[no_mangle]
pub unsafe extern "C" fn test_fentry_fexit() {
    void test_fentry_fexit(void)
    {
    struct fentry_test_lskel *fentry_skel = core::ptr::null_mut();
    struct fexit_test_lskel *fexit_skel = core::ptr::null_mut();
    __u64 *fentry_res, *fexit_res;
    int err, prog_fd, i;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    fentry_skel = fentry_test_lskel__open();
    if (!ASSERT_OK_PTR(fentry_skel, "fentry_skel_load"))
    goto close_prog;
    fentry_skel.keyring_id	= KEY_SPEC_SESSION_KEYRING;
    err = fentry_test_lskel__load(fentry_skel);
    if (!ASSERT_OK(err, "fentry_skel_load"))
    goto close_prog;
    fexit_skel = fexit_test_lskel__open();
    if (!ASSERT_OK_PTR(fexit_skel, "fexit_skel_load"))
    goto close_prog;
    fexit_skel.keyring_id	= KEY_SPEC_SESSION_KEYRING;
    err = fexit_test_lskel__load(fexit_skel);
    if (!ASSERT_OK(err, "fexit_skel_load"))
    goto close_prog;
    err = fentry_test_lskel__attach(fentry_skel);
    if (!ASSERT_OK(err, "fentry_attach"))
    goto close_prog;
    err = fexit_test_lskel__attach(fexit_skel);
    if (!ASSERT_OK(err, "fexit_attach"))
    goto close_prog;
    prog_fd = fexit_skel.progs.test1.prog_fd;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "ipv6 test_run");
    ASSERT_OK(topts.retval, "ipv6 test retval");
    fentry_res = (__u64 *)fentry_skel.bss;
    fexit_res = (__u64 *)fexit_skel.bss;
    printf("%lld\n", fentry_skel.bss.test1_result);
    for (i = 0; i < 8; i++) {
    ASSERT_EQ(fentry_res[i], 1, "fentry result");
    ASSERT_EQ(fexit_res[i], 1, "fexit result");
    }
    close_prog:
    fentry_test_lskel__destroy(fentry_skel);
    fexit_test_lskel__destroy(fexit_skel);
    }
