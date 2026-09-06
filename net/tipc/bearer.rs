//! Automatically rewritten from C Header to Rust Module
//! Source: net/tipc/bearer.h
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


//
// net/tipc/bearer.h: Include file for TIPC bearer code
//
// Copyright (c) 1996-2006, 2013-2016, Ericsson AB
// Copyright (c) 2005, 2010-2011, Wind River Systems
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the names of the copyright holders nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//

pub const MAX_MEDIA: c_int = 3;
// Identifiers associated with TIPC message header media address info
// - address info field is 32 bytes long
// - the field's actual content and length is defined per media
// - remaining unused bytes in the field are set to zero
//
pub const TIPC_MEDIA_INFO_SIZE: c_int = 32;
pub const TIPC_MEDIA_TYPE_OFFSET: c_int = 3;
pub const TIPC_MEDIA_ADDR_OFFSET: c_int = 4;
//
// Identifiers of supported TIPC media types
//
pub const TIPC_MEDIA_TYPE_ETH: c_int = 1;
pub const TIPC_MEDIA_TYPE_IB: c_int = 2;
pub const TIPC_MEDIA_TYPE_UDP: c_int = 3;
// Minimum bearer MTU

// Identifiers for distinguishing between broadcast/multicast and replicast
//
pub const TIPC_BROADCAST_SUPPORT: c_int = 1;
pub const TIPC_REPLICAST_SUPPORT: c_int = 2;
//
// struct tipc_media_addr - destination address used by TIPC bearers
// @value: address info (format defined by media)
// @media_id: TIPC media type identifier
// @broadcast: non-zero if address is a broadcast address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_media_addr {
    pub value: [u8; TIPC_MEDIA_INFO_SIZE],
    pub media_id: u8,
    pub broadcast: u8,
}

//
// struct tipc_media - Media specific info exposed to generic bearer layer
// @send_msg: routine which handles buffer transmission
// @enable_media: routine which enables a media
// @disable_media: routine which disables a media
// @addr2str: convert media address format to string
// @addr2msg: convert from media addr format to discovery msg addr format
// @msg2addr: convert from discovery msg addr format to media addr format
// @raw2addr: convert from raw addr format to media addr format
// @priority: default link (and bearer) priority
// @tolerance: default time (in ms) before declaring link failure
// @min_win: minimum window (in packets) before declaring link congestion
// @max_win: maximum window (in packets) before declaring link congestion
// @mtu: max packet size bearer can support for media type not dependent on
// underlying device MTU
// @type_id: TIPC media identifier
// @hwaddr_len: TIPC media address len
// @name: media name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_media {
    pub dest): *mut tipc_media_addr,
    pub attr[]): *mut nlattr,
    pub b): *mut *mut void (disable_media)(struct tipc_bearer,
    pub bufsz): c_int,
    pub addr): *mut *mut *mut int (addr2msg)(char msg, struct tipc_media_addr,
    pub msg): *mut c_char,
    pub raw): *const c_char,
    pub priority: u32,
    pub tolerance: u32,
    pub min_win: u32,
    pub max_win: u32,
    pub mtu: u32,
    pub type_id: u32,
    pub hwaddr_len: u32,
    pub name: [c_char; TIPC_MAX_MEDIA_NAME],
}

//
// struct tipc_bearer - Generic TIPC bearer structure
// @media_ptr: pointer to additional media-specific information about bearer
// @mtu: max packet size bearer can support
// @addr: media-specific address associated with bearer
// @name: bearer name (format = media:interface)
// @media: ptr to media structure associated with bearer
// @bcast_addr: media address used in broadcasting
// @pt: packet type for bearer
// @rcu: rcu struct for tipc_bearer
// @priority: default link priority for bearer
// @min_win: minimum window (in packets) before declaring link congestion
// @max_win: maximum window (in packets) before declaring link congestion
// @tolerance: default link tolerance for bearer
// @domain: network domain to which links can be established
// @identity: array index of this bearer within TIPC bearer array
// @disc: ptr to link setup request
// @net_plane: network plane ('A' through 'H') currently associated with bearer
// @encap_hlen: encap headers length
// @up: bearer up flag (bit 0)
// @refcnt: tipc_bearer reference counter
//
// Note: media-specific code is responsible for initialization of the fields
// indicated below when a bearer is enabled; TIPC's generic bearer code takes
// care of initializing all other fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_bearer {
    pub /: *mut *mut *mut void __rcu media_ptr; / initialized by media,
    pub /: *mut *mut u32 mtu; / initialized by media,
    pub /: *mut *mut tipc_media_addr addr; / initialized by media,
    pub name: [c_char; TIPC_MAX_BEARER_NAME],
    pub media: *mut tipc_media,
    pub bcast_addr: tipc_media_addr,
    pub pt: packet_type,
    pub rcu: rcu_head,
    pub priority: u32,
    pub min_win: u32,
    pub max_win: u32,
    pub tolerance: u32,
    pub domain: u32,
    pub identity: u32,
    pub disc: *mut tipc_discoverer,
    pub net_plane: c_char,
    pub encap_hlen: u16,
    pub up: c_ulong,
    pub refcnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_bearer_names {
    pub media_name: [c_char; TIPC_MAX_MEDIA_NAME],
    pub if_name: [c_char; TIPC_MAX_IF_NAME],
}

//
// TIPC routines available to supported media types
//
extern "C" {
    pub fn tipc_rcv(net: *mut net, skb: *mut sk_buff, b: *mut tipc_bearer);
}
//
// Routines made available to TIPC by supported media types
//

extern "C" {
    pub fn tipc_nl_bearer_disable(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn __tipc_nl_bearer_disable(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_bearer_enable(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn __tipc_nl_bearer_enable(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_bearer_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn tipc_nl_bearer_get(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_bearer_set(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn __tipc_nl_bearer_set(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_bearer_add(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_media_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn tipc_nl_media_get(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_media_set(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn __tipc_nl_media_set(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_media_addr_printf(buf: *mut c_char, len: c_int, a: *mut tipc_media_addr) -> c_int;
}
extern "C" {
    pub fn tipc_bearer_hold(b: *mut tipc_bearer) -> bool;
}
extern "C" {
    pub fn tipc_bearer_put(b: *mut tipc_bearer);
}
extern "C" {
    pub fn tipc_disable_l2_media(b: *mut tipc_bearer);
}
extern "C" {
    pub fn tipc_bearer_add_dest(net: *mut net, bearer_id: u32, dest: u32);
}
extern "C" {
    pub fn tipc_bearer_remove_dest(net: *mut net, bearer_id: u32, dest: u32);
}
extern "C" {
    pub fn tipc_bearer_get_name(net: *mut net, name: *mut c_char, bearer_id: u32) -> c_int;
}
extern "C" {
    pub fn tipc_bearer_setup() -> c_int;
}
extern "C" {
    pub fn tipc_bearer_cleanup();
}
extern "C" {
    pub fn tipc_bearer_stop(net: *mut net);
}
extern "C" {
    pub fn tipc_bearer_mtu(net: *mut net, bearer_id: u32) -> c_int;
}
extern "C" {
    pub fn tipc_bearer_min_mtu(net: *mut net, bearer_id: u32) -> c_int;
}
extern "C" {
    pub fn tipc_bearer_bcast_support(net: *mut net, bearer_id: u32) -> bool;
}
extern "C" {
    pub fn tipc_clone_to_loopback(net: *mut net, pkts: *mut sk_buff_head);
}
extern "C" {
    pub fn tipc_attach_loopback(net: *mut net) -> c_int;
}
extern "C" {
    pub fn tipc_detach_loopback(net: *mut net);
}
// check if device MTU is too low for tipc headers
