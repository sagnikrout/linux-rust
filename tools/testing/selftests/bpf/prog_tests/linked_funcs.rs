//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/linked_funcs.c
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
pub unsafe extern "C" fn test_linked_funcs() {
    void test_linked_funcs(void)
    {
    int err;
    struct linked_funcs *skel;
    skel = linked_funcs__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
// handler1 and handler2 are marked as SEC("?raw_tp/sys_enter") and
// are set to not autoload by default
//
    bpf_program__set_autoload(skel.progs.handler1, true);
    bpf_program__set_autoload(skel.progs.handler2, true);
    skel.rodata.my_tid = sys_gettid();
    skel.bss.syscall_id = SYS_getpgid;
    err = linked_funcs__load(skel);
    if (!ASSERT_OK(err, "skel_load"))
    goto cleanup;
    err = linked_funcs__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto cleanup;
// trigger
    syscall(SYS_getpgid);
    ASSERT_EQ(skel.bss.output_val1, 2000 + 2000, "output_val1");
    ASSERT_EQ(skel.bss.output_ctx1, SYS_getpgid, "output_ctx1");
    ASSERT_EQ(skel.bss.output_weak1, 42, "output_weak1");
    ASSERT_EQ(skel.bss.output_val2, 2 * 1000 + 2 * (2 * 1000), "output_val2");
    ASSERT_EQ(skel.bss.output_ctx2, SYS_getpgid, "output_ctx2");
// output_weak2 should never be updated
    ASSERT_EQ(skel.bss.output_weak2, 0, "output_weak2");
    cleanup:
    linked_funcs__destroy(skel);
    }
