//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/subprogs.c
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
pub struct toggler_ctx {
    pub fd: c_int,
    pub stop: bool,
}

    static void *toggle_jit_harden(void *arg)
    {
    struct toggler_ctx *ctx = arg;
    let mut two: c_char = '2';
    let mut zero: c_char = '0';
    while (!ctx.stop) {
    lseek(ctx.fd, SEEK_SET, 0);
    write(ctx.fd, &two, sizeof(two));
    lseek(ctx.fd, SEEK_SET, 0);
    write(ctx.fd, &zero, sizeof(zero));
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn test_subprogs_with_jit_harden_toggling() {
    static void test_subprogs_with_jit_harden_toggling(void)
    {
    struct toggler_ctx ctx;
    pthread_t toggler;
    int err;
    unsigned int i, loop = 10;
    ctx.fd = open("/proc/sys/net/core/bpf_jit_harden", O_RDWR);
    if (!ASSERT_GE(ctx.fd, 0, "open bpf_jit_harden"))
    return;
    ctx.stop = false;
    err = pthread_create(&toggler, core::ptr::null_mut(), toggle_jit_harden, &ctx);
    if (!ASSERT_OK(err, "new toggler"))
    goto out;
// Make toggler thread to run
    usleep(1);
    for (i = 0; i < loop; i++) {
    struct test_subprogs *skel = test_subprogs__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel open"))
    break;
    test_subprogs__destroy(skel);
    }
    ctx.stop = true;
    pthread_join(toggler, core::ptr::null_mut());
    out:
    close(ctx.fd);
    }
#[no_mangle]
unsafe extern "C" fn test_subprogs_alone() {
    static void test_subprogs_alone(void)
    {
    struct test_subprogs *skel;
    struct test_subprogs_unused *skel2;
    int err;
    skel = test_subprogs__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    err = test_subprogs__attach(skel);
    if (!ASSERT_OK(err, "skel attach"))
    goto cleanup;
    usleep(1);
    ASSERT_EQ(skel.bss.res1, 12, "res1");
    ASSERT_EQ(skel.bss.res2, 17, "res2");
    ASSERT_EQ(skel.bss.res3, 19, "res3");
    ASSERT_EQ(skel.bss.res4, 36, "res4");
    skel2 = test_subprogs_unused__open_and_load();
    ASSERT_OK_PTR(skel2, "unused_progs_skel");
    test_subprogs_unused__destroy(skel2);
    cleanup:
    test_subprogs__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_subprogs() {
    void test_subprogs(void)
    {
    if (test__start_subtest("subprogs_alone"))
    test_subprogs_alone();
    if (test__start_subtest("subprogs_and_jit_harden"))
    test_subprogs_with_jit_harden_toggling();
    }
