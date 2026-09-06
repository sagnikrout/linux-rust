//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/tailcall_bpf2bpf_hierarchy2.c
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

    int classifier_0(struct __sk_buff *skb);
    int classifier_1(struct __sk_buff *skb);
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 2);
    __uint(key_size, sizeof(__u32));
    __array(values, void (void));
    } jmp_table SEC(".maps") = {
    .values = {
    [0] = (void *) &classifier_0,
    [1] = (void *) &classifier_1,
    },
    };
    let mut count0: c_int = 0;
    let mut count1: c_int = 0;
    static __noinline
#[no_mangle]
pub unsafe extern "C" fn subprog_tail0(skb: *mut __sk_buff) -> c_int {
    int subprog_tail0(struct __sk_buff *skb)
    {
    let mut ret: c_int = 0;
    bpf_tail_call_static(skb, &jmp_table, 0);
    barrier_var(ret);
    return ret;
    }
    __auxiliary
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> c_int {
    int classifier_0(struct __sk_buff *skb)
    {
    count0++;
    subprog_tail0(skb);
    return 0;
    }
    static __noinline
#[no_mangle]
pub unsafe extern "C" fn subprog_tail1(skb: *mut __sk_buff) -> c_int {
    int subprog_tail1(struct __sk_buff *skb)
    {
    let mut ret: c_int = 0;
    bpf_tail_call_static(skb, &jmp_table, 1);
    barrier_var(ret);
    return ret;
    }
    __auxiliary
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_1(skb: *mut __sk_buff) -> c_int {
    int classifier_1(struct __sk_buff *skb)
    {
    int ret;
    count1++;
    ret = subprog_tail1(skb);
    __sink(ret);
    return 0;
    }
    __success
    __retval(33)
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tailcall_bpf2bpf_hierarchy_2(skb: *mut __sk_buff) -> c_int {
    int tailcall_bpf2bpf_hierarchy_2(struct __sk_buff *skb)
    {
    let mut ret: c_int = 0, ret1, ret2;
    clobber_regs_stack();
    ret1 = subprog_tail0(skb);
    ret2 = subprog_tail1(skb);
    __sink(ret1);
    __sink(ret2);
    __sink(ret);
    return (count1 << 16) | count0;
    }
    char __license[] SEC("license") = "GPL";
