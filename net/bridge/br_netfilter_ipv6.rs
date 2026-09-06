//! Automatically rewritten from C to Rust
//! Source: net/bridge/br_netfilter_ipv6.c
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
// Handle firewalling
// Linux ethernet bridge
//
// Authors:
// Lennert Buytenhek		<buytenh@gnu.org>
// Bart De Schuymer		<bdschuym@pandora.be>
//
// Lennert dedicates this file to Kerstin Wurdinger.
//

#[no_mangle]
pub unsafe extern "C" fn br_validate_ipv6(net: *mut net, skb: *mut sk_buff) -> c_int {
    int br_validate_ipv6(struct net *net, struct sk_buff *skb)
    {
    const struct ipv6hdr *hdr;
    struct inet6_dev *idev = __in6_dev_get(skb.dev);
    u32 pkt_len;
    let mut ip6h_len: u8 = sizeof(struct ipv6hdr);
    if (!pskb_may_pull(skb, ip6h_len))
    goto inhdr_error;
    if (skb.len < ip6h_len)
    goto drop;
    hdr = ipv6_hdr(skb);
    if (hdr.version != 6)
    goto inhdr_error;
    pkt_len = ipv6_payload_len(skb, hdr);
    if (hdr.nexthdr == NEXTHDR_HOP && nf_ip6_check_hbh_len(skb, &pkt_len))
    goto drop;
    if (pkt_len + ip6h_len > skb.len) {
    __IP6_INC_STATS(net, idev,
    IPSTATS_MIB_INTRUNCATEDPKTS);
    goto drop;
    }
    if (pskb_trim_rcsum(skb, pkt_len + ip6h_len)) {
    __IP6_INC_STATS(net, idev,
    IPSTATS_MIB_INDISCARDS);
    goto drop;
    }
    memset(IP6CB(skb), 0, sizeof(struct inet6_skb_parm));
// No IP options in IPv6 header; however it should be
// checked if some next headers need special treatment
//
    return 0;
    inhdr_error:
    __IP6_INC_STATS(net, idev, IPSTATS_MIB_INHDRERRORS);
    drop:
    return -1;
    }
    static inline bool
    br_nf_ipv6_daddr_was_changed(const struct sk_buff *skb,
    const struct nf_bridge_info *nf_bridge)
    {
    return memcmp(&nf_bridge.ipv6_daddr, &ipv6_hdr(skb).daddr,
    sizeof(ipv6_hdr(skb).daddr)) != 0;
    }
// PF_BRIDGE/PRE_ROUTING: Undo the changes made for ip6tables
// PREROUTING and continue the bridge PRE_ROUTING hook. See comment
// for br_nf_pre_routing_finish(), same logic is used here.
//
#[no_mangle]
unsafe extern "C" fn br_nf_pre_routing_finish_ipv6(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    static int br_nf_pre_routing_finish_ipv6(struct net *net, struct sock *sk, struct sk_buff *skb)
    {
    struct nf_bridge_info *nf_bridge = nf_bridge_info_get(skb);
    struct rtable *rt;
    struct net_device *dev = skb.dev, *br_indev;
    br_indev = nf_bridge_get_physindev(skb, net);
    if (!br_indev) {
    kfree_skb(skb);
    return 0;
    }
    nf_bridge.frag_max_size = IP6CB(skb).frag_max_size;
    if (nf_bridge.pkt_otherhost) {
    skb.pkt_type = PACKET_OTHERHOST;
    nf_bridge.pkt_otherhost = false;
    }
    nf_bridge.in_prerouting = 0;
    if (br_nf_ipv6_daddr_was_changed(skb, nf_bridge)) {
    skb_dst_drop(skb);
    ip6_route_input(skb);
    if (skb_dst(skb).error) {
    kfree_skb(skb);
    return 0;
    }
    if (skb_dst(skb).dev == dev) {
    skb.dev = br_indev;
    nf_bridge_update_protocol(skb);
    nf_bridge_push_encap_header(skb);
    br_nf_hook_thresh(NF_BR_PRE_ROUTING,
    net, sk, skb, skb.dev, core::ptr::null_mut(),
    br_nf_pre_routing_finish_bridge);
    return 0;
    }
    ether_addr_copy(eth_hdr(skb).h_dest, dev.dev_addr);
    skb.pkt_type = PACKET_HOST;
    } else {
    rt = bridge_parent_rtable(br_indev);
    if (!rt) {
    kfree_skb(skb);
    return 0;
    }
    skb_dst_drop(skb);
    skb_dst_set_noref(skb, &rt.dst);
    }
    skb.dev = br_indev;
    nf_bridge_update_protocol(skb);
    nf_bridge_push_encap_header(skb);
    br_nf_hook_thresh(NF_BR_PRE_ROUTING, net, sk, skb,
    skb.dev, core::ptr::null_mut(), br_handle_frame_finish);
    return 0;
    }
// Replicate the checks that IPv6 does on packet reception and pass the packet
// to ip6tables.
//
    unsigned int br_nf_pre_routing_ipv6(void *priv,
    struct sk_buff *skb,
    const struct nf_hook_state *state)
    {
    struct nf_bridge_info *nf_bridge;
    if (br_validate_ipv6(state.net, skb))
    return NF_DROP_REASON(skb, SKB_DROP_REASON_IP_INHDR, 0);
    nf_bridge = nf_bridge_alloc(skb);
    if (!nf_bridge)
    return NF_DROP_REASON(skb, SKB_DROP_REASON_NOMEM, 0);
    if (!setup_pre_routing(skb, state.net))
    return NF_DROP_REASON(skb, SKB_DROP_REASON_DEV_READY, 0);
    nf_bridge = nf_bridge_info_get(skb);
    nf_bridge.ipv6_daddr = ipv6_hdr(skb).daddr;
    skb.protocol = htons(ETH_P_IPV6);
    skb.transport_header = skb.network_header + sizeof(struct ipv6hdr);
    NF_HOOK(NFPROTO_IPV6, NF_INET_PRE_ROUTING, state.net, state.sk, skb,
    skb.dev, core::ptr::null_mut(),
    br_nf_pre_routing_finish_ipv6);
    return NF_STOLEN;
    }
