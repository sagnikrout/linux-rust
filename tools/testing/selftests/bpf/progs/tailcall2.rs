//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tailcall2.c
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
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 5);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    } jmp_table SEC(".maps");
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> c_int {
    int classifier_0(struct __sk_buff *skb)
    {
    bpf_tail_call_static(skb, &jmp_table, 1);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_1(skb: *mut __sk_buff) -> c_int {
    int classifier_1(struct __sk_buff *skb)
    {
    bpf_tail_call_static(skb, &jmp_table, 2);
    return 1;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_2(skb: *mut __sk_buff) -> c_int {
    int classifier_2(struct __sk_buff *skb)
    {
    return 2;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_3(skb: *mut __sk_buff) -> c_int {
    int classifier_3(struct __sk_buff *skb)
    {
    bpf_tail_call_static(skb, &jmp_table, 4);
    return 3;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_4(skb: *mut __sk_buff) -> c_int {
    int classifier_4(struct __sk_buff *skb)
    {
    bpf_tail_call_static(skb, &jmp_table, 3);
    return 4;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> c_int {
    int entry(struct __sk_buff *skb)
    {
    bpf_tail_call_static(skb, &jmp_table, 0);
// Check multi-prog update.
    bpf_tail_call_static(skb, &jmp_table, 2);
// Check tail call limit.
    bpf_tail_call_static(skb, &jmp_table, 3);
    return 3;
    }
    char __license[] SEC("license") = "GPL";
