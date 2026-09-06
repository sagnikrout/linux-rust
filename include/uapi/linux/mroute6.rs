//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mroute6.h
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
// Based on the MROUTING 3.5 defines primarily to keep
// source compatibility with BSD.
//
// See the pim6sd code for the original history.
//
// Protocol Independent Multicast (PIM) data structures included
// Carlos Picoto (cap@di.fc.ul.pt)
//
pub const MRT6_BASE: c_int = 200;

// MRT6_FLUSH optional flags

pub const MAXMIFS: c_int = 32;
pub type mifi_t = c_ushort;

pub const IF_SETSIZE: c_int = 256;

//
// Passed by mrouted for an MRT_ADD_MIF - again we use the
// mrouted 3.6 structures for compatibility
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mif6ctl {
    pub /: *mut *mut mifi_t mif6c_mifi; / Index of MIF,
    pub /: *mut *mut unsigned char mif6c_flags; / MIFF_ flags,
    pub /: *mut *mut unsigned char vifc_threshold; / ttl limit,
    pub /: *mut *mut __u16 mif6c_pifi; / the index of the physical IF,
    pub /: *mut *mut unsigned int vifc_rate_limit; / Rate limiter values (NI),
}

pub const MIFF_REGISTER: c_uint = 0x1	/* register vif	*/;
//
// Cache manipulation structures for mrouted and PIMd
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mf6cctl {
    pub /: *mut *mut sockaddr_in6 mf6cc_origin; / Origin of mcast,
    pub /: *mut *mut sockaddr_in6 mf6cc_mcastgrp; / Group in question,
    pub /: *mut *mut mifi_t mf6cc_parent; / Where it arrived,
    pub /: *mut *mut if_set mf6cc_ifset; / Where it is going,
}

//
// Group count retrieval for pim6sd
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sioc_sg_req6 {
    pub src: sockaddr_in6,
    pub grp: sockaddr_in6,
    pub pktcnt: c_ulong,
    pub bytecnt: c_ulong,
    pub wrong_if: c_ulong,
}

//
// To get vif packet counts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sioc_mif_req6 {
    pub /: *mut *mut mifi_t mifi; / Which iface,
    pub /: *mut *mut unsigned long icount; / In packets,
    pub /: *mut *mut unsigned long ocount; / Out packets,
    pub /: *mut *mut unsigned long ibytes; / In bytes,
    pub /: *mut *mut unsigned long obytes; / Out bytes,
}

//
// That's all usermode folks
//
// Structure used to communicate from kernel to multicast router.
// We'll overlay the structure onto an MLD header (not an IPv6 heder like igmpmsg{}
// used for IPv4 implementation). This is because this structure will be passed via an
// IPv6 raw socket, on which an application will only receiver the payload i.e the data after
// the IPv6 header and all the extension headers. (See section 3 of RFC 3542)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrt6msg {
pub const MRT6MSG_NOCACHE: c_int = 1;
pub const MRT6MSG_WRONGMIF: c_int = 2;

    pub /: *mut *mut __u8 im6_mbz; / must be zero,
    pub /: *mut *mut __u8 im6_msgtype; / what type of message,
    pub /: *mut *mut __u16 im6_mif; / mif rec'd on,
    pub /: *mut *mut __u32 im6_pad; / padding for 64 bit arch,
    pub im6_dst: in6_addr im6_src,,
}

// ip6mr netlink cache report attributes

