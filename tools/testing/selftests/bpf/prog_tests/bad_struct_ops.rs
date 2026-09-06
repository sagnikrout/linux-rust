//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/bad_struct_ops.c
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

#[no_mangle]
unsafe extern "C" fn invalid_prog_reuse() {
    static void invalid_prog_reuse(void)
    {
    struct bad_struct_ops *skel;
    char *log = core::ptr::null_mut();
    int err;
    skel = bad_struct_ops__open();
    if (!ASSERT_OK_PTR(skel, "bad_struct_ops__open"))
    return;
    if (start_libbpf_log_capture())
    goto cleanup;
    err = bad_struct_ops__load(skel);
    log = stop_libbpf_log_capture();
    ASSERT_ERR(err, "bad_struct_ops__load should fail");
    ASSERT_HAS_SUBSTR(log,
    "struct_ops init_kern testmod_2 func ptr test_1: invalid reuse of prog test_1",
    "expected init_kern message");
    cleanup:
    free(log);
    bad_struct_ops__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn unused_program() {
    static void unused_program(void)
    {
    struct bad_struct_ops2 *skel;
    char *log = core::ptr::null_mut();
    int err;
    skel = bad_struct_ops2__open();
    if (!ASSERT_OK_PTR(skel, "bad_struct_ops2__open"))
    return;
// struct_ops programs not referenced from any maps are open
// with autoload set to true.
//
    ASSERT_TRUE(bpf_program__autoload(skel.progs.foo), "foo autoload == true");
    if (start_libbpf_log_capture())
    goto cleanup;
    err = bad_struct_ops2__load(skel);
    ASSERT_ERR(err, "bad_struct_ops2__load should fail");
    log = stop_libbpf_log_capture();
    ASSERT_HAS_SUBSTR(log, "prog 'foo': failed to load",
    "message about 'foo' failing to load");
    cleanup:
    free(log);
    bad_struct_ops2__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_bad_struct_ops() {
    void test_bad_struct_ops(void)
    {
    if (test__start_subtest("invalid_prog_reuse"))
    invalid_prog_reuse();
    if (test__start_subtest("unused_program"))
    unused_program();
    }
