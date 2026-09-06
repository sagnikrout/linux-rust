//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_udp6.c
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

    char _license[] SEC("license") = "GPL";

    "  sl  "					\
    "local_address                         "	\
    "remote_address                        "	\
    "st tx_queue rx_queue tr tm.when retrnsmt"	\
    "   uid  timeout inode ref pointer drops\n"
#[no_mangle]
unsafe extern "C" fn sock_i_ino(sk: *const sock) -> c_long {
    static long sock_i_ino(const struct sock *sk)
    {
    const struct socket *sk_socket = sk.sk_socket;
    const struct inode *inode;
    unsigned long ino;
    if (!sk_socket)
    return 0;
    inode = &container_of(sk_socket, struct socket_alloc, socket).vfs_inode;
    bpf_probe_read_kernel(&ino, sizeof(ino), &inode.i_ino);
    return ino;
    }
    SEC("iter/udp")
#[no_mangle]
pub unsafe extern "C" fn dump_udp6(ctx: *mut bpf_iter__udp) -> c_int {
    int dump_udp6(struct bpf_iter__udp *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct udp_sock *udp_sk = ctx.udp_sk;
    const struct in6_addr *dest, *src;
    struct udp6_sock *udp6_sk;
    struct inet_sock *inet;
    __u16 srcp, destp;
    __u32 seq_num;
    int rqueue;
    if (udp_sk == (void *)0)
    return 0;
    seq_num = ctx.meta.seq_num;
    if (seq_num == 0)
    BPF_SEQ_PRINTF(seq, IPV6_SEQ_DGRAM_HEADER);
    udp6_sk = bpf_skc_to_udp6_sock(udp_sk);
    if (udp6_sk == (void *)0)
    return 0;
    inet = &udp_sk.inet;
    srcp = bpf_ntohs(inet.inet_sport);
    destp = bpf_ntohs(inet.inet_dport);
    rqueue = inet.sk.sk_rmem_alloc.counter - udp_sk.forward_deficit;
    dest  = &inet.sk.sk_v6_daddr;
    src   = &inet.sk.sk_v6_rcv_saddr;
    BPF_SEQ_PRINTF(seq, "%5d: %08X%08X%08X%08X:%04X %08X%08X%08X%08X:%04X ",
    ctx.bucket,
    src.s6_addr32[0], src.s6_addr32[1],
    src.s6_addr32[2], src.s6_addr32[3], srcp,
    dest.s6_addr32[0], dest.s6_addr32[1],
    dest.s6_addr32[2], dest.s6_addr32[3], destp);
    BPF_SEQ_PRINTF(seq, "%02X %08X:%08X %02X:%08lX %08X %5u %8d %lu %d %pK %u\n",
    inet.sk.sk_state,
    inet.sk.sk_wmem_alloc.refs.counter - 1,
    rqueue,
    0, 0L, 0, ctx.uid, 0,
    sock_i_ino(&inet.sk),
    inet.sk.sk_refcnt.refs.counter, udp_sk,
    udp_sk.drop_counters.drops0.counter +
    udp_sk.drop_counters.drops1.counter);
    return 0;
    }
