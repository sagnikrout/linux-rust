//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/opa_addr.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2017 Intel Corporation.
//

pub const OPA_GID_INDEX: c_uint = 0x1;
//
// 0xF8 - 4 bits of multicast range and 1 bit for collective range
// Example: For 24 bit LID space,
// Multicast range: 0xF00000 to 0xF7FFFF
// Collective range: 0xF80000 to 0xFFFFFE
//
pub const OPA_MCAST_NR: c_uint = 0x4 /* Number of top bits set */;
pub const OPA_COLLECTIVE_NR: c_uint = 0x1 /* Number of bits after MCAST_NR */;
//
// ib_is_opa_gid: Returns true if the top 24 bits of the gid
// contains the OPA_STL_OUI identifier. This identifies that
// the provided gid is a special purpose GID meant to carry
// extended LID information.
//
// @gid: The Global identifier
//
// opa_get_lid_from_gid: Returns the last 32 bits of the gid.
// OPA devices use one of the gids in the gid table to also
// store the lid.
//
// @gid: The Global identifier
//
// opa_is_extended_lid: Returns true if dlid or slid are
// extended.
//
// @dlid: The DLID
// @slid: The SLID
//
// Get multicast lid base
// Check for a valid unicast LID for non-SM traffic types
