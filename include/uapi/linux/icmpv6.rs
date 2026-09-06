//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/icmpv6.h
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
pub struct icmp6hdr {
    pub icmp6_type: __u8,
    pub icmp6_code: __u8,
    pub icmp6_cksum: __sum16,
    pub un_data32: [__be32; 1],
    pub un_data16: [__be16; 2],
    pub un_data8: [__u8; 4],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpv6_echo {
    pub identifier: __be16,
    pub sequence: __be16,
    pub u_echo: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpv6_nd_advt {

    pub u_nd_advt: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmpv6_nd_ra {
    pub hop_limit: __u8,

    pub rt_lifetime: __be16,
    pub u_nd_ra: },
    pub icmp6_dataun: },

}

pub const ICMPV6_ROUTER_PREF_LOW: c_uint = 0x3;
pub const ICMPV6_ROUTER_PREF_MEDIUM: c_uint = 0x0;
pub const ICMPV6_ROUTER_PREF_HIGH: c_uint = 0x1;
pub const ICMPV6_ROUTER_PREF_INVALID: c_uint = 0x2;
pub const ICMPV6_DEST_UNREACH: c_int = 1;
pub const ICMPV6_PKT_TOOBIG: c_int = 2;
pub const ICMPV6_TIME_EXCEED: c_int = 3;
pub const ICMPV6_PARAMPROB: c_int = 4;
pub const ICMPV6_ERRMSG_MAX: c_int = 127;
pub const ICMPV6_INFOMSG_MASK: c_uint = 0x80;
pub const ICMPV6_ECHO_REQUEST: c_int = 128;
pub const ICMPV6_ECHO_REPLY: c_int = 129;
pub const ICMPV6_MGM_QUERY: c_int = 130;
pub const ICMPV6_MGM_REPORT: c_int = 131;
pub const ICMPV6_MGM_REDUCTION: c_int = 132;
pub const ICMPV6_NI_QUERY: c_int = 139;
pub const ICMPV6_NI_REPLY: c_int = 140;
pub const ICMPV6_MLD2_REPORT: c_int = 143;
pub const ICMPV6_DHAAD_REQUEST: c_int = 144;
pub const ICMPV6_DHAAD_REPLY: c_int = 145;
pub const ICMPV6_MOBILE_PREFIX_SOL: c_int = 146;
pub const ICMPV6_MOBILE_PREFIX_ADV: c_int = 147;
pub const ICMPV6_MRDISC_ADV: c_int = 151;
pub const ICMPV6_MRDISC_SOL: c_int = 152;
pub const ICMPV6_MSG_MAX: c_int = 255;
//
// Codes for Destination Unreachable
//
pub const ICMPV6_NOROUTE: c_int = 0;
pub const ICMPV6_ADM_PROHIBITED: c_int = 1;
pub const ICMPV6_NOT_NEIGHBOUR: c_int = 2;
pub const ICMPV6_ADDR_UNREACH: c_int = 3;
pub const ICMPV6_PORT_UNREACH: c_int = 4;
pub const ICMPV6_POLICY_FAIL: c_int = 5;
pub const ICMPV6_REJECT_ROUTE: c_int = 6;
//
// Codes for Time Exceeded
//
pub const ICMPV6_EXC_HOPLIMIT: c_int = 0;
pub const ICMPV6_EXC_FRAGTIME: c_int = 1;
//
// Codes for Parameter Problem
//
pub const ICMPV6_HDR_FIELD: c_int = 0;
pub const ICMPV6_UNK_NEXTHDR: c_int = 1;
pub const ICMPV6_UNK_OPTION: c_int = 2;
pub const ICMPV6_HDR_INCOMP: c_int = 3;
// Codes for EXT_ECHO (PROBE)
pub const ICMPV6_EXT_ECHO_REQUEST: c_int = 160;
pub const ICMPV6_EXT_ECHO_REPLY: c_int = 161;
//
// constants for (set|get)sockopt
//
pub const ICMPV6_FILTER: c_int = 1;
//
// ICMPV6 filter
//
pub const ICMPV6_FILTER_BLOCK: c_int = 1;
pub const ICMPV6_FILTER_PASS: c_int = 2;
pub const ICMPV6_FILTER_BLOCKOTHERS: c_int = 3;
pub const ICMPV6_FILTER_PASSONLY: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmp6_filter {
    pub data: [__u32; 8],
}

//
// Definitions for MLDv2
//
pub const MLD2_MODE_IS_INCLUDE: c_int = 1;
pub const MLD2_MODE_IS_EXCLUDE: c_int = 2;
pub const MLD2_CHANGE_TO_INCLUDE: c_int = 3;
pub const MLD2_CHANGE_TO_EXCLUDE: c_int = 4;
pub const MLD2_ALLOW_NEW_SOURCES: c_int = 5;
pub const MLD2_BLOCK_OLD_SOURCES: c_int = 6;

