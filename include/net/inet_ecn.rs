//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/inet_ecn.h
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

//
// RFC 3168 9.1.1
// The full-functionality option for ECN encapsulation is to copy the
// ECN codepoint of the inside header to the outside header on
// encapsulation if the inside header is not-ECT or ECT, and to set the
// ECN codepoint of the outside header to ECT(0) if the ECN codepoint of
// the inside header is CE.
//
// Apply either ECT(0) or ECT(1)
// Mask the complete byte in case the connection alternates between
// ECT(0) and ECT(1).
//

//
// After the last operation we have (in binary):
// INET_ECN_NOT_ECT => 01
// INET_ECN_ECT_1   => 10
// INET_ECN_ECT_0   => 11
// INET_ECN_CE      => 00
//
// The following gives us:
// INET_ECN_ECT_1 => check += htons(0xFFFD)
// INET_ECN_ECT_0 => check += htons(0xFFFE)
//
// Note:
// IP_ECN_set_ce() has to tweak IPV4 checksum when setting CE,
// meaning both changes have no effect on skb->csum if/when CHECKSUM_COMPLETE
// In IPv6 case, no checksum compensates the change in IPv6 header,
// so we have to update skb->csum.
//
// (__be32 *)iph = to;
extern "C" {
    pub fn IP_ECN_set_ce(_arg: ip_hdr(skb)) -> return;
}
extern "C" {
    pub fn IP6_ECN_set_ce(_arg: skb, _arg: ipv6_hdr(skb)) -> return;
}
extern "C" {
    pub fn ipv4_get_dsfield(_arg: ip_hdr(skb)) -> return;
}
extern "C" {
    pub fn ipv6_get_dsfield(_arg: ipv6_hdr(skb)) -> return;
}
extern "C" {
    pub fn IP_ECN_set_ect1(_arg: ip_hdr(skb)) -> return;
}
extern "C" {
    pub fn IP6_ECN_set_ect1(_arg: skb, _arg: ipv6_hdr(skb)) -> return;
}
//
// RFC 6040 4.2
// To decapsulate the inner header at the tunnel egress, a compliant
// tunnel egress MUST set the outgoing ECN field to the codepoint at the
// intersection of the appropriate arriving inner header (row) and outer
// header (column) in Figure 4
//
// +---------+------------------------------------------------+
// |Arriving |            Arriving Outer Header               |
// |   Inner +---------+------------+------------+------------+
// |  Header | Not-ECT | ECT(0)     | ECT(1)     |     CE     |
// +---------+---------+------------+------------+------------+
// | Not-ECT | Not-ECT |Not-ECT(!!!)|Not-ECT(!!!)| <drop>(!!!)|
// |  ECT(0) |  ECT(0) | ECT(0)     | ECT(1)     |     CE     |
// |  ECT(1) |  ECT(1) | ECT(1) (!) | ECT(1)     |     CE     |
// |    CE   |      CE |     CE     |     CE(!!!)|     CE     |
// +---------+---------+------------+------------+------------+
//
// Figure 4: New IP in IP Decapsulation Behaviour
//
// returns 0 on success
// 1 if something is broken and should be logged (!!! above)
// 2 if packet should be dropped
//
// set_ce = INET_ECN_is_ce(outer);
extern "C" {
    pub fn INET_ECN_decapsulate(_arg: skb, _arg: oiph->tos, _arg: inner) -> return;
}
extern "C" {
    pub fn INET_ECN_decapsulate(_arg: skb, _arg: ipv6_get_dsfield(oipv6h), _arg: inner) -> return;
}
