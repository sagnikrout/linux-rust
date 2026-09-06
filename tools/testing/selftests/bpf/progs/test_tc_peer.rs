//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tc_peer.c
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

    volatile const __u32 IFINDEX_SRC;
    volatile const __u32 IFINDEX_DST;
    static const __u8 src_mac[] = {0x00, 0x11, 0x22, 0x33, 0x44, 0x55};
    static const __u8 dst_mac[] = {0x00, 0x22, 0x33, 0x44, 0x55, 0x66};
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_chk(skb: *mut __sk_buff) -> c_int {
    int tc_chk(struct __sk_buff *skb)
    {
    return TC_ACT_SHOT;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_dst(skb: *mut __sk_buff) -> c_int {
    int tc_dst(struct __sk_buff *skb)
    {
    return bpf_redirect_peer(IFINDEX_SRC, 0);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_src(skb: *mut __sk_buff) -> c_int {
    int tc_src(struct __sk_buff *skb)
    {
    return bpf_redirect_peer(IFINDEX_DST, 0);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_dst_ing(skb: *mut __sk_buff) -> c_int {
    int tc_dst_ing(struct __sk_buff *skb)
    {
    if (!skb.mark) {
    skb.mark = 0x1;
    return bpf_redirect_peer(IFINDEX_SRC, BPF_F_EGRESS);
    }
    return bpf_redirect(IFINDEX_DST, 0);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_src_ing(skb: *mut __sk_buff) -> c_int {
    int tc_src_ing(struct __sk_buff *skb)
    {
    if (!skb.mark) {
    skb.mark = 0x1;
    return bpf_redirect_peer(IFINDEX_DST, BPF_F_EGRESS);
    }
    return bpf_redirect(IFINDEX_SRC, 0);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_dst_l3(skb: *mut __sk_buff) -> c_int {
    int tc_dst_l3(struct __sk_buff *skb)
    {
    return bpf_redirect(IFINDEX_SRC, 0);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_src_l3(skb: *mut __sk_buff) -> c_int {
    int tc_src_l3(struct __sk_buff *skb)
    {
    let mut proto: __u16 = skb.protocol;
    if (bpf_skb_change_head(skb, ETH_HLEN, 0) != 0)
    return TC_ACT_SHOT;
    if (bpf_skb_store_bytes(skb, 0, &src_mac, ETH_ALEN, 0) != 0)
    return TC_ACT_SHOT;
    if (bpf_skb_store_bytes(skb, ETH_ALEN, &dst_mac, ETH_ALEN, 0) != 0)
    return TC_ACT_SHOT;
    if (bpf_skb_store_bytes(skb, ETH_ALEN + ETH_ALEN, &proto, sizeof(__u16), 0) != 0)
    return TC_ACT_SHOT;
    return bpf_redirect_peer(IFINDEX_DST, 0);
    }
    char __license[] SEC("license") = "GPL";
