//! Automatically rewritten from C Header to Rust Module
//! Source: net/tipc/name_table.h
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
// net/tipc/name_table.h: Include file for TIPC name table code
//
// Copyright (c) 2000-2006, 2014-2018, Ericsson AB
// Copyright (c) 2004-2005, 2010-2011, Wind River Systems
// Copyright (c) 2020-2021, Red Hat Inc
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
// TIPC name types reserved for internal TIPC use (both current and planned)
//

//
// struct publication - info about a published service address or range
// @sr: service range represented by this publication
// @sk: address of socket bound to this publication
// @scope: scope of publication, TIPC_NODE_SCOPE or TIPC_CLUSTER_SCOPE
// @key: publication key, unique across the cluster
// @id: publication id
// @binding_node: all publications from the same node which bound this one
// - Remote publications: in node->publ_list;
// Used by node/name distr to withdraw publications when node is lost
// - Local/node scope publications: in name_table->node_scope list
// - Local/cluster scope publications: in name_table->cluster_scope list
// @binding_sock: all publications from the same socket which bound this one
// Used by socket to withdraw publications when socket is unbound/released
// @local_publ: list of identical publications made from this node
// Used by closest_first and multicast receive lookup algorithms
// @all_publ: all publications identical to this one, whatever node and scope
// Used by round-robin lookup algorithm
// @list: to form a list of publications in temporal order
// @rcu: RCU callback head used for deferred freeing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct publication {
    pub sr: tipc_service_range,
    pub sk: tipc_socket_addr,
    pub scope: u16,
    pub key: u32,
    pub id: u32,
    pub binding_node: list_head,
    pub binding_sock: list_head,
    pub local_publ: list_head,
    pub all_publ: list_head,
    pub list: list_head,
    pub rcu: rcu_head,
}

//
// struct name_table - table containing all existing port name publications
// @rcu: RCU callback head used for deferred freeing
// @services: name sequence hash lists
// @node_scope: all local publications with node scope
// - used by name_distr during re-init of name table
// @cluster_scope: all local publications with cluster scope
// - used by name_distr to send bulk updates to new nodes
// - used by name_distr during re-init of name table
// @cluster_scope_lock: lock for accessing @cluster_scope
// @local_publ_count: number of publications issued by this node
// @rc_dests: destination node counter
// @snd_nxt: next sequence number to be used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct name_table {
    pub rcu: rcu_head,
    pub services: [hlist_head; TIPC_NAMETBL_SIZE],
    pub node_scope: list_head,
    pub cluster_scope: list_head,
    pub cluster_scope_lock: rwlock_t,
    pub local_publ_count: u32,
    pub rc_dests: u32,
    pub snd_nxt: u32,
}

extern "C" {
    pub fn tipc_nl_name_table_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn tipc_nametbl_subscribe(s: *mut tipc_subscription) -> bool;
}
extern "C" {
    pub fn tipc_nametbl_unsubscribe(s: *mut tipc_subscription);
}
extern "C" {
    pub fn tipc_nametbl_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn tipc_nametbl_stop(net: *mut net);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_dest {
    pub list: list_head,
    pub port: u32,
    pub node: u32,
}

extern "C" {
    pub fn tipc_dest_push(l: *mut list_head, node: u32, port: u32) -> bool;
}
extern "C" {
    pub fn tipc_dest_pop(l: *mut list_head, node: *mut u32, port: *mut u32) -> bool;
}
extern "C" {
    pub fn tipc_dest_del(l: *mut list_head, node: u32, port: u32) -> bool;
}
extern "C" {
    pub fn tipc_dest_list_purge(l: *mut list_head);
}
