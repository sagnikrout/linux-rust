//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/in.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions of the Internet Protocol.
//
// Version:	@(#)in.h	1.0.1	04/21/93
//
// Authors:	Original taken from the GNU Project <netinet/in.h> file.
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// Standard well-defined IP protocols.

// Internet address.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: __be32,
}

pub const IP_TOS: c_int = 1;
pub const IP_TTL: c_int = 2;
pub const IP_HDRINCL: c_int = 3;
pub const IP_OPTIONS: c_int = 4;
pub const IP_ROUTER_ALERT: c_int = 5;
pub const IP_RECVOPTS: c_int = 6;
pub const IP_RETOPTS: c_int = 7;
pub const IP_PKTINFO: c_int = 8;
pub const IP_PKTOPTIONS: c_int = 9;
pub const IP_MTU_DISCOVER: c_int = 10;
pub const IP_RECVERR: c_int = 11;
pub const IP_RECVTTL: c_int = 12;
pub const IP_RECVTOS: c_int = 13;
pub const IP_MTU: c_int = 14;
pub const IP_FREEBIND: c_int = 15;
pub const IP_IPSEC_POLICY: c_int = 16;
pub const IP_XFRM_POLICY: c_int = 17;
pub const IP_PASSSEC: c_int = 18;
pub const IP_TRANSPARENT: c_int = 19;
// BSD compatibility

// TProxy original addresses
pub const IP_ORIGDSTADDR: c_int = 20;

pub const IP_MINTTL: c_int = 21;
pub const IP_NODEFRAG: c_int = 22;
pub const IP_CHECKSUM: c_int = 23;
pub const IP_BIND_ADDRESS_NO_PORT: c_int = 24;
pub const IP_RECVFRAGSIZE: c_int = 25;
pub const IP_RECVERR_RFC4884: c_int = 26;
// IP_MTU_DISCOVER values

// Always use interface mtu (ignores dst pmtu) but don't set DF flag.
// Also incoming ICMP frag_needed notifications will be ignored on
// this socket to prevent accepting spoofed ones.
//
pub const IP_PMTUDISC_INTERFACE: c_int = 4;
// weaker version of IP_PMTUDISC_INTERFACE, which allows packets to get
// fragmented if they exceed the interface mtu
//
pub const IP_PMTUDISC_OMIT: c_int = 5;
pub const IP_MULTICAST_IF: c_int = 32;
pub const IP_MULTICAST_TTL: c_int = 33;
pub const IP_MULTICAST_LOOP: c_int = 34;
pub const IP_ADD_MEMBERSHIP: c_int = 35;
pub const IP_DROP_MEMBERSHIP: c_int = 36;
pub const IP_UNBLOCK_SOURCE: c_int = 37;
pub const IP_BLOCK_SOURCE: c_int = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: c_int = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: c_int = 40;
pub const IP_MSFILTER: c_int = 41;
pub const MCAST_JOIN_GROUP: c_int = 42;
pub const MCAST_BLOCK_SOURCE: c_int = 43;
pub const MCAST_UNBLOCK_SOURCE: c_int = 44;
pub const MCAST_LEAVE_GROUP: c_int = 45;
pub const MCAST_JOIN_SOURCE_GROUP: c_int = 46;
pub const MCAST_LEAVE_SOURCE_GROUP: c_int = 47;
pub const MCAST_MSFILTER: c_int = 48;
pub const IP_MULTICAST_ALL: c_int = 49;
pub const IP_UNICAST_IF: c_int = 50;
pub const IP_LOCAL_PORT_RANGE: c_int = 51;
pub const IP_PROTOCOL: c_int = 52;
pub const MCAST_EXCLUDE: c_int = 0;
pub const MCAST_INCLUDE: c_int = 1;
// These need to appear somewhere around here
pub const IP_DEFAULT_MULTICAST_TTL: c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: c_int = 1;
// Request struct for multicast socket ops

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_mreq {
    pub /: *mut *mut in_addr imr_multiaddr; / IP multicast address of group,
    pub /: *mut *mut in_addr imr_interface; / local IP address of interface,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_mreqn {
    pub /: *mut *mut in_addr imr_multiaddr; / IP multicast address of group,
    pub /: *mut *mut in_addr imr_address; / local IP address of interface,
    pub /: *mut *mut int imr_ifindex; / Interface index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_mreq_source {
    pub imr_multiaddr: __be32,
    pub imr_interface: __be32,
    pub imr_sourceaddr: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_msfilter {
    pub imsf_multiaddr: __be32,
    pub imsf_interface: __be32,
    pub imsf_fmode: __u32,
    pub imsf_numsrc: __u32,
    pub imsf_slist: [__be32; 1],
    pub imsf_slist_flex): __DECLARE_FLEX_ARRAY(__be32,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_req {
    pub /: *mut *mut __u32 gr_interface; / interface index,
    pub /: *mut *mut __kernel_sockaddr_storage gr_group; / group address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_source_req {
    pub /: *mut *mut __u32 gsr_interface; / interface index,
    pub /: *mut *mut __kernel_sockaddr_storage gsr_group; / group address,
    pub /: *mut *mut __kernel_sockaddr_storage gsr_source; / source address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_filter {
    pub /: *mut *mut __u32 gf_interface_aux; / interface index,
    pub /: *mut *mut __kernel_sockaddr_storage gf_group_aux; / multicast address,
    pub /: *mut *mut __u32 gf_fmode_aux; / filter mode,
    pub /: *mut *mut __u32 gf_numsrc_aux; / number of sources,
    pub /: *mut *mut __kernel_sockaddr_storage gf_slist[1]; / interface index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_pktinfo {
    pub ipi_ifindex: c_int,
    pub ipi_spec_dst: in_addr,
    pub ipi_addr: in_addr,
}

// Structure describing an Internet (IP) socket address.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub /: *mut *mut __kernel_sa_family_t sin_family; / Address family,
    pub /: *mut *mut __be16 sin_port; / Port number,
    pub /: *mut *mut in_addr sin_addr; / Internet address,
// Pad to size of `struct sockaddr'.
    pub in_addr)]: sizeof(unsigned short int) - sizeof(struct,
}

//
// Definitions of the bits in an Internet address integer.
// On subnets, host and network parts are found according
// to the subnet mask, not these masks.
//

pub const IN_CLASSA_NET: c_uint = 0xff000000;
pub const IN_CLASSA_NSHIFT: c_int = 24;

pub const IN_CLASSA_MAX: c_int = 128;

pub const IN_CLASSB_NET: c_uint = 0xffff0000;
pub const IN_CLASSB_NSHIFT: c_int = 16;

pub const IN_CLASSB_MAX: c_int = 65536;

pub const IN_CLASSC_NET: c_uint = 0xffffff00;
pub const IN_CLASSC_NSHIFT: c_int = 8;

pub const IN_MULTICAST_NET: c_uint = 0xe0000000;

pub const IN_CLASSE_NET: c_uint = 0xffffffff;
pub const IN_CLASSE_NSHIFT: c_int = 0;
// Address to accept any incoming messages.

// Address to send to all hosts.

// Address indicating an error return.

// Dummy address for src of ICMP replies if no real address is set (RFC7600).

// Network number for local host loopback.
pub const IN_LOOPBACKNET: c_int = 127;
// Address to loopback in software to local host.
pub const INADDR_LOOPBACK: c_uint = 0x7f000001	/* 127.0.0.1   */;

// Defines for Multicast INADDR
pub const INADDR_UNSPEC_GROUP: c_uint = 0xe0000000U	/* 224.0.0.0   */;
pub const INADDR_ALLHOSTS_GROUP: c_uint = 0xe0000001U	/* 224.0.0.1   */;
pub const INADDR_ALLRTRS_GROUP: c_uint = 0xe0000002U	/* 224.0.0.2 */;
pub const INADDR_ALLSNOOPERS_GROUP: c_uint = 0xe000006aU	/* 224.0.0.106 */;
pub const INADDR_MAX_LOCAL_GROUP: c_uint = 0xe00000ffU	/* 224.0.0.255 */;

// <asm/byteorder.h> contains the htonl type stuff..

