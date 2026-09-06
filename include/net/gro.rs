//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/gro.h
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

// This should be increased if a protocol with a bigger head is added.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_gro_cb {
// Virtual address of skb_shinfo(skb)->frags[0].page + offset.
    pub frag0: *mut c_void,
// Length of frag0.
    pub frag0_len: c_uint,
}

// used in skb_gro_receive() slow path
// jiffies when first packet was created/queued
// This indicates where we are processing relative to skb->data.
// This is non-zero if the packet cannot be merged with the new skb.
// Number of segments aggregated.
// Used in ipv6_gro_receive() and foo-over-udp and esp-in-udp
// Used in napi_gro_cb::free
pub const NAPI_GRO_FREE: c_int = 1;
pub const NAPI_GRO_FREE_STOLEN_HEAD: c_int = 2;
// portion of the cb set to zero at every gro iteration
// Start offset for remote checksum offload
// This is non-zero if the packet may be of the same flow.
// Used in tunnel GRO receive
// GRO checksum is valid
// Number of checksums via CHECKSUM_UNNECESSARY
// Free the skb?
// Used in GRE, set in fou/gue_gro_receive
// Used to determine if ipid_offset can be ignored
// Number of gro_receive callbacks this packet already went through
// GRO is done by frag_list pointer chaining.
// used to support CHECKSUM_COMPLETE for tunneling protocols
// L3 offsets

pub const GRO_RECURSION_LIMIT: c_int = 15;
extern "C" {
    pub fn cb(_arg: head, _arg: skb) -> return;
}
extern "C" {
    pub fn cb(_arg: sk, _arg: head, _arg: skb) -> return;
}
extern "C" {
    pub fn likely(NAPI_GRO_CB(skb)->frag0_len: hlen <=) -> return;
}
extern "C" {
    pub fn skb_gro_header_fast(_arg: skb, _arg: skb_gro_receive_network_offset(skb)) -> return;
}
// GRO checksum functions. These are logical equivalents of the normal
// checksum functions (in skbuff.h) except that they operate on the GRO
// offsets and fields in sk_buff.
//
extern "C" {
    pub fn __skb_gro_checksum_complete(skb: *mut sk_buff) -> __sum16;
}
extern "C" {
    pub fn __skb_gro_checksum_complete(_arg: skb) -> return;
}
// Consume a checksum from CHECKSUM_UNNECESSARY
// Update skb for CHECKSUM_UNNECESSARY and csum_level when we
// verified a new top level checksum or an encapsulated one
// during GRO. This saves work if we fallback to normal path.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gro_remcsum {
    pub offset: c_int,
    pub delta: __wsum,
}

// Adjust skb->csum since we changed the packet

extern "C" {
    pub fn udp6_gro_complete(: *mut sk_buff, _arg: c_int) -> c_int;
}

extern "C" {
    pub fn udp_gro_complete(skb: *mut sk_buff, nhoff: c_int, lookup: udp_lookup_t) -> c_int;
}
// All fields must match except length and checksum.
// When we receive our second frame we can make a decision on if we
// continue this flow as an atomic flow with a fixed ID or if we use
// an incrementing ID.
//
// <Version:4><Traffic_Class:8><Flow_Label:20>
// Flush if Traffic Class fields are different.
extern "C" {
    pub fn ipv6_gro_flush(_arg: nh, _arg: nh2) -> return;
}
extern "C" {
    pub fn inet_gro_flush(_arg: nh, _arg: nh2, _arg: p, _arg: inner) -> return;
}
extern "C" {
    pub fn skb_gro_receive(p: *mut sk_buff, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn skb_gro_receive_list(p: *mut sk_buff, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn __gro_flush(gro: *mut gro_node, flush_old: bool);
}
// Pass the currently batched GRO_NORMAL SKBs up to the stack.
// Queue one GRO_NORMAL SKB up for list processing. If batch size exceeded,
// pass the whole batch up to the stack.
//
extern "C" {
    pub fn gro_init(gro: *mut gro_node);
}
extern "C" {
    pub fn gro_cleanup(gro: *mut gro_node);
}
// This function is the alternative of 'inet_iif' and 'inet_sdif'
// functions in case we can not rely on fields of IPCB.
//
// The caller must verify skb_valid_dst(skb) is false and skb->dev is initialized.
// The caller must hold the RCU read lock.
//
// iif = inet_iif(skb) ?: skb->dev->ifindex;
// sdif = 0;

// sdif = *iif;
// iif = master ? master->ifindex : 0;

// This function is the alternative of 'inet6_iif' and 'inet6_sdif'
// functions in case we can not rely on fields of IP6CB.
//
// The caller must verify skb_valid_dst(skb) is false and skb->dev is initialized.
// The caller must hold the RCU read lock.
//
// using skb->dev->ifindex because skb_dst(skb) is not initialized
// iif = skb->dev->ifindex;
// sdif = 0;

// sdif = *iif;
// iif = master ? master->ifindex : 0;

