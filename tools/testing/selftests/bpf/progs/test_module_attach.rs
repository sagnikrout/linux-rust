//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_module_attach.c
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

    let mut sz: __u32 = 0;
    SEC("?raw_tp/bpf_testmod_test_read")
    int BPF_PROG(handle_raw_tp,
    struct task_struct *task, struct bpf_testmod_test_read_ctx *read_ctx)
    {
    sz = BPF_CORE_READ(read_ctx, len);
    return 0;
    }
    SEC("?raw_tp/bpf_testmod_test_write_bare_tp")
    int BPF_PROG(handle_raw_tp_bare,
    struct task_struct *task, struct bpf_testmod_test_write_ctx *write_ctx)
    {
    sz = BPF_CORE_READ(write_ctx, len);
    return 0;
    }
    let mut raw_tp_writable_bare_in_val: c_int = 0;
    let mut raw_tp_writable_bare_early_ret: c_int = 0;
    let mut raw_tp_writable_bare_out_val: c_int = 0;
    SEC("?raw_tp.w/bpf_testmod_test_writable_bare_tp")
    int BPF_PROG(handle_raw_tp_writable_bare,
    struct bpf_testmod_test_writable_ctx *writable)
    {
    raw_tp_writable_bare_in_val = writable.val;
    writable.early_ret = raw_tp_writable_bare_early_ret;
    writable.val = raw_tp_writable_bare_out_val;
    return 0;
    }
    SEC("?tp_btf/bpf_testmod_test_read")
    int BPF_PROG(handle_tp_btf,
    struct task_struct *task, struct bpf_testmod_test_read_ctx *read_ctx)
    {
    sz = read_ctx.len;
    return 0;
    }
    SEC("?fentry/bpf_testmod_test_read")
    int BPF_PROG(handle_fentry,
    struct file *file, struct kobject *kobj,
    struct bin_attribute *bin_attr, char *buf, loff_t off, size_t len)
    {
    sz = len;
    return 0;
    }
    SEC("?fentry")
    int BPF_PROG(handle_fentry_manual,
    struct file *file, struct kobject *kobj,
    struct bin_attribute *bin_attr, char *buf, loff_t off, size_t len)
    {
    sz = len;
    return 0;
    }
    SEC("?fentry/bpf_testmod:bpf_testmod_test_read")
    int BPF_PROG(handle_fentry_explicit,
    struct file *file, struct kobject *kobj,
    struct bin_attribute *bin_attr, char *buf, loff_t off, size_t len)
    {
    sz = len;
    return 0;
    }
    SEC("?fentry")
    int BPF_PROG(handle_fentry_explicit_manual,
    struct file *file, struct kobject *kobj,
    struct bin_attribute *bin_attr, char *buf, loff_t off, size_t len)
    {
    sz = len;
    return 0;
    }
    let mut retval: c_int = 0;
    SEC("?fexit/bpf_testmod_test_read")
    int BPF_PROG(handle_fexit,
    struct file *file, struct kobject *kobj,
    struct bin_attribute *bin_attr, char *buf, loff_t off, size_t len,
    int ret)
    {
    sz = len;
    retval = ret;
    return 0;
    }
    SEC("?fexit/bpf_testmod_return_ptr")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handle_fexit_ret, arg: c_int, ret: *mut file) -> c_int {
    int BPF_PROG(handle_fexit_ret, int arg, struct file *ret)
    {
    let mut buf: c_long = 0;
    bpf_probe_read_kernel(&buf, 8, ret);
    bpf_probe_read_kernel(&buf, 8, (char *)ret + 256);
// (volatile int *)ret;
// (volatile int *)&ret->f_mode;
    return 0;
    }
    SEC("?fmod_ret/bpf_testmod_test_read")
    int BPF_PROG(handle_fmod_ret,
    struct file *file, struct kobject *kobj,
    struct bin_attribute *bin_attr, char *buf, loff_t off, size_t len)
    {
    sz = len;
    return 0; /* don't override the exit code */
    }
    SEC("?kprobe.multi/bpf_testmod_test_read")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: kprobe_multi) -> c_int {
    int BPF_PROG(kprobe_multi)
    {
    return 0;
    }
    char _license[] SEC("license") = "GPL";
