//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/prog_array_init.c
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
// Copyright (c) 2021 Hengqi Chen

#[no_mangle]
pub unsafe extern "C" fn test_prog_array_init() {
    void test_prog_array_init(void)
    {
    struct test_prog_array_init *skel;
    int err;
    skel = test_prog_array_init__open();
    if (!ASSERT_OK_PTR(skel, "could not open BPF object"))
    return;
    skel.rodata.my_pid = getpid();
    err = test_prog_array_init__load(skel);
    if (!ASSERT_OK(err, "could not load BPF object"))
    goto cleanup;
    skel.links.entry = bpf_program__attach_raw_tracepoint(skel.progs.entry, "sys_enter");
    if (!ASSERT_OK_PTR(skel.links.entry, "could not attach BPF program"))
    goto cleanup;
    usleep(1);
    ASSERT_EQ(skel.bss.value, 42, "unexpected value");
    cleanup:
    test_prog_array_init__destroy(skel);
    }
