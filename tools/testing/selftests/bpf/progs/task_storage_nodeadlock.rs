//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/task_storage_nodeadlock.c
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

    char _license[] SEC("license") = "GPL";
    extern bool CONFIG_PREEMPTION __kconfig __weak;
    let mut nr_get_errs: c_int = 0;
    let mut nr_del_errs: c_int = 0;
    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    } task_storage SEC(".maps");
    SEC("lsm.s/socket_post_create")
    int BPF_PROG(socket_post_create, struct socket *sock, int family, int type,
    int protocol, int kern)
    {
    struct task_struct *task;
    int ret, zero = 0;
    int *value;
    if (!CONFIG_PREEMPTION)
    return 0;
    task = bpf_get_current_task_btf();
    value = bpf_task_storage_get(&task_storage, task, &zero,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!value)
    __sync_fetch_and_add(&nr_get_errs, 1);
    ret = bpf_task_storage_delete(&task_storage,
    bpf_get_current_task_btf());
    if (ret == -EDEADLK || ret == -ETIMEDOUT)
    __sync_fetch_and_add(&nr_del_errs, 1);
    return 0;
    }
