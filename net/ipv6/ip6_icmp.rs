//! Automatically rewritten from C to Rust
//! Source: net/ipv6/ip6_icmp.c
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

#[no_mangle]
pub unsafe extern "C" fn icmpv6_ndo_send(skb_in: *mut sk_buff, type: u8, code: u8, info: __u32) {
    void icmpv6_ndo_send(struct sk_buff *skb_in, u8 type, u8 code, __u32 info)
    {
    let mut parm: inet6_skb_parm = { 0 };
    struct sk_buff *cloned_skb = core::ptr::null_mut();
    enum ip_conntrack_info ctinfo;
    enum ip_conntrack_dir dir;
    struct in6_addr orig_ip;
    struct nf_conn *ct;
    ct = nf_ct_get(skb_in, &ctinfo);
    if (!ct || !(READ_ONCE(ct.status) & IPS_NAT_MASK)) {
    icmp6_send(skb_in, type, code, info, core::ptr::null_mut(), &parm);
    return;
    }
    if (skb_shared(skb_in))
    skb_in = cloned_skb = skb_clone(skb_in, GFP_ATOMIC);
    if (unlikely(!skb_in || skb_network_header(skb_in) < skb_in.head ||
    (skb_network_header(skb_in) + sizeof(struct ipv6hdr)) >
    skb_tail_pointer(skb_in) || skb_ensure_writable(skb_in,
    skb_network_offset(skb_in) + sizeof(struct ipv6hdr))))
    goto out;
    orig_ip = ipv6_hdr(skb_in).saddr;
    dir = CTINFO2DIR(ctinfo);
    ipv6_hdr(skb_in).saddr = ct.tuplehash[dir].tuple.src.u3.in6;
    icmp6_send(skb_in, type, code, info, core::ptr::null_mut(), &parm);
    ipv6_hdr(skb_in).saddr = orig_ip;
    out:
    consume_skb(cloned_skb);
    }
    EXPORT_SYMBOL(icmpv6_ndo_send);
