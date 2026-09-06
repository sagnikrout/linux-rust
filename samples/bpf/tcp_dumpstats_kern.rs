//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tcp_dumpstats_kern.c
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
// Refer to samples/bpf/tcp_bpf.readme for the instructions on
// how to run this sample program.
//

    int _version SEC("version") = 1;
    char _license[] SEC("license") = "GPL";
    struct {
    __u32 type;
    __u32 map_flags;
    int *key;
    __u64 *value;
    } bpf_next_dump SEC(".maps") = {
    .type = BPF_MAP_TYPE_SK_STORAGE,
    .map_flags = BPF_F_NO_PREALLOC,
    };
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn _sockops(ctx: *mut bpf_sock_ops) -> c_int {
    int _sockops(struct bpf_sock_ops *ctx)
    {
    struct bpf_tcp_sock *tcp_sk;
    struct bpf_sock *sk;
    __u64 *next_dump;
    __u64 now;
    switch (ctx.op) {
    case BPF_SOCK_OPS_TCP_CONNECT_CB:
    bpf_sock_ops_cb_flags_set(ctx, BPF_SOCK_OPS_RTT_CB_FLAG);
    return 1;
    case BPF_SOCK_OPS_RTT_CB:
    break;
    default:
    return 1;
    }
    sk = ctx.sk;
    if (!sk)
    return 1;
    next_dump = bpf_sk_storage_get(&bpf_next_dump, sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!next_dump)
    return 1;
    now = bpf_ktime_get_ns();
    if (now < *next_dump)
    return 1;
    tcp_sk = bpf_tcp_sock(sk);
    if (!tcp_sk)
    return 1;
// next_dump = now + INTERVAL;
    bpf_printk("dsack_dups=%u delivered=%u\n",
    tcp_sk.dsack_dups, tcp_sk.delivered);
    bpf_printk("delivered_ce=%u icsk_retransmits=%u\n",
    tcp_sk.delivered_ce, tcp_sk.icsk_retransmits);
    return 1;
    }
