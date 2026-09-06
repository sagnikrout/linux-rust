//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/gso.h
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

// Keeps track of mac header offset relative to skb->head.
// It is useful for TSO of Tunneling protocol. e.g. GRE.
// For non-tunnel skb it points to skb_mac_header() and for
// tunnel skb it points to outer mac header.
// Keeps track of level of encapsulation of network headers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_gso_cb {
    pub mac_offset: c_int,
    pub data_offset: c_int,
}

pub const SKB_GSO_CB_OFFSET: c_int = 32;

// Do not update partial checksums if remote checksum is enabled.
// Compute the checksum for a gso segment. First compute the checksum value
// from the start of transport header to SKB_GSO_CB(skb)->csum_start, and
// then add in skb->csum (checksum from csum_start to end of packet).
// skb->csum and csum_start are then updated to reflect the checksum of the
// resultant packet starting from the transport header-- the resultant checksum
// is in the res argument (i.e. normally zero or ~ of checksum of a pseudo
// header.
//
extern "C" {
    pub fn csum_fold(_arg: csum_partial(csum_start, _arg: plen, _arg: partial)) -> return;
}
extern "C" {
    pub fn __skb_gso_segment(_arg: skb, _arg: features, _arg: true) -> return;
}
extern "C" {
    pub fn skb_gso_validate_network_len(skb: *const sk_buff, mtu: c_uint) -> bool;
}
extern "C" {
    pub fn skb_gso_validate_mac_len(skb: *const sk_buff, len: c_uint) -> bool;
}
