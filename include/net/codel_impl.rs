//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/codel_impl.h
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

//
// http://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Iterative_methods_for_reciprocal_square_roots
// new_invsqrt = (invsqrt / 2) * (3 - count * invsqrt^2)
//
// Here, invsqrt is a fixed point number (< 1.0), 32bit mantissa, aka Q0.32
//
// CoDel control_law is t + interval/sqrt(count)
// We maintain in rec_inv_sqrt the reciprocal value of sqrt(count) to avoid
// both sqrt() and divide operation.
//
// backlog <= params->mtu) {
// went below - stay below for at least interval
// just went above from below. If we stay above
// for at least interval we'll say it's ok to drop
//
// sojourn time below target - leave dropping state
// It's time for the next drop. Drop the current
// packet and dequeue the next. The dequeue might
// take us out of dropping state.
// If not, schedule the next drop.
// A large backlog might result in drop rates so high
// that the next drop should happen now,
// hence the while loop.
//
// dont care of possible wrap
// since there is no more divide.
//
// leave dropping state
// and schedule the next drop
// if min went above target close to when we last went below it
// assume that the drop rate that controlled the queue on the
// last cycle is a good starting point to control it now.
//
// we dont care if rec_inv_sqrt approximation
// is not very precise :
// Next Newton steps will correct it quadratically.
//
