//! Automatically rewritten from C to Rust
//! Source: net/ipv4/xfrm4_output.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// xfrm4_output.c - Common IPsec encapsulation code for IPv4.
// Copyright (c) 2004 Herbert Xu <herbert@gondor.apana.org.au>
//

#[no_mangle]
unsafe extern "C" fn __xfrm4_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    static int __xfrm4_output(struct net *net, struct sock *sk, struct sk_buff *skb)
    {

    struct xfrm_state *x = skb_dst(skb).xfrm;
    if (!x) {
    IPCB(skb).flags |= IPSKB_REROUTED;
    return dst_output(net, sk, skb);
    }

    return xfrm_output(sk, skb);
    }
#[no_mangle]
pub unsafe extern "C" fn xfrm4_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    int xfrm4_output(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    return NF_HOOK_COND(NFPROTO_IPV4, NF_INET_POST_ROUTING,
    net, sk, skb, skb.dev, skb_dst_dev(skb),
    __xfrm4_output,
    !(IPCB(skb).flags & IPSKB_REROUTED));
    }
#[no_mangle]
pub unsafe extern "C" fn xfrm4_local_error(skb: *mut sk_buff, mtu: u32) {
    void xfrm4_local_error(struct sk_buff *skb, u32 mtu)
    {
    struct iphdr *hdr;
    hdr = skb.encapsulation ? inner_ip_hdr(skb) : ip_hdr(skb);
    ip_local_error(skb.sk, EMSGSIZE, hdr.daddr,
    inet_sk(skb.sk).inet_dport, mtu);
    }
