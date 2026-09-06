//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_struct_ops_multi_pages.c
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
unsafe extern "C" fn do_struct_ops_multi_pages() {
    static void do_struct_ops_multi_pages(void)
    {
    struct struct_ops_multi_pages *skel;
    struct bpf_link *link;
// The size of all trampolines of skel->maps.multi_pages should be
// over 1 page (at least for x86).
//
    skel = struct_ops_multi_pages__open_and_load();
    if (!ASSERT_OK_PTR(skel, "struct_ops_multi_pages_open_and_load"))
    return;
    link = bpf_map__attach_struct_ops(skel.maps.multi_pages);
    ASSERT_OK_PTR(link, "attach_multi_pages");
    bpf_link__destroy(link);
    struct_ops_multi_pages__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_struct_ops_multi_pages() {
    void test_struct_ops_multi_pages(void)
    {
    if (test__start_subtest("multi_pages"))
    do_struct_ops_multi_pages();
    }
