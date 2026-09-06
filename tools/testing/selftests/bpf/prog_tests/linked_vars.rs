//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/linked_vars.c
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
// Copyright (c) 2021 Facebook

#[no_mangle]
pub unsafe extern "C" fn test_linked_vars() {
    void test_linked_vars(void)
    {
    int err;
    struct linked_vars *skel;
    skel = linked_vars__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    skel.bss.input_bss1 = 1000;
    skel.bss.input_bss2 = 2000;
    skel.bss.input_bss_weak = 3000;
    err = linked_vars__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto cleanup;
    err = linked_vars__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto cleanup;
// trigger
    syscall(SYS_getpgid);
    ASSERT_EQ(skel.bss.output_bss1, 1000 + 2000 + 3000, "output_bss1");
    ASSERT_EQ(skel.bss.output_bss2, 1000 + 2000 + 3000, "output_bss2");
// 10 comes from "winner" input_data_weak in first obj file
    ASSERT_EQ(skel.bss.output_data1, 1 + 2 + 10, "output_bss1");
    ASSERT_EQ(skel.bss.output_data2, 1 + 2 + 10, "output_bss2");
// 100 comes from "winner" input_rodata_weak in first obj file
    ASSERT_EQ(skel.bss.output_rodata1, 11 + 22 + 100, "output_weak1");
    ASSERT_EQ(skel.bss.output_rodata2, 11 + 22 + 100, "output_weak2");
    cleanup:
    linked_vars__destroy(skel);
    }
