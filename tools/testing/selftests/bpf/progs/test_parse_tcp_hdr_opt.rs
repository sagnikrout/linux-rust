//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_parse_tcp_hdr_opt.c
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
// This parsing logic is taken from the open source library katran, a layer 4
// load balancer.
//
// This code logic using dynptrs can be found in test_parse_tcp_hdr_opt_dynptr.c
//
// https://github.com/facebookincubator/katran/blob/main/katran/lib/bpf/pckt_parsing.h
//

    char _license[] SEC("license") = "GPL";
// Kind number used for experiments
    let mut tcp_hdr_opt_kind_tpr: __u32 = 0xFD;
// Length of the tcp header option
    let mut tcp_hdr_opt_len_tpr: __u32 = 6;
// maximum number of header options to check to lookup server_id
    let mut tcp_hdr_opt_max_opt_checks: __u32 = 15;
    __u32 server_id;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_opt_state {
    pub server_id: __u32,
    pub byte_offset: __u8,
    pub hdr_bytes_remaining: __u8,
}

#[no_mangle]
unsafe extern "C" fn parse_hdr_opt(xdp: *const xdp_md, state: *mut hdr_opt_state) -> c_int {
    static int parse_hdr_opt(const struct xdp_md *xdp, struct hdr_opt_state *state)
    {
    const void *data = (void *)(long)xdp.data;
    const void *data_end = (void *)(long)xdp.data_end;
    __u8 *tcp_opt, kind, hdr_len;
    tcp_opt = (__u8 *)(data + state.byte_offset);
    if (tcp_opt + 1 > data_end)
    return -1;
    kind = tcp_opt[0];
    if (kind == TCPOPT_EOL)
    return -1;
    if (kind == TCPOPT_NOP) {
    state.hdr_bytes_remaining--;
    state.byte_offset++;
    return 0;
    }
    if (state.hdr_bytes_remaining < 2 ||
    tcp_opt + sizeof(__u8) + sizeof(__u8) > data_end)
    return -1;
    hdr_len = tcp_opt[1];
    if (hdr_len > state.hdr_bytes_remaining)
    return -1;
    if (kind == tcp_hdr_opt_kind_tpr) {
    if (hdr_len != tcp_hdr_opt_len_tpr)
    return -1;
    if (tcp_opt + tcp_hdr_opt_len_tpr > data_end)
    return -1;
    state.server_id = *(__u32 *)&tcp_opt[2];
    return 1;
    }
    state.hdr_bytes_remaining -= hdr_len;
    state.byte_offset += hdr_len;
    return 0;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_ingress_v6(xdp: *mut xdp_md) -> c_int {
    int xdp_ingress_v6(struct xdp_md *xdp)
    {
    const void *data = (void *)(long)xdp.data;
    const void *data_end = (void *)(long)xdp.data_end;
    let mut opt_state: hdr_opt_state = {};
    let mut tcp_hdr_opt_len: __u8 = 0;
    struct tcphdr *tcp_hdr;
    let mut tcp_offset: __u64 = 0;
    int err;
    tcp_offset = sizeof(struct ethhdr) + sizeof(struct ipv6hdr);
    tcp_hdr = (struct tcphdr *)(data + tcp_offset);
    if (tcp_hdr + 1 > data_end)
    return XDP_DROP;
    tcp_hdr_opt_len = (tcp_hdr.doff * 4) - sizeof(struct tcphdr);
    if (tcp_hdr_opt_len < tcp_hdr_opt_len_tpr)
    return XDP_DROP;
    opt_state.hdr_bytes_remaining = tcp_hdr_opt_len;
    opt_state.byte_offset = sizeof(struct tcphdr) + tcp_offset;
// max number of bytes of options in tcp header is 40 bytes
    for (int i = 0; i < tcp_hdr_opt_max_opt_checks; i++) {
    err = parse_hdr_opt(xdp, &opt_state);
    if (err || !opt_state.hdr_bytes_remaining)
    break;
    }
    if (!opt_state.server_id)
    return XDP_DROP;
    server_id = opt_state.server_id;
    return XDP_PASS;
    }
