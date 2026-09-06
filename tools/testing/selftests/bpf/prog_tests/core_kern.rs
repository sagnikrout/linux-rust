//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/core_kern.c
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
pub unsafe extern "C" fn test_core_kern_lskel() {
    void test_core_kern_lskel(void)
    {
    struct core_kern_lskel *skel;
    int link_fd;
    skel = core_kern_lskel__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_and_load"))
    return;
    link_fd = core_kern_lskel__core_relo_proto__attach(skel);
    if (!ASSERT_GT(link_fd, 0, "attach(core_relo_proto)"))
    goto cleanup;
// trigger tracepoints
    usleep(1);
    ASSERT_TRUE(skel.bss.proto_out[0], "bpf_core_type_exists");
    ASSERT_FALSE(skel.bss.proto_out[1], "!bpf_core_type_exists");
    ASSERT_TRUE(skel.bss.proto_out[2], "bpf_core_type_exists. nested");
    cleanup:
    core_kern_lskel__destroy(skel);
    }
