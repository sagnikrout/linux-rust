//! Automatically rewritten from C to Rust
//! Source: net/ipv4/ip_forward.c
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
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// The IP forwarding functionality.
//
// Authors:	see ip.c
//
// Fixes:
// Many		:	Split from ip.c , see ip_input.c for
// history.
// Dave Gregorich	:	NULL ip_rt_put fix for multicast
// routing.
// Jos Vos		:	Add call_out_firewall before sending,
// use output device for accounting.
// Jos Vos		:	Call forward firewall after routing
// (always use output device).
// Mike McLagan	:	Routing by source
//

#[no_mangle]
unsafe extern "C" fn ip_exceeds_mtu(skb: *const sk_buff, mtu: c_uint) -> bool {
    static bool ip_exceeds_mtu(const struct sk_buff *skb, unsigned int mtu)
    {
    if (skb.len <= mtu)
    return false;
    if (unlikely((ip_hdr(skb).frag_off & htons(IP_DF)) == 0))
    return false;
// original fragment exceeds mtu and DF is set
    if (unlikely(IPCB(skb).frag_max_size > mtu))
    return true;
    if (skb.ignore_df)
    return false;
    if (skb_is_gso(skb) && skb_gso_validate_network_len(skb, mtu))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ip_forward_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    static int ip_forward_finish(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    struct ip_options *opt	= &(IPCB(skb).opt);

    if (skb.offload_l3_fwd_mark) {
    consume_skb(skb);
    return 0;
    }

    if (unlikely(opt.optlen))
    ip_forward_options(skb);
    skb_clear_tstamp(skb);
    return dst_output(net, sk, skb);
    }
#[no_mangle]
pub unsafe extern "C" fn ip_forward(skb: *mut sk_buff) -> c_int {
    int ip_forward(struct sk_buff *skb)
    {
    u32 mtu;
    struct iphdr *iph;	/* Our header */
    struct rtable *rt;	/* Route we use */
    struct ip_options *opt	= &(IPCB(skb).opt);
    struct net *net;
    SKB_DR(reason);
// that should never happen
    if (skb.pkt_type != PACKET_HOST)
    goto drop;
    if (unlikely(skb.sk))
    goto drop;
    if (skb_warn_if_lro(skb))
    goto drop;
    if (!xfrm4_policy_check(core::ptr::null_mut(), XFRM_POLICY_FWD, skb)) {
    SKB_DR_SET(reason, XFRM_POLICY);
    goto drop;
    }
    if (IPCB(skb).opt.router_alert && ip_call_ra_chain(skb))
    return NET_RX_SUCCESS;
    skb_forward_csum(skb);
    net = dev_net(skb.dev);
//
// According to the RFC, we must first decrease the TTL field. If
// that reaches zero, we must reply an ICMP control message telling
// that the packet's lifetime expired.
//
    if (ip_hdr(skb).ttl <= 1)
    goto too_many_hops;
    if (!xfrm4_route_forward(skb)) {
    SKB_DR_SET(reason, XFRM_POLICY);
    goto drop;
    }
    rt = skb_rtable(skb);
    if (opt.is_strictroute && rt.rt_uses_gateway)
    goto sr_failed;
    __IP_INC_STATS(net, IPSTATS_MIB_OUTFORWDATAGRAMS);
    IPCB(skb).flags |= IPSKB_FORWARDED;
    mtu = ip_dst_mtu_maybe_forward(&rt.dst, true);
    if (ip_exceeds_mtu(skb, mtu)) {
    IP_INC_STATS(net, IPSTATS_MIB_FRAGFAILS);
    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_FRAG_NEEDED,
    htonl(mtu));
    SKB_DR_SET(reason, PKT_TOO_BIG);
    goto drop;
    }
// We are about to mangle packet. Copy it!
    if (skb_cow(skb, LL_RESERVED_SPACE(rt.dst.dev)+rt.dst.header_len))
    goto drop;
    iph = ip_hdr(skb);
// Decrease ttl after skb cow done
    ip_decrease_ttl(iph);
//
// We now generate an ICMP HOST REDIRECT giving the route
// we calculated.
//
    if (IPCB(skb).flags & IPSKB_DOREDIRECT && !opt.srr &&
    !skb_sec_path(skb))
    ip_rt_send_redirect(skb);
    if (READ_ONCE(net.ipv4.sysctl_ip_fwd_update_priority))
    skb.priority = rt_tos2priority(iph.tos);
    return NF_HOOK(NFPROTO_IPV4, NF_INET_FORWARD,
    net, core::ptr::null_mut(), skb, skb.dev, rt.dst.dev,
    ip_forward_finish);
    sr_failed:
//
// Strict routing permits no gatewaying
//
    icmp_send(skb, ICMP_DEST_UNREACH, ICMP_SR_FAILED, 0);
    goto drop;
    too_many_hops:
// Tell the sender its packet died...
    __IP_INC_STATS(net, IPSTATS_MIB_INHDRERRORS);
    icmp_send(skb, ICMP_TIME_EXCEEDED, ICMP_EXC_TTL, 0);
    SKB_DR_SET(reason, IP_INHDR);
    drop:
    kfree_skb_reason(skb, reason);
    return NET_RX_DROP;
    }
