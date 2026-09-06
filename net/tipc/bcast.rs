//! Automatically rewritten from C Header to Rust Module
//! Source: net/tipc/bcast.h
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
// net/tipc/bcast.h: Include file for TIPC broadcast code
//
// Copyright (c) 2003-2006, 2014-2015, Ericsson AB
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

pub const BCLINK_MODE_BCAST: c_uint = 0x1;
pub const BCLINK_MODE_RCAST: c_uint = 0x2;
pub const BCLINK_MODE_SEL: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_nlist {
    pub list: list_head,
    pub self: u32,
    pub remote: u16,
    pub local: bool,
}

extern "C" {
    pub fn tipc_nlist_init(nl: *mut tipc_nlist, self: u32);
}
extern "C" {
    pub fn tipc_nlist_purge(nl: *mut tipc_nlist);
}
extern "C" {
    pub fn tipc_nlist_add(nl: *mut tipc_nlist, node: u32);
}
extern "C" {
    pub fn tipc_nlist_del(nl: *mut tipc_nlist, node: u32);
}
// Cookie to be used between socket and broadcast layer
// @rcast: replicast (instead of broadcast) was used at previous xmit
// @mandatory: broadcast/replicast indication was set by user
// @deferredq: defer queue to make message in order
// @expires: re-evaluate non-mandatory transmit method if we are past this
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_mc_method {
    pub rcast: bool,
    pub mandatory: bool,
    pub deferredq: sk_buff_head,
    pub expires: c_ulong,
}

extern "C" {
    pub fn tipc_bcast_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn tipc_bcast_stop(net: *mut net);
}
extern "C" {
    pub fn tipc_bcast_remove_peer(net: *mut net, rcv_bcl: *mut tipc_link);
}
extern "C" {
    pub fn tipc_bcast_inc_bearer_dst_cnt(net: *mut net, bearer_id: c_int);
}
extern "C" {
    pub fn tipc_bcast_dec_bearer_dst_cnt(net: *mut net, bearer_id: c_int);
}
extern "C" {
    pub fn tipc_bcast_get_mtu(net: *mut net) -> c_int;
}
extern "C" {
    pub fn tipc_bcast_toggle_rcast(net: *mut net, supp: bool);
}
extern "C" {
    pub fn tipc_bcast_rcv(net: *mut net, l: *mut tipc_link, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn tipc_nl_bc_link_set(net: *mut net, attrs[]: *mut nlattr) -> c_int;
}
extern "C" {
    pub fn tipc_bclink_reset_stats(net: *mut net, l: *mut tipc_link) -> c_int;
}
extern "C" {
    pub fn tipc_bcast_get_mode(net: *mut net) -> u32;
}
extern "C" {
    pub fn tipc_bcast_get_broadcast_ratio(net: *mut net) -> u32;
}
