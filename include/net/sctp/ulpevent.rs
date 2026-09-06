//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/ulpevent.h
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
// These are the definitions needed for the sctp_ulpevent type.  The
// sctp_ulpevent type is used to carry information from the state machine
// upwards to the ULP.
//
// This file is part of the SCTP kernel implementation
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

// Macro flag: #define __sctp_ulpevent_h__
// A structure to carry information to the ULP (e.g. Sockets API)
// Warning: This sits inside an skb.cb[] area.  Be very careful of
// growing this structure as it is at the maximum limit now.
//
// sctp_ulpevent is saved in sk->cb(48 bytes), whose last 4 bytes
// have been taken by sock_skb_cb, So here it has to use 'packed'
// to make sctp_ulpevent fit into the rest 44 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_ulpevent {
    pub asoc: *mut sctp_association,
    pub chunk: *mut sctp_chunk,
    pub rmem_len: c_uint,
    pub mid: __u32,
    pub ssn: __u16,
}

// Retrieve the skb this event sits inside of.
extern "C" {
    pub fn container_of()ev: *mut (void, sk_buff: struct, _arg: cb) -> return;
}
// Retrieve & cast the event sitting inside the skb.
extern "C" {
    pub fn sctp_ulpevent_free(: *mut sctp_ulpevent);
}
extern "C" {
    pub fn sctp_ulpevent_is_notification(: *const sctp_ulpevent) -> c_int;
}
extern "C" {
    pub fn sctp_queue_purge_ulpevents(list: *mut sk_buff_head) -> c_uint;
}
extern "C" {
    pub fn sctp_ulpevent_get_notification_type(event: *const sctp_ulpevent) -> __u16;
}
// subscribe |=  (1 << (sn_type - SCTP_SN_TYPE_BASE));
// subscribe &= ~(1 << (sn_type - SCTP_SN_TYPE_BASE));
// Is this event type enabled?
// Given an event subscription, is this event enabled?
extern "C" {
    pub fn sctp_ulpevent_type_enabled(_arg: subscribe, _arg: sn_type) -> return;
}
