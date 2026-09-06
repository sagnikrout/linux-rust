//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/file_reader_fail.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    int err;
    void *user_ptr;
    SEC("lsm/file_open")
    __failure
    __msg("Unreleased reference id=")
#[no_mangle]
pub unsafe extern "C" fn on_nanosleep_unreleased_ref(ctx: *mut c_void) -> c_int {
    int on_nanosleep_unreleased_ref(void *ctx)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    struct file *file = bpf_get_task_exe_file(task);
    struct bpf_dynptr dynptr;
    if (!file)
    return 0;
    err = bpf_dynptr_from_file(file, 0, &dynptr);
    return err ? 1 : 0;
    }
    SEC("xdp")
    __failure
    __msg("Expected a dynptr of type file as R1")
#[no_mangle]
pub unsafe extern "C" fn xdp_wrong_dynptr_type(xdp: *mut xdp_md) -> c_int {
    int xdp_wrong_dynptr_type(struct xdp_md *xdp)
    {
    struct bpf_dynptr dynptr;
    bpf_dynptr_from_xdp(xdp, 0, &dynptr);
    bpf_dynptr_file_discard(&dynptr);
    return 0;
    }
    SEC("xdp")
    __failure
    __msg("Expected an initialized dynptr as R1")
#[no_mangle]
pub unsafe extern "C" fn xdp_no_dynptr_type(xdp: *mut xdp_md) -> c_int {
    int xdp_no_dynptr_type(struct xdp_md *xdp)
    {
    struct bpf_dynptr dynptr;
    bpf_dynptr_file_discard(&dynptr);
    return 0;
    }
    SEC("lsm/file_open")
    __failure
    __msg("Leaking reference id={{[0-9]+}} alloc_insn={{[0-9]+}}. Release it first.")
#[no_mangle]
pub unsafe extern "C" fn use_file_dynptr_after_put_file(ctx: *mut c_void) -> c_int {
    int use_file_dynptr_after_put_file(void *ctx)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    struct file *file = bpf_get_task_exe_file(task);
    struct bpf_dynptr dynptr;
    char buf[64];
    if (!file)
    return 0;
    if (bpf_dynptr_from_file(file, 0, &dynptr))
    goto out;
// this should fail - file dynptr should be discarded first to prevent resource leak
    bpf_put_file(file);
    bpf_dynptr_read(buf, sizeof(buf), &dynptr, 0, 0);
    return 0;
    out:
    bpf_dynptr_file_discard(&dynptr);
    bpf_put_file(file);
    return 0;
    }
    SEC("lsm/file_open")
    __failure
    __msg("Leaking reference id={{[0-9]+}} alloc_insn={{[0-9]+}}. Release it first.")
#[no_mangle]
pub unsafe extern "C" fn use_file_dynptr_slice_after_put_file(ctx: *mut c_void) -> c_int {
    int use_file_dynptr_slice_after_put_file(void *ctx)
    {
    struct task_struct *task = bpf_get_current_task_btf();
    struct file *file = bpf_get_task_exe_file(task);
    struct bpf_dynptr dynptr;
    char buf[1];
    const char *data;
    if (!file)
    return 0;
    if (bpf_dynptr_from_file(file, 0, &dynptr))
    goto out;
    data = bpf_dynptr_slice(&dynptr, 0, buf, sizeof(buf));
    if (!data)
    goto out;
// this should fail - file dynptr should be discarded first to prevent resource leak
    bpf_put_file(file);
    return data[0];
    out:
    bpf_dynptr_file_discard(&dynptr);
    bpf_put_file(file);
    return 0;
    }
