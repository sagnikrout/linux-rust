//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/af_vsock.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// VMware vSockets Driver
//
// Copyright (C) 2007-2013 VMware, Inc. All rights reserved.
//

pub const LAST_RESERVED_PORT: c_int = 1023;
pub const VSOCK_HASH_SIZE: c_int = 251;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsock_sock {
// sk must be the first member.
    pub sk: sock,
    pub transport: *const vsock_transport,
    pub local_addr: sockaddr_vm,
    pub remote_addr: sockaddr_vm,
// Links for the global tables of bound and connected sockets.
    pub bound_table: list_head,
    pub connected_table: list_head,
// Accessed without the socket lock held. This means it can never be
// modified outsided of socket create or destruct.
//
    pub trusted: bool,
    pub to: *mut *mut bool cached_peer_allow_dgram; / Dgram communication allowed,
// cached peer?
//
    pub /: *mut *mut u32 cached_peer; / Context ID of last dgram destination check.,
    pub owner: *const cred,
// Rest are SOCK_STREAM only.
    pub connect_timeout: c_long,
// Listening socket that this came from.
    pub listener: *mut sock,
// Used for pending list and accept queue during connection handshake.
// The listening socket is the head for both lists.  Sockets created
// for connection requests are placed in the pending list until they
// are connected, at which point they are put in the accept queue list
// so they can be accepted in accept().
//
    pub pending_links: list_head,
    pub accept_queue: list_head,
    pub connect_work: delayed_work,
    pub pending_work: delayed_work,
    pub close_work: delayed_work,
    pub close_work_scheduled: bool,
    pub peer_shutdown: u32,
    pub sent_request: bool,
    pub ignore_connecting_rst: bool,
// Protected by lock_sock(sk)
    pub buffer_size: u64,
    pub buffer_min_size: u64,
    pub buffer_max_size: u64,
// Private to transport.
    pub trans: *mut c_void,
}

extern "C" {
    pub fn vsock_connectible_has_data(vsk: *mut vsock_sock) -> i64;
}
extern "C" {
    pub fn vsock_stream_has_data(vsk: *mut vsock_sock) -> i64;
}
extern "C" {
    pub fn vsock_stream_has_space(vsk: *mut vsock_sock) -> i64;
}
extern "C" {
    pub fn vsock_data_ready(sk: *mut sock);
}
// TRANSPORT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsock_transport_recv_notify_data {
    pub /: *mut *mut u64 data1; / Transport-defined.,
    pub /: *mut *mut u64 data2; / Transport-defined.,
    pub notify_on_block: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsock_transport_send_notify_data {
    pub /: *mut *mut u64 data1; / Transport-defined.,
    pub /: *mut *mut u64 data2; / Transport-defined.,
}

// Transport features flags
// Transport provides host->guest communication
pub const VSOCK_TRANSPORT_F_H2G: c_uint = 0x00000001;
// Transport provides guest->host communication
pub const VSOCK_TRANSPORT_F_G2H: c_uint = 0x00000002;
// Transport provides DGRAM communication
pub const VSOCK_TRANSPORT_F_DGRAM: c_uint = 0x00000004;
// Transport provides local (loopback) communication
pub const VSOCK_TRANSPORT_F_LOCAL: c_uint = 0x00000008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsock_transport {
    pub module: *mut module,
// Initialize/tear-down socket.
    pub ): *mut *mut *mut int (init)(struct vsock_sock , struct vsock_sock,
    pub ): *mut *mut void (destruct)(struct vsock_sock,
    pub ): *mut *mut void (release)(struct vsock_sock,
// Cancel all pending packets sent on vsock.
    pub vsk): *mut *mut int (cancel_pkt)(struct vsock_sock,
// Connections.
    pub ): *mut *mut int (connect)(struct vsock_sock,
// DGRAM.
    pub ): *mut *mut *mut int (dgram_bind)(struct vsock_sock , struct sockaddr_vm,
    pub flags): size_t len, int,
    pub len): *mut *mut msghdr , size_t,
    pub port): *mut *mut *mut bool (dgram_allow)(struct vsock_sock vsk, u32 cid, u32,
// STREAM.
// TODO: stream_bind()
    pub flags): size_t len, int,
    pub len): usize,
    pub ): *mut *mut s64 (stream_has_data)(struct vsock_sock,
    pub ): *mut *mut s64 (stream_has_space)(struct vsock_sock,
    pub ): *mut *mut u64 (stream_rcvhiwat)(struct vsock_sock,
    pub ): *mut *mut bool (stream_is_active)(struct vsock_sock,
    pub port): *mut *mut *mut bool (stream_allow)(struct vsock_sock vsk, u32 cid, u32,
// SEQ_PACKET.
    pub flags): c_int,
    pub len): usize,
    pub remote_cid): *mut *mut *mut bool (seqpacket_allow)(struct vsock_sock vsk, u32,
    pub vsk): *mut *mut u32 (seqpacket_has_data)(struct vsock_sock,
// Notification.
    pub ): *mut *mut *mut int (notify_poll_in)(struct vsock_sock , size_t, bool,
    pub ): *mut *mut *mut int (notify_poll_out)(struct vsock_sock , size_t, bool,
    pub ): *mut vsock_transport_recv_notify_data,
    pub ): *mut vsock_transport_recv_notify_data,
    pub ): *mut vsock_transport_recv_notify_data,
    pub ): *mut ssize_t, bool, struct vsock_transport_recv_notify_data,
    pub ): *mut vsock_transport_send_notify_data,
    pub ): *mut vsock_transport_send_notify_data,
    pub ): *mut vsock_transport_send_notify_data,
    pub ): *mut vsock_transport_send_notify_data,
// sk_lock held by the caller
    pub ): *mut *mut *mut void (notify_buffer_size)(struct vsock_sock , u64,
    pub val): *mut *mut *mut int (notify_set_rcvlowat)(struct vsock_sock vsk, int,
// SIOCOUTQ ioctl
    pub vsk): *mut *mut ssize_t (unsent_bytes)(struct vsock_sock,
// Shutdown.
    pub int): *mut *mut *mut int (shutdown)(struct vsock_sock ,,
// Addressing.
    pub (*get_local_cid)(void): *mut u32,
// Check if this transport serves a specific remote CID.
// For H2G transports: return true if the CID belongs to a registered
// guest. If not implemented, all CIDs > VMADDR_CID_HOST go to H2G.
// For G2H transports: return true if the transport can reach arbitrary
// CIDs via the hypervisor (i.e. supports the fallback overlay). VMCI
// does not implement this as it only serves CIDs 0 and 2.
//
    pub remote_cid): *mut *mut *mut bool (has_remote_cid)(struct vsock_sock vsk, u32,
// Read a single skb
    pub skb_read_actor_t): *mut *mut *mut int (read_skb)(struct vsock_sock ,,
// Zero-copy.
    pub (*msgzerocopy_allow)(void): *mut bool,
}

// CORE
extern "C" {
    pub fn vsock_core_register(t: *const vsock_transport, features: c_int) -> c_int;
}
extern "C" {
    pub fn vsock_core_unregister(t: *const vsock_transport);
}
// The transport may downcast this to access transport-specific functions
// UTILS
// vsock_table_lock must be held
extern "C" {
    pub fn vsock_add_pending(listener: *mut sock, pending: *mut sock);
}
extern "C" {
    pub fn vsock_remove_pending(listener: *mut sock, pending: *mut sock);
}
extern "C" {
    pub fn vsock_enqueue_accept(listener: *mut sock, connected: *mut sock);
}
extern "C" {
    pub fn vsock_pending_to_accept(listener: *mut sock, pending: *mut sock);
}
extern "C" {
    pub fn vsock_insert_connected(vsk: *mut vsock_sock);
}
extern "C" {
    pub fn vsock_remove_bound(vsk: *mut vsock_sock);
}
extern "C" {
    pub fn vsock_remove_connected(vsk: *mut vsock_sock);
}
extern "C" {
    pub fn vsock_remove_sock(vsk: *mut vsock_sock);
}
extern "C" {
    pub fn vsock_assign_transport(vsk: *mut vsock_sock, psk: *mut vsock_sock) -> c_int;
}
extern "C" {
    pub fn vsock_find_cid(cid: c_uint) -> bool;
}
extern "C" {
    pub fn vsock_linger(sk: *mut sock);
}
// TAP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsock_tap {
    pub dev: *mut net_device,
    pub module: *mut module,
    pub list: list_head,
}

extern "C" {
    pub fn vsock_add_tap(vt: *mut vsock_tap) -> c_int;
}
extern "C" {
    pub fn vsock_remove_tap(vt: *mut vsock_tap) -> c_int;
}
extern "C" {
    pub fn vsock_deliver_tap(opaque): *mut *mut sk_buff build_skb(void, opaque: *mut c_void);
}

extern "C" {
    pub fn vsock_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> c_int;
}
extern "C" {
    pub fn vsock_bpf_build_proto() -> void __init;
}

extern "C" {
    pub fn READ_ONCE(_arg: net->vsock.mode) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: net->vsock.child_ns_mode) -> return;
}
// Return true if two namespaces pass the mode rules. Otherwise, return false.
//
// A NULL namespace is treated as VSOCK_NET_MODE_GLOBAL.
//
// Read more about modes in the comment header of net/vmw_vsock/af_vsock.c.
//
// Any vsocks within the same network namespace are always reachable,
// regardless of the mode.
//
// Different namespaces are only reachable if they are both
// global mode.
//
