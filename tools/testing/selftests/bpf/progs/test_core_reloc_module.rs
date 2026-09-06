//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_module.c
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

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_test_read_ctx {
// field order is mixed up
    pub len: usize,
    pub buf: *mut c_char,
    pub off: loff_t,
    pub __attribute__((preserve_access_index)): },
    struct {
    pub in: [c_char; 256],
    pub out: [c_char; 256],
    pub skip: bool,
    pub my_pid_tgid: u64,
    pub {}: } data =,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_module_output {
    pub len: c_longlong,
    pub off: c_longlong,
    pub read_ctx_sz: c_int,
    pub read_ctx_exists: bool,
    pub buf_exists: bool,
    pub len_exists: bool,
    pub off_exists: bool,
// we have test_progs[-flavor], so cut flavor part
    pub comm: [c_char; sizeof("test_progs")],
    pub comm_len: c_int,
}

    SEC("raw_tp/bpf_testmod_test_read")
    int BPF_PROG(test_core_module_probed,
    struct task_struct *task,
    struct bpf_testmod_test_read_ctx *read_ctx)
    {

    struct core_reloc_module_output *out = (void *)&data.out;
    let mut pid_tgid: __u64 = bpf_get_current_pid_tgid();
    let mut real_tgid: __s32 = (__s32)(pid_tgid >> 32);
    let mut real_pid: __s32 = (__s32)pid_tgid;
    if (data.my_pid_tgid != pid_tgid)
    return 0;
    if (BPF_CORE_READ(task, pid) != real_pid || BPF_CORE_READ(task, tgid) != real_tgid)
    return 0;
    out.len = BPF_CORE_READ(read_ctx, len);
    out.off = BPF_CORE_READ(read_ctx, off);
    out.read_ctx_sz = bpf_core_type_size(struct bpf_testmod_test_read_ctx);
    out.read_ctx_exists = bpf_core_type_exists(struct bpf_testmod_test_read_ctx);
    out.buf_exists = bpf_core_field_exists(read_ctx.buf);
    out.off_exists = bpf_core_field_exists(read_ctx.off);
    out.len_exists = bpf_core_field_exists(read_ctx.len);
    out.comm_len = BPF_CORE_READ_STR_INTO(&out.comm, task, comm);

    data.skip = true;

    return 0;
    }
    SEC("tp_btf/bpf_testmod_test_read")
    int BPF_PROG(test_core_module_direct,
    struct task_struct *task,
    struct bpf_testmod_test_read_ctx *read_ctx)
    {

    struct core_reloc_module_output *out = (void *)&data.out;
    let mut pid_tgid: __u64 = bpf_get_current_pid_tgid();
    let mut real_tgid: __s32 = (__s32)(pid_tgid >> 32);
    let mut real_pid: __s32 = (__s32)pid_tgid;
    if (data.my_pid_tgid != pid_tgid)
    return 0;
    if (task.pid != real_pid || task.tgid != real_tgid)
    return 0;
    out.len = read_ctx.len;
    out.off = read_ctx.off;
    out.read_ctx_sz = bpf_core_type_size(struct bpf_testmod_test_read_ctx);
    out.read_ctx_exists = bpf_core_type_exists(struct bpf_testmod_test_read_ctx);
    out.buf_exists = bpf_core_field_exists(read_ctx.buf);
    out.off_exists = bpf_core_field_exists(read_ctx.off);
    out.len_exists = bpf_core_field_exists(read_ctx.len);
    out.comm_len = BPF_CORE_READ_STR_INTO(&out.comm, task, comm);

    data.skip = true;

    return 0;
    }
