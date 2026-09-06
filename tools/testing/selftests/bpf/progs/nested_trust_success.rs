//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/nested_trust_success.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, u64);
    } sk_storage_map SEC(".maps");
    SEC("tp_btf/task_newtask")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_read_cpumask, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_read_cpumask, struct task_struct *task, u64 clone_flags)
    {
    bpf_cpumask_test_cpu(0, task.cpus_ptr);
    return 0;
    }
    SEC("tp_btf/tcp_probe")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_skb_field, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    int BPF_PROG(test_skb_field, struct sock *sk, struct sk_buff *skb)
    {
    bpf_sk_storage_get(&sk_storage_map, skb.sk, 0, 0);
    return 0;
    }
    SEC("tp_btf/task_newtask")
    __success
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_nested_offset, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(test_nested_offset, struct task_struct *task, u64 clone_flags)
    {
    bpf_cpumask_first_zero(&task.cpus_mask);
    return 0;
    }
