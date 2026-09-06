//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/codel.h
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
// Codel - The Controlled-Delay Active Queue Management algorithm
//
// Copyright (C) 2011-2012 Kathleen Nichols <nichols@pollere.com>
// Copyright (C) 2011-2012 Van Jacobson <van@pollere.net>
// Copyright (C) 2012 Michael D. Taht <dave.taht@bufferbloat.net>
// Copyright (C) 2012,2015 Eric Dumazet <edumazet@google.com>
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. The names of the authors may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// Alternatively, provided that this notice is retained in full, this
// software may be distributed under the terms of the GNU General
// Public License ("GPL") version 2, in which case the provisions of the
// GPL apply INSTEAD OF those given above.
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

// Controlling Queue Delay (CoDel) algorithm
// =========================================
// Source : Kathleen Nichols and Van Jacobson
// http://queue.acm.org/detail.cfm?id=2209336
//
// Implemented on linux by Dave Taht and Eric Dumazet
//
// CoDel uses a 1024 nsec clock, encoded in u32
// This gives a range of 2199 seconds, because of signed compares
//
pub type codel_time_t = u32;
pub type codel_tdiff_t = i32;
pub const CODEL_SHIFT: c_int = 10;

// Dealing with timer wrapping, according to RFC 1982, as desc in wikipedia:
// https://en.wikipedia.org/wiki/Serial_number_arithmetic#General_Solution
// codel_time_after(a,b) returns true if the time a is after time b.
//

//
// struct codel_params - contains codel parameters
// @target:	target queue size (in time units)
// @ce_threshold:  threshold for marking packets with ECN CE
// @interval:	width of moving time window
// @mtu:	device mtu, or minimal queue backlog in bytes.
// @ecn:	is Explicit Congestion Notification enabled
// @ce_threshold_selector: apply ce_threshold to packets matching this value
// in the diffserv/ECN byte of the IP header
// @ce_threshold_mask: mask to apply to ce_threshold_selector comparison
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct codel_params {
    pub target: codel_time_t,
    pub ce_threshold: codel_time_t,
    pub interval: codel_time_t,
    pub mtu: u32,
    pub ecn: bool,
    pub ce_threshold_selector: u8,
    pub ce_threshold_mask: u8,
}

//
// struct codel_vars - contains codel variables
// @count:		how many drops we've done since the last time we
// entered dropping state
// @lastcount:		count at entry to dropping state
// @dropping:		set to true if in dropping state
// @rec_inv_sqrt:	reciprocal value of sqrt(count) >> 1
// @first_above_time:	when we went (or will go) continuously above target
// for interval
// @drop_next:		time to drop next packet, or when we dropped last
// @ldelay:		sojourn time of last dequeued packet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct codel_vars {
    pub count: u32,
    pub lastcount: u32,
    pub dropping: bool,
    pub rec_inv_sqrt: u16,
    pub first_above_time: codel_time_t,
    pub drop_next: codel_time_t,
    pub ldelay: codel_time_t,
}

// needed shift to get a Q0.32 number from rec_inv_sqrt

//
// struct codel_stats - contains codel shared variables and stats
// @maxpacket:	largest packet we've seen so far
// @drop_count:	temp count of dropped packets in dequeue()
// @drop_len:	bytes of dropped packets in dequeue()
// @ecn_mark:	number of packets we ECN marked instead of dropping
// @ce_mark:	number of packets CE marked because sojourn time was above ce_threshold
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct codel_stats {
    pub maxpacket: u32,
    pub drop_count: u32,
    pub drop_len: u32,
    pub ecn_mark: u32,
    pub ce_mark: u32,
}

extern "C" {
    pub fn u32(skb: *const *const codel_skb_len_t)(struct sk_buff) -> typedef;
}
extern "C" {
    pub fn codel_time_t(skb: *const *const codel_skb_time_t)(struct sk_buff) -> typedef;
}
extern "C" {
    pub fn void(skb: *mut *mut codel_skb_drop_t)(struct sk_buff, ctx: *mut c_void) -> typedef;
}
