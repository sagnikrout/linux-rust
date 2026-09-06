//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tailcall_bpf2bpf4.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    } nop_table SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 3);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    } jmp_table SEC(".maps");
    let mut count: c_int = 0;
    let mut noise: c_int = 0;
#[no_mangle]
unsafe extern "C" fn subprog_noise() -> __always_inline int {
    static __always_inline int subprog_noise(void)
    {
    let mut key: __u32 = 0;
    bpf_map_lookup_elem(&nop_table, &key);
    return 0;
    }
    __noinline
#[no_mangle]
pub unsafe extern "C" fn subprog_tail_2(skb: *mut __sk_buff) -> c_int {
    int subprog_tail_2(struct __sk_buff *skb)
    {
    if (noise)
    subprog_noise();
    bpf_tail_call_static(skb, &jmp_table, 2);
    return skb.len * 3;
    }
    __noinline
#[no_mangle]
pub unsafe extern "C" fn subprog_tail_1(skb: *mut __sk_buff) -> c_int {
    int subprog_tail_1(struct __sk_buff *skb)
    {
    bpf_tail_call_static(skb, &jmp_table, 1);
    return skb.len * 2;
    }
    __noinline
#[no_mangle]
pub unsafe extern "C" fn subprog_tail(skb: *mut __sk_buff) -> c_int {
    int subprog_tail(struct __sk_buff *skb)
    {
    bpf_tail_call_static(skb, &jmp_table, 0);
    return skb.len;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_1(skb: *mut __sk_buff) -> c_int {
    int classifier_1(struct __sk_buff *skb)
    {
    return subprog_tail_2(skb);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_2(skb: *mut __sk_buff) -> c_int {
    int classifier_2(struct __sk_buff *skb)
    {
    count++;
    return subprog_tail_2(skb);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> c_int {
    int classifier_0(struct __sk_buff *skb)
    {
    return subprog_tail_1(skb);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> c_int {
    int entry(struct __sk_buff *skb)
    {
    return subprog_tail(skb);
    }
    char __license[] SEC("license") = "GPL";
