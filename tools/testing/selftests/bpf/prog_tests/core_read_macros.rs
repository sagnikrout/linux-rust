//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/core_read_macros.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_head {
    pub next: *mut callback_head,
    pub func: Option<unsafe extern "C" fn()>,
}

// ___shuffled flavor is just an illusion for BPF code, it doesn't really
// exist and user-space needs to provide data in the memory layout that
// matches callback_head. We just defined ___shuffled flavor to make it easier
// to work with the skeleton
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_head___shuffled {
    pub next: *mut callback_head___shuffled,
    pub func: Option<unsafe extern "C" fn()>,
}

#[no_mangle]
pub unsafe extern "C" fn test_core_read_macros() {
    void test_core_read_macros(void)
    {
    let mut duration: c_int = 0, err;
    struct test_core_read_macros* skel;
    struct test_core_read_macros__bss *bss;
    struct callback_head u_probe_in;
    struct callback_head___shuffled u_core_in;
    skel = test_core_read_macros__open_and_load();
    if (CHECK(!skel, "skel_open", "failed to open skeleton\n"))
    return;
    bss = skel.bss;
    bss.my_pid = getpid();
// next pointers have to be set from the kernel side
    bss.k_probe_in.func = (void *)(long)0x1234;
    bss.k_core_in.func = (void *)(long)0xabcd;
    u_probe_in.next = &u_probe_in;
    u_probe_in.func = (void *)(long)0x5678;
    bss.u_probe_in = &u_probe_in;
    u_core_in.next = &u_core_in;
    u_core_in.func = (void *)(long)0xdbca;
    bss.u_core_in = &u_core_in;
    err = test_core_read_macros__attach(skel);
    if (CHECK(err, "skel_attach", "skeleton attach failed: %d\n", err))
    goto cleanup;
// trigger tracepoint
    usleep(1);
    ASSERT_EQ(bss.k_probe_out, 0x1234, "k_probe_out");
    ASSERT_EQ(bss.k_core_out, 0xabcd, "k_core_out");
    ASSERT_EQ(bss.u_probe_out, 0x5678, "u_probe_out");
    ASSERT_EQ(bss.u_core_out, 0xdbca, "u_core_out");
    cleanup:
    test_core_read_macros__destroy(skel);
    }
