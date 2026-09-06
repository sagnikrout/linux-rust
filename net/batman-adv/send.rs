//! Automatically rewritten from C Header to Rust Module
//! Source: net/batman-adv/send.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) B.A.T.M.A.N. contributors:
//
// Marek Lindner, Simon Wunderlich
//

extern "C" {
    pub fn batadv_forw_packet_steal(packet: *mut batadv_forw_packet, l: *mut spinlock_t) -> bool;
}
extern "C" {
    pub fn batadv_forw_packet_is_rebroadcast(forw_packet: *mut batadv_forw_packet) -> bool;
}
//
// batadv_send_skb_via_tt() - send an skb via TT lookup
// @bat_priv: the bat priv with all the mesh interface information
// @skb: the payload to send
// @dst_hint: can be used to override the destination contained in the skb
// @vid: the vid to be used to search the translation table
//
// Look up the recipient node for the destination address in the ethernet
// header via the translation table. Wrap the given skb into a batman-adv
// unicast header. Then send this frame to the according destination node.
//
// Return: NET_XMIT_DROP in case of error or NET_XMIT_SUCCESS otherwise.
//
// batadv_send_skb_via_tt_4addr() - send an skb via TT lookup
// @bat_priv: the bat priv with all the mesh interface information
// @skb: the payload to send
// @packet_subtype: the unicast 4addr packet subtype to use
// @dst_hint: can be used to override the destination contained in the skb
// @vid: the vid to be used to search the translation table
//
// Look up the recipient node for the destination address in the ethernet
// header via the translation table. Wrap the given skb into a batman-adv
// unicast-4addr header. Then send this frame to the according destination
// node.
//
// Return: NET_XMIT_DROP in case of error or NET_XMIT_SUCCESS otherwise.
//
