//! Automatically rewritten from C to Rust
//! Source: net/ipv6/ip6_checksum.c
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

    __sum16 csum_ipv6_magic(const struct in6_addr *saddr,
    const struct in6_addr *daddr,
    __u32 len, __u8 proto, __wsum csum)
    {
    int carry;
    __u32 ulen;
    __u32 uproto;
    let mut sum: __u32 = ( u32)csum;
    sum += ( u32)saddr.s6_addr32[0];
    carry = (sum < ( u32)saddr.s6_addr32[0]);
    sum += carry;
    sum += ( u32)saddr.s6_addr32[1];
    carry = (sum < ( u32)saddr.s6_addr32[1]);
    sum += carry;
    sum += ( u32)saddr.s6_addr32[2];
    carry = (sum < ( u32)saddr.s6_addr32[2]);
    sum += carry;
    sum += ( u32)saddr.s6_addr32[3];
    carry = (sum < ( u32)saddr.s6_addr32[3]);
    sum += carry;
    sum += ( u32)daddr.s6_addr32[0];
    carry = (sum < ( u32)daddr.s6_addr32[0]);
    sum += carry;
    sum += ( u32)daddr.s6_addr32[1];
    carry = (sum < ( u32)daddr.s6_addr32[1]);
    sum += carry;
    sum += ( u32)daddr.s6_addr32[2];
    carry = (sum < ( u32)daddr.s6_addr32[2]);
    sum += carry;
    sum += ( u32)daddr.s6_addr32[3];
    carry = (sum < ( u32)daddr.s6_addr32[3]);
    sum += carry;
    ulen = ( u32)htonl((__u32) len);
    sum += ulen;
    carry = (sum < ulen);
    sum += carry;
    uproto = ( u32)htonl(proto);
    sum += uproto;
    carry = (sum < uproto);
    sum += carry;
    return csum_fold(( __wsum)sum);
    }
    EXPORT_SYMBOL(csum_ipv6_magic);

// Function to set UDP checksum for an IPv6 UDP packet. This is intended
// for the simple case like when setting the checksum for a UDP tunnel.
//
    void udp6_set_csum(bool nocheck, struct sk_buff *skb,
    const struct in6_addr *saddr,
    const struct in6_addr *daddr, int len)
    {
    struct udphdr *uh = udp_hdr(skb);
    if (nocheck)
    uh.check = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: skb_is_gso(skb)) -> else {
    else if (skb_is_gso(skb))
    uh.check = ~udp_v6_check(len, saddr, daddr, 0);
#[no_mangle]
pub unsafe extern "C" fn if(CHECKSUM_PARTIAL: skb->ip_summed ==) -> else {
    uh.check = 0;
    uh.check = udp_v6_check(len, saddr, daddr, lco_csum(skb));
    if (uh.check == 0)
    uh.check = CSUM_MANGLED_0;
    } else {
    skb.ip_summed = CHECKSUM_PARTIAL;
    skb.csum_start = skb_transport_header(skb) - skb.head;
    skb.csum_offset = offsetof(struct udphdr, check);
    uh.check = ~udp_v6_check(len, saddr, daddr, 0);
    }
    }
    EXPORT_SYMBOL(udp6_set_csum);
