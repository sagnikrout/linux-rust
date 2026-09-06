//! Automatically rewritten from C Header to Rust Module
//! Source: net/bridge/br_private_tunnel.h
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
// Bridge per vlan tunnels
//
// Authors:
// Roopa Prabhu		<roopa@cumulusnetworks.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vtunnel_info {
    pub tunid: u32,
    pub vid: u16,
    pub flags: u16,
}

// br_netlink_tunnel.c
extern "C" {
    pub fn br_get_vlan_tunnel_info_size(vg: *mut net_bridge_vlan_group) -> c_int;
}

// br_vlan_tunnel.c
extern "C" {
    pub fn vlan_tunnel_init(vg: *mut net_bridge_vlan_group) -> c_int;
}
extern "C" {
    pub fn vlan_tunnel_deinit(vg: *mut net_bridge_vlan_group);
}
extern "C" {
    pub fn nbp_vlan_tunnel_info_delete(port: *const net_bridge_port, vid: u16) -> c_int;
}
extern "C" {
    pub fn nbp_vlan_tunnel_info_flush(port: *mut net_bridge_port);
}

