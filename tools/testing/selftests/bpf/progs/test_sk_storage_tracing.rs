//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sk_storage_tracing.c
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
pub struct sk_stg {
    pub pid: __u32,
    pub last_notclose_state: __u32,
    pub comm: [c_char; 16],
}

    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct sk_stg);
    } sk_stg_map SEC(".maps");
// Testing delete
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    } del_sk_stg_map SEC(".maps");
    char task_comm[16] = "";
    SEC("tp_btf/inet_sock_set_state")
    int BPF_PROG(trace_inet_sock_set_state, struct sock *sk, int oldstate,
    int newstate)
    {
    struct sk_stg *stg;
    if (newstate == BPF_TCP_CLOSE)
    return 0;
    stg = bpf_sk_storage_get(&sk_stg_map, sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!stg)
    return 0;
    stg.last_notclose_state = newstate;
    bpf_sk_storage_delete(&del_sk_stg_map, sk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_task_info(sk: *mut sock) {
    static void set_task_info(struct sock *sk)
    {
    struct task_struct *task;
    struct sk_stg *stg;
    stg = bpf_sk_storage_get(&sk_stg_map, sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!stg)
    return;
    stg.pid = bpf_get_current_pid_tgid();
    task = (struct task_struct *)bpf_get_current_task();
    bpf_core_read_str(&stg.comm, sizeof(stg.comm), &task.comm);
    bpf_core_read_str(&task_comm, sizeof(task_comm), &task.comm);
    }
    SEC("fentry/inet_csk_listen_start")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_inet_csk_listen_start, sk: *mut sock) -> c_int {
    int BPF_PROG(trace_inet_csk_listen_start, struct sock *sk)
    {
    set_task_info(sk);
    return 0;
    }
    SEC("fentry/tcp_connect")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_tcp_connect, sk: *mut sock) -> c_int {
    int BPF_PROG(trace_tcp_connect, struct sock *sk)
    {
    set_task_info(sk);
    return 0;
    }
    SEC("fexit/inet_csk_accept")
    int BPF_PROG(inet_csk_accept, struct sock *sk, struct proto_accept_arg *arg,
    struct sock *accepted_sk)
    {
    set_task_info(accepted_sk);
    return 0;
    }
    SEC("tp_btf/tcp_retransmit_synack")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: tcp_retransmit_synack, sk: *mut *mut sock, req: *mut *mut request_sock) -> c_int {
    int BPF_PROG(tcp_retransmit_synack, struct sock* sk, struct request_sock* req)
    {
// load only test
    bpf_sk_storage_get(&sk_stg_map, sk, 0, 0);
    bpf_sk_storage_get(&sk_stg_map, req.sk, 0, 0);
    return 0;
    }
    SEC("tp_btf/tcp_bad_csum")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: tcp_bad_csum, skb: *mut *mut sk_buff) -> c_int {
    int BPF_PROG(tcp_bad_csum, struct sk_buff* skb)
    {
    bpf_sk_storage_get(&sk_stg_map, skb.sk, 0, 0);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
