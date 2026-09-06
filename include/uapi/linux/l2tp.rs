//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/l2tp.h
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
// L2TP-over-IP socket for L2TPv3.
//
// Author: James Chapman <jchapman@katalix.com>
//

//
// struct sockaddr_l2tpip - the sockaddr structure for L2TP-over-IP sockets
// @l2tp_family:  address family number AF_L2TPIP.
// @l2tp_addr:    protocol specific address information
// @l2tp_conn_id: connection id of tunnel
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_l2tpip {
// The first fields must match struct sockaddr_in
    pub /: *mut *mut __kernel_sa_family_t l2tp_family; / AF_INET,
    pub /: *mut *mut __be16 l2tp_unused; / INET port number (unused),
    pub /: *mut *mut in_addr l2tp_addr; / Internet address,
    pub /: *mut *mut __u32 l2tp_conn_id; / Connection ID of tunnel,
// Pad to size of `struct sockaddr'.
}

//
// struct sockaddr_l2tpip6 - the sockaddr structure for L2TP-over-IPv6 sockets
// @l2tp_family:  address family number AF_L2TPIP.
// @l2tp_addr:    protocol specific address information
// @l2tp_conn_id: connection id of tunnel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_l2tpip6 {
// The first fields must match struct sockaddr_in6
    pub /: *mut *mut __kernel_sa_family_t l2tp_family; / AF_INET6,
    pub /: *mut *mut __be16 l2tp_unused; / INET port number (unused),
    pub /: *mut *mut __be32 l2tp_flowinfo; / IPv6 flow information,
    pub /: *mut *mut in6_addr l2tp_addr; / IPv6 address,
    pub /: *mut *mut __u32 l2tp_scope_id; / scope id (new in RFC2553),
    pub /: *mut *mut __u32 l2tp_conn_id; / Connection ID of tunnel,
}

//
// NETLINK_GENERIC netlink family.
//
// Commands.
// Valid TLVs of each command are:-
// TUNNEL_CREATE	- CONN_ID, pw_type, netns, ifname, ipinfo, udpinfo, udpcsum
// TUNNEL_DELETE	- CONN_ID
// TUNNEL_MODIFY	- CONN_ID, udpcsum
// TUNNEL_GETSTATS	- CONN_ID, (stats)
// TUNNEL_GET		- CONN_ID, (...)
// SESSION_CREATE	- SESSION_ID, PW_TYPE, cookie, peer_cookie, l2spec
// SESSION_DELETE	- SESSION_ID
// SESSION_MODIFY	- SESSION_ID
// SESSION_GET		- SESSION_ID, (...)
// SESSION_GETSTATS	- SESSION_ID, (stats)
//

//
// ATTR types defined for L2TP
//

// Nested in L2TP_ATTR_STATS

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l2tp_pwtype {
    L2TP_PWTYPE_NONE = 0x0000,
    L2TP_PWTYPE_ETH_VLAN = 0x0004,
    L2TP_PWTYPE_ETH = 0x0005,
    L2TP_PWTYPE_PPP = 0x0007,
    L2TP_PWTYPE_PPP_AC = 0x0008,
    L2TP_PWTYPE_IP = 0x000b,
    __L2TP_PWTYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l2tp_l2spec_type {
    L2TP_L2SPECTYPE_NONE,
    L2TP_L2SPECTYPE_DEFAULT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l2tp_encap_type {
    L2TP_ENCAPTYPE_UDP,
    L2TP_ENCAPTYPE_IP,
}

// For L2TP_ATTR_DATA_SEQ. Unused.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l2tp_seqmode {
    L2TP_SEQ_NONE = 0,
    L2TP_SEQ_IP = 1,
    L2TP_SEQ_ALL = 2,
}

//
// enum l2tp_debug_flags - debug message categories for L2TP tunnels/sessions.
//
// Unused.
//
// @L2TP_MSG_DEBUG: verbose debug (if compiled in)
// @L2TP_MSG_CONTROL: userspace - kernel interface
// @L2TP_MSG_SEQ: sequence numbers
// @L2TP_MSG_DATA: data packets
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l2tp_debug_flags {
    L2TP_MSG_DEBUG		= (1 << 0),
    L2TP_MSG_CONTROL	= (1 << 1),
    L2TP_MSG_SEQ		= (1 << 2),
    L2TP_MSG_DATA		= (1 << 3),
}

//
// NETLINK_GENERIC related info
//

pub const L2TP_GENL_VERSION: c_uint = 0x1;

