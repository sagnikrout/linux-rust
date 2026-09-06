//! Automatically rewritten from C Header to Rust Module
//! Source: net/tipc/node.h
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
// net/tipc/node.h: Include file for TIPC node management routines
//
// Copyright (c) 2000-2006, 2014-2016, Ericsson AB
// Copyright (c) 2005, 2010-2014, Wind River Systems
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

// Optional capabilities supported by this code version
//

extern "C" {
    pub fn tipc_node_stop(net: *mut net);
}
extern "C" {
    pub fn tipc_node_get_id(net: *mut net, addr: u32, id: *mut u8) -> bool;
}
extern "C" {
    pub fn tipc_node_get_addr(node: *mut tipc_node) -> u32;
}
extern "C" {
    pub fn tipc_node_put(node: *mut tipc_node);
}
extern "C" {
    pub fn tipc_node_get(node: *mut tipc_node);
}

extern "C" {
    pub fn tipc_node_try_addr(net: *mut net, id: *mut u8, addr: u32) -> u32;
}
extern "C" {
    pub fn tipc_node_delete_links(net: *mut net, bearer_id: c_int);
}
extern "C" {
    pub fn tipc_node_apply_property(net: *mut net, b: *mut tipc_bearer, prop: c_int);
}
extern "C" {
    pub fn tipc_node_distr_xmit(net: *mut net, list: *mut sk_buff_head) -> c_int;
}
extern "C" {
    pub fn tipc_node_subscribe(net: *mut net, subscr: *mut list_head, addr: u32);
}
extern "C" {
    pub fn tipc_node_unsubscribe(net: *mut net, subscr: *mut list_head, addr: u32);
}
extern "C" {
    pub fn tipc_node_broadcast(net: *mut net, skb: *mut sk_buff, rc_dests: c_int);
}
extern "C" {
    pub fn tipc_node_add_conn(net: *mut net, dnode: u32, port: u32, peer_port: u32) -> c_int;
}
extern "C" {
    pub fn tipc_node_remove_conn(net: *mut net, dnode: u32, port: u32);
}
extern "C" {
    pub fn tipc_node_get_mtu(net: *mut net, addr: u32, sel: u32, connected: bool) -> c_int;
}
extern "C" {
    pub fn tipc_node_is_up(net: *mut net, addr: u32) -> bool;
}
extern "C" {
    pub fn tipc_node_get_capabilities(net: *mut net, addr: u32) -> u16;
}
extern "C" {
    pub fn tipc_nl_node_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn tipc_nl_node_dump_link(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn tipc_nl_node_reset_link_stats(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_node_get_link(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_node_set_link(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_peer_rm(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_node_set_monitor(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_node_get_monitor(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_node_dump_monitor(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}

extern "C" {
    pub fn tipc_nl_node_set_key(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_nl_node_flush_key(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}

extern "C" {
    pub fn tipc_node_pre_cleanup_net(exit_net: *mut net);
}
