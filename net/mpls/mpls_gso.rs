//! Automatically rewritten from C to Rust
//! Source: net/mpls/mpls_gso.c
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
// MPLS GSO Support
//
// Authors: Simon Horman (horms@verge.net.au)
//
// Based on: GSO portions of net/ipv4/gre.c
//

    static struct sk_buff *mpls_gso_segment(struct sk_buff *skb,
    netdev_features_t features)
    {
    struct sk_buff *segs = ERR_PTR(-EINVAL);
    let mut mac_offset: u16 = skb.mac_header;
    netdev_features_t mpls_features;
    let mut mac_len: u16 = skb.mac_len;
    __be16 mpls_protocol;
    unsigned int mpls_hlen;
    if (!skb_inner_network_header_was_set(skb))
    goto out;
    skb_reset_network_header(skb);
    mpls_hlen = skb_inner_network_header(skb) - skb_network_header(skb);
    if (unlikely(!mpls_hlen || mpls_hlen % MPLS_HLEN))
    goto out;
    if (unlikely(!pskb_may_pull(skb, mpls_hlen)))
    goto out;
// Setup inner SKB.
    mpls_protocol = skb.protocol;
    skb.protocol = skb.inner_protocol;
    __skb_pull(skb, mpls_hlen);
    skb.mac_len = 0;
    skb_reset_mac_header(skb);
// Segment inner packet.
    mpls_features = skb.dev.mpls_features & features;
    segs = skb_mac_gso_segment(skb, mpls_features);
    if (IS_ERR_OR_NULL(segs)) {
    skb_gso_error_unwind(skb, mpls_protocol, mpls_hlen, mac_offset,
    mac_len);
    goto out;
    }
    skb = segs;
    mpls_hlen += mac_len;
    do {
    skb.mac_len = mac_len;
    skb.protocol = mpls_protocol;
    skb_reset_inner_network_header(skb);
    __skb_push(skb, mpls_hlen);
    skb_reset_mac_header(skb);
    skb_set_network_header(skb, mac_len);
    } while ((skb = skb.next));
    out:
    return segs;
    }
    static struct packet_offload mpls_mc_offload __read_mostly = {
    .type = cpu_to_be16(ETH_P_MPLS_MC),
    .priority = 15,
    .callbacks = {
    .gso_segment    =	mpls_gso_segment,
    },
    };
    static struct packet_offload mpls_uc_offload __read_mostly = {
    .type = cpu_to_be16(ETH_P_MPLS_UC),
    .priority = 15,
    .callbacks = {
    .gso_segment    =	mpls_gso_segment,
    },
    };
#[no_mangle]
unsafe extern "C" fn mpls_gso_init() -> int __init {
    static int __init mpls_gso_init(void)
    {
    pr_info("MPLS GSO support\n");
    dev_add_offload(&mpls_uc_offload);
    dev_add_offload(&mpls_mc_offload);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpls_gso_exit() -> void __exit {
    static void __exit mpls_gso_exit(void)
    {
    dev_remove_offload(&mpls_uc_offload);
    dev_remove_offload(&mpls_mc_offload);
    }
    module_init(mpls_gso_init);
    module_exit(mpls_gso_exit);
    MODULE_DESCRIPTION("MPLS GSO support");
    MODULE_AUTHOR("Simon Horman <horms@verge.net.au>");
    MODULE_LICENSE("GPL");
