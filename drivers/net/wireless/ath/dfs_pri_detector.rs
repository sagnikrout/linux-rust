//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/dfs_pri_detector.h
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
// Copyright (c) 2012 Neratec Solutions AG
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

//
// struct pri_sequence - sequence of pulses matching one PRI
// @head: list_head
// @pri: pulse repetition interval (PRI) in usecs
// @dur: duration of sequence in usecs
// @count: number of pulses in this sequence
// @count_falses: number of not matching pulses in this sequence
// @first_ts: time stamp of first pulse in usecs
// @last_ts: time stamp of last pulse in usecs
// @deadline_ts: deadline when this sequence becomes invalid (first_ts + dur)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pri_sequence {
    pub head: list_head,
    pub pri: u32,
    pub dur: u32,
    pub count: u32,
    pub count_falses: u32,
    pub first_ts: u64,
    pub last_ts: u64,
    pub deadline_ts: u64,
}

//
// struct pri_detector - PRI detector element for a dedicated radar type
// @exit(): destructor
// @add_pulse(): add pulse event, returns pri_sequence if pattern was detected
// @reset(): clear states and reset to given time stamp
// @rs: detector specs for this detector element
// @last_ts: last pulse time stamp considered for this element in usecs
// @sequences: list_head holding potential pulse sequences
// @pulses: list connecting pulse_elem objects
// @count: number of pulses in queue
// @max_count: maximum number of pulses to be queued
// @window_size: window size back from newest pulse time stamp in usecs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pri_detector {
    pub de): *mut *mut void (exit) (struct pri_detector,
    pub e): *mut *mut *mut (add_pulse)(struct pri_detector de, struct pulse_event,
    pub ts): *mut *mut *mut void (reset) (struct pri_detector de, u64,
    pub rs: *const radar_detector_specs,
// private: internal use only
    pub last_ts: u64,
    pub sequences: list_head,
    pub pulses: list_head,
    pub count: u32,
    pub max_count: u32,
    pub window_size: u32,
}
