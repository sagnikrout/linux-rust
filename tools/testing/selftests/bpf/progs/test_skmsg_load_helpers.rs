//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_skmsg_load_helpers.c
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
// Copyright (c) 2020 Isovalent, Inc.

    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, __u64);
    } sock_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKHASH);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, __u64);
    } sock_hash SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, __u32);
    __type(value, __u64);
    } socket_storage SEC(".maps");
#[no_mangle]
unsafe extern "C" fn prog_msg_verdict_common(msg: *mut sk_msg_md) -> c_int {
    static int prog_msg_verdict_common(struct sk_msg_md *msg)
    {
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    let mut verdict: c_int = SK_PASS;
    __u32 pid, tpid;
    __u64 *sk_stg;
    pid = bpf_get_current_pid_tgid() >> 32;
    sk_stg = bpf_sk_storage_get(&socket_storage, msg.sk, 0, BPF_SK_STORAGE_GET_F_CREATE);
    if (!sk_stg)
    return SK_DROP;
// sk_stg = pid;
    bpf_probe_read_kernel(&tpid , sizeof(tpid), &task.tgid);
    if (pid != tpid)
    verdict = SK_DROP;
    bpf_sk_storage_delete(&socket_storage, (void *)msg.sk);
    return verdict;
    }
    SEC("sk_msg")
#[no_mangle]
pub unsafe extern "C" fn prog_msg_verdict(msg: *mut sk_msg_md) -> c_int {
    int prog_msg_verdict(struct sk_msg_md *msg)
    {
    return prog_msg_verdict_common(msg);
    }
    SEC("sk_msg")
#[no_mangle]
pub unsafe extern "C" fn prog_msg_verdict_clone(msg: *mut sk_msg_md) -> c_int {
    int prog_msg_verdict_clone(struct sk_msg_md *msg)
    {
    return prog_msg_verdict_common(msg);
    }
    SEC("sk_msg")
#[no_mangle]
pub unsafe extern "C" fn prog_msg_verdict_clone2(msg: *mut sk_msg_md) -> c_int {
    int prog_msg_verdict_clone2(struct sk_msg_md *msg)
    {
    return prog_msg_verdict_common(msg);
    }
    SEC("sk_skb/stream_verdict")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> c_int {
    int prog_skb_verdict(struct __sk_buff *skb)
    {
    return SK_PASS;
    }
    char _license[] SEC("license") = "GPL";
