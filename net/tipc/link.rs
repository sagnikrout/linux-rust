//! Automatically rewritten from C Header to Rust Module
//! Source: net/tipc/link.h
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
// net/tipc/link.h: Include file for TIPC link code
//
// Copyright (c) 1995-2006, 2013-2014, Ericsson AB
// Copyright (c) 2004-2005, 2010-2011, Wind River Systems
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

// TIPC-specific error codes
//

// Link FSM events:
//
// Events returned from link at packet reception or at timeout
//
// Starting value for maximum packet size negotiation on unicast links
// (unless bearer MTU is less)
//
pub const MAX_PKT_DEFAULT: c_int = 1500;
extern "C" {
    pub fn tipc_link_build_reset_msg(l: *mut tipc_link, xmitq: *mut sk_buff_head);
}
extern "C" {
    pub fn tipc_link_fsm_evt(l: *mut tipc_link, evt: c_int) -> c_int;
}
extern "C" {
    pub fn tipc_link_is_up(l: *mut tipc_link) -> bool;
}
extern "C" {
    pub fn tipc_link_peer_is_down(l: *mut tipc_link) -> bool;
}
extern "C" {
    pub fn tipc_link_is_reset(l: *mut tipc_link) -> bool;
}
extern "C" {
    pub fn tipc_link_is_establishing(l: *mut tipc_link) -> bool;
}
extern "C" {
    pub fn tipc_link_is_synching(l: *mut tipc_link) -> bool;
}
extern "C" {
    pub fn tipc_link_is_failingover(l: *mut tipc_link) -> bool;
}
extern "C" {
    pub fn tipc_link_is_blocked(l: *mut tipc_link) -> bool;
}
extern "C" {
    pub fn tipc_link_set_active(l: *mut tipc_link, active: bool);
}
extern "C" {
    pub fn tipc_link_reset(l: *mut tipc_link);
}
extern "C" {
    pub fn tipc_link_reset_stats(l: *mut tipc_link);
}
extern "C" {
    pub fn tipc_link_rcv_nxt(l: *mut tipc_link) -> u16;
}
extern "C" {
    pub fn tipc_link_acked(l: *mut tipc_link) -> u16;
}
extern "C" {
    pub fn tipc_link_id(l: *mut tipc_link) -> u32;
}
extern "C" {
    pub fn tipc_link_state(l: *mut tipc_link) -> u32;
}
extern "C" {
    pub fn tipc_link_plane(l: *mut tipc_link) -> c_char;
}
extern "C" {
    pub fn tipc_link_prio(l: *mut tipc_link) -> c_int;
}
extern "C" {
    pub fn tipc_link_min_win(l: *mut tipc_link) -> c_int;
}
extern "C" {
    pub fn tipc_link_max_win(l: *mut tipc_link) -> c_int;
}
extern "C" {
    pub fn tipc_link_update_caps(l: *mut tipc_link, capabilities: u16);
}
extern "C" {
    pub fn tipc_link_validate_msg(l: *mut tipc_link, hdr: *mut tipc_msg) -> bool;
}
extern "C" {
    pub fn tipc_link_tolerance(l: *mut tipc_link) -> c_ulong;
}
extern "C" {
    pub fn tipc_link_set_abort_limit(l: *mut tipc_link, limit: u32);
}
extern "C" {
    pub fn tipc_link_set_queue_limits(l: *mut tipc_link, min_win: u32, max_win: u32);
}
extern "C" {
    pub fn tipc_nl_parse_link_prop(prop: *mut nlattr, props[]: *mut nlattr) -> c_int;
}
extern "C" {
    pub fn tipc_link_timeout(l: *mut tipc_link, xmitq: *mut sk_buff_head) -> c_int;
}
extern "C" {
    pub fn tipc_link_build_state_msg(l: *mut tipc_link, xmitq: *mut sk_buff_head) -> c_int;
}
extern "C" {
    pub fn tipc_link_bc_peers(l: *mut tipc_link) -> c_int;
}
extern "C" {
    pub fn tipc_link_set_mtu(l: *mut tipc_link, mtu: c_int);
}
extern "C" {
    pub fn tipc_link_mtu(l: *mut tipc_link) -> c_int;
}
extern "C" {
    pub fn tipc_link_mss(l: *mut tipc_link) -> c_int;
}
extern "C" {
    pub fn tipc_link_bc_init_rcv(l: *mut tipc_link, hdr: *mut tipc_msg);
}
extern "C" {
    pub fn tipc_link_too_silent(l: *mut tipc_link) -> bool;
}
