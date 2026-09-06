//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tcpbpf_kern.c
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

    let mut global: tcpbpf_globals = {};
//
// SOL_TCP is defined in <netinet/tcp.h> while
// TCP_SAVED_SYN is defined in already included <linux/tcp.h>
//

pub const SOL_TCP: c_int = 6;

#[no_mangle]
unsafe extern "C" fn get_tp_window_clamp(skops: *mut bpf_sock_ops) -> __always_inline int {
    static __always_inline int get_tp_window_clamp(struct bpf_sock_ops *skops)
    {
    struct bpf_sock *sk;
    struct tcp_sock *tp;
    sk = skops.sk;
    if (!sk)
    return -1;
    tp = bpf_skc_to_tcp_sock(sk);
    if (!tp)
    return -1;
    return tp.window_clamp;
    }
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn bpf_testcb(skops: *mut bpf_sock_ops) -> c_int {
    int bpf_testcb(struct bpf_sock_ops *skops)
    {
    char header[sizeof(struct ipv6hdr) + sizeof(struct tcphdr)];
    struct bpf_sock_ops *reuse = skops;
    struct tcphdr *thdr;
    let mut window_clamp: c_int = 9216;
    let mut save_syn: c_int = 1;
    let mut rv: c_int = -1;
    let mut v: c_int = 0;
    int op;
// Test reading fields in bpf_sock_ops using single register
    asm volatile (
    "%[reuse] = *(u32 *)(%[reuse] +96)"
    : [reuse] "+r"(reuse)
    :);
    asm volatile (
    "%[op] = *(u32 *)(%[skops] +96)"
    : [op] "=r"(op)
    : [skops] "r"(skops)
    :);
    asm volatile (
    "r9 = %[skops];\n"
    "r8 = *(u32 *)(r9 +164);\n"
    "*(u32 *)(r9 +164) = r8;\n"
    :: [skops] "r"(skops)
    : "r9", "r8");
    asm volatile (
    "r1 = %[skops];\n"
    "r1 = *(u64 *)(r1 +184);\n"
    "if r1 == 0 goto +1;\n"
    "r1 = *(u32 *)(r1 +4);\n"
    :: [skops] "r"(skops):"r1");
    asm volatile (
    "r9 = %[skops];\n"
    "r9 = *(u64 *)(r9 +184);\n"
    "if r9 == 0 goto +1;\n"
    "r9 = *(u32 *)(r9 +4);\n"
    :: [skops] "r"(skops):"r9");
    asm volatile (
    "r1 = %[skops];\n"
    "r2 = *(u64 *)(r1 +184);\n"
    "if r2 == 0 goto +1;\n"
    "r2 = *(u32 *)(r2 +4);\n"
    :: [skops] "r"(skops):"r1", "r2");
    op = (int) skops.op;
    global.event_map |= (1 << op);
    switch (op) {
    case BPF_SOCK_OPS_TCP_CONNECT_CB:
    rv = bpf_setsockopt(skops, SOL_TCP, TCP_WINDOW_CLAMP,
    &window_clamp, sizeof(window_clamp));
    global.window_clamp_client = get_tp_window_clamp(skops);
    break;
    case BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB:
// Test failure to set largest cb flag (assumes not defined)
    global.bad_cb_test_rv = bpf_sock_ops_cb_flags_set(skops, 0x80);
// Set callback
    global.good_cb_test_rv = bpf_sock_ops_cb_flags_set(skops,
    BPF_SOCK_OPS_STATE_CB_FLAG);
    break;
    case BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB:
    skops.sk_txhash = 0x12345f;
    v = 0xff;
    rv = bpf_setsockopt(skops, SOL_IPV6, IPV6_TCLASS, &v,
    sizeof(v));
    if (skops.family == AF_INET6) {
    v = bpf_getsockopt(skops, IPPROTO_TCP, TCP_SAVED_SYN,
    header, (sizeof(struct ipv6hdr) +
    sizeof(struct tcphdr)));
    if (!v) {
    let mut offset: c_int = sizeof(struct ipv6hdr);
    thdr = (struct tcphdr *)(header + offset);
    v = thdr.syn;
    global.tcp_saved_syn = v;
    }
    }
    rv = bpf_setsockopt(skops, SOL_TCP, TCP_WINDOW_CLAMP,
    &window_clamp, sizeof(window_clamp));
    global.window_clamp_server = get_tp_window_clamp(skops);
    break;
    case BPF_SOCK_OPS_RTO_CB:
    break;
    case BPF_SOCK_OPS_RETRANS_CB:
    break;
    case BPF_SOCK_OPS_STATE_CB:
    if (skops.args[1] == BPF_TCP_CLOSE) {
    if (skops.args[0] == BPF_TCP_LISTEN) {
    global.num_listen++;
    } else {
    global.total_retrans = skops.total_retrans;
    global.data_segs_in = skops.data_segs_in;
    global.data_segs_out = skops.data_segs_out;
    global.bytes_received = skops.bytes_received;
    global.bytes_acked = skops.bytes_acked;
    }
    global.num_close_events++;
    }
    break;
    case BPF_SOCK_OPS_TCP_LISTEN_CB:
    bpf_sock_ops_cb_flags_set(skops, BPF_SOCK_OPS_STATE_CB_FLAG);
    v = bpf_setsockopt(skops, IPPROTO_TCP, TCP_SAVE_SYN,
    &save_syn, sizeof(save_syn));
// Update global map w/ result of setsock opt
    global.tcp_save_syn = v;
    break;
    default:
    rv = -1;
    }
    skops.reply = rv;
    return 1;
    }
    char _license[] SEC("license") = "GPL";
