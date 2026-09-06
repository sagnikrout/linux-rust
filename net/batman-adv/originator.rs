//! Automatically rewritten from C Header to Rust Module
//! Source: net/batman-adv/originator.h
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
    pub fn batadv_compare_orig(node: *const hlist_node, data2: *const c_void) -> bool;
}
extern "C" {
    pub fn batadv_originator_init(bat_priv: *mut batadv_priv) -> c_int;
}
extern "C" {
    pub fn batadv_originator_free(bat_priv: *mut batadv_priv);
}
extern "C" {
    pub fn batadv_purge_orig_ref(bat_priv: *mut batadv_priv);
}
extern "C" {
    pub fn batadv_orig_node_release(ref: *mut kref);
}
extern "C" {
    pub fn batadv_hardif_neigh_release(ref: *mut kref);
}
extern "C" {
    pub fn batadv_neigh_node_release(ref: *mut kref);
}
extern "C" {
    pub fn batadv_neigh_ifinfo_release(ref: *mut kref);
}
extern "C" {
    pub fn batadv_hardif_neigh_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn batadv_orig_ifinfo_release(ref: *mut kref);
}
extern "C" {
    pub fn batadv_orig_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn batadv_orig_node_vlan_release(ref: *mut kref);
}
//
// batadv_choose_orig() - Return the index of the orig entry in the hash table
// @data: mac address of the originator node
// @size: the size of the hash table
//
// Return: the hash index where the object represented by @data should be
// stored at.
//
// batadv_orig_node_vlan_put() - decrement the refcounter and possibly release
// the originator-vlan object
// @orig_vlan: the originator-vlan object to release
//
// batadv_neigh_ifinfo_put() - decrement the refcounter and possibly release
// the neigh_ifinfo
// @neigh_ifinfo: the neigh_ifinfo object to release
//
// batadv_hardif_neigh_put() - decrement the hardif neighbors refcounter
// and possibly release it
// @hardif_neigh: hardif neigh neighbor to free
//
// batadv_neigh_node_put() - decrement the neighbors refcounter and possibly
// release it
// @neigh_node: neigh neighbor to free
//
// batadv_orig_ifinfo_put() - decrement the refcounter and possibly release
// the orig_ifinfo
// @orig_ifinfo: the orig_ifinfo object to release
//
// batadv_orig_node_put() - decrement the orig node refcounter and possibly
// release it
// @orig_node: the orig node to free
//
