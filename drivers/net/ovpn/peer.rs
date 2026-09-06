//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ovpn/peer.h
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
// OpenVPN data channel offload
//
// Copyright (C) 2020-2025 OpenVPN, Inc.
//
// Author:	James Yonan <james@openvpn.net>
// Antonio Quartulli <antonio@openvpn.net>
//

//
// struct ovpn_peer - the main remote peer object
// @ovpn: main openvpn instance this peer belongs to
// @dev_tracker: reference tracker for associated dev
// @id: unique identifier, used to match incoming packets
// @tx_id: identifier to be used in TX packets
// @vpn_addrs: IP addresses assigned over the tunnel
// @vpn_addrs.ipv4: IPv4 assigned to peer on the tunnel
// @vpn_addrs.ipv6: IPv6 assigned to peer on the tunnel
// @hash_entry_id: entry in the peer ID hashtable
// @hash_entry_addr4: entry in the peer IPv4 hashtable
// @hash_entry_addr6: entry in the peer IPv6 hashtable
// @hash_entry_transp_addr: entry in the peer transport address hashtable
// @sock: the socket being used to talk to this peer
// @tcp: keeps track of TCP specific state
// @tcp.strp: stream parser context (TCP only)
// @tcp.user_queue: received packets that have to go to userspace (TCP only)
// @tcp.out_queue: packets on hold while socket is taken by user (TCP only)
// @tcp.tx_in_progress: true if TX is already ongoing (TCP only)
// @tcp.out_msg.skb: packet scheduled for sending (TCP only)
// @tcp.out_msg.offset: offset where next send should start (TCP only)
// @tcp.out_msg.len: remaining data to send within packet (TCP only)
// @tcp.sk_cb.sk_data_ready: pointer to original cb (TCP only)
// @tcp.sk_cb.sk_write_space: pointer to original cb (TCP only)
// @tcp.sk_cb.prot: pointer to original prot object (TCP only)
// @tcp.sk_cb.ops: pointer to the original prot_ops object (TCP only)
// @crypto: the crypto configuration (ciphers, keys, etc..)
// @dst_cache: cache for dst_entry used to send to peer
// @bind: remote peer binding
// @keepalive_interval: seconds after which a new keepalive should be sent
// @keepalive_xmit_exp: future timestamp when next keepalive should be sent
// @last_sent: timestamp of the last successfully sent packet
// @keepalive_timeout: seconds after which an inactive peer is considered dead
// @keepalive_recv_exp: future timestamp when the peer should expire
// @last_recv: timestamp of the last authenticated received packet
// @vpn_stats: per-peer in-VPN TX/RX stats
// @link_stats: per-peer link/transport TX/RX stats
// @delete_reason: why peer was deleted (i.e. timeout, transport error, ..)
// @lock: protects binding to peer (bind) and keepalive* fields
// @refcount: reference counter
// @rcu: used to free peer in an RCU safe way
// @release_entry: entry for the socket release list
// @keepalive_work: used to schedule keepalive sending
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_peer {
    pub ovpn: *mut ovpn_priv,
    pub dev_tracker: netdevice_tracker,
    pub id: u32,
    pub tx_id: u32,
    pub ipv4: in_addr,
    pub ipv6: in6_addr,
    pub vpn_addrs: },
    pub hash_entry_id: hlist_node,
    pub hash_entry_addr4: hlist_nulls_node,
    pub hash_entry_addr6: hlist_nulls_node,
    pub hash_entry_transp_addr: hlist_nulls_node,
    pub sock: *mut ovpn_socket __rcu,
    pub strp: strparser,
    pub user_queue: sk_buff_head,
    pub out_queue: sk_buff_head,
    pub tx_in_progress: bool,
    pub skb: *mut sk_buff,
    pub offset: c_int,
    pub len: c_int,
    pub out_msg: },
    pub sk): *mut *mut void (sk_data_ready)(struct sock,
    pub sk): *mut *mut void (sk_write_space)(struct sock,
    pub prot: *mut proto,
    pub ops: *const proto_ops,
    pub sk_cb: },
    pub defer_del_work: work_struct,
    pub tcp: },
    pub crypto: ovpn_crypto_state,
    pub dst_cache: dst_cache,
    pub bind: *mut ovpn_bind __rcu,
    pub keepalive_interval: c_ulong,
    pub keepalive_xmit_exp: c_ulong,
    pub last_sent: time64_t,
    pub keepalive_timeout: c_ulong,
    pub keepalive_recv_exp: c_ulong,
    pub last_recv: time64_t,
    pub vpn_stats: ovpn_peer_stats,
    pub link_stats: ovpn_peer_stats,
    pub delete_reason: ovpn_del_peer_reason,
    pub /: *mut *mut *mut spinlock_t lock; / protects bind and keepalive,
    pub refcount: kref,
    pub rcu: rcu_head,
    pub release_entry: llist_node,
    pub keepalive_work: work_struct,
}

//
// ovpn_peer_hold - increase reference counter
// @peer: the peer whose counter should be increased
//
// Return: true if the counter was increased or false if it was zero already
//
extern "C" {
    pub fn kref_get_unless_zero(_arg: &peer->refcount) -> return;
}
extern "C" {
    pub fn ovpn_peer_release_kref(kref: *mut kref);
}
//
// ovpn_peer_put - decrease reference counter
// @peer: the peer whose counter should be decreased
//
extern "C" {
    pub fn ovpn_peer_add(ovpn: *mut ovpn_priv, peer: *mut ovpn_peer) -> c_int;
}
extern "C" {
    pub fn ovpn_peer_del(peer: *mut ovpn_peer, reason: ovpn_del_peer_reason) -> c_int;
}
extern "C" {
    pub fn ovpn_peer_hash_vpn_ip(peer: *mut ovpn_peer);
}
extern "C" {
    pub fn ovpn_peer_hash_transp_addr(peer: *mut ovpn_peer);
}
extern "C" {
    pub fn ovpn_peer_keepalive_set(peer: *mut ovpn_peer, interval: u32, timeout: u32);
}
extern "C" {
    pub fn ovpn_peer_keepalive_work(work: *mut work_struct);
}
extern "C" {
    pub fn ovpn_peer_endpoints_update(peer: *mut ovpn_peer, skb: *mut sk_buff);
}
