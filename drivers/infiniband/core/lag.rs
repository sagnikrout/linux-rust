//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/core/lag.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2020 Mellanox Technologies. All rights reserved.
//

    static struct sk_buff *rdma_build_skb(struct net_device *netdev,
    struct rdma_ah_attr *ah_attr,
    gfp_t flags)
    {
    struct ipv6hdr *ip6h;
    struct sk_buff *skb;
    struct ethhdr *eth;
    struct iphdr *iph;
    struct udphdr *uh;
    u8 smac[ETH_ALEN];
    bool is_ipv4;
    int hdr_len;
    is_ipv4 = ipv6_addr_v4mapped((struct in6_addr *)ah_attr.grh.dgid.raw);
    hdr_len = ETH_HLEN + sizeof(struct udphdr) + LL_RESERVED_SPACE(netdev);
    hdr_len += is_ipv4 ? sizeof(struct iphdr) : sizeof(struct ipv6hdr);
    skb = alloc_skb(hdr_len, flags);
    if (!skb)
    return core::ptr::null_mut();
    skb.dev = netdev;
    skb_reserve(skb, hdr_len);
    skb_push(skb, sizeof(struct udphdr));
    skb_reset_transport_header(skb);
    uh = udp_hdr(skb);
    uh.source =
    htons(rdma_flow_label_to_udp_sport(ah_attr.grh.flow_label));
    uh.dest = htons(ROCE_V2_UDP_DPORT);
    udp_set_len_short(uh, sizeof(struct udphdr));
    if (is_ipv4) {
    skb_push(skb, sizeof(struct iphdr));
    skb_reset_network_header(skb);
    iph = ip_hdr(skb);
    iph.frag_off = 0;
    iph.version = 4;
    iph.protocol = IPPROTO_UDP;
    iph.ihl = 0x5;
    iph.tot_len = htons(sizeof(struct udphdr) + sizeof(struct
    iphdr));
    memcpy(&iph.saddr, ah_attr.grh.sgid_attr.gid.raw + 12,
    sizeof(struct in_addr));
    memcpy(&iph.daddr, ah_attr.grh.dgid.raw + 12,
    sizeof(struct in_addr));
    } else {
    skb_push(skb, sizeof(struct ipv6hdr));
    skb_reset_network_header(skb);
    ip6h = ipv6_hdr(skb);
    ip6h.version = 6;
    ip6h.nexthdr = IPPROTO_UDP;
    memcpy(&ip6h.flow_lbl, &ah_attr.grh.flow_label,
    sizeof(*ip6h.flow_lbl));
    memcpy(&ip6h.saddr, ah_attr.grh.sgid_attr.gid.raw,
    sizeof(struct in6_addr));
    memcpy(&ip6h.daddr, ah_attr.grh.dgid.raw,
    sizeof(struct in6_addr));
    }
    skb_push(skb, sizeof(struct ethhdr));
    skb_reset_mac_header(skb);
    eth = eth_hdr(skb);
    skb.protocol = eth.h_proto = htons(is_ipv4 ? ETH_P_IP : ETH_P_IPV6);
    rdma_read_gid_l2_fields(ah_attr.grh.sgid_attr, core::ptr::null_mut(), smac);
    memcpy(eth.h_source, smac, ETH_ALEN);
    memcpy(eth.h_dest, ah_attr.roce.dmac, ETH_ALEN);
    return skb;
    }
    static struct net_device *rdma_get_xmit_slave_udp(struct ib_device *device,
    struct net_device *master,
    struct rdma_ah_attr *ah_attr,
    gfp_t flags)
    {
    struct net_device *slave;
    struct sk_buff *skb;
    skb = rdma_build_skb(master, ah_attr, flags);
    if (!skb)
    return ERR_PTR(-ENOMEM);
    rcu_read_lock();
    slave = netdev_get_xmit_slave(master, skb,
    !!(device.lag_flags &
    RDMA_LAG_FLAGS_HASH_ALL_SLAVES));
    dev_hold(slave);
    rcu_read_unlock();
    kfree_skb(skb);
    return slave;
    }
#[no_mangle]
pub unsafe extern "C" fn rdma_lag_put_ah_roce_slave(xmit_slave: *mut net_device) {
    void rdma_lag_put_ah_roce_slave(struct net_device *xmit_slave)
    {
    dev_put(xmit_slave);
    }
    struct net_device *rdma_lag_get_ah_roce_slave(struct ib_device *device,
    struct rdma_ah_attr *ah_attr,
    gfp_t flags)
    {
    struct net_device *slave = core::ptr::null_mut();
    struct net_device *master;
    if (!(ah_attr.type == RDMA_AH_ATTR_TYPE_ROCE &&
    ah_attr.grh.sgid_attr.gid_type == IB_GID_TYPE_ROCE_UDP_ENCAP &&
    ah_attr.grh.flow_label))
    return core::ptr::null_mut();
    rcu_read_lock();
    master = rdma_read_gid_attr_ndev_rcu(ah_attr.grh.sgid_attr);
    if (IS_ERR(master)) {
    rcu_read_unlock();
    return master;
    }
    dev_hold(master);
    rcu_read_unlock();
    if (!netif_is_bond_master(master))
    goto put;
    slave = rdma_get_xmit_slave_udp(device, master, ah_attr, flags);
    put:
    dev_put(master);
    return slave;
    }
