//! Automatically rewritten from C Header to Rust Module
//! Source: net/can/af_can.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright (c) 2002-2007 Volkswagen Group Electronic Research
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of Volkswagen nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
//
// Alternatively, provided that this notice is retained in full, this
// software may be distributed under the terms of the GNU General
// Public License ("GPL") version 2, in which case the provisions of the
// GPL apply INSTEAD OF those given above.
//
// The provided data structures and external interfaces from this code
// are not restricted to be used by modules with a GPL compatible license.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
// DAMAGE.
//

// af_can rx dispatcher structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct receiver {
    pub list: hlist_node,
    pub can_id: canid_t,
    pub mask: canid_t,
    pub matches: atomic_long_t,
    pub data): *mut *mut *mut void (func)(struct sk_buff skb, void,
    pub data: *mut c_void,
    pub ident: *mut c_char,
    pub sk: *mut sock,
    pub rcu: rcu_head,
}

// statistic structures
// can be reset e.g. by can_init_stats()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_pkg_stats {
    pub jiffies_init: c_ulong,
    pub rx_frames: atomic_long_t,
    pub tx_frames: atomic_long_t,
    pub matches: atomic_long_t,
    pub total_rx_rate: c_ulong,
    pub total_tx_rate: c_ulong,
    pub total_rx_match_ratio: c_ulong,
    pub current_rx_rate: c_ulong,
    pub current_tx_rate: c_ulong,
    pub current_rx_match_ratio: c_ulong,
    pub max_rx_rate: c_ulong,
    pub max_tx_rate: c_ulong,
    pub max_rx_match_ratio: c_ulong,
    pub rx_frames_delta: atomic_long_t,
    pub tx_frames_delta: atomic_long_t,
    pub matches_delta: atomic_long_t,
}

// persistent statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_rcv_lists_stats {
    pub stats_reset: c_ulong,
    pub user_reset: c_ulong,
    pub rcv_entries: c_ulong,
    pub rcv_entries_max: c_ulong,
}

// function prototypes for the CAN networklayer procfs (proc.c)
extern "C" {
    pub fn can_init_proc(net: *mut net);
}
extern "C" {
    pub fn can_remove_proc(net: *mut net);
}
extern "C" {
    pub fn can_stat_update(t: *mut timer_list);
}
