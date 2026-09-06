//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/struct_ops_private_stack.c
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
unsafe extern "C" fn test_private_stack() {
    static void test_private_stack(void)
    {
    struct struct_ops_private_stack *skel;
    struct bpf_link *link;
    int err;
    skel = struct_ops_private_stack__open();
    if (!ASSERT_OK_PTR(skel, "struct_ops_private_stack__open"))
    return;
    err = struct_ops_private_stack__load(skel);
    if (!ASSERT_OK(err, "struct_ops_private_stack__load"))
    goto cleanup;
    link = bpf_map__attach_struct_ops(skel.maps.testmod_1);
    if (!ASSERT_OK_PTR(link, "attach_struct_ops"))
    goto cleanup;
    ASSERT_OK(trigger_module_test_read(256), "trigger_read");
    ASSERT_EQ(skel.bss.val_i, 3, "val_i");
    ASSERT_EQ(skel.bss.val_j, 8, "val_j");
    bpf_link__destroy(link);
    cleanup:
    struct_ops_private_stack__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_private_stack_fail() {
    static void test_private_stack_fail(void)
    {
    struct struct_ops_private_stack_fail *skel;
    int err;
    skel = struct_ops_private_stack_fail__open();
    if (!ASSERT_OK_PTR(skel, "struct_ops_private_stack_fail__open"))
    return;
    err = struct_ops_private_stack_fail__load(skel);
    ASSERT_ERR(err, "struct_ops_private_stack_fail__load");
    struct_ops_private_stack_fail__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_private_stack_recur() {
    static void test_private_stack_recur(void)
    {
    struct struct_ops_private_stack_recur *skel;
    struct bpf_link *link;
    int err;
    skel = struct_ops_private_stack_recur__open();
    if (!ASSERT_OK_PTR(skel, "struct_ops_private_stack_recur__open"))
    return;
    err = struct_ops_private_stack_recur__load(skel);
    if (!ASSERT_OK(err, "struct_ops_private_stack_recur__load"))
    goto cleanup;
    link = bpf_map__attach_struct_ops(skel.maps.testmod_1);
    if (!ASSERT_OK_PTR(link, "attach_struct_ops"))
    goto cleanup;
    ASSERT_OK(trigger_module_test_read(256), "trigger_read");
    ASSERT_EQ(skel.bss.val_j, 3, "val_j");
    bpf_link__destroy(link);
    cleanup:
    struct_ops_private_stack_recur__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn __test_struct_ops_private_stack() {
    static void __test_struct_ops_private_stack(void)
    {
    if (test__start_subtest("private_stack"))
    test_private_stack();
    if (test__start_subtest("private_stack_fail"))
    test_private_stack_fail();
    if (test__start_subtest("private_stack_recur"))
    test_private_stack_recur();
    }

#[no_mangle]
unsafe extern "C" fn __test_struct_ops_private_stack() {
    static void __test_struct_ops_private_stack(void)
    {
    test__skip();
    }

#[no_mangle]
pub unsafe extern "C" fn test_struct_ops_private_stack() {
    void test_struct_ops_private_stack(void)
    {
    __test_struct_ops_private_stack();
    }
