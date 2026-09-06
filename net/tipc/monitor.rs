//! Automatically rewritten from C Header to Rust Module
//! Source: net/tipc/monitor.h
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
// net/tipc/monitor.h
//
// Copyright (c) 2015, Ericsson AB
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

// struct tipc_mon_state: link instance's cache of monitor list and domain state
// @list_gen: current generation of this node's monitor list
// @gen: current generation of this node's local domain
// @peer_gen: most recent domain generation received from peer
// @acked_gen: most recent generation of self's domain acked by peer
// @monitoring: this peer endpoint should continuously monitored
// @probing: peer endpoint should be temporarily probed for potential loss
// @synched: domain record's generation has been synched with peer after reset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_mon_state {
    pub list_gen: u16,
    pub peer_gen: u16,
    pub acked_gen: u16,
    pub :1: bool monitoring,
    pub :1: bool probing,
    pub :1: bool reset,
    pub :1: bool synched,
}

extern "C" {
    pub fn tipc_mon_create(net: *mut net, bearer_id: c_int) -> c_int;
}
extern "C" {
    pub fn tipc_mon_delete(net: *mut net, bearer_id: c_int);
}
extern "C" {
    pub fn tipc_mon_peer_up(net: *mut net, addr: u32, bearer_id: c_int);
}
extern "C" {
    pub fn tipc_mon_peer_down(net: *mut net, addr: u32, bearer_id: c_int);
}
extern "C" {
    pub fn tipc_mon_remove_peer(net: *mut net, addr: u32, bearer_id: c_int);
}
extern "C" {
    pub fn tipc_nl_monitor_set_threshold(net: *mut net, cluster_size: u32) -> c_int;
}
extern "C" {
    pub fn tipc_nl_monitor_get_threshold(net: *mut net) -> c_int;
}
extern "C" {
    pub fn tipc_mon_reinit_self(net: *mut net);
}
