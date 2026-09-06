//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_dup_netdev.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2015 Pablo Neira Ayuso <pablo@netfilter.org>
//

    static void nf_do_netdev_egress(struct sk_buff *skb, struct net_device *dev,
    enum nf_dev_hooks hook)
    {
    if (hook == NF_NETDEV_INGRESS && skb_mac_header_was_set(skb)) {
    if (skb_cow_head(skb, skb.mac_len))
    goto err;
    skb_push(skb, skb.mac_len);
    }
    skb.dev = dev;
    skb_clear_tstamp(skb);
    local_bh_disable();
    if (nf_dev_xmit_recursion()) {
    local_bh_enable();
    goto err;
    }
    nf_dev_xmit_recursion_inc();
    dev_queue_xmit(skb);
    nf_dev_xmit_recursion_dec();
    local_bh_enable();
    return;
    err:
    kfree_skb(skb);
    }
#[no_mangle]
pub unsafe extern "C" fn nf_fwd_netdev_egress(pkt: *const nft_pktinfo, oif: c_int) {
    void nf_fwd_netdev_egress(const struct nft_pktinfo *pkt, int oif)
    {
    struct net_device *dev;
    dev = dev_get_by_index_rcu(nft_net(pkt), oif);
    if (!dev) {
    kfree_skb(pkt.skb);
    return;
    }
    nf_do_netdev_egress(pkt.skb, dev, nft_hook(pkt));
    }
    EXPORT_SYMBOL_GPL(nf_fwd_netdev_egress);
#[no_mangle]
pub unsafe extern "C" fn nf_dup_netdev_egress(pkt: *const nft_pktinfo, oif: c_int) {
    void nf_dup_netdev_egress(const struct nft_pktinfo *pkt, int oif)
    {
    struct net_device *dev;
    struct sk_buff *skb;
    dev = dev_get_by_index_rcu(nft_net(pkt), oif);
    if (dev == core::ptr::null_mut())
    return;
    skb = skb_clone(pkt.skb, GFP_ATOMIC);
    if (skb)
    nf_do_netdev_egress(skb, dev, nft_hook(pkt));
    }
    EXPORT_SYMBOL_GPL(nf_dup_netdev_egress);
    int nft_fwd_dup_netdev_offload(struct nft_offload_ctx *ctx,
    struct nft_flow_rule *flow,
    enum flow_action_id id, int oif)
    {
    struct flow_action_entry *entry;
    struct net_device *dev;
    dev = dev_get_by_index(ctx.net, oif);
    if (!dev)
    return -EOPNOTSUPP;
    entry = nft_flow_action_entry_next(ctx, flow);
    if (!entry) {
    dev_put(dev);
    return -E2BIG;
    }
    entry.id = id;
// nft_flow_rule_destroy() releases the reference on this device.
    entry.dev = dev;
    return 0;
    }
    EXPORT_SYMBOL_GPL(nft_fwd_dup_netdev_offload);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
    MODULE_DESCRIPTION("Netfilter packet duplication support");
