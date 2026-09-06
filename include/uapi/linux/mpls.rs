//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mpls.h
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

// Reference: RFC 5462, RFC 3032
//
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                Label                  | TC  |S|       TTL     |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Label:  Label Value, 20 bits
// TC:     Traffic Class field, 3 bits
// S:      Bottom of Stack, 1 bit
// TTL:    Time to Live, 8 bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpls_label {
    pub entry: __be32,
}

pub const MPLS_LS_LABEL_MASK: c_uint = 0xFFFFF000;
pub const MPLS_LS_LABEL_SHIFT: c_int = 12;
pub const MPLS_LS_TC_MASK: c_uint = 0x00000E00;
pub const MPLS_LS_TC_SHIFT: c_int = 9;
pub const MPLS_LS_S_MASK: c_uint = 0x00000100;
pub const MPLS_LS_S_SHIFT: c_int = 8;
pub const MPLS_LS_TTL_MASK: c_uint = 0x000000FF;
pub const MPLS_LS_TTL_SHIFT: c_int = 0;
// Reserved labels

// These are embedded into IFLA_STATS_AF_SPEC:
// [IFLA_STATS_AF_SPEC]
// -> [AF_MPLS]
// -> [MPLS_STATS_xxx]
//
// Attributes:
// [MPLS_STATS_LINK] = {
// struct mpls_link_stats
// }
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpls_link_stats {
    pub /: *mut *mut __u64 rx_packets; / total packets received,
    pub /: *mut *mut __u64 tx_packets; / total packets transmitted,
    pub /: *mut *mut __u64 rx_bytes; / total bytes received,
    pub /: *mut *mut __u64 tx_bytes; / total bytes transmitted,
    pub /: *mut *mut __u64 rx_errors; / bad packets received,
    pub /: *mut *mut __u64 tx_errors; / packet transmit problems,
    pub /: *mut *mut __u64 rx_dropped; / packet dropped on receive,
    pub /: *mut *mut __u64 tx_dropped; / packet dropped on transmit,
    pub /: *mut *mut __u64 rx_noroute; / no route for packet dest,
}
