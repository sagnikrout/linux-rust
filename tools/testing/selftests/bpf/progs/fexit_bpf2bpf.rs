//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/fexit_bpf2bpf.c
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
// Copyright (c) 2019 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    pub len: c_uint,
}

    let mut test_result: __u64 = 0;
    SEC("fexit/test_pkt_access")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_main, skb: *mut sk_buff, ret: c_int) -> c_int {
    int BPF_PROG(test_main, struct sk_buff *skb, int ret)
    {
    int len;
    __builtin_preserve_access_index(({
    len = skb.len;
    }));
    if (len != 74 || ret != 0)
    return 0;
    test_result = 1;
    return 0;
    }
    let mut test_result_subprog1: __u64 = 0;
    SEC("fexit/test_pkt_access_subprog1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_subprog1, skb: *mut sk_buff, ret: c_int) -> c_int {
    int BPF_PROG(test_subprog1, struct sk_buff *skb, int ret)
    {
    int len;
    __builtin_preserve_access_index(({
    len = skb.len;
    }));
    if (len != 74 || ret != 148)
    return 0;
    test_result_subprog1 = 1;
    return 0;
    }
// Though test_pkt_access_subprog2() is defined in C as:
// static __attribute__ ((noinline))
// int test_pkt_access_subprog2(int val, volatile struct __sk_buff *skb)
// {
// return skb->len * val;
// }
// llvm optimizations remove 'int val' argument and generate BPF assembly:
// r0 = *(u32 *)(r1 + 0)
// w0 <<= 1
// exit
// Before llvm23, in such case the verifier falls back to conservative and
// tracing program can access arguments and return value as u64
// instead of accurate types. With llvm23, the true signature
// int test_pkt_access_subprog2(volatile struct __sk_buff *skb)
// is available in btf.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_subprog2 {
    pub args: [__u64; 1],
    pub ret: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_subprog2 {
    pub args: [__u64; 5],
    pub ret: __u64,
}

    let mut test_result_subprog2: __u64 = 0;
    SEC("fexit/test_pkt_access_subprog2")
#[no_mangle]
pub unsafe extern "C" fn test_subprog2(ctx: *mut args_subprog2) -> c_int {
    int test_subprog2(struct args_subprog2 *ctx)
    {
    struct sk_buff *skb = (void *)ctx.args[0];
    __u64 ret;
    int len;
    bpf_probe_read_kernel(&len, sizeof(len),
    __builtin_preserve_access_index(&skb.len));
    ret = ctx.ret;
// bpf_prog_test_load() loads "test_pkt_access.bpf.o" with
// BPF_F_TEST_RND_HI32 which randomizes upper 32 bits after BPF_ALU32
// insns. Hence after 'w0 <<= 1' upper bits of $rax are random. That is
// expected and correct. Trim them.
//
    ret = (__u32) ret;
    if (len != 74 || ret != 148)
    return 0;
    test_result_subprog2 = 1;
    return 0;
    }
    let mut test_result_subprog3: __u64 = 0;
    SEC("fexit/test_pkt_access_subprog3")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_subprog3, val: c_int, skb: *mut sk_buff, ret: c_int) -> c_int {
    int BPF_PROG(test_subprog3, int val, struct sk_buff *skb, int ret)
    {
    int len;
    __builtin_preserve_access_index(({
    len = skb.len;
    }));
    if (len != 74 || ret != 74 * val || val != 3)
    return 0;
    test_result_subprog3 = 1;
    return 0;
    }
    let mut test_get_skb_len: __u64 = 0;
    SEC("freplace/get_skb_len")
#[no_mangle]
pub unsafe extern "C" fn new_get_skb_len(skb: *mut __sk_buff) -> c_int {
    int new_get_skb_len(struct __sk_buff *skb)
    {
    let mut len: c_int = skb.len;
    if (len != 74)
    return 0;
    test_get_skb_len = 1;
    return 74; /* original get_skb_len() returns skb.len */
    }
    let mut test_get_skb_ifindex: __u64 = 0;
    SEC("freplace/get_skb_ifindex")
#[no_mangle]
pub unsafe extern "C" fn new_get_skb_ifindex(val: c_int, skb: *mut __sk_buff, var: c_int) -> c_int {
    int new_get_skb_ifindex(int val, struct __sk_buff *skb, int var)
    {
    void *data_end = (void *)(long)skb.data_end;
    void *data = (void *)(long)skb.data;
    struct ipv6hdr ip6, *ip6p;
    let mut ifindex: c_int = skb.ifindex;
// check that BPF extension can read packet via direct packet access
    if (data + 14 + sizeof(ip6) > data_end)
    return 0;
    ip6p = data + 14;
    if (ip6p.nexthdr != 6 || ip6p.payload_len != __bpf_constant_htons(123))
    return 0;
// check that legacy packet access helper works too
    if (bpf_skb_load_bytes(skb, 14, &ip6, sizeof(ip6)) < 0)
    return 0;
    ip6p = &ip6;
    if (ip6p.nexthdr != 6 || ip6p.payload_len != __bpf_constant_htons(123))
    return 0;
    if (ifindex != 1 || val != 3 || var != 1)
    return 0;
    test_get_skb_ifindex = 1;
    return 3; /* original get_skb_ifindex() returns val * ifindex * var */
    }
    let mut test_get_constant: volatile __u64 = 0;
    SEC("freplace/get_constant")
#[no_mangle]
pub unsafe extern "C" fn new_get_constant(val: c_long) -> c_int {
    int new_get_constant(long val)
    {
    if (val != 123)
    return 0;
    test_get_constant = 1;
    return test_get_constant; /* original get_constant() returns val - 122 */
    }
    let mut test_pkt_write_access_subprog: __u64 = 0;
    SEC("freplace/test_pkt_write_access_subprog")
#[no_mangle]
pub unsafe extern "C" fn new_test_pkt_write_access_subprog(skb: *mut __sk_buff, off: __u32) -> c_int {
    int new_test_pkt_write_access_subprog(struct __sk_buff *skb, __u32 off)
    {
    void *data = (void *)(long)skb.data;
    void *data_end = (void *)(long)skb.data_end;
    struct tcphdr *tcp;
    if (off > sizeof(struct ethhdr) + sizeof(struct ipv6hdr))
    return -1;
    tcp = data + off;
    if (tcp + 1 > data_end)
    return -1;
// make modifications to the packet data
    tcp.check++;
    tcp.syn = 0;
    test_pkt_write_access_subprog = 1;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
