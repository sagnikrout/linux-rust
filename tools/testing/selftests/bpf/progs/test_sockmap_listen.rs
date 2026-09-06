//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sockmap_listen.c
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
// Copyright (c) 2020 Cloudflare

    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, __u64);
    } sock_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, __u64);
    } nop_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKHASH);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, __u64);
    } sock_hash SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 2);
    __type(key, int);
    __type(value, unsigned int);
    } verdict_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    } parser_map SEC(".maps");
    bool test_sockmap = false; /* toggled by user-space */
    bool test_ingress = false; /* toggled by user-space */
    SEC("sk_skb/stream_parser")
#[no_mangle]
pub unsafe extern "C" fn prog_stream_parser(skb: *mut __sk_buff) -> c_int {
    int prog_stream_parser(struct __sk_buff *skb)
    {
    int *value;
    let mut key: __u32 = 0;
    value = bpf_map_lookup_elem(&parser_map, &key);
    if (value && *value)
    return *value;
    return skb.len;
    }
    SEC("sk_skb/stream_verdict")
#[no_mangle]
pub unsafe extern "C" fn prog_stream_verdict(skb: *mut __sk_buff) -> c_int {
    int prog_stream_verdict(struct __sk_buff *skb)
    {
    unsigned int *count;
    let mut zero: __u32 = 0;
    int verdict;
    if (test_sockmap)
    verdict = bpf_sk_redirect_map(skb, &sock_map, zero, 0);
    else
    verdict = bpf_sk_redirect_hash(skb, &sock_hash, &zero, 0);
    count = bpf_map_lookup_elem(&verdict_map, &verdict);
    if (count)
    (*count)++;
    return verdict;
    }
    SEC("sk_skb")
#[no_mangle]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> c_int {
    int prog_skb_verdict(struct __sk_buff *skb)
    {
    unsigned int *count;
    let mut zero: __u32 = 0;
    int verdict;
    if (test_sockmap)
    verdict = bpf_sk_redirect_map(skb, &sock_map, zero,
    test_ingress ? BPF_F_INGRESS : 0);
    else
    verdict = bpf_sk_redirect_hash(skb, &sock_hash, &zero,
    test_ingress ? BPF_F_INGRESS : 0);
    count = bpf_map_lookup_elem(&verdict_map, &verdict);
    if (count)
    (*count)++;
    return verdict;
    }
    SEC("sk_msg")
#[no_mangle]
pub unsafe extern "C" fn prog_msg_verdict(msg: *mut sk_msg_md) -> c_int {
    int prog_msg_verdict(struct sk_msg_md *msg)
    {
    unsigned int *count;
    let mut zero: __u32 = 0;
    int verdict;
    if (test_sockmap)
    verdict = bpf_msg_redirect_map(msg, &sock_map, zero, 0);
    else
    verdict = bpf_msg_redirect_hash(msg, &sock_hash, &zero, 0);
    count = bpf_map_lookup_elem(&verdict_map, &verdict);
    if (count)
    (*count)++;
    return verdict;
    }
    SEC("sk_reuseport")
#[no_mangle]
pub unsafe extern "C" fn prog_reuseport(reuse: *mut sk_reuseport_md) -> c_int {
    int prog_reuseport(struct sk_reuseport_md *reuse)
    {
    unsigned int *count;
    int err, verdict;
    let mut zero: __u32 = 0;
    if (test_sockmap)
    err = bpf_sk_select_reuseport(reuse, &sock_map, &zero, 0);
    else
    err = bpf_sk_select_reuseport(reuse, &sock_hash, &zero, 0);
    verdict = err ? SK_DROP : SK_PASS;
    count = bpf_map_lookup_elem(&verdict_map, &verdict);
    if (count)
    (*count)++;
    return verdict;
    }
    char _license[] SEC("license") = "GPL";
