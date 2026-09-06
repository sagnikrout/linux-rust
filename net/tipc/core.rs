//! Automatically rewritten from C Header to Rust Module
//! Source: net/tipc/core.h
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
// net/tipc/core.h: Include file for TIPC global declarations
//
// Copyright (c) 2005-2006, 2013-2018 Ericsson AB
// Copyright (c) 2005-2007, 2010-2013, Wind River Systems
// Copyright (c) 2020, Red Hat Inc
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

pub const NODE_HTABLE_SIZE: c_int = 512;
pub const MAX_BEARERS: c_int = 3;
pub const TIPC_DEF_MON_THRESHOLD: c_int = 32;
pub const NODE_ID_LEN: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_net {
    pub node_id: [u8; NODE_ID_LEN],
    pub node_addr: u32,
    pub trial_addr: u32,
    pub addr_trial_end: c_ulong,
    pub node_id_string: [c_char; NODE_ID_STR_LEN],
    pub net_id: c_int,
    pub random: c_int,
    pub legacy_addr_format: bool,
// Node table and node list
    pub node_list_lock: spinlock_t,
    pub node_htable: [hlist_head; NODE_HTABLE_SIZE],
    pub node_list: list_head,
    pub num_nodes: u32,
    pub num_links: u32,
// Neighbor monitoring list
    pub monitors: [*mut tipc_monitor; MAX_BEARERS],
    pub mon_threshold: c_int,
// Bearer list
    pub 1]: *mut *mut tipc_bearer __rcu bearer_list[MAX_BEARERS +,
// Broadcast link
    pub bclock: spinlock_t,
    pub bcbase: *mut tipc_bc_base,
    pub bcl: *mut tipc_link,
// Socket hash table
    pub sk_rht: rhashtable,
// Name table
    pub nametbl_lock: spinlock_t,
    pub nametbl: *mut name_table,
// Topology subscription server
    pub topsrv: *mut tipc_topsrv,
    pub subscription_count: core::sync::atomic::AtomicI32,
// Cluster capabilities
    pub capabilities: u16,
// Tracing of node internal messages
    pub loopback_pt: packet_type,

// TX crypto handler
    pub crypto_tx: *mut tipc_crypto,

// Work item for net finalize
    pub work: work_struct,
// The numbers of work queues in schedule
    pub wq_count: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn net_generic(_arg: net, _arg: tipc_net_id) -> return;
}
extern "C" {
    pub fn less_eq(_arg: left, mod(left): right) && (mod(right) !=) -> return;
}
extern "C" {
    pub fn ntohl(tmp[3]: tmp[0] | tmp[1] | tmp[2] |) -> return;
}

extern "C" {
    pub fn tipc_register_sysctl() -> c_int;
}
extern "C" {
    pub fn tipc_unregister_sysctl();
}

pub const tipc_register_sysctl(): c_int = 0;
// Macro flag: #define tipc_unregister_sysctl()

