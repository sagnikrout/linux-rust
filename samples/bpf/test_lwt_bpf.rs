//! Automatically rewritten from C to Rust
//! Source: samples/bpf/test_lwt_bpf.c
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


// Copyright (c) 2016 Thomas Graf <tgraf@tgraf.ch>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//

    ({							\
    char ____fmt[] = fmt;				\
    bpf_trace_printk(____fmt, sizeof(____fmt),	\

    })
pub const CB_MAGIC: c_int = 1234;
// Test: Pass all packets through
    SEC("nop")
#[no_mangle]
pub unsafe extern "C" fn do_nop(skb: *mut __sk_buff) -> c_int {
    int do_nop(struct __sk_buff *skb)
    {
    return BPF_OK;
    }
// Test: Verify context information can be accessed
    SEC("test_ctx")
#[no_mangle]
pub unsafe extern "C" fn do_test_ctx(skb: *mut __sk_buff) -> c_int {
    int do_test_ctx(struct __sk_buff *skb)
    {
    skb.cb[0] = CB_MAGIC;
    printk("len %d hash %d protocol %d", skb.len, skb.hash,
    skb.protocol);
    printk("cb %d ingress_ifindex %d ifindex %d", skb.cb[0],
    skb.ingress_ifindex, skb.ifindex);
    return BPF_OK;
    }
// Test: Ensure skb->cb[] buffer is cleared
    SEC("test_cb")
#[no_mangle]
pub unsafe extern "C" fn do_test_cb(skb: *mut __sk_buff) -> c_int {
    int do_test_cb(struct __sk_buff *skb)
    {
    printk("cb0: %x cb1: %x cb2: %x", skb.cb[0], skb.cb[1],
    skb.cb[2]);
    printk("cb3: %x cb4: %x", skb.cb[3], skb.cb[4]);
    return BPF_OK;
    }
// Test: Verify skb data can be read
    SEC("test_data")
#[no_mangle]
pub unsafe extern "C" fn do_test_data(skb: *mut __sk_buff) -> c_int {
    int do_test_data(struct __sk_buff *skb)
    {
    void *data = (void *)(long)skb.data;
    void *data_end = (void *)(long)skb.data_end;
    struct iphdr *iph = data;
    if (data + sizeof(*iph) > data_end) {
    printk("packet truncated");
    return BPF_DROP;
    }
    printk("src: %x dst: %x", iph.saddr, iph.daddr);
    return BPF_OK;
    }

pub const IS_PSEUDO: c_uint = 0x10;
    static inline int rewrite(struct __sk_buff *skb, uint32_t old_ip,
    uint32_t new_ip, int rw_daddr)
    {
    int ret, off = 0, flags = IS_PSEUDO;
    uint8_t proto;
    ret = bpf_skb_load_bytes(skb, IP_PROTO_OFF, &proto, 1);
    if (ret < 0) {
    printk("bpf_l4_csum_replace failed: %d", ret);
    return BPF_DROP;
    }
    switch (proto) {
    case IPPROTO_TCP:
    off = TCP_CSUM_OFF;
    break;
    case IPPROTO_UDP:
    off = UDP_CSUM_OFF;
    flags |= BPF_F_MARK_MANGLED_0;
    break;
    case IPPROTO_ICMPV6:
    off = offsetof(struct icmp6hdr, icmp6_cksum);
    break;
    }
    if (off) {
    ret = bpf_l4_csum_replace(skb, off, old_ip, new_ip,
    flags | sizeof(new_ip));
    if (ret < 0) {
    printk("bpf_l4_csum_replace failed: %d");
    return BPF_DROP;
    }
    }
    ret = bpf_l3_csum_replace(skb, IP_CSUM_OFF, old_ip, new_ip, sizeof(new_ip));
    if (ret < 0) {
    printk("bpf_l3_csum_replace failed: %d", ret);
    return BPF_DROP;
    }
    if (rw_daddr)
    ret = bpf_skb_store_bytes(skb, IP_DST_OFF, &new_ip, sizeof(new_ip), 0);
    else
    ret = bpf_skb_store_bytes(skb, IP_SRC_OFF, &new_ip, sizeof(new_ip), 0);
    if (ret < 0) {
    printk("bpf_skb_store_bytes() failed: %d", ret);
    return BPF_DROP;
    }
    return BPF_OK;
    }
// Test: Verify skb data can be modified
    SEC("test_rewrite")
#[no_mangle]
pub unsafe extern "C" fn do_test_rewrite(skb: *mut __sk_buff) -> c_int {
    int do_test_rewrite(struct __sk_buff *skb)
    {
    uint32_t old_ip, new_ip = 0x3fea8c0;
    int ret;
    ret = bpf_skb_load_bytes(skb, IP_DST_OFF, &old_ip, 4);
    if (ret < 0) {
    printk("bpf_skb_load_bytes failed: %d", ret);
    return BPF_DROP;
    }
    if (old_ip == 0x2fea8c0) {
    printk("out: rewriting from %x to %x", old_ip, new_ip);
    return rewrite(skb, old_ip, new_ip, 1);
    }
    return BPF_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn __do_push_ll_and_redirect(skb: *mut __sk_buff) -> c_int {
    static inline int __do_push_ll_and_redirect(struct __sk_buff *skb)
    {
    let mut smac: u64 = SRC_MAC, dmac = DST_MAC;
    int ret, ifindex = DST_IFINDEX;
    struct ethhdr ehdr;
    ret = bpf_skb_change_head(skb, 14, 0);
    if (ret < 0) {
    printk("skb_change_head() failed: %d", ret);
    }
    ehdr.h_proto = bpf_htons(ETH_P_IP);
    memcpy(&ehdr.h_source, &smac, 6);
    memcpy(&ehdr.h_dest, &dmac, 6);
    ret = bpf_skb_store_bytes(skb, 0, &ehdr, sizeof(ehdr), 0);
    if (ret < 0) {
    printk("skb_store_bytes() failed: %d", ret);
    return BPF_DROP;
    }
    return bpf_redirect(ifindex, 0);
    }
    SEC("push_ll_and_redirect_silent")
#[no_mangle]
pub unsafe extern "C" fn do_push_ll_and_redirect_silent(skb: *mut __sk_buff) -> c_int {
    int do_push_ll_and_redirect_silent(struct __sk_buff *skb)
    {
    return __do_push_ll_and_redirect(skb);
    }
    SEC("push_ll_and_redirect")
#[no_mangle]
pub unsafe extern "C" fn do_push_ll_and_redirect(skb: *mut __sk_buff) -> c_int {
    int do_push_ll_and_redirect(struct __sk_buff *skb)
    {
    int ret, ifindex = DST_IFINDEX;
    ret = __do_push_ll_and_redirect(skb);
    if (ret >= 0)
    printk("redirected to %d", ifindex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __fill_garbage(skb: *mut __sk_buff) {
    static inline void __fill_garbage(struct __sk_buff *skb)
    {
    let mut f: u64 = 0xFFFFFFFFFFFFFFFF;
    bpf_skb_store_bytes(skb, 0, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 8, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 16, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 24, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 32, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 40, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 48, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 56, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 64, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 72, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 80, &f, sizeof(f), 0);
    bpf_skb_store_bytes(skb, 88, &f, sizeof(f), 0);
    }
    SEC("fill_garbage")
#[no_mangle]
pub unsafe extern "C" fn do_fill_garbage(skb: *mut __sk_buff) -> c_int {
    int do_fill_garbage(struct __sk_buff *skb)
    {
    __fill_garbage(skb);
    printk("Set initial 96 bytes of header to FF");
    return BPF_OK;
    }
    SEC("fill_garbage_and_redirect")
#[no_mangle]
pub unsafe extern "C" fn do_fill_garbage_and_redirect(skb: *mut __sk_buff) -> c_int {
    int do_fill_garbage_and_redirect(struct __sk_buff *skb)
    {
    let mut ifindex: c_int = DST_IFINDEX;
    __fill_garbage(skb);
    printk("redirected to %d", ifindex);
    return bpf_redirect(ifindex, 0);
    }
// Drop all packets
    SEC("drop_all")
#[no_mangle]
pub unsafe extern "C" fn do_drop_all(skb: *mut __sk_buff) -> c_int {
    int do_drop_all(struct __sk_buff *skb)
    {
    printk("dropping with: %d", BPF_DROP);
    return BPF_DROP;
    }
    char _license[] SEC("license") = "GPL";
