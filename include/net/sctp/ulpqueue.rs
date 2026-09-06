//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/ulpqueue.h
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
// Copyright (c) 2001 Nokia, Inc.
// Copyright (c) 2001 La Monte H.P. Yarroll
//
// These are the definitions needed for the sctp_ulpq type.  The
// sctp_ulpq is the interface between the Upper Layer Protocol, or ULP,
// and the core SCTP state machine.  This is the component which handles
// reassembly and ordering.
//
// Please send any bug reports or fixes you make to the
// email addresses:
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// Jon Grimm             <jgrimm@us.ibm.com>
// La Monte H.P. Yarroll <piggy@acm.org>
// Sridhar Samudrala     <sri@us.ibm.com>
//

// Macro flag: #define __sctp_ulpqueue_h__
// A structure to carry information to the ULP (e.g. Sockets API)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_ulpq {
    pub pd_mode: c_char,
    pub asoc: *mut sctp_association,
    pub reasm: sk_buff_head,
    pub reasm_uo: sk_buff_head,
    pub lobby: sk_buff_head,
}

// Prototypes.
extern "C" {
    pub fn sctp_ulpq_init(ulpq: *mut sctp_ulpq, asoc: *mut sctp_association);
}
extern "C" {
    pub fn sctp_ulpq_flush(ulpq: *mut sctp_ulpq);
}
extern "C" {
    pub fn sctp_ulpq_free(: *mut sctp_ulpq);
}
// Add a new DATA chunk for processing.
extern "C" {
    pub fn sctp_ulpq_tail_data(: *mut sctp_ulpq, : *mut sctp_chunk, _arg: gfp_t) -> c_int;
}
// Add a new event for propagation to the ULP.
extern "C" {
    pub fn sctp_ulpq_tail_event(: *mut sctp_ulpq, skb_list: *mut sk_buff_head) -> c_int;
}
// Renege previously received chunks.
extern "C" {
    pub fn sctp_ulpq_renege(: *mut sctp_ulpq, : *mut sctp_chunk, _arg: gfp_t);
}
// Perform partial delivery.
extern "C" {
    pub fn sctp_ulpq_partial_delivery(: *mut sctp_ulpq, _arg: gfp_t);
}
// Abort the partial delivery.
extern "C" {
    pub fn sctp_ulpq_abort_pd(: *mut sctp_ulpq, _arg: gfp_t);
}
// Clear the partial data delivery condition on this socket.
extern "C" {
    pub fn sctp_clear_pd(sk: *mut sock, asoc: *mut sctp_association) -> c_int;
}
// Skip over an SSN.
extern "C" {
    pub fn sctp_ulpq_skip(ulpq: *mut sctp_ulpq, sid: __u16, ssn: __u16);
}
extern "C" {
    pub fn sctp_ulpq_reasm_flushtsn(: *mut sctp_ulpq, _arg: __u32);
}
