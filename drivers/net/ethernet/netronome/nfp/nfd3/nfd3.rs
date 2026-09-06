//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfd3/nfd3.h
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
// Copyright (C) 2015-2019 Netronome Systems, Inc.
// TX descriptor format

// Flags in the host TX descriptor

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_nfd3_tx_desc {
    pub /: *mut *mut u8 dma_addr_hi; / High bits of host buf address,
    pub /: *mut *mut __le16 dma_len; / Length to DMA for this desc,
    pub +: *mut *mut u8 offset_eop; / Offset in buf where pkt starts,
// highest bit is eop flag.
//
    pub /: *mut *mut __le32 dma_addr_lo; / Low 32bit of host buf addr,
    pub /: *mut *mut __le16 mss; / MSS to be used for LSO,
    pub /: *mut *mut u8 lso_hdrlen; / LSO, TCP payload offset,
    pub /: *mut *mut *mut u8 flags; / TX Flags, see @NFD3_DESC_TX_,
    pub /: *mut *mut u8 l3_offset; / L3 header offset,
    pub /: *mut *mut u8 l4_offset; / L4 header offset,
}

//
// struct nfp_nfd3_tx_buf - software TX buffer descriptor
// @skb:	normal ring, sk_buff associated with this buffer
// @frag:	XDP ring, page frag associated with this buffer
// @xdp:	XSK buffer pool handle (for AF_XDP)
// @dma_addr:	DMA mapping address of the buffer
// @fidx:	Fragment index (-1 for the head and [0..nr_frags-1] for frags)
// @pkt_cnt:	Number of packets to be produced out of the skb associated
// with this buffer (valid only on the head's buffer).
// Will be 1 for all non-TSO packets.
// @is_xsk_tx:	Flag if buffer is a RX buffer after a XDP_TX action and not a
// buffer from the TX queue (for AF_XDP).
// @real_len:	Number of bytes which to be produced out of the skb (valid only
// on the head's buffer). Equal to skb->len for non-TSO packets.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_nfd3_tx_buf {
    pub skb: *mut sk_buff,
    pub frag: *mut c_void,
    pub xdp: *mut xdp_buff,
}

extern "C" {
    pub fn nfp_nfd3_tx_complete(tx_ring: *mut nfp_net_tx_ring, budget: c_int);
}
extern "C" {
    pub fn nfp_nfd3_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn nfp_nfd3_tx(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn nfp_nfd3_ctrl_poll(t: *mut tasklet_struct);
}
extern "C" {
    pub fn nfp_nfd3_xsk_tx_free(txbuf: *mut nfp_nfd3_tx_buf);
}
extern "C" {
    pub fn nfp_nfd3_xsk_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}

extern "C" {
    pub fn nfp_nfd3_ipsec_tx(txd: *mut nfp_nfd3_tx_desc, skb: *mut sk_buff);
}

