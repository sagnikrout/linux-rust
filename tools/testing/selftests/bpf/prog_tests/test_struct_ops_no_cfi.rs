//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_struct_ops_no_cfi.c
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
unsafe extern "C" fn load_bpf_test_no_cfi() {
    static void load_bpf_test_no_cfi(void)
    {
    int fd;
    int err;
    fd = open("bpf_test_no_cfi.ko", O_RDONLY);
    if (!ASSERT_GE(fd, 0, "open"))
    return;
// The module will try to register a struct_ops type without
// cfi_stubs and with cfi_stubs.
//
// The one without cfi_stub should fail. The module will be loaded
// successfully only if the result of the registration is as
// expected, or it fails.
//
    err = finit_module(fd, "", 0);
    close(fd);
    if (!ASSERT_OK(err, "finit_module"))
    return;
    err = delete_module("bpf_test_no_cfi", 0);
    ASSERT_OK(err, "delete_module");
    }
#[no_mangle]
pub unsafe extern "C" fn test_struct_ops_no_cfi() {
    void test_struct_ops_no_cfi(void)
    {
    if (test__start_subtest("load_bpf_test_no_cfi"))
    load_bpf_test_no_cfi();
    }
