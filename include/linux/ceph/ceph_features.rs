//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/ceph_features.h
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
// Each time we reclaim bits for reuse we need to specify another bit
// that, if present, indicates we have the new incarnation of that
// feature.  Base case is 1 (first use).
//

// this bit is ignored but still advertised by release *when*

//
// this bit is ignored by release *unused* and not advertised by
// release *unadvertised
//

//
// test for a feature.  this test is safer than a typical mask against
// the bit because it ensures that we have the bit AND the marker for the
// bit's incarnation.  this must be used in any case where the features
// bits may include an old meaning of the bit.
//

//
// Notes on deprecation:
//
// A *major* release is a release through which all upgrades must pass
// (e.g., jewel).  For example, no pre-jewel server will ever talk to
// a post-jewel server (mon, osd, etc).
//
// For feature bits used *only* on the server-side:
//
// - In the first phase we indicate that a feature is DEPRECATED as of
// a particular release.  This is the first major release X (say,
// jewel) that does not depend on its peers advertising the feature.
// That is, it safely assumes its peers all have the feature.  We
// indicate this with the DEPRECATED macro.  For example,
//
// DEFINE_CEPH_FEATURE_DEPRECATED( 2, 1, MONCLOCKCHECK, JEWEL)
//
// because 10.2.z (jewel) did not care if its peers advertised this
// feature bit.
//
// - In the second phase we stop advertising the bit and call it
// RETIRED.  This can normally be done in the *next* major release
// following the one in which we marked the feature DEPRECATED.  In
// the above example, for 12.0.z (luminous) we can say:
//
// DEFINE_CEPH_FEATURE_RETIRED( 2, 1, MONCLOCKCHECK, JEWEL, LUMINOUS)
//
// - The bit can be reused in the first post-luminous release, 13.0.z
// (m).
//
// This ensures that no two versions who have different meanings for
// the bit ever speak to each other.
//
// Features supported.
//

pub const CEPH_FEATURES_REQUIRED_DEFAULT: c_int = 0;
