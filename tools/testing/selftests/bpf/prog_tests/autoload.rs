//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/autoload.c
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
// Copyright (c) 2020 Facebook

#[no_mangle]
pub unsafe extern "C" fn test_autoload() {
    void test_autoload(void)
    {
    let mut duration: c_int = 0, err;
    struct test_autoload* skel;
    skel = test_autoload__open_and_load();
// prog3 should be broken
    if (CHECK(skel, "skel_open_and_load", "unexpected success\n"))
    goto cleanup;
    skel = test_autoload__open();
    if (CHECK(!skel, "skel_open", "failed to open skeleton\n"))
    goto cleanup;
// don't load prog3
    bpf_program__set_autoload(skel.progs.prog3, false);
    err = test_autoload__load(skel);
    if (CHECK(err, "skel_load", "failed to load skeleton: %d\n", err))
    goto cleanup;
    err = test_autoload__attach(skel);
    if (CHECK(err, "skel_attach", "skeleton attach failed: %d\n", err))
    goto cleanup;
    usleep(1);
    CHECK(!skel.bss.prog1_called, "prog1", "not called\n");
    CHECK(!skel.bss.prog2_called, "prog2", "not called\n");
    CHECK(skel.bss.prog3_called, "prog3", "called?!\n");
    cleanup:
    test_autoload__destroy(skel);
    }
