//! Automatically rewritten from C Header to Rust Module
//! Source: net/batman-adv/multicast.h
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
// Linus Lüssing
//

//
// enum batadv_forw_mode - the way a packet should be forwarded as
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum batadv_forw_mode {
//
// @BATADV_FORW_BCAST: forward the packet to all nodes via a batman-adv
// broadcast packet
//
    BATADV_FORW_BCAST,

//
// @BATADV_FORW_UCASTS: forward the packet to some nodes via one
// or more batman-adv unicast packets
//
    BATADV_FORW_UCASTS,

//
// @BATADV_FORW_MCAST: forward the packet to some nodes via a
// batman-adv multicast packet
//
    BATADV_FORW_MCAST,

// @BATADV_FORW_NONE: don't forward, drop it
    BATADV_FORW_NONE,
}

extern "C" {
    pub fn batadv_mcast_init(bat_priv: *mut batadv_priv);
}
extern "C" {
    pub fn batadv_mcast_flags_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn batadv_mcast_free(bat_priv: *mut batadv_priv);
}
extern "C" {
    pub fn batadv_mcast_purge_orig(orig_node: *mut batadv_orig_node);
}
// multicast_forw.c
extern "C" {
    pub fn batadv_mcast_forw_packet_hdrlen(num_dests: c_uint) -> c_uint;
}
extern "C" {
    pub fn batadv_mcast_forw_mcsend(bat_priv: *mut batadv_priv, skb: *mut sk_buff) -> c_int;
}

