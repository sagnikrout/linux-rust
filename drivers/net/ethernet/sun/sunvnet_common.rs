//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sun/sunvnet_common.h
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

// length of time (or less) we expect pending descriptors to be marked
// as VIO_DESC_DONE and skbs ready to be freed
//

pub const VNET_TX_RING_SIZE: c_int = 512;

pub const VNET_MAX_MTU: c_int = 65535;
// VNET packets are sent in buffers with the first 6 bytes skipped
// so that after the ethernet header the IPv4/IPv6 headers are aligned
// properly.
//
pub const VNET_PACKET_SKIP: c_int = 6;

pub const VNET_MAX_TXQS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnet_tx_entry {
    pub skb: *mut sk_buff,
    pub ncookies: c_uint,
    pub cookies: [ldc_trans_cookie; VNET_MAXCOOKIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnet_port_stats {
// keep them all the same size
    pub rx_bytes: u32,
    pub tx_bytes: u32,
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub event_up: u32,
    pub event_reset: u32,
    pub q_placeholder: u32,
}

// Structure to describe a vnet-port or vsw-port in the MD.
// If the vsw bit is set, this structure represents a vswitch
// port, and the net_device can be found from ->dev. If the
// vsw bit is not set, the net_device is available from ->vp->dev.
// See the VNET_PORT_TO_NET_DEVICE macro below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnet_port {
    pub vio: vio_driver_state,
    pub stats: vnet_port_stats,
    pub hash: hlist_node,
    pub raddr: [u8; ETH_ALEN],
    pub switch_port:1: unsigned,
    pub tso:1: unsigned,
    pub vsw:1: unsigned,
    pub __pad:13: unsigned,
    pub vp: *mut vnet,
    pub dev: *mut net_device,
    pub tx_bufs: [vnet_tx_entry; VNET_TX_RING_SIZE],
    pub list: list_head,
    pub stop_rx_idx: u32,
    pub stop_rx: bool,
    pub start_cons: bool,
    pub clean_timer: timer_list,
    pub rmtu: u64,
    pub tsolen: u16,
    pub napi: napi_struct,
    pub napi_stop_idx: u32,
    pub napi_resume: bool,
    pub rx_event: c_int,
    pub q_index: u16,
}

extern "C" {
    pub fn container_of(_arg: vio, vnet_port: struct, _arg: vio) -> return;
}
pub const VNET_PORT_HASH_SIZE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnet_mcast_entry {
    pub addr: [u8; ETH_ALEN],
    pub sent: u8,
    pub hit: u8,
    pub next: *mut vnet_mcast_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnet {
    pub /: *mut *mut spinlock_t lock; / Protects port_list and port_hash.,
    pub dev: *mut net_device,
    pub msg_enable: u32,
    pub q_used: [u8; VNET_MAX_TXQS],
    pub port_list: list_head,
    pub port_hash: [hlist_head; VNET_PORT_HASH_SIZE],
    pub mcast_list: *mut vnet_mcast_entry,
    pub list: list_head,
    pub local_mac: u64,
    pub nports: c_int,
}

// Def used by common code to get the net_device from the proper location

// Common funcs
extern "C" {
    pub fn sunvnet_clean_timer_expire_common(t: *mut timer_list);
}
extern "C" {
    pub fn sunvnet_open_common(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn sunvnet_close_common(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn sunvnet_set_rx_mode_common(dev: *mut net_device, vp: *mut vnet);
}
extern "C" {
    pub fn sunvnet_set_mac_addr_common(dev: *mut net_device, p: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sunvnet_tx_timeout_common(dev: *mut net_device, txqueue: c_uint);
}

extern "C" {
    pub fn sunvnet_poll_controller_common(dev: *mut net_device, vp: *mut vnet);
}

extern "C" {
    pub fn sunvnet_event_common(arg: *mut c_void, event: c_int);
}
extern "C" {
    pub fn sunvnet_send_attr_common(vio: *mut vio_driver_state) -> c_int;
}
extern "C" {
    pub fn sunvnet_handle_attr_common(vio: *mut vio_driver_state, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sunvnet_handshake_complete_common(vio: *mut vio_driver_state);
}
extern "C" {
    pub fn sunvnet_poll_common(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn sunvnet_port_free_tx_bufs_common(port: *mut vnet_port);
}
extern "C" {
    pub fn vnet_port_reset(port: *mut vnet_port);
}
extern "C" {
    pub fn sunvnet_port_is_up_common(vnet: *mut vnet_port) -> bool;
}
extern "C" {
    pub fn sunvnet_port_add_txq_common(port: *mut vnet_port);
}
extern "C" {
    pub fn sunvnet_port_rm_txq_common(port: *mut vnet_port);
}
