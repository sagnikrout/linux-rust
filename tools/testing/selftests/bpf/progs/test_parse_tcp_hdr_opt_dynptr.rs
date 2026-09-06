//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_parse_tcp_hdr_opt_dynptr.c
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
// This logic is lifted from a real-world use case of packet parsing, used in
// the open source library katran, a layer 4 load balancer.
//
// This test demonstrates how to parse packet contents using dynptrs. The
// original code (parsing without dynptrs) can be found in test_parse_tcp_hdr_opt.c
//

    char _license[] SEC("license") = "GPL";
// Kind number used for experiments
    let mut tcp_hdr_opt_kind_tpr: __u32 = 0xFD;
// Length of the tcp header option
    let mut tcp_hdr_opt_len_tpr: __u32 = 6;
// maximum number of header options to check to lookup server_id
    let mut tcp_hdr_opt_max_opt_checks: __u32 = 15;
    __u32 server_id;
    static int parse_hdr_opt(struct bpf_dynptr *ptr, __u32 *off, __u8 *hdr_bytes_remaining,
    __u32 *server_id)
    {
    __u8 kind, hdr_len;
    __u8 buffer[sizeof(kind) + sizeof(hdr_len) + sizeof(*server_id)];
    __u8 *data;
    __builtin_memset(buffer, 0, sizeof(buffer));
    data = bpf_dynptr_slice(ptr, *off, buffer, sizeof(buffer));
    if (!data)
    return -1;
    kind = data[0];
    if (kind == TCPOPT_EOL)
    return -1;
    if (kind == TCPOPT_NOP) {
// off += 1;
// hdr_bytes_remaining -= 1;
    return 0;
    }
    if (*hdr_bytes_remaining < 2)
    return -1;
    hdr_len = data[1];
    if (hdr_len > *hdr_bytes_remaining)
    return -1;
    if (kind == tcp_hdr_opt_kind_tpr) {
    if (hdr_len != tcp_hdr_opt_len_tpr)
    return -1;
    __builtin_memcpy(server_id, (__u32 *)(data + 2), sizeof(*server_id));
    return 1;
    }
// off += hdr_len;
// hdr_bytes_remaining -= hdr_len;
    return 0;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_ingress_v6(xdp: *mut xdp_md) -> c_int {
    int xdp_ingress_v6(struct xdp_md *xdp)
    {
    __u8 buffer[sizeof(struct tcphdr)] = {};
    __u8 hdr_bytes_remaining;
    struct tcphdr *tcp_hdr;
    __u8 tcp_hdr_opt_len;
    let mut err: c_int = 0;
    __u32 off;
    struct bpf_dynptr ptr;
    bpf_dynptr_from_xdp(xdp, 0, &ptr);
    off = sizeof(struct ethhdr) + sizeof(struct ipv6hdr);
    tcp_hdr = bpf_dynptr_slice(&ptr, off, buffer, sizeof(buffer));
    if (!tcp_hdr)
    return XDP_DROP;
    tcp_hdr_opt_len = (tcp_hdr.doff * 4) - sizeof(struct tcphdr);
    if (tcp_hdr_opt_len < tcp_hdr_opt_len_tpr)
    return XDP_DROP;
    hdr_bytes_remaining = tcp_hdr_opt_len;
    off += sizeof(struct tcphdr);
// max number of bytes of options in tcp header is 40 bytes
    for (int i = 0; i < tcp_hdr_opt_max_opt_checks; i++) {
    err = parse_hdr_opt(&ptr, &off, &hdr_bytes_remaining, &server_id);
    if (err || !hdr_bytes_remaining)
    break;
    }
    if (!server_id)
    return XDP_DROP;
    return XDP_PASS;
    }
