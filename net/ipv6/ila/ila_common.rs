//! Automatically rewritten from C to Rust
//! Source: net/ipv6/ila/ila_common.c
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
pub unsafe extern "C" fn ila_init_saved_csum(p: *mut ila_params) {
    void ila_init_saved_csum(struct ila_params *p)
    {
    if (!p.locator_match.v64)
    return;
    p.csum_diff = compute_csum_diff8(
    (__be32 *)&p.locator,
    (__be32 *)&p.locator_match);
    }
#[no_mangle]
unsafe extern "C" fn get_csum_diff_iaddr(iaddr: *mut ila_addr, p: *mut ila_params) -> __wsum {
    static __wsum get_csum_diff_iaddr(struct ila_addr *iaddr, struct ila_params *p)
    {
    if (p.locator_match.v64)
    return p.csum_diff;
    else
    return compute_csum_diff8((__be32 *)&p.locator,
    (__be32 *)&iaddr.loc);
    }
#[no_mangle]
unsafe extern "C" fn get_csum_diff(ip6h: *mut ipv6hdr, p: *mut ila_params) -> __wsum {
    static __wsum get_csum_diff(struct ipv6hdr *ip6h, struct ila_params *p)
    {
    return get_csum_diff_iaddr(ila_a2i(&ip6h.daddr), p);
    }
    static void ila_csum_do_neutral_fmt(struct ila_addr *iaddr,
    struct ila_params *p)
    {
    __sum16 *adjust = ( __sum16 *)&iaddr.ident.v16[3];
    __wsum diff, fval;
    diff = get_csum_diff_iaddr(iaddr, p);
    fval = ( __wsum)(ila_csum_neutral_set(iaddr.ident) ?
    CSUM_NEUTRAL_FLAG : ~CSUM_NEUTRAL_FLAG);
    diff = csum_add(diff, fval);
// adjust = ~csum_fold(csum_add(diff, csum_unfold(*adjust)));
// Flip the csum-neutral bit. Either we are doing a SIR->ILA
// translation with ILA_CSUM_NEUTRAL_MAP as the csum_method
// and the C-bit is not set, or we are doing an ILA-SIR
// tranlsation and the C-bit is set.
//
    iaddr.ident.csum_neutral ^= 1;
    }
    static void ila_csum_do_neutral_nofmt(struct ila_addr *iaddr,
    struct ila_params *p)
    {
    __sum16 *adjust = ( __sum16 *)&iaddr.ident.v16[3];
    __wsum diff;
    diff = get_csum_diff_iaddr(iaddr, p);
// adjust = ~csum_fold(csum_add(diff, csum_unfold(*adjust)));
    }
    static void ila_csum_adjust_transport(struct sk_buff *skb,
    struct ila_params *p)
    {
    let mut nhoff: usize = sizeof(struct ipv6hdr);
    struct ipv6hdr *ip6h = ipv6_hdr(skb);
    __wsum diff;
    switch (ip6h.nexthdr) {
    case NEXTHDR_TCP:
    if (likely(pskb_may_pull(skb, nhoff + sizeof(struct tcphdr)))) {
    struct tcphdr *th = (struct tcphdr *)
    (skb_network_header(skb) + nhoff);
    ip6h = ipv6_hdr(skb);
    diff = get_csum_diff(ip6h, p);
    inet_proto_csum_replace_by_diff(&th.check, skb,
    diff, true, true);
    }
    break;
    case NEXTHDR_UDP:
    if (likely(pskb_may_pull(skb, nhoff + sizeof(struct udphdr)))) {
    struct udphdr *uh = (struct udphdr *)
    (skb_network_header(skb) + nhoff);
    if (uh.check || skb.ip_summed == CHECKSUM_PARTIAL) {
    ip6h = ipv6_hdr(skb);
    diff = get_csum_diff(ip6h, p);
    inet_proto_csum_replace_by_diff(&uh.check, skb,
    diff, true, true);
    if (!uh.check)
    uh.check = CSUM_MANGLED_0;
    }
    }
    break;
    case NEXTHDR_ICMP:
    if (likely(pskb_may_pull(skb,
    nhoff + sizeof(struct icmp6hdr)))) {
    struct icmp6hdr *ih = (struct icmp6hdr *)
    (skb_network_header(skb) + nhoff);
    ip6h = ipv6_hdr(skb);
    diff = get_csum_diff(ip6h, p);
    inet_proto_csum_replace_by_diff(&ih.icmp6_cksum, skb,
    diff, true, true);
    }
    break;
    }
    }
    void ila_update_ipv6_locator(struct sk_buff *skb, struct ila_params *p,
    bool sir2ila)
    {
    struct ipv6hdr *ip6h = ipv6_hdr(skb);
    struct ila_addr *iaddr = ila_a2i(&ip6h.daddr);
    switch (p.csum_mode) {
    case ILA_CSUM_ADJUST_TRANSPORT:
    ila_csum_adjust_transport(skb, p);
//
// ila_csum_adjust_transport() calls pskb_may_pull(), which can
// reallocate the skb head and leave ip6h (and the iaddr derived
// from it) dangling; reload both before the write below.  The
// other csum modes do not pull, so their cached pointers stay
// valid.
//
    ip6h = ipv6_hdr(skb);
    iaddr = ila_a2i(&ip6h.daddr);
    break;
    case ILA_CSUM_NEUTRAL_MAP:
    if (sir2ila) {
    if (WARN_ON(ila_csum_neutral_set(iaddr.ident))) {
// Checksum flag should never be
// set in a formatted SIR address.
//
    break;
    }
    } else if (!ila_csum_neutral_set(iaddr.ident)) {
// ILA to SIR translation and C-bit isn't
// set so we're good.
//
    break;
    }
    ila_csum_do_neutral_fmt(iaddr, p);
    break;
    case ILA_CSUM_NEUTRAL_MAP_AUTO:
    ila_csum_do_neutral_nofmt(iaddr, p);
    break;
    case ILA_CSUM_NO_ACTION:
    break;
    }
// Now change destination address
    iaddr.loc = p.locator;
    }
