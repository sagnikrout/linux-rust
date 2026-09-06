//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/mptcp_sockmap.c
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
    int sk_index;
    int redirect_idx;
    int trace_port;
    int helper_ret;
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    __uint(max_entries, 100);
    } sock_map SEC(".maps");
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn mptcp_sockmap_inject(skops: *mut bpf_sock_ops) -> c_int {
    int mptcp_sockmap_inject(struct bpf_sock_ops *skops)
    {
    struct bpf_sock *sk;
// only accept specified connection
    if (skops.local_port != trace_port ||
    skops.op != BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB)
    return 1;
    sk = skops.sk;
    if (!sk)
    return 1;
// update sk handler
    helper_ret = bpf_sock_map_update(skops, &sock_map, &sk_index, BPF_NOEXIST);
    return 1;
    }
    SEC("sk_skb/stream_verdict")
#[no_mangle]
pub unsafe extern "C" fn mptcp_sockmap_redirect(skb: *mut __sk_buff) -> c_int {
    int mptcp_sockmap_redirect(struct __sk_buff *skb)
    {
// redirect skb to the sk under sock_map[redirect_idx]
    return bpf_sk_redirect_map(skb, &sock_map, redirect_idx, 0);
    }
