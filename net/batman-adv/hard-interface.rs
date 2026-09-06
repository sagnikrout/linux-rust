//! Automatically rewritten from C Header to Rust Module
//! Source: net/batman-adv/hard-interface.h
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

//
// enum batadv_hard_if_state - State of a hard interface
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum batadv_hard_if_state {
//
// @BATADV_IF_TO_BE_REMOVED: interface will be removed from mesh
// interface
//
    BATADV_IF_TO_BE_REMOVED,

// @BATADV_IF_INACTIVE: interface is deactivated
    BATADV_IF_INACTIVE,

// @BATADV_IF_ACTIVE: interface is used
    BATADV_IF_ACTIVE,

// @BATADV_IF_TO_BE_ACTIVATED: interface is getting activated
    BATADV_IF_TO_BE_ACTIVATED,
}

//
// enum batadv_hard_if_bcast - broadcast avoidance options
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum batadv_hard_if_bcast {
// @BATADV_HARDIF_BCAST_OK: Do broadcast on according hard interface
    BATADV_HARDIF_BCAST_OK = 0,

//
// @BATADV_HARDIF_BCAST_NORECIPIENT: Broadcast not needed, there is no
// recipient
//
    BATADV_HARDIF_BCAST_NORECIPIENT,

//
// @BATADV_HARDIF_BCAST_DUPFWD: There is just the neighbor we got it
// from
//
    BATADV_HARDIF_BCAST_DUPFWD,

// @BATADV_HARDIF_BCAST_DUPORIG: There is just the originator
    BATADV_HARDIF_BCAST_DUPORIG,
}

extern "C" {
    pub fn batadv_netdev_get_wifi_flags(net_dev: *mut net_device) -> u32;
}
extern "C" {
    pub fn batadv_hardif_get_wifi_flags(hard_iface: *mut batadv_hard_iface) -> u32;
}
extern "C" {
    pub fn batadv_is_wifi_hardif(hard_iface: *mut batadv_hard_iface) -> bool;
}
extern "C" {
    pub fn batadv_hardif_disable_interface(hard_iface: *mut batadv_hard_iface);
}
extern "C" {
    pub fn batadv_hardif_min_mtu(mesh_iface: *mut net_device) -> c_int;
}
extern "C" {
    pub fn batadv_update_min_mtu(mesh_iface: *mut net_device);
}
extern "C" {
    pub fn batadv_hardif_release(ref: *mut kref);
}
extern "C" {
    pub fn batadv_wifi_net_devices_init() -> int __init;
}
extern "C" {
    pub fn batadv_wifi_net_devices_deinit();
}
//
// batadv_hardif_put() - decrement the hard interface refcounter and possibly
// release it
// @hard_iface: the hard interface to free
//
// batadv_primary_if_get_selected() - Get reference to primary interface
// @bat_priv: the bat priv with all the mesh interface information
//
// Return: primary interface (with increased refcnt), otherwise NULL
//
// batadv_is_cfg80211() - check if the given hardif is a cfg80211
// wifi interface
// @wifi_flags: extracted batadv_hard_iface_wifi_flags of a net_device
//
// Return: true if the net device is a cfg80211 wireless device, false
// otherwise.
//
// batadv_is_wifi() - check if flags belong to wifi interface
// @wifi_flags: extracted batadv_hard_iface_wifi_flags of a net_device
//
// Return: true if the net device is a 802.11 wireless device, false otherwise.
//
