//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/deny_namespace.c
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
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn wait_for_pid(pid: pid_t) -> c_int {
    static int wait_for_pid(pid_t pid)
    {
    int status, ret;
    again:
    ret = waitpid(pid, &status, 0);
    if (ret == -1) {
    if (errno == EINTR)
    goto again;
    return -1;
    }
    if (!WIFEXITED(status))
    return -1;
    return WEXITSTATUS(status);
    }
// negative return value -> some internal error
// positive return value -> userns creation failed
// 0                     -> userns creation succeeded
//
#[no_mangle]
unsafe extern "C" fn create_user_ns() -> c_int {
    static int create_user_ns(void)
    {
    pid_t pid;
    pid = fork();
    if (pid < 0)
    return -1;
    if (pid == 0) {
    if (unshare(CLONE_NEWUSER))
    _exit(EXIT_FAILURE);
    _exit(EXIT_SUCCESS);
    }
    return wait_for_pid(pid);
    }
#[no_mangle]
unsafe extern "C" fn test_userns_create_bpf() {
    static void test_userns_create_bpf(void)
    {
    let mut cap_mask: __u32 = 1ULL << CAP_SYS_ADMIN;
    let mut old_caps: __u64 = 0;
    cap_enable_effective(cap_mask, &old_caps);
    ASSERT_OK(create_user_ns(), "priv new user ns");
    cap_disable_effective(cap_mask, &old_caps);
    ASSERT_EQ(create_user_ns(), EPERM, "unpriv new user ns");
    if (cap_mask & old_caps)
    cap_enable_effective(cap_mask, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn test_unpriv_userns_create_no_bpf() {
    static void test_unpriv_userns_create_no_bpf(void)
    {
    let mut cap_mask: __u32 = 1ULL << CAP_SYS_ADMIN;
    let mut old_caps: __u64 = 0;
    cap_disable_effective(cap_mask, &old_caps);
    ASSERT_OK(create_user_ns(), "no-bpf unpriv new user ns");
    if (cap_mask & old_caps)
    cap_enable_effective(cap_mask, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn test_deny_namespace() {
    void test_deny_namespace(void)
    {
    struct test_deny_namespace *skel = core::ptr::null_mut();
    int err;
    if (test__start_subtest("unpriv_userns_create_no_bpf"))
    test_unpriv_userns_create_no_bpf();
    skel = test_deny_namespace__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel load"))
    goto close_prog;
    err = test_deny_namespace__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    goto close_prog;
    if (test__start_subtest("userns_create_bpf"))
    test_userns_create_bpf();
    test_deny_namespace__detach(skel);
    close_prog:
    test_deny_namespace__destroy(skel);
    }
