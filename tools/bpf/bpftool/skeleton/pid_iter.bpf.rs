//! Automatically rewritten from C to Rust
//! Source: tools/bpf/bpftool/skeleton/pid_iter.bpf.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2020 Facebook

// keep in sync with the definition in main.h
    enum bpf_obj_type {
    BPF_OBJ_UNKNOWN,
    BPF_OBJ_PROG,
    BPF_OBJ_MAP,
    BPF_OBJ_LINK,
    BPF_OBJ_BTF,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_perf_link___local {
    pub link: bpf_link,
    pub perf_file: *mut file,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event___local {
    pub bpf_cookie: u64,
    pub __attribute__((preserve_access_index)): },
    enum bpf_link_type___local {
    BPF_LINK_TYPE_PERF_EVENT___local = 7,
}

    extern const void bpf_link_fops __ksym;
    extern const void bpf_link_fops_poll __ksym __weak;
    extern const void bpf_map_fops __ksym;
    extern const void bpf_prog_fops __ksym;
    extern const void btf_fops __ksym;
    let mut obj_type: volatile enum bpf_obj_type = BPF_OBJ_UNKNOWN;
#[no_mangle]
unsafe extern "C" fn get_obj_id(ent: *mut c_void, type: enum bpf_obj_type) -> __always_inline __u32 {
    static __always_inline __u32 get_obj_id(void *ent, enum bpf_obj_type type)
    {
    switch (type) {
    case BPF_OBJ_PROG:
    return BPF_CORE_READ((struct bpf_prog *)ent, aux, id);
    case BPF_OBJ_MAP:
    return BPF_CORE_READ((struct bpf_map *)ent, id);
    case BPF_OBJ_BTF:
    return BPF_CORE_READ((struct btf *)ent, id);
    case BPF_OBJ_LINK:
    return BPF_CORE_READ((struct bpf_link *)ent, id);
    default:
    return 0;
    }
    }
// could be used only with BPF_LINK_TYPE_PERF_EVENT links
#[no_mangle]
unsafe extern "C" fn get_bpf_cookie(link: *mut bpf_link) -> __u64 {
    static __u64 get_bpf_cookie(struct bpf_link *link)
    {
    struct bpf_perf_link___local *perf_link;
    struct perf_event___local *event;
    perf_link = container_of(link, struct bpf_perf_link___local, link);
    event = BPF_CORE_READ(perf_link, perf_file, private_data);
    return BPF_CORE_READ(event, bpf_cookie);
    }
    SEC("iter/task_file")
#[no_mangle]
pub unsafe extern "C" fn iter(ctx: *mut bpf_iter__task_file) -> c_int {
    int iter(struct bpf_iter__task_file *ctx)
    {
    struct file *file = ctx.file;
    struct task_struct *task = ctx.task;
    struct pid_iter_entry e;
    const void *fops;
    if (!file || !task)
    return 0;
    switch (obj_type) {
    case BPF_OBJ_PROG:
    fops = &bpf_prog_fops;
    break;
    case BPF_OBJ_MAP:
    fops = &bpf_map_fops;
    break;
    case BPF_OBJ_BTF:
    fops = &btf_fops;
    break;
    case BPF_OBJ_LINK:
    if (&bpf_link_fops_poll &&
    file.f_op == &bpf_link_fops_poll)
    fops = &bpf_link_fops_poll;
    else
    fops = &bpf_link_fops;
    break;
    default:
    return 0;
    }
    if (file.f_op != fops)
    return 0;
    __builtin_memset(&e, 0, sizeof(e));
    e.pid = task.tgid;
    e.id = get_obj_id(file.private_data, obj_type);
    if (obj_type == BPF_OBJ_LINK &&
    bpf_core_enum_value_exists(enum bpf_link_type___local,
    BPF_LINK_TYPE_PERF_EVENT___local)) {
    struct bpf_link *link = (struct bpf_link *) file.private_data;
    if (BPF_CORE_READ(link, type) == bpf_core_enum_value(enum bpf_link_type___local,
    BPF_LINK_TYPE_PERF_EVENT___local)) {
    e.has_bpf_cookie = true;
    e.bpf_cookie = get_bpf_cookie(link);
    }
    }
    bpf_probe_read_kernel_str(&e.comm, sizeof(e.comm),
    task.group_leader.comm);
    bpf_seq_write(ctx.meta.seq, &e, sizeof(e));
    return 0;
    }
    char LICENSE[] SEC("license") = "Dual BSD/GPL";
