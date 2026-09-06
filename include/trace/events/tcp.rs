//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/tcp.h
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

// p32 = inet->inet_saddr;
// p32 =  inet->inet_daddr;

//
// skb of trace_tcp_send_reset is the skb that caused RST. In case of
// active reset, skb should be NULL
//
// Zero means unknown state.
//
// We should reverse the 4-tuple of skb, so later
// it can print the right flow direction of rst.
//

//
// tcp event with arguments sk
//
// Note: this class requires a valid sk pointer.
//
// p32 = inet->inet_saddr;
// p32 =  inet->inet_daddr;
// p32 = inet->inet_saddr;
// p32 = inet->inet_daddr;
// p32 = ireq->ir_loc_addr;
// p32 = ireq->ir_rmt_addr;

// sockaddr_in6 is always bigger than sockaddr_in
// For filtering use
//
// tcp event with only skb
//
// p32 = inet->inet_saddr;
// p32 =  inet->inet_daddr;
// sockaddr_in6 is always bigger than sockaddr_in
// For filtering use
// sockaddr_in6 is always bigger than sockaddr_in
// For filtering use

// sockaddr_in6 is always bigger than sockaddr_in
// For filtering use
// sockaddr_in6 is always bigger than sockaddr_in
// For filtering use

// This part must be outside protection
