//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/in6.h
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
// Types and definitions for AF_INET6
// Linux INET6 implementation
//
// Authors:
// Pedro Roque		<roque@di.fc.ul.pt>
//
// Sources:
// IPv6 Program Interfaces for BSD Systems
// <draft-ietf-ipngwg-bsd-api-05.txt>
//
// Advanced Sockets API for IPv6
// <draft-stevens-advanced-api-00.txt>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

//
// IPv6 address structure
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub u6_addr8: [__u8; 16],    pub u6_addr16: [__be16; 8],
    pub u6_addr32: [__be32; 4],
    pub in6_u: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub /: *mut *mut unsigned short int sin6_family; / AF_INET6,
    pub /: *mut *mut __be16 sin6_port; / Transport layer port #,
    pub /: *mut *mut __be32 sin6_flowinfo; / IPv6 flow information,
    pub /: *mut *mut in6_addr sin6_addr; / IPv6 address,
    pub /: *mut *mut __u32 sin6_scope_id; / scope id (new in RFC2553),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_mreq {
// IPv6 multicast address of group
    pub ipv6mr_multiaddr: in6_addr,
// local IPv6 address of interface
    pub ipv6mr_ifindex: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_flowlabel_req {
    pub flr_dst: in6_addr,
    pub flr_label: __be32,
    pub flr_action: __u8,
    pub flr_share: __u8,
    pub flr_flags: __u16,
    pub flr_expires: __u16,
    pub flr_linger: __u16,
    pub __flr_pad: __u32,
// Options in format of IPV6_PKTOPTIONS
}

pub const IPV6_FL_A_GET: c_int = 0;
pub const IPV6_FL_A_PUT: c_int = 1;
pub const IPV6_FL_A_RENEW: c_int = 2;
pub const IPV6_FL_F_CREATE: c_int = 1;
pub const IPV6_FL_F_EXCL: c_int = 2;
pub const IPV6_FL_F_REFLECT: c_int = 4;
pub const IPV6_FL_F_REMOTE: c_int = 8;
pub const IPV6_FL_S_NONE: c_int = 0;
pub const IPV6_FL_S_EXCL: c_int = 1;
pub const IPV6_FL_S_PROCESS: c_int = 2;
pub const IPV6_FL_S_USER: c_int = 3;
pub const IPV6_FL_S_ANY: c_int = 255;
//
// Bitmask constant declarations to help applications select out the
// flow label and priority fields.
//
// Note that this are in host byte order while the flowinfo field of
// sockaddr_in6 is in network byte order.
//
pub const IPV6_FLOWINFO_FLOWLABEL: c_uint = 0x000fffff;
pub const IPV6_FLOWINFO_PRIORITY: c_uint = 0x0ff00000;
// These definitions are obsolete
pub const IPV6_PRIORITY_UNCHARACTERIZED: c_uint = 0x0000;
pub const IPV6_PRIORITY_FILLER: c_uint = 0x0100;
pub const IPV6_PRIORITY_UNATTENDED: c_uint = 0x0200;
pub const IPV6_PRIORITY_RESERVED1: c_uint = 0x0300;
pub const IPV6_PRIORITY_BULK: c_uint = 0x0400;
pub const IPV6_PRIORITY_RESERVED2: c_uint = 0x0500;
pub const IPV6_PRIORITY_INTERACTIVE: c_uint = 0x0600;
pub const IPV6_PRIORITY_CONTROL: c_uint = 0x0700;
pub const IPV6_PRIORITY_8: c_uint = 0x0800;
pub const IPV6_PRIORITY_9: c_uint = 0x0900;
pub const IPV6_PRIORITY_10: c_uint = 0x0a00;
pub const IPV6_PRIORITY_11: c_uint = 0x0b00;
pub const IPV6_PRIORITY_12: c_uint = 0x0c00;
pub const IPV6_PRIORITY_13: c_uint = 0x0d00;
pub const IPV6_PRIORITY_14: c_uint = 0x0e00;
pub const IPV6_PRIORITY_15: c_uint = 0x0f00;
//
// IPV6 extension headers
//

//
// IPv6 TLV options.
//
pub const IPV6_TLV_PAD1: c_int = 0;
pub const IPV6_TLV_PADN: c_int = 1;
pub const IPV6_TLV_ROUTERALERT: c_int = 5;

pub const IPV6_TLV_JUMBO: c_int = 194;

//
// IPV6 socket options
//
pub const IPV6_ADDRFORM: c_int = 1;
pub const IPV6_2292PKTINFO: c_int = 2;
pub const IPV6_2292HOPOPTS: c_int = 3;
pub const IPV6_2292DSTOPTS: c_int = 4;
pub const IPV6_2292RTHDR: c_int = 5;
pub const IPV6_2292PKTOPTIONS: c_int = 6;
pub const IPV6_CHECKSUM: c_int = 7;
pub const IPV6_2292HOPLIMIT: c_int = 8;
pub const IPV6_NEXTHOP: c_int = 9;

pub const IPV6_FLOWINFO: c_int = 11;
pub const IPV6_UNICAST_HOPS: c_int = 16;
pub const IPV6_MULTICAST_IF: c_int = 17;
pub const IPV6_MULTICAST_HOPS: c_int = 18;
pub const IPV6_MULTICAST_LOOP: c_int = 19;

pub const IPV6_ADD_MEMBERSHIP: c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: c_int = 21;

pub const IPV6_ROUTER_ALERT: c_int = 22;
pub const IPV6_MTU_DISCOVER: c_int = 23;
pub const IPV6_MTU: c_int = 24;
pub const IPV6_RECVERR: c_int = 25;
pub const IPV6_V6ONLY: c_int = 26;
pub const IPV6_JOIN_ANYCAST: c_int = 27;
pub const IPV6_LEAVE_ANYCAST: c_int = 28;
pub const IPV6_MULTICAST_ALL: c_int = 29;
pub const IPV6_ROUTER_ALERT_ISOLATE: c_int = 30;
pub const IPV6_RECVERR_RFC4884: c_int = 31;
// IPV6_MTU_DISCOVER values
pub const IPV6_PMTUDISC_DONT: c_int = 0;
pub const IPV6_PMTUDISC_WANT: c_int = 1;
pub const IPV6_PMTUDISC_DO: c_int = 2;
pub const IPV6_PMTUDISC_PROBE: c_int = 3;
// same as IPV6_PMTUDISC_PROBE, provided for symetry with IPv4
// also see comments on IP_PMTUDISC_INTERFACE
//
pub const IPV6_PMTUDISC_INTERFACE: c_int = 4;
// weaker version of IPV6_PMTUDISC_INTERFACE, which allows packets to
// get fragmented if they exceed the interface mtu
//
pub const IPV6_PMTUDISC_OMIT: c_int = 5;
// Flowlabel
pub const IPV6_FLOWLABEL_MGR: c_int = 32;
pub const IPV6_FLOWINFO_SEND: c_int = 33;
pub const IPV6_IPSEC_POLICY: c_int = 34;
pub const IPV6_XFRM_POLICY: c_int = 35;
pub const IPV6_HDRINCL: c_int = 36;
//
// Multicast:
// Following socket options are shared between IPv4 and IPv6.
//
// MCAST_JOIN_GROUP		42
// MCAST_BLOCK_SOURCE		43
// MCAST_UNBLOCK_SOURCE		44
// MCAST_LEAVE_GROUP		45
// MCAST_JOIN_SOURCE_GROUP	46
// MCAST_LEAVE_SOURCE_GROUP	47
// MCAST_MSFILTER		48
//
// Advanced API (RFC3542) (1)
//
// Note: IPV6_RECVRTHDRDSTOPTS does not exist. see net/ipv6/datagram.c.
//
pub const IPV6_RECVPKTINFO: c_int = 49;
pub const IPV6_PKTINFO: c_int = 50;
pub const IPV6_RECVHOPLIMIT: c_int = 51;
pub const IPV6_HOPLIMIT: c_int = 52;
pub const IPV6_RECVHOPOPTS: c_int = 53;
pub const IPV6_HOPOPTS: c_int = 54;
pub const IPV6_RTHDRDSTOPTS: c_int = 55;
pub const IPV6_RECVRTHDR: c_int = 56;
pub const IPV6_RTHDR: c_int = 57;
pub const IPV6_RECVDSTOPTS: c_int = 58;
pub const IPV6_DSTOPTS: c_int = 59;
pub const IPV6_RECVPATHMTU: c_int = 60;
pub const IPV6_PATHMTU: c_int = 61;
pub const IPV6_DONTFRAG: c_int = 62;

pub const IPV6_USE_MIN_MTU: c_int = 63;

//
// Netfilter (1)
//
// Following socket options are used in ip6_tables;
// see include/linux/netfilter_ipv6/ip6_tables.h.
//
// IP6T_SO_SET_REPLACE / IP6T_SO_GET_INFO		64
// IP6T_SO_SET_ADD_COUNTERS / IP6T_SO_GET_ENTRIES	65
//
// Advanced API (RFC3542) (2)
//
pub const IPV6_RECVTCLASS: c_int = 66;
pub const IPV6_TCLASS: c_int = 67;
//
// Netfilter (2)
//
// Following socket options are used in ip6_tables;
// see include/linux/netfilter_ipv6/ip6_tables.h.
//
// IP6T_SO_GET_REVISION_MATCH	68
// IP6T_SO_GET_REVISION_TARGET	69
// IP6T_SO_ORIGINAL_DST		80
//
pub const IPV6_AUTOFLOWLABEL: c_int = 70;
// RFC5014: Source address selection
pub const IPV6_ADDR_PREFERENCES: c_int = 72;
pub const IPV6_PREFER_SRC_TMP: c_uint = 0x0001;
pub const IPV6_PREFER_SRC_PUBLIC: c_uint = 0x0002;
pub const IPV6_PREFER_SRC_PUBTMP_DEFAULT: c_uint = 0x0100;
pub const IPV6_PREFER_SRC_COA: c_uint = 0x0004;
pub const IPV6_PREFER_SRC_HOME: c_uint = 0x0400;
pub const IPV6_PREFER_SRC_CGA: c_uint = 0x0008;
pub const IPV6_PREFER_SRC_NONCGA: c_uint = 0x0800;
// RFC5082: Generalized Ttl Security Mechanism
pub const IPV6_MINHOPCOUNT: c_int = 73;
pub const IPV6_ORIGDSTADDR: c_int = 74;

pub const IPV6_TRANSPARENT: c_int = 75;
pub const IPV6_UNICAST_IF: c_int = 76;
pub const IPV6_RECVFRAGSIZE: c_int = 77;
pub const IPV6_FREEBIND: c_int = 78;
//
// Multicast Routing:
// see include/uapi/linux/mroute6.h.
//
// MRT6_BASE			200
// ...
// MRT6_MAX
//
