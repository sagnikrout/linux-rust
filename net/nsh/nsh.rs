//! Automatically rewritten from C to Rust
//! Source: net/nsh/nsh.c
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
// Network Service Header
//
// Copyright (c) 2017 Red Hat, Inc. -- Jiri Benc <jbenc@redhat.com>
//

#[no_mangle]
pub unsafe extern "C" fn nsh_push(skb: *mut sk_buff, pushed_nh: *const nshhdr) -> c_int {
    int nsh_push(struct sk_buff *skb, const struct nshhdr *pushed_nh)
    {
    struct nshhdr *nh;
    let mut length: usize = nsh_hdr_len(pushed_nh);
    u8 next_proto;
    if (skb.mac_len) {
    next_proto = TUN_P_ETHERNET;
    } else {
    next_proto = tun_p_from_eth_p(skb.protocol);
    if (!next_proto)
    return -EAFNOSUPPORT;
    }
// Add the NSH header
    if (skb_cow_head(skb, length) < 0)
    return -ENOMEM;
    skb_push(skb, length);
    nh = (struct nshhdr *)(skb.data);
    memcpy(nh, pushed_nh, length);
    nh.np = next_proto;
    skb_postpush_rcsum(skb, nh, length);
    skb.protocol = htons(ETH_P_NSH);
    skb_reset_mac_header(skb);
    skb_reset_network_header(skb);
    skb_reset_mac_len(skb);
    return 0;
    }
    EXPORT_SYMBOL_GPL(nsh_push);
#[no_mangle]
pub unsafe extern "C" fn nsh_pop(skb: *mut sk_buff) -> c_int {
    int nsh_pop(struct sk_buff *skb)
    {
    struct nshhdr *nh;
    size_t length;
    __be16 inner_proto;
    if (!pskb_may_pull(skb, NSH_BASE_HDR_LEN))
    return -ENOMEM;
    nh = (struct nshhdr *)(skb.data);
    length = nsh_hdr_len(nh);
    if (length < NSH_BASE_HDR_LEN)
    return -EINVAL;
    inner_proto = tun_p_to_eth_p(nh.np);
    if (!pskb_may_pull(skb, length))
    return -ENOMEM;
    if (!inner_proto)
    return -EAFNOSUPPORT;
    skb_pull_rcsum(skb, length);
    skb_reset_mac_header(skb);
    skb_reset_network_header(skb);
    skb_reset_mac_len(skb);
    skb.protocol = inner_proto;
    return 0;
    }
    EXPORT_SYMBOL_GPL(nsh_pop);
    static struct sk_buff *nsh_gso_segment(struct sk_buff *skb,
    netdev_features_t features)
    {
    unsigned int outer_hlen, mac_len, nsh_len;
    struct sk_buff *segs = ERR_PTR(-EINVAL);
    let mut mac_offset: u16 = skb.mac_header;
    __be16 outer_proto, proto;
    skb_reset_network_header(skb);
    outer_proto = skb.protocol;
    outer_hlen = skb_mac_header_len(skb);
    mac_len = skb.mac_len;
    if (unlikely(!pskb_may_pull(skb, NSH_BASE_HDR_LEN)))
    goto out;
    nsh_len = nsh_hdr_len(nsh_hdr(skb));
    if (nsh_len < NSH_BASE_HDR_LEN)
    goto out;
    if (unlikely(!pskb_may_pull(skb, nsh_len)))
    goto out;
    proto = tun_p_to_eth_p(nsh_hdr(skb).np);
    if (!proto)
    goto out;
    __skb_pull(skb, nsh_len);
    skb_reset_mac_header(skb);
    skb.mac_len = proto == htons(ETH_P_TEB) ? ETH_HLEN : 0;
    skb.protocol = proto;
    features &= NETIF_F_SG;
    segs = skb_mac_gso_segment(skb, features);
    if (IS_ERR_OR_NULL(segs)) {
    skb_gso_error_unwind(skb, htons(ETH_P_NSH), nsh_len,
    mac_offset, mac_len);
    goto out;
    }
    for (skb = segs; skb; skb = skb.next) {
    skb.protocol = outer_proto;
    __skb_push(skb, nsh_len + outer_hlen);
    skb_reset_mac_header(skb);
    skb_set_network_header(skb, outer_hlen);
    skb.mac_len = mac_len;
    }
    out:
    return segs;
    }
    static struct packet_offload nsh_packet_offload __read_mostly = {
    .type = htons(ETH_P_NSH),
    .priority = 15,
    .callbacks = {
    .gso_segment = nsh_gso_segment,
    },
    };
#[no_mangle]
unsafe extern "C" fn nsh_init_module() -> int __init {
    static int __init nsh_init_module(void)
    {
    dev_add_offload(&nsh_packet_offload);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nsh_cleanup_module() -> void __exit {
    static void __exit nsh_cleanup_module(void)
    {
    dev_remove_offload(&nsh_packet_offload);
    }
    module_init(nsh_init_module);
    module_exit(nsh_cleanup_module);
    MODULE_AUTHOR("Jiri Benc <jbenc@redhat.com>");
    MODULE_DESCRIPTION("NSH protocol");
    MODULE_LICENSE("GPL v2");
