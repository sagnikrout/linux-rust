//! Automatically rewritten from C Header to Rust Module
//! Source: net/batman-adv/distributed-arp-table.h
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
// Antonio Quartulli
//

// BATADV_DAT_ADDR_MAX - maximum address value in the DHT space

extern "C" {
    pub fn batadv_dat_status_update(net_dev: *mut net_device);
}
//
// batadv_dat_init_orig_node_addr() - assign a DAT address to the orig_node
// @orig_node: the node to assign the DAT address to
//
// batadv_dat_init_own_addr() - assign a DAT address to the node itself
// @bat_priv: the bat priv with all the mesh interface information
// @primary_if: a pointer to the primary interface
//
extern "C" {
    pub fn batadv_dat_init(bat_priv: *mut batadv_priv) -> c_int;
}
extern "C" {
    pub fn batadv_dat_free(bat_priv: *mut batadv_priv);
}
extern "C" {
    pub fn batadv_dat_cache_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
//
// batadv_dat_inc_counter() - increment the correct DAT packet counter
// @bat_priv: the bat priv with all the mesh interface information
// @subtype: the 4addr subtype of the packet to be counted
//
// Updates the ethtool statistics for the received packet if it is a DAT subtype
//

