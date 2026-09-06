//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_bridge/ebt_ip.h
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
//
// ebt_ip
//
// Authors:
// Bart De Schuymer <bart.de.schuymer@pandora.be>
//
// April, 2002
//
// Changes:
// added ip-sport and ip-dport
// Innominate Security Technologies AG <mhopf@innominate.com>
// September, 2002
//

pub const EBT_IP_SOURCE: c_uint = 0x01;
pub const EBT_IP_DEST: c_uint = 0x02;
pub const EBT_IP_TOS: c_uint = 0x04;
pub const EBT_IP_PROTO: c_uint = 0x08;
pub const EBT_IP_SPORT: c_uint = 0x10;
pub const EBT_IP_DPORT: c_uint = 0x20;
pub const EBT_IP_ICMP: c_uint = 0x40;
pub const EBT_IP_IGMP: c_uint = 0x80;

// the same values are used for the invflags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_ip_info {
    pub saddr: __be32,
    pub daddr: __be32,
    pub smsk: __be32,
    pub dmsk: __be32,
    pub tos: __u8,
    pub protocol: __u8,
    pub bitmask: __u8,
    pub invflags: __u8,
    pub sport: [__u16; 2],
    pub icmp_type: [__u8; 2],
    pub igmp_type: [__u8; 2],
}
