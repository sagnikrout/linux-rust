//! Automatically rewritten from C to Rust
//! Source: net/802/fc.c
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
// NET3:	Fibre Channel device handling subroutines
//
// Vineet Abraham <vma@iol.unh.edu>
// v 1.0 03/22/99
//

//
// Put the headers on a Fibre Channel packet.
//
    static int fc_header(struct sk_buff *skb, struct net_device *dev,
    unsigned short type,
    const void *daddr, const void *saddr, unsigned int len)
    {
    struct fch_hdr *fch;
    int hdr_len;
//
// Add the 802.2 SNAP header if IP as the IPv4 code calls
// dev->hard_header directly.
//
    if (type == ETH_P_IP || type == ETH_P_ARP)
    {
    struct fcllc *fcllc;
    hdr_len = sizeof(struct fch_hdr) + sizeof(struct fcllc);
    fch = skb_push(skb, hdr_len);
    fcllc = (struct fcllc *)(fch+1);
    fcllc.dsap = fcllc.ssap = EXTENDED_SAP;
    fcllc.llc = UI_CMD;
    fcllc.protid[0] = fcllc.protid[1] = fcllc.protid[2] = 0x00;
    fcllc.ethertype = htons(type);
    }
    else
    {
    hdr_len = sizeof(struct fch_hdr);
    fch = skb_push(skb, hdr_len);
    }
    if(saddr)
    memcpy(fch.saddr,saddr,dev.addr_len);
    else
    memcpy(fch.saddr,dev.dev_addr,dev.addr_len);
    if(daddr)
    {
    memcpy(fch.daddr,daddr,dev.addr_len);
    return hdr_len;
    }
    return -hdr_len;
    }
    static const struct header_ops fc_header_ops = {
    .create	 = fc_header,
    };
#[no_mangle]
unsafe extern "C" fn fc_setup(dev: *mut net_device) {
    static void fc_setup(struct net_device *dev)
    {
    dev.header_ops		= &fc_header_ops;
    dev.type		= ARPHRD_IEEE802;
    dev.hard_header_len	= FC_HLEN;
    dev.mtu		= 2024;
    dev.addr_len		= FC_ALEN;
    dev.tx_queue_len	= 100; /* Long queues on fc */
    dev.flags		= IFF_BROADCAST;
    memset(dev.broadcast, 0xFF, FC_ALEN);
    }
//
// alloc_fcdev - Register fibre channel device
// @sizeof_priv: Size of additional driver-private structure to be allocated
// for this fibre channel device
//
// Fill in the fields of the device structure with fibre channel-generic values.
//
// Constructs a new net device, complete with a private data area of
// size @sizeof_priv.  A 32-byte (not bit) alignment is enforced for
// this private data area.
//
    struct net_device *alloc_fcdev(int sizeof_priv)
    {
    return alloc_netdev(sizeof_priv, "fc%d", NET_NAME_UNKNOWN, fc_setup);
    }
    EXPORT_SYMBOL(alloc_fcdev);
