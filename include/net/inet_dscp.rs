//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/inet_dscp.h
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
//
// inet_dscp.h: helpers for handling differentiated services codepoints (DSCP)
//
// DSCP is defined in RFC 2474:
//
// 0   1   2   3   4   5   6   7
// +---+---+---+---+---+---+---+---+
// |         DSCP          |  CU   |
// +---+---+---+---+---+---+---+---+
//
// DSCP: differentiated services codepoint
// CU:   currently unused
//
// The whole DSCP + CU bits form the DS field.
// The DS field is also commonly called TOS or Traffic Class (for IPv6).
//
// Note: the CU bits are now used for Explicit Congestion Notification
// (RFC 3168).
//

// Special type for storing DSCP values.
//
// A dscp_t variable stores a DS field with the CU (ECN) bits cleared.
// Using dscp_t allows to strictly separate DSCP and ECN bits, thus avoiding
// bugs where ECN bits are erroneously taken into account during FIB lookups
// or policy routing.
//
// Note: to get the real DSCP value contained in a dscp_t variable one would
// have to do a bit shift after calling inet_dscp_to_dsfield(). We could have
// a helper for that, but there's currently no users.
//
pub type dscp_t = u8 ;
pub const INET_DSCP_MASK: c_uint = 0xfc;
// A few places in the IPv4 code need to ignore the three high order bits of
// DSCP because of backward compatibility (as these bits used to represent the
// IPv4 Precedence in RFC 791's TOS field and were ignored).
//

