//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_struct_ops_maybe_null.c
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

// Test that the verifier accepts a program that access a nullable pointer
// with a proper check.
//
#[no_mangle]
unsafe extern "C" fn maybe_null() {
    static void maybe_null(void)
    {
    struct struct_ops_maybe_null *skel;
    skel = struct_ops_maybe_null__open_and_load();
    if (!ASSERT_OK_PTR(skel, "struct_ops_module_open_and_load"))
    return;
    struct_ops_maybe_null__destroy(skel);
    }
// Test that the verifier rejects a program that access a nullable pointer
// without a check beforehand.
//
#[no_mangle]
unsafe extern "C" fn maybe_null_fail() {
    static void maybe_null_fail(void)
    {
    struct struct_ops_maybe_null_fail *skel;
    skel = struct_ops_maybe_null_fail__open_and_load();
    if (ASSERT_ERR_PTR(skel, "struct_ops_module_fail__open_and_load"))
    return;
    struct_ops_maybe_null_fail__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_struct_ops_maybe_null() {
    void test_struct_ops_maybe_null(void)
    {
// The verifier verifies the programs at load time, so testing both
// programs in the same compile-unit is complicated. We run them in
// separate objects to simplify the testing.
//
    if (test__start_subtest("maybe_null"))
    maybe_null();
    if (test__start_subtest("maybe_null_fail"))
    maybe_null_fail();
    }
