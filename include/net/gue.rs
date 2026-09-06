//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/gue.h
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
// Definitions for the GUE header, standard and private flags, lengths
// of optional fields are below.
//
// Diagram of GUE header:
//
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |Ver|C|  Hlen   | Proto/ctype   |        Standard flags       |P|
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               |
// ~                      Fields (optional)                        ~
// |                                                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |            Private flags (optional, P bit is set)             |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               |
// ~                   Private fields (optional)                   ~
// |                                                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// C bit indicates control message when set, data message when unset.
// For a control message, proto/ctype is interpreted as a type of
// control message. For data messages, proto/ctype is the IP protocol
// of the next header.
//
// P bit indicates private flags field is present. The private flags
// may refer to options placed after this field.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct guehdr {

    pub proto_ctype: __u8,
    pub flags: __be16,
}

// Standard flags in GUE header

pub const GUE_LEN_PRIV: c_int = 4;

// Private flags in the private option extension

pub const GUE_PLEN_REMCSUM: c_int = 4;

// Functions to compute options length corresponding to flags.
// If we ever have a lot of flags this can be potentially be
// converted to a more optimized algorithm (table lookup
// for instance).
//
// Validate standard and private flags. Returns non-zero (meaning invalid)
// if there is an unknown standard or private flags, or the options length for
// the flags exceeds the options length specific in hlen of the GUE header.
//
// Private flags are last four bytes accounted in
// guehdr_flags_len
//
