//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/uprobe_autoattach.c
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
// Copyright (c) 2022, Oracle and/or its affiliates.

// uprobe attach point
    static noinline int autoattach_trigger_func(int arg1, int arg2, int arg3,
    int arg4, int arg5, int arg6,
    int arg7, int arg8)
    {
    asm volatile ("");
    return arg1 + arg2 + arg3 + arg4 + arg5 + arg6 + arg7 + arg8 + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn test_uprobe_autoattach() {
    void test_uprobe_autoattach(void)
    {
    const char *devnull_str = "/dev/null";
    struct test_uprobe_autoattach *skel;
    int trigger_ret;
    FILE *devnull;
    skel = test_uprobe_autoattach__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    if (!ASSERT_OK(test_uprobe_autoattach__attach(skel), "skel_attach"))
    goto cleanup;
    skel.bss.test_pid = getpid();
// trigger & validate uprobe & uretprobe
    trigger_ret = autoattach_trigger_func(1, 2, 3, 4, 5, 6, 7, 8);
    skel.bss.test_pid = getpid();
// trigger & validate shared library u[ret]probes attached by name
    devnull = fopen(devnull_str, "r");
    ASSERT_EQ(skel.bss.uprobe_byname_parm1, 1, "check_uprobe_byname_parm1");
    ASSERT_EQ(skel.bss.uprobe_byname_ran, 1, "check_uprobe_byname_ran");
    ASSERT_EQ(skel.bss.uretprobe_byname_rc, trigger_ret, "check_uretprobe_byname_rc");
    ASSERT_EQ(skel.bss.uretprobe_byname_ret, trigger_ret, "check_uretprobe_byname_ret");
    ASSERT_EQ(skel.bss.uretprobe_byname_ran, 2, "check_uretprobe_byname_ran");
    ASSERT_EQ(skel.bss.uprobe_byname2_parm1, (__u64)(long)devnull_str,
    "check_uprobe_byname2_parm1");
    ASSERT_EQ(skel.bss.uprobe_byname2_ran, 3, "check_uprobe_byname2_ran");
    ASSERT_EQ(skel.bss.uretprobe_byname2_rc, (__u64)(long)devnull,
    "check_uretprobe_byname2_rc");
    ASSERT_EQ(skel.bss.uretprobe_byname2_ran, 4, "check_uretprobe_byname2_ran");
    ASSERT_EQ(skel.bss.a[0], 1, "arg1");
    ASSERT_EQ(skel.bss.a[1], 2, "arg2");
    ASSERT_EQ(skel.bss.a[2], 3, "arg3");

    ASSERT_EQ(skel.bss.a[3], 4, "arg4");

    ASSERT_EQ(skel.bss.a[4], 5, "arg5");

    ASSERT_EQ(skel.bss.a[5], 6, "arg6");

    ASSERT_EQ(skel.bss.a[6], 7, "arg7");

    ASSERT_EQ(skel.bss.a[7], 8, "arg8");

    fclose(devnull);
    cleanup:
    test_uprobe_autoattach__destroy(skel);
    }
