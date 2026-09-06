//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/stream_sched.h
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
// (C) Copyright Red Hat Inc. 2017
//
// These are definitions used by the stream schedulers, defined in RFC
// draft ndata (https://tools.ietf.org/html/draft-ietf-tsvwg-sctp-ndata-11)
//
// Please send any bug reports or fixes you make to the
// email addresses:
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// Marcelo Ricardo Leitner <marcelo.leitner@gmail.com>
//

// Macro flag: #define __sctp_stream_sched_h__
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_sched_ops {
// Property handling for a given stream
    pub gfp): gfp_t,
    pub value): *mut *mut *mut int (get)(struct sctp_stream stream, __u16 sid, __u16,
// Init the specific scheduler
    pub stream): *mut *mut int (init)(struct sctp_stream,
// Init a stream
    pub gfp): *mut *mut *mut int (init_sid)(struct sctp_stream stream, __u16 sid, gfp_t,
// free a stream
    pub sid): *mut *mut *mut void (free_sid)(struct sctp_stream stream, __u16,
// Enqueue a chunk
    pub msg): *mut *mut *mut void (enqueue)(struct sctp_outq q, struct sctp_datamsg,
// Dequeue a chunk
    pub q): *mut *mut *mut sctp_chunk (dequeue)(sctp_outq,
// Called only if the chunk fit the packet
    pub chunk): *mut *mut *mut void (dequeue_done)(struct sctp_outq q, struct sctp_chunk,
// Schedule all chunks already enqueued
    pub stream): *mut *mut void (sched_all)(struct sctp_stream,
// Unschedule all chunks already enqueued
    pub stream): *mut *mut void (unsched_all)(struct sctp_stream,
}

extern "C" {
    pub fn sctp_sched_get_sched(asoc: *mut sctp_association) -> c_int;
}
extern "C" {
    pub fn sctp_sched_dequeue_done(q: *mut sctp_outq, ch: *mut sctp_chunk);
}
extern "C" {
    pub fn sctp_sched_dequeue_common(q: *mut sctp_outq, ch: *mut sctp_chunk);
}
extern "C" {
    pub fn sctp_sched_init_sid(stream: *mut sctp_stream, sid: __u16, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn sctp_sched_ops_prio_init();
}
extern "C" {
    pub fn sctp_sched_ops_rr_init();
}
extern "C" {
    pub fn sctp_sched_ops_fc_init();
}
extern "C" {
    pub fn sctp_sched_ops_wfq_init();
}
