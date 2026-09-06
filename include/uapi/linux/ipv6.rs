//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ipv6.h
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

// The latest drafts declared increase in minimal mtu up to 1280.
pub const IPV6_MIN_MTU: c_int = 1280;
//
// Advanced API
// source interface/address selection, source routing, etc...
// *under construction
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_pktinfo {
    pub ipi6_addr: in6_addr,
    pub ipi6_ifindex: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_mtuinfo {
    pub ip6m_addr: sockaddr_in6,
    pub ip6m_mtu: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_ifreq {
    pub ifr6_addr: in6_addr,
    pub ifr6_prefixlen: __u32,
    pub ifr6_ifindex: c_int,
}

pub const IPV6_SRCRT_STRICT: c_uint = 0x01	/* Deprecated; will be removed */;

//
// routing header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_rt_hdr {
    pub nexthdr: __u8,
    pub hdrlen: __u8,
    pub type: __u8,
    pub segments_left: __u8,
//
// type specific data
// variable length field
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_opt_hdr {
    pub nexthdr: __u8,
    pub hdrlen: __u8,
//
// TLV encoded option data follows.
//
    pub /: *mut *mut } __attribute__((packed)); / required for some archs,

// Router Alert option values (RFC2711)
pub const IPV6_OPT_ROUTERALERT_MLD: c_uint = 0x0000	/* MLD(RFC2710) */;
//
// routing header type 0 (used in cmsghdr struct)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt0_hdr {
    pub rt_hdr: ipv6_rt_hdr,
    pub reserved: __u32,
    pub addr: [in6_addr; ],
}

//
// routing header type 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2_hdr {
    pub rt_hdr: ipv6_rt_hdr,
    pub reserved: __u32,
    pub addr: in6_addr,

}

//
// home address option in destination options header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_destopt_hao {
    pub type: __u8,
    pub length: __u8,
    pub addr: in6_addr,
    pub __attribute__((packed)): },
//
// IPv6 fixed header
//
// BEWARE, it is incorrect. The first 4 bits of flow_lbl
// are glued to priority now, forming "class".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6hdr {
    pub flow_lbl: [__u8; 3],
    pub payload_len: __be16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

// index values for the variables in ipv6_devconf
