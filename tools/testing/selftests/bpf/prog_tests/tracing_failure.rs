//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/tracing_failure.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn test_bpf_spin_lock(is_spin_lock: bool) {
    static void test_bpf_spin_lock(bool is_spin_lock)
    {
    struct tracing_failure *skel;
    int err;
    skel = tracing_failure__open();
    if (!ASSERT_OK_PTR(skel, "tracing_failure__open"))
    return;
    if (is_spin_lock)
    bpf_program__set_autoload(skel.progs.test_spin_lock, true);
    else
    bpf_program__set_autoload(skel.progs.test_spin_unlock, true);
    err = tracing_failure__load(skel);
    if (!ASSERT_OK(err, "tracing_failure__load"))
    goto out;
    err = tracing_failure__attach(skel);
    ASSERT_ERR(err, "tracing_failure__attach");
    out:
    tracing_failure__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_tracing_fail_prog(prog_name: *const c_char, exp_msg: *const c_char) {
    static void test_tracing_fail_prog(const char *prog_name, const char *exp_msg)
    {
    struct tracing_failure *skel;
    struct bpf_program *prog;
    char log_buf[256];
    int err;
    skel = tracing_failure__open();
    if (!ASSERT_OK_PTR(skel, "tracing_failure__open"))
    return;
    prog = bpf_object__find_program_by_name(skel.obj, prog_name);
    if (!ASSERT_OK_PTR(prog, "bpf_object__find_program_by_name"))
    goto out;
    bpf_program__set_autoload(prog, true);
    bpf_program__set_log_buf(prog, log_buf, sizeof(log_buf));
    err = tracing_failure__load(skel);
    if (!ASSERT_ERR(err, "tracing_failure__load"))
    goto out;
    ASSERT_HAS_SUBSTR(log_buf, exp_msg, "log_buf");
    out:
    tracing_failure__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_tracing_deny() {
    static void test_tracing_deny(void)
    {
    int btf_id;
// __rcu_read_lock depends on CONFIG_PREEMPT_RCU
    btf_id = libbpf_find_vmlinux_btf_id("__rcu_read_lock", BPF_TRACE_FENTRY);
    if (btf_id <= 0) {
    test__skip();
    return;
    }
    test_tracing_fail_prog("tracing_deny",
    "Attaching tracing programs to function '__rcu_read_lock' is rejected.");
    }
#[no_mangle]
unsafe extern "C" fn test_fexit_noreturns() {
    static void test_fexit_noreturns(void)
    {
    test_tracing_fail_prog("fexit_noreturns",
    "Attaching fexit/fsession/fmod_ret to __noreturn function 'do_exit' is rejected.");
    }
#[no_mangle]
unsafe extern "C" fn test_fexit_int128_ret() {
    static void test_fexit_int128_ret(void)
    {
//
// __int128 is returned in a register pair on x86_64 and arm64, so
// bpf_testmod_test_int128_ret() is BTF-encoded and attachable and the
// verifier can reject its >8 byte return value. Other architectures
// return a __int128 differently (e.g. s390x returns larger values by
// reference, which makes pahole skip BTF encoding of the function), so
// only exercise this on x86_64 and arm64.
//

    test_tracing_fail_prog("fexit_int128_ret",
    "with a >8 byte return value is not supported for this attach type");

    test__skip();

    }
#[no_mangle]
pub unsafe extern "C" fn test_tracing_failure() {
    void test_tracing_failure(void)
    {
    if (test__start_subtest("bpf_spin_lock"))
    test_bpf_spin_lock(true);
    if (test__start_subtest("bpf_spin_unlock"))
    test_bpf_spin_lock(false);
    if (test__start_subtest("tracing_deny"))
    test_tracing_deny();
    if (test__start_subtest("fexit_noreturns"))
    test_fexit_noreturns();
    if (test__start_subtest("fexit_int128_ret"))
    test_fexit_int128_ret();
    }
