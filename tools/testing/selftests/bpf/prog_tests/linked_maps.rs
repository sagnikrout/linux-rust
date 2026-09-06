//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/linked_maps.c
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
pub unsafe extern "C" fn test_linked_maps() {
    void test_linked_maps(void)
    {
    int err;
    struct linked_maps *skel;
    skel = linked_maps__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    err = linked_maps__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto cleanup;
// trigger
    syscall(SYS_getpgid);
    ASSERT_EQ(skel.bss.output_first1, 2000, "output_first1");
    ASSERT_EQ(skel.bss.output_second1, 2, "output_second1");
    ASSERT_EQ(skel.bss.output_weak1, 2, "output_weak1");
    cleanup:
    linked_maps__destroy(skel);
    }
