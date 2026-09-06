//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ppp/ppp_mppe.h
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

// option bits for ccp_options.mppe
pub const MPPE_OPT_40: c_uint = 0x01    /* 40 bit */;
pub const MPPE_OPT_128: c_uint = 0x02    /* 128 bit */;
pub const MPPE_OPT_STATEFUL: c_uint = 0x04    /* stateful mode */;
// unsupported opts
pub const MPPE_OPT_56: c_uint = 0x08    /* 56 bit */;
pub const MPPE_OPT_MPPC: c_uint = 0x10    /* MPPC compression */;
pub const MPPE_OPT_D: c_uint = 0x20    /* Unknown */;

pub const MPPE_OPT_UNKNOWN: c_uint = 0x40    /* Bits !defined in RFC 3078 were set */;
//
// This is not nice ... the alternative is a bitfield struct though.
// And unfortunately, we cannot share the same bits for the option
// names above since C and H are the same bit.  We could do a u_int32
// but then we have to do a htonl() all the time and/or we still need
// to know which octet is which.
//
pub const MPPE_C_BIT: c_uint = 0x01    /* MPPC */;
pub const MPPE_D_BIT: c_uint = 0x10    /* Obsolete, usage unknown */;
pub const MPPE_L_BIT: c_uint = 0x20    /* 40-bit */;
pub const MPPE_S_BIT: c_uint = 0x40    /* 128-bit */;
pub const MPPE_M_BIT: c_uint = 0x80    /* 56-bit, not supported */;
pub const MPPE_H_BIT: c_uint = 0x01    /* Stateless (in a different byte) */;
// Does not include H bit; used for least significant octet only.

// Build a CI from mppe opts (see RFC 3078)

// H bit */                             \
// ptr++ = 0x0;                       \
// ptr++ = MPPE_H_BIT;                \
// ptr++ = 0;                             \
// S,L bits */                          \
// ptr = 0;                               \
// ptr |= MPPE_S_BIT;                 \
// ptr |= MPPE_L_BIT;                 \
// M,D,C bits not supported */          \
// The reverse of the above

// H bit */                             \
// S,L bits */                          \
// M,D,C bits */                        \
// Other bits */                        \
