//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/tsnmap.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// SCTP kernel implementation
// (C) Copyright IBM Corp. 2001, 2004
// Copyright (c) 1999-2000 Cisco, Inc.
// Copyright (c) 1999-2001 Motorola, Inc.
// Copyright (c) 2001 Intel Corp.
//
// This file is part of the SCTP kernel implementation
//
// These are the definitions needed for the tsnmap type.  The tsnmap is used
// to track out of order TSNs received.
//
// Please send any bug reports or fixes you make to the
// email address(es):
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// Jon Grimm             <jgrimm@us.ibm.com>
// La Monte H.P. Yarroll <piggy@acm.org>
// Karl Knutson          <karl@athena.chicago.il.us>
// Sridhar Samudrala     <sri@us.ibm.com>
//

// Macro flag: #define __sctp_tsnmap_h__
// RFC 2960 12.2 Parameters necessary per association (i.e. the TCB)
// Mapping  An array of bits or bytes indicating which out of
// Array    order TSN's have been received (relative to the
// Last Rcvd TSN). If no gaps exist, i.e. no out of
// order packets have been received, this array
// will be set to all zero. This structure may be
// in the form of a circular buffer or bit array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_tsnmap {
// This array counts the number of chunks with each TSN.
// It points at one of the two buffers with which we will
// ping-pong between.
//
    pub tsn_map: *mut c_ulong,
// This is the TSN at tsn_map[0].
    pub base_tsn: __u32,
// Last Rcvd   : This is the last TSN received in
// TSN	       : sequence. This value is set initially by
// : taking the peer's Initial TSN, received in
// : the INIT or INIT ACK chunk, and subtracting
// : one from it.
//
// Throughout most of the specification this is called the
// "Cumulative TSN ACK Point".  In this case, we
// ignore the advice in 12.2 in favour of the term
// used in the bulk of the text.
//
    pub cumulative_tsn_ack_point: __u32,
// This is the highest TSN we've marked.
    pub max_tsn_seen: __u32,
// This is the minimum number of TSNs we can track.  This corresponds
// to the size of tsn_map.   Note: the overflow_map allows us to
// potentially track more than this quantity.
//
    pub len: __u16,
// Data chunks pending receipt. used by SCTP_STATUS sockopt
    pub pending_data: __u16,
// Record duplicate TSNs here.  We clear this after
// every SACK.  Store up to SCTP_MAX_DUP_TSNS worth of
// information.
//
    pub num_dup_tsns: __u16,
    pub dup_tsns: [__be32; SCTP_MAX_DUP_TSNS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_tsnmap_iter {
    pub start: __u32,
}

// Initialize a block of memory as a tsnmap.
extern "C" {
    pub fn sctp_tsnmap_free(map: *mut sctp_tsnmap);
}
// Test the tracking state of this TSN.
// Returns:
// 0 if the TSN has not yet been seen
// >0 if the TSN has been seen (duplicate)
// <0 if the TSN is invalid (too large to track)
//
extern "C" {
    pub fn sctp_tsnmap_check(: *const sctp_tsnmap, tsn: __u32) -> c_int;
}
// Mark this TSN as seen.
// Mark this TSN and all lower as seen.
extern "C" {
    pub fn sctp_tsnmap_skip(map: *mut sctp_tsnmap, tsn: __u32);
}
// Retrieve the Cumulative TSN ACK Point.
// Retrieve the highest TSN we've seen.
// How many duplicate TSNs are stored?
// Return pointer to duplicate tsn array as needed by SACK.
// How many gap ack blocks do we have recorded?
// Refresh the count on pending data.
extern "C" {
    pub fn sctp_tsnmap_pending(map: *mut sctp_tsnmap) -> __u16;
}
// Is there a gap in the TSN map?
// Mark a duplicate TSN.  Note:  limit the storage of duplicate TSN
// information.
//
// Renege a TSN that was seen.
extern "C" {
    pub fn sctp_tsnmap_renege(: *mut sctp_tsnmap, tsn: __u32);
}
// Is there a gap in the TSN map?
extern "C" {
    pub fn sctp_tsnmap_has_gap(: *const sctp_tsnmap) -> c_int;
}
