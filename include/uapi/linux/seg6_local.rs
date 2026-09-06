//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/seg6_local.h
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
// SR-IPv6 implementation
//
// Author:
// David Lebrun <david.lebrun@uclouvain.be>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// node segment
// adjacency segment (IPv6 cross-connect)
// lookup of next seg NH in table
// decap and L2 cross-connect
// decap and IPv6 cross-connect
// decap and IPv4 cross-connect
// decap and lookup of DA in v6 table
// decap and lookup of DA in v4 table
// binding segment with insertion
// binding segment with encapsulation
// binding segment with MPLS encap
// lookup last seg in table
// forward to SR-unaware VNF with static proxy
// forward to SR-unaware VNF with masquerading
// custom BPF action
// decap and lookup of DA in v4 or v6 table

// SRv6 Behavior counters are encoded as netlink attributes guaranteeing the
// correct alignment.
// Each counter is identified by a different attribute type (i.e.
// SEG6_LOCAL_CNT_PACKETS).
//
// - SEG6_LOCAL_CNT_PACKETS: identifies a counter that counts the number of
// packets that have been CORRECTLY processed by an SRv6 Behavior instance
// (i.e., packets that generate errors or are dropped are NOT counted).
//
// - SEG6_LOCAL_CNT_BYTES: identifies a counter that counts the total amount
// of traffic in bytes of all packets that have been CORRECTLY processed by
// an SRv6 Behavior instance (i.e., packets that generate errors or are
// dropped are NOT counted).
//
// - SEG6_LOCAL_CNT_ERRORS: identifies a counter that counts the number of
// packets that have NOT been properly processed by an SRv6 Behavior instance
// (i.e., packets that generate errors or are dropped).
//

// SRv6 End* Flavor attributes

// Designed flavor operations for SRv6 End* Behavior

