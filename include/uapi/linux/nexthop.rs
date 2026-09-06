//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nexthop.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nhmsg {
    pub nh_family: c_uchar,
    pub /: *mut *mut unsigned char nh_scope; / return only,
    pub /: *mut *mut unsigned char nh_protocol; / Routing protocol that installed nh,
    pub resvd: c_uchar,
    pub /: *mut *mut unsigned int nh_flags; / RTNH_F flags,
}

// entry in a nexthop group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nexthop_grp {
    pub /: *mut *mut __u32 id; / nexthop id - must exist,
    pub /: *mut *mut __u8 weight; / weight of this nexthop,
    pub /: *mut *mut __u8 weight_high; / high order bits of weight,
    pub resvd2: __u16,
}

// default type if not specified
//

// Response OP_FLAGS.

// if NHA_GROUP attribute is added, no other attributes can be set
// if NHA_BLACKHOLE is added, OIF, GATEWAY, ENCAP can not be set
// NHA_OIF can be appended to dump request to return only
// nexthops using given device
//
// if NHA_FDB is added, OIF, BLACKHOLE, ENCAP cannot be set
// nested; resilient nexthop group attributes
// nested; nexthop bucket attributes
// u32; operation-specific flags
// nested; nexthop group stats
// u32; nexthop hardware stats enable
// u32; read-only; whether any driver collects HW stats
// be16; UDP destination port for an fdb nexthop (e.g. VXLAN)

// Pad attribute for 64-bit alignment.
// u16; number of nexthop buckets in a resilient nexthop group
// clock_t as u32; nexthop bucket idle timer (per-group)
// clock_t as u32; nexthop unbalanced timer
// clock_t as u64; nexthop unbalanced time

// Pad attribute for 64-bit alignment.
// u16; nexthop bucket index
// clock_t as u64; nexthop bucket idle time
// u32; nexthop id assigned to the nexthop bucket

// nested; nexthop group entry stats

// u32; nexthop id of the nexthop group entry
// uint; number of packets forwarded via the nexthop group entry
// uint; number of packets forwarded via the nexthop group entry in
// hardware
//

