//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kfunc_call_test_subprog.c
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
// Copyright (c) 2021 Facebook

    extern const int bpf_prog_active __ksym;
    let mut active_res: c_int = -1;
    let mut sk_state_res: c_int = -1;
#[no_mangle]
pub unsafe extern "C" fn f1(skb: *mut __sk_buff) -> int __noinline {
    int __noinline f1(struct __sk_buff *skb)
    {
    struct bpf_sock *sk = skb.sk;
    int *active;
    if (!sk)
    return -1;
    sk = bpf_sk_fullsock(sk);
    if (!sk)
    return -1;
    active = (int *)bpf_per_cpu_ptr(&bpf_prog_active,
    bpf_get_smp_processor_id());
    if (active)
    active_res = *active;
    sk_state_res = bpf_kfunc_call_test3((struct sock *)sk).__sk_common.skc_state;
    return (__u32)bpf_kfunc_call_test1((struct sock *)sk, 1, 2, 3, 4);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn kfunc_call_test1(skb: *mut __sk_buff) -> c_int {
    int kfunc_call_test1(struct __sk_buff *skb)
    {
    return f1(skb);
    }
    char _license[] SEC("license") = "GPL";
