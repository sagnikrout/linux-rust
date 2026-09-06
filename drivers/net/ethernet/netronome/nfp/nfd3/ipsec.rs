//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/nfd3/ipsec.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2018 Netronome Systems, Inc
// Copyright (C) 2021 Corigine, Inc

#[no_mangle]
pub unsafe extern "C" fn nfp_nfd3_ipsec_tx(txd: *mut nfp_nfd3_tx_desc, skb: *mut sk_buff) {
    void nfp_nfd3_ipsec_tx(struct nfp_nfd3_tx_desc *txd, struct sk_buff *skb)
    {
    struct xfrm_state *x = xfrm_input_state(skb);
    struct xfrm_offload *xo = xfrm_offload(skb);
    struct iphdr *iph = ip_hdr(skb);
    int l4_proto;
    if (x.xso.dev && (x.xso.dev.features & NETIF_F_HW_ESP_TX_CSUM)) {
    txd.flags |= NFD3_DESC_TX_CSUM;
    if (iph.version == 4)
    txd.flags |= NFD3_DESC_TX_IP4_CSUM;
    if (x.props.mode == XFRM_MODE_TRANSPORT)
    l4_proto = xo.proto;
#[no_mangle]
pub unsafe extern "C" fn if(XFRM_MODE_TUNNEL: x->props.mode ==) -> else {
    else if (x.props.mode == XFRM_MODE_TUNNEL)
    l4_proto = xo.inner_ipproto;
    else
    return;
    switch (l4_proto) {
    case IPPROTO_UDP:
    txd.flags |= NFD3_DESC_TX_UDP_CSUM;
    return;
    case IPPROTO_TCP:
    txd.flags |= NFD3_DESC_TX_TCP_CSUM;
    return;
    }
    }
    }
