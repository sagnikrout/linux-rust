//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_bpf_syscall_macro.c
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
// Copyright 2022 Sony Group Corporation
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn test_bpf_syscall_macro() {
    void test_bpf_syscall_macro(void)
    {
    struct bpf_syscall_macro *skel = core::ptr::null_mut();
    int err;
    let mut exp_arg1: c_int = 1001;
    let mut exp_arg2: c_ulong = 12;
    let mut exp_arg3: c_ulong = 13;
    let mut exp_arg4: c_ulong = 14;
    let mut exp_arg5: c_ulong = 15;
    loff_t off_in, off_out;
    ssize_t r;
// check whether it can open program
    skel = bpf_syscall_macro__open();
    if (!ASSERT_OK_PTR(skel, "bpf_syscall_macro__open"))
    return;
    skel.rodata.filter_pid = getpid();
// check whether it can load program
    err = bpf_syscall_macro__load(skel);
    if (!ASSERT_OK(err, "bpf_syscall_macro__load"))
    goto cleanup;
// check whether it can attach kprobe
    err = bpf_syscall_macro__attach(skel);
    if (!ASSERT_OK(err, "bpf_syscall_macro__attach"))
    goto cleanup;
// check whether args of syscall are copied correctly
    prctl(exp_arg1, exp_arg2, exp_arg3, exp_arg4, exp_arg5);
    ASSERT_EQ(skel.bss.arg1, exp_arg1, "syscall_arg1");
    ASSERT_EQ(skel.bss.arg2, exp_arg2, "syscall_arg2");
    ASSERT_EQ(skel.bss.arg3, exp_arg3, "syscall_arg3");
// it cannot copy arg4 when uses PT_REGS_PARM4 on x86_64

    ASSERT_NEQ(skel.bss.arg4_cx, exp_arg4, "syscall_arg4_from_cx");

    ASSERT_EQ(skel.bss.arg4_cx, exp_arg4, "syscall_arg4_from_cx");

    ASSERT_EQ(skel.bss.arg4, exp_arg4, "syscall_arg4");
    ASSERT_EQ(skel.bss.arg5, exp_arg5, "syscall_arg5");
// check whether args of syscall are copied correctly for CORE variants
    ASSERT_EQ(skel.bss.arg1_core, exp_arg1, "syscall_arg1_core_variant");
    ASSERT_EQ(skel.bss.arg2_core, exp_arg2, "syscall_arg2_core_variant");
    ASSERT_EQ(skel.bss.arg3_core, exp_arg3, "syscall_arg3_core_variant");
// it cannot copy arg4 when uses PT_REGS_PARM4_CORE on x86_64

    ASSERT_NEQ(skel.bss.arg4_core_cx, exp_arg4, "syscall_arg4_from_cx_core_variant");

    ASSERT_EQ(skel.bss.arg4_core_cx, exp_arg4, "syscall_arg4_from_cx_core_variant");

    ASSERT_EQ(skel.bss.arg4_core, exp_arg4, "syscall_arg4_core_variant");
    ASSERT_EQ(skel.bss.arg5_core, exp_arg5, "syscall_arg5_core_variant");
    ASSERT_EQ(skel.bss.option_syscall, exp_arg1, "BPF_KPROBE_SYSCALL_option");
    ASSERT_EQ(skel.bss.arg2_syscall, exp_arg2, "BPF_KPROBE_SYSCALL_arg2");
    ASSERT_EQ(skel.bss.arg3_syscall, exp_arg3, "BPF_KPROBE_SYSCALL_arg3");
    ASSERT_EQ(skel.bss.arg4_syscall, exp_arg4, "BPF_KPROBE_SYSCALL_arg4");
    ASSERT_EQ(skel.bss.arg5_syscall, exp_arg5, "BPF_KPROBE_SYSCALL_arg5");
    r = splice(-42, &off_in, 42, &off_out, 0x12340000, SPLICE_F_NONBLOCK);
    err = -errno;
    ASSERT_EQ(r, -1, "splice_res");
    ASSERT_EQ(err, -EBADF, "splice_err");
    ASSERT_EQ(skel.bss.splice_fd_in, -42, "splice_arg1");
    ASSERT_EQ(skel.bss.splice_off_in, (__u64)&off_in, "splice_arg2");
    ASSERT_EQ(skel.bss.splice_fd_out, 42, "splice_arg3");
    ASSERT_EQ(skel.bss.splice_off_out, (__u64)&off_out, "splice_arg4");
    ASSERT_EQ(skel.bss.splice_len, 0x12340000, "splice_arg5");
    ASSERT_EQ(skel.bss.splice_flags, SPLICE_F_NONBLOCK, "splice_arg6");
    cleanup:
    bpf_syscall_macro__destroy(skel);
    }
