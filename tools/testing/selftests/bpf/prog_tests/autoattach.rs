//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/autoattach.c
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
// Copyright (c) 2022 Google

#[no_mangle]
pub unsafe extern "C" fn test_autoattach() {
    void test_autoattach(void)
    {
    struct test_autoattach *skel;
    skel = test_autoattach__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    goto cleanup;
// disable auto-attach for prog2
    bpf_program__set_autoattach(skel.progs.prog2, false);
    ASSERT_TRUE(bpf_program__autoattach(skel.progs.prog1), "autoattach_prog1");
    ASSERT_FALSE(bpf_program__autoattach(skel.progs.prog2), "autoattach_prog2");
    if (!ASSERT_OK(test_autoattach__attach(skel), "skel_attach"))
    goto cleanup;
    usleep(1);
    ASSERT_TRUE(skel.bss.prog1_called, "attached_prog1");
    ASSERT_FALSE(skel.bss.prog2_called, "attached_prog2");
    cleanup:
    test_autoattach__destroy(skel);
    }
