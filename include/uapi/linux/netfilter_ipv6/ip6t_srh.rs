//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_ipv6/ip6t_srh.h
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

// Values for "mt_flags" field in struct ip6t_srh
pub const IP6T_SRH_NEXTHDR: c_uint = 0x0001;
pub const IP6T_SRH_LEN_EQ: c_uint = 0x0002;
pub const IP6T_SRH_LEN_GT: c_uint = 0x0004;
pub const IP6T_SRH_LEN_LT: c_uint = 0x0008;
pub const IP6T_SRH_SEGS_EQ: c_uint = 0x0010;
pub const IP6T_SRH_SEGS_GT: c_uint = 0x0020;
pub const IP6T_SRH_SEGS_LT: c_uint = 0x0040;
pub const IP6T_SRH_LAST_EQ: c_uint = 0x0080;
pub const IP6T_SRH_LAST_GT: c_uint = 0x0100;
pub const IP6T_SRH_LAST_LT: c_uint = 0x0200;
pub const IP6T_SRH_TAG: c_uint = 0x0400;
pub const IP6T_SRH_PSID: c_uint = 0x0800;
pub const IP6T_SRH_NSID: c_uint = 0x1000;
pub const IP6T_SRH_LSID: c_uint = 0x2000;
pub const IP6T_SRH_MASK: c_uint = 0x3FFF;
// Values for "mt_invflags" field in struct ip6t_srh
pub const IP6T_SRH_INV_NEXTHDR: c_uint = 0x0001;
pub const IP6T_SRH_INV_LEN_EQ: c_uint = 0x0002;
pub const IP6T_SRH_INV_LEN_GT: c_uint = 0x0004;
pub const IP6T_SRH_INV_LEN_LT: c_uint = 0x0008;
pub const IP6T_SRH_INV_SEGS_EQ: c_uint = 0x0010;
pub const IP6T_SRH_INV_SEGS_GT: c_uint = 0x0020;
pub const IP6T_SRH_INV_SEGS_LT: c_uint = 0x0040;
pub const IP6T_SRH_INV_LAST_EQ: c_uint = 0x0080;
pub const IP6T_SRH_INV_LAST_GT: c_uint = 0x0100;
pub const IP6T_SRH_INV_LAST_LT: c_uint = 0x0200;
pub const IP6T_SRH_INV_TAG: c_uint = 0x0400;
pub const IP6T_SRH_INV_PSID: c_uint = 0x0800;
pub const IP6T_SRH_INV_NSID: c_uint = 0x1000;
pub const IP6T_SRH_INV_LSID: c_uint = 0x2000;
pub const IP6T_SRH_INV_MASK: c_uint = 0x3FFF;
//
// struct ip6t_srh - SRH match options
// @next_hdr: Next header field of SRH
// @hdr_len: Extension header length field of SRH
// @segs_left: Segments left field of SRH
// @last_entry: Last entry field of SRH
// @tag: Tag field of SRH
// @mt_flags: match options
// @mt_invflags: Invert the sense of match options
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6t_srh {
    pub next_hdr: __u8,
    pub hdr_len: __u8,
    pub segs_left: __u8,
    pub last_entry: __u8,
    pub tag: __u16,
    pub mt_flags: __u16,
    pub mt_invflags: __u16,
}

//
// struct ip6t_srh1 - SRH match options (revision 1)
// @next_hdr: Next header field of SRH
// @hdr_len: Extension header length field of SRH
// @segs_left: Segments left field of SRH
// @last_entry: Last entry field of SRH
// @tag: Tag field of SRH
// @psid_addr: Address of previous SID in SRH SID list
// @nsid_addr: Address of NEXT SID in SRH SID list
// @lsid_addr: Address of LAST SID in SRH SID list
// @psid_msk: Mask of previous SID in SRH SID list
// @nsid_msk: Mask of next SID in SRH SID list
// @lsid_msk: MAsk of last SID in SRH SID list
// @mt_flags: match options
// @mt_invflags: Invert the sense of match options
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6t_srh1 {
    pub next_hdr: __u8,
    pub hdr_len: __u8,
    pub segs_left: __u8,
    pub last_entry: __u8,
    pub tag: __u16,
    pub psid_addr: in6_addr,
    pub nsid_addr: in6_addr,
    pub lsid_addr: in6_addr,
    pub psid_msk: in6_addr,
    pub nsid_msk: in6_addr,
    pub lsid_msk: in6_addr,
    pub mt_flags: __u16,
    pub mt_invflags: __u16,
}
