//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgroup_tcp_skb.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    let mut g_sock_port: __u16 = 0;
    let mut g_sock_state: __u32 = 0;
    let mut g_unexpected: c_int = 0;
    let mut g_packet_count: __u32 = 0;
#[no_mangle]
pub unsafe extern "C" fn needed_tcp_pkt(skb: *mut __sk_buff, tcph: *mut tcphdr) -> c_int {
    int needed_tcp_pkt(struct __sk_buff *skb, struct tcphdr *tcph)
    {
    struct ipv6hdr ip6h;
    if (skb.protocol != bpf_htons(ETH_P_IPV6))
    return 0;
    if (bpf_skb_load_bytes(skb, 0, &ip6h, sizeof(ip6h)))
    return 0;
    if (ip6h.nexthdr != IPPROTO_TCP)
    return 0;
    if (bpf_skb_load_bytes(skb, sizeof(ip6h), tcph, sizeof(*tcph)))
    return 0;
    if (tcph.source != bpf_htons(g_sock_port) &&
    tcph.dest != bpf_htons(g_sock_port))
    return 0;
    return 1;
    }
// Run accept() on a socket in the cgroup to receive a new connection.
#[no_mangle]
unsafe extern "C" fn egress_accept(tcph: *mut tcphdr) -> c_int {
    static int egress_accept(struct tcphdr *tcph)
    {
    if (g_sock_state ==  SYN_RECV_SENDING_SYN_ACK) {
    if (tcph.fin || !tcph.syn || !tcph.ack)
    g_unexpected++;
    else
    g_sock_state = SYN_RECV;
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingress_accept(tcph: *mut tcphdr) -> c_int {
    static int ingress_accept(struct tcphdr *tcph)
    {
    switch (g_sock_state) {
    case INIT:
    if (!tcph.syn || tcph.fin || tcph.ack)
    g_unexpected++;
    else
    g_sock_state = SYN_RECV_SENDING_SYN_ACK;
    break;
    case SYN_RECV:
    if (tcph.fin || tcph.syn || !tcph.ack)
    g_unexpected++;
    else
    g_sock_state = ESTABLISHED;
    break;
    default:
    return 0;
    }
    return 1;
    }
// Run connect() on a socket in the cgroup to start a new connection.
#[no_mangle]
unsafe extern "C" fn egress_connect(tcph: *mut tcphdr) -> c_int {
    static int egress_connect(struct tcphdr *tcph)
    {
    if (g_sock_state == INIT) {
    if (!tcph.syn || tcph.fin || tcph.ack)
    g_unexpected++;
    else
    g_sock_state = SYN_SENT;
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingress_connect(tcph: *mut tcphdr) -> c_int {
    static int ingress_connect(struct tcphdr *tcph)
    {
    if (g_sock_state == SYN_SENT) {
    if (tcph.fin || !tcph.syn || !tcph.ack)
    g_unexpected++;
    else
    g_sock_state = ESTABLISHED;
    return 1;
    }
    return 0;
    }
// The connection is closed by the peer outside the cgroup.
#[no_mangle]
unsafe extern "C" fn egress_close_remote(tcph: *mut tcphdr) -> c_int {
    static int egress_close_remote(struct tcphdr *tcph)
    {
    switch (g_sock_state) {
    case ESTABLISHED:
    break;
    case CLOSE_WAIT_SENDING_ACK:
    if (tcph.fin || tcph.syn || !tcph.ack)
    g_unexpected++;
    else
    g_sock_state = CLOSE_WAIT;
    break;
    case CLOSE_WAIT:
    if (!tcph.fin)
    g_unexpected++;
    else
    g_sock_state = LAST_ACK;
    break;
    default:
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn ingress_close_remote(tcph: *mut tcphdr) -> c_int {
    static int ingress_close_remote(struct tcphdr *tcph)
    {
    switch (g_sock_state) {
    case ESTABLISHED:
    if (tcph.fin)
    g_sock_state = CLOSE_WAIT_SENDING_ACK;
    break;
    case LAST_ACK:
    if (tcph.fin || tcph.syn || !tcph.ack)
    g_unexpected++;
    else
    g_sock_state = CLOSED;
    break;
    default:
    return 0;
    }
    return 1;
    }
// The connection is closed by the endpoint inside the cgroup.
#[no_mangle]
unsafe extern "C" fn egress_close_local(tcph: *mut tcphdr) -> c_int {
    static int egress_close_local(struct tcphdr *tcph)
    {
    switch (g_sock_state) {
    case ESTABLISHED:
    if (tcph.fin)
    g_sock_state = FIN_WAIT1;
    break;
    case TIME_WAIT_SENDING_ACK:
    if (tcph.fin || tcph.syn || !tcph.ack)
    g_unexpected++;
    else
    g_sock_state = TIME_WAIT;
    break;
    default:
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn ingress_close_local(tcph: *mut tcphdr) -> c_int {
    static int ingress_close_local(struct tcphdr *tcph)
    {
    switch (g_sock_state) {
    case ESTABLISHED:
    break;
    case FIN_WAIT1:
    if (tcph.fin || tcph.syn || !tcph.ack)
    g_unexpected++;
    else
    g_sock_state = FIN_WAIT2;
    break;
    case FIN_WAIT2:
    if (!tcph.fin || tcph.syn || !tcph.ack)
    g_unexpected++;
    else
    g_sock_state = TIME_WAIT_SENDING_ACK;
    break;
    default:
    return 0;
    }
    return 1;
    }
// Check the types of outgoing packets of a server socket to make sure they
// are consistent with the state of the server socket.
//
// The connection is closed by the client side.
//
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn server_egress(skb: *mut __sk_buff) -> c_int {
    int server_egress(struct __sk_buff *skb)
    {
    struct tcphdr tcph;
    if (!needed_tcp_pkt(skb, &tcph))
    return 1;
    g_packet_count++;
// Egress of the server socket.
    if (egress_accept(&tcph) || egress_close_remote(&tcph))
    return 1;
    g_unexpected++;
    return 1;
    }
// Check the types of incoming packets of a server socket to make sure they
// are consistent with the state of the server socket.
//
// The connection is closed by the client side.
//
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn server_ingress(skb: *mut __sk_buff) -> c_int {
    int server_ingress(struct __sk_buff *skb)
    {
    struct tcphdr tcph;
    if (!needed_tcp_pkt(skb, &tcph))
    return 1;
    g_packet_count++;
// Ingress of the server socket.
    if (ingress_accept(&tcph) || ingress_close_remote(&tcph))
    return 1;
    g_unexpected++;
    return 1;
    }
// Check the types of outgoing packets of a server socket to make sure they
// are consistent with the state of the server socket.
//
// The connection is closed by the server side.
//
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn server_egress_srv(skb: *mut __sk_buff) -> c_int {
    int server_egress_srv(struct __sk_buff *skb)
    {
    struct tcphdr tcph;
    if (!needed_tcp_pkt(skb, &tcph))
    return 1;
    g_packet_count++;
// Egress of the server socket.
    if (egress_accept(&tcph) || egress_close_local(&tcph))
    return 1;
    g_unexpected++;
    return 1;
    }
// Check the types of incoming packets of a server socket to make sure they
// are consistent with the state of the server socket.
//
// The connection is closed by the server side.
//
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn server_ingress_srv(skb: *mut __sk_buff) -> c_int {
    int server_ingress_srv(struct __sk_buff *skb)
    {
    struct tcphdr tcph;
    if (!needed_tcp_pkt(skb, &tcph))
    return 1;
    g_packet_count++;
// Ingress of the server socket.
    if (ingress_accept(&tcph) || ingress_close_local(&tcph))
    return 1;
    g_unexpected++;
    return 1;
    }
// Check the types of outgoing packets of a client socket to make sure they
// are consistent with the state of the client socket.
//
// The connection is closed by the server side.
//
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn client_egress_srv(skb: *mut __sk_buff) -> c_int {
    int client_egress_srv(struct __sk_buff *skb)
    {
    struct tcphdr tcph;
    if (!needed_tcp_pkt(skb, &tcph))
    return 1;
    g_packet_count++;
// Egress of the server socket.
    if (egress_connect(&tcph) || egress_close_remote(&tcph))
    return 1;
    g_unexpected++;
    return 1;
    }
// Check the types of incoming packets of a client socket to make sure they
// are consistent with the state of the client socket.
//
// The connection is closed by the server side.
//
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn client_ingress_srv(skb: *mut __sk_buff) -> c_int {
    int client_ingress_srv(struct __sk_buff *skb)
    {
    struct tcphdr tcph;
    if (!needed_tcp_pkt(skb, &tcph))
    return 1;
    g_packet_count++;
// Ingress of the server socket.
    if (ingress_connect(&tcph) || ingress_close_remote(&tcph))
    return 1;
    g_unexpected++;
    return 1;
    }
// Check the types of outgoing packets of a client socket to make sure they
// are consistent with the state of the client socket.
//
// The connection is closed by the client side.
//
    SEC("cgroup_skb/egress")
#[no_mangle]
pub unsafe extern "C" fn client_egress(skb: *mut __sk_buff) -> c_int {
    int client_egress(struct __sk_buff *skb)
    {
    struct tcphdr tcph;
    if (!needed_tcp_pkt(skb, &tcph))
    return 1;
    g_packet_count++;
// Egress of the server socket.
    if (egress_connect(&tcph) || egress_close_local(&tcph))
    return 1;
    g_unexpected++;
    return 1;
    }
// Check the types of incoming packets of a client socket to make sure they
// are consistent with the state of the client socket.
//
// The connection is closed by the client side.
//
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn client_ingress(skb: *mut __sk_buff) -> c_int {
    int client_ingress(struct __sk_buff *skb)
    {
    struct tcphdr tcph;
    if (!needed_tcp_pkt(skb, &tcph))
    return 1;
    g_packet_count++;
// Ingress of the server socket.
    if (ingress_connect(&tcph) || ingress_close_local(&tcph))
    return 1;
    g_unexpected++;
    return 1;
    }
