//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/sm.h
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
// These are definitions needed by the state machine.
//
// Please send any bug reports or fixes you make to the
// email addresses:
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// La Monte H.P. Yarroll <piggy@acm.org>
// Karl Knutson <karl@athena.chicago.il.us>
// Xingang Guo <xingang.guo@intel.com>
// Jon Grimm <jgrimm@us.ibm.com>
// Dajiang Zhang <dajiang.zhang@nokia.com>
// Sridhar Samudrala <sri@us.ibm.com>
// Daisy Chang <daisyc@us.ibm.com>
// Ardelle Fan <ardelle.fan@intel.com>
// Kevin Gao <kevin.gao@intel.com>
//

// Macro flag: #define __sctp_sm_h__
//
// Possible values for the disposition are:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_disposition {
    SCTP_DISPOSITION_DISCARD,	 /* No further processing.  */
    SCTP_DISPOSITION_CONSUME,	 /* Process return values normally.  */
    SCTP_DISPOSITION_NOMEM,		 /* We ran out of memory--recover.  */
    SCTP_DISPOSITION_DELETE_TCB,	 /* Close the association.  */
    SCTP_DISPOSITION_ABORT,		 /* Close the association NOW.  */
    SCTP_DISPOSITION_VIOLATION,	 /* The peer is misbehaving.  */
    SCTP_DISPOSITION_NOT_IMPL,	 /* This entry is not implemented.  */
    SCTP_DISPOSITION_ERROR,		 /* This is plain old user error.  */
    SCTP_DISPOSITION_BUG,		 /* This is a bug.  */
}

extern "C" {
    pub fn void(: *mut sctp_timer_event_t) (struct timer_list) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sm_table_entry {
    pub fn: *mut sctp_state_fn_t,
    pub name: *const c_char,
}

// A naming convention of "sctp_sf_xxx" applies to all the state functions
// currently in use.
//
// Prototypes for generic state functions.
// Prototypes for gener timer state functions.
// Prototypes for chunk state functions.
// Prototypes for primitive event state functions.
// Prototypes for other event state functions.
// Prototypes for timeout event state functions.
// Prototypes for utility support functions.
// Prototypes for chunk-building functions.
extern "C" {
    pub fn sctp_init_cause(chunk: *mut sctp_chunk, cause: __be16, paylen: usize) -> c_int;
}
extern "C" {
    pub fn sctp_chunk_assign_tsn(chunk: *mut sctp_chunk);
}
extern "C" {
    pub fn sctp_chunk_assign_ssn(chunk: *mut sctp_chunk);
}
// Prototypes for stream-processing functions.
// Prototypes for statetable processing.
// 2nd level prototypes
extern "C" {
    pub fn sctp_generate_t3_rtx_event(t: *mut timer_list);
}
extern "C" {
    pub fn sctp_generate_heartbeat_event(t: *mut timer_list);
}
extern "C" {
    pub fn sctp_generate_reconf_event(t: *mut timer_list);
}
extern "C" {
    pub fn sctp_generate_probe_event(t: *mut timer_list);
}
extern "C" {
    pub fn sctp_generate_proto_unreach_event(t: *mut timer_list);
}
extern "C" {
    pub fn sctp_ootb_pkt_free(packet: *mut sctp_packet);
}
// 3rd level prototypes
extern "C" {
    pub fn sctp_generate_tag(ep: *const sctp_endpoint) -> __u32;
}
extern "C" {
    pub fn sctp_generate_tsn(ep: *const sctp_endpoint) -> __u32;
}
// Extern declarations for major data structures.
// Get the size of a DATA chunk payload.
// Compare two TSNs

// Compare two MIDs

// Compare two SSNs

// ADDIP 3.1.1

// Check VTAG of the packet matches the sender's own tag.
// RFC 2960 Sec 8.5 When receiving an SCTP packet, the endpoint
// MUST ensure that the value in the Verification Tag field of
// the received SCTP packet matches its own Tag. If the received
// Verification Tag value does not match the receiver's own
// tag value, the receiver shall silently discard the packet...
//
// Check VTAG of the packet matches the sender's own tag and the T bit is
// not set, OR its peer's tag and the T bit is set in the Chunk Flags.
//
// RFC 2960 Section 8.5.1, sctpimpguide Section 2.41
//
// B) The receiver of a ABORT MUST accept the packet
// if the Verification Tag field of the packet matches its own tag
// and the T bit is not set
// OR
// it is set to its peer's tag and the T bit is set in the Chunk
// Flags.
// Otherwise, the receiver MUST silently discard the packet
// and take no further action.
//
// C) The receiver of a SHUTDOWN COMPLETE shall accept the packet
// if the Verification Tag field of the packet matches its own tag
// and the T bit is not set
// OR
// it is set to its peer's tag and the T bit is set in the Chunk
// Flags.
// Otherwise, the receiver MUST silently discard the packet
// and take no further action.  An endpoint MUST ignore the
// SHUTDOWN COMPLETE if it is not in the SHUTDOWN-ACK-SENT state.
//
