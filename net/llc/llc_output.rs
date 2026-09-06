//! Automatically rewritten from C to Rust
//! Source: net/llc/llc_output.c
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
// llc_output.c - LLC minimal output path
//
// Copyright (c) 1997 by Procom Technology, Inc.
// 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

//
// llc_mac_hdr_init - fills MAC header fields
// @skb: Address of the frame to initialize its MAC header
// @sa: The MAC source address
// @da: The MAC destination address
//
// Fills MAC header fields, depending on MAC type. Returns 0, If MAC type
// is a valid type and initialization completes correctly 1, otherwise.
//
    int llc_mac_hdr_init(struct sk_buff *skb,
    const unsigned char *sa, const unsigned char *da)
    {
    let mut rc: c_int = -EINVAL;
    switch (skb.dev.type) {
    case ARPHRD_ETHER:
    case ARPHRD_LOOPBACK:
    rc = dev_hard_header(skb, skb.dev, ETH_P_802_2, da, sa,
    skb.len);
    if (rc > 0)
    rc = 0;
    break;
    default:
    break;
    }
    return rc;
    }
//
// llc_build_and_send_ui_pkt - unitdata request interface for upper layers
// @sap: sap to use
// @skb: packet to send
// @dmac: destination mac address
// @dsap: destination sap
//
// Upper layers calls this function when upper layer wants to send data
// using connection-less mode communication (UI pdu).
//
// Accept data frame from network layer to be sent using connection-
// less mode communication; timeout/retries handled by network layer;
// package primitive as an event and send to SAP event handler
//
    int llc_build_and_send_ui_pkt(struct llc_sap *sap, struct sk_buff *skb,
    const unsigned char *dmac, unsigned char dsap)
    {
    int rc;
    llc_pdu_header_init(skb, LLC_PDU_TYPE_U, sap.laddr.lsap,
    dsap, LLC_PDU_CMD);
    llc_pdu_init_as_ui_cmd(skb);
    rc = llc_mac_hdr_init(skb, skb.dev.dev_addr, dmac);
    if (likely(!rc))
    rc = dev_queue_xmit(skb);
    else
    kfree_skb(skb);
    return rc;
    }
    EXPORT_SYMBOL(llc_mac_hdr_init);
    EXPORT_SYMBOL(llc_build_and_send_ui_pkt);
