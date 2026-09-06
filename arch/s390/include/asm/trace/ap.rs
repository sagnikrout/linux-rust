//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/trace/ap.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Tracepoint definitions for s390 ap bus related trace events
//
// There are two AP bus related tracepoint events defined here:
// There is a tracepoint s390_ap_nqap event immediately after a request
// has been pushed into the AP firmware queue with the NQAP AP command.
// The other tracepoint s390_ap_dqap event fires immediately after a
// reply has been pulled out of the AP firmware queue via DQAP AP command.
// The idea of these two trace events focuses on performance to measure
// the runtime of a crypto request/reply as close as possible at the
// firmware level. In combination with the two zcrypt tracepoints (see the
// zcrypt.h trace event definition file) this gives measurement data about
// the runtime of a request/reply within the zcrpyt and AP bus layer.
//

//
// trace_s390_ap_nqap - ap msg nqap tracepoint function
// @card:   Crypto card number addressed.
// @dom:    Domain within the crypto card addressed.
// @status: AP queue status (GR1 on return of nqap).
// @psmid:  Unique id identifying this request/reply.
//
// Called immediately after a request has been enqueued into
// the AP firmware queue with the NQAP command.
//
// trace_s390_ap_dqap - ap msg dqap tracepoint function
// @card:  Crypto card number addressed.
// @dom:   Domain within the crypto card addressed.
// @status: AP queue status (GR1 on return of dqap).
// @psmid: Unique id identifying this request/reply.
//
// Called immediately after a reply has been dequeued from
// the AP firmware queue with the DQAP command.
//

// This part must be outside protection

