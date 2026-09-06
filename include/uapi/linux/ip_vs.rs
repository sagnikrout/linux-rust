//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ip_vs.h
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
// IP Virtual Server
// data structure and functionality definitions
//

pub const IP_VS_VERSION_CODE: c_uint = 0x010201;

//
// Virtual Service Flags
//
pub const IP_VS_SVC_F_PERSISTENT: c_uint = 0x0001		/* persistent port */;
pub const IP_VS_SVC_F_HASHED: c_uint = 0x0002		/* hashed entry */;
pub const IP_VS_SVC_F_ONEPACKET: c_uint = 0x0004		/* one-packet scheduling */;
pub const IP_VS_SVC_F_SCHED1: c_uint = 0x0008		/* scheduler flag 1 */;
pub const IP_VS_SVC_F_SCHED2: c_uint = 0x0010		/* scheduler flag 2 */;
pub const IP_VS_SVC_F_SCHED3: c_uint = 0x0020		/* scheduler flag 3 */;

//
// IPVS sync daemon states
//
pub const IP_VS_STATE_NONE: c_uint = 0x0000		/* daemon is stopped */;
pub const IP_VS_STATE_MASTER: c_uint = 0x0001		/* started as master */;
pub const IP_VS_STATE_BACKUP: c_uint = 0x0002		/* started as backup */;
//
// IPVS socket options
//

//
// IPVS Connection Flags
// Only flags 0..15 are sent to backup server
//
pub const IP_VS_CONN_F_FWD_MASK: c_uint = 0x0007		/* mask for the fwd methods */;
pub const IP_VS_CONN_F_MASQ: c_uint = 0x0000		/* masquerading/NAT */;
pub const IP_VS_CONN_F_LOCALNODE: c_uint = 0x0001		/* local node */;
pub const IP_VS_CONN_F_TUNNEL: c_uint = 0x0002		/* tunneling */;
pub const IP_VS_CONN_F_DROUTE: c_uint = 0x0003		/* direct routing */;
pub const IP_VS_CONN_F_BYPASS: c_uint = 0x0004		/* cache bypass */;
pub const IP_VS_CONN_F_SYNC: c_uint = 0x0020		/* entry created by sync */;
pub const IP_VS_CONN_F_HASHED: c_uint = 0x0040		/* hashed entry */;
pub const IP_VS_CONN_F_NOOUTPUT: c_uint = 0x0080		/* no output packets */;
pub const IP_VS_CONN_F_INACTIVE: c_uint = 0x0100		/* not established */;
pub const IP_VS_CONN_F_OUT_SEQ: c_uint = 0x0200		/* must do output seq adjust */;
pub const IP_VS_CONN_F_IN_SEQ: c_uint = 0x0400		/* must do input seq adjust */;
pub const IP_VS_CONN_F_SEQ_MASK: c_uint = 0x0600		/* in/out sequence mask */;
pub const IP_VS_CONN_F_NO_CPORT: c_uint = 0x0800		/* no client port set yet */;
pub const IP_VS_CONN_F_TEMPLATE: c_uint = 0x1000		/* template, not connection */;
pub const IP_VS_CONN_F_ONE_PACKET: c_uint = 0x2000		/* forward only one packet */;
// Initial bits allowed in backup server

// Bits allowed to update in backup server

// Flags that are not sent to backup server start from bit 16

// Connection flags from destination that can be changed by user space

pub const IP_VS_SCHEDNAME_MAXLEN: c_int = 16;
pub const IP_VS_PENAME_MAXLEN: c_int = 16;
pub const IP_VS_IFNAME_MAXLEN: c_int = 16;
pub const IP_VS_PEDATA_MAXLEN: c_int = 255;
// Tunnel types
// Tunnel encapsulation flags

//
// The struct ip_vs_service_user and struct ip_vs_dest_user are
// used to set IPVS rules through setsockopt.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_service_user {
// virtual service addresses
    pub protocol: __u16,
    pub /: *mut *mut __be32 addr; / virtual ip address,
    pub port: __be16,
    pub /: *mut *mut __u32 fwmark; / firwall mark of service,
// virtual service options
    pub sched_name: [c_char; IP_VS_SCHEDNAME_MAXLEN],
    pub /: *mut *mut unsigned int flags; / virtual service flags,
    pub /: *mut *mut unsigned int timeout; / persistent timeout in sec,
    pub /: *mut *mut __be32 netmask; / persistent netmask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_dest_user {
// destination server address
    pub addr: __be32,
    pub port: __be16,
// real server options
    pub /: *mut *mut unsigned int conn_flags; / connection flags,
    pub /: *mut *mut int weight; / destination weight,
// thresholds for active connections
    pub /: *mut *mut __u32 u_threshold; / upper threshold,
    pub /: *mut *mut __u32 l_threshold; / lower threshold,
}

//
// IPVS statistics object (for user space)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_stats_user {
    pub /: *mut *mut __u32 conns; / connections scheduled,
    pub /: *mut *mut __u32 inpkts; / incoming packets,
    pub /: *mut *mut __u32 outpkts; / outgoing packets,
    pub /: *mut *mut __u64 inbytes; / incoming bytes,
    pub /: *mut *mut __u64 outbytes; / outgoing bytes,
    pub /: *mut *mut __u32 cps; / current connection rate,
    pub /: *mut *mut __u32 inpps; / current in packet rate,
    pub /: *mut *mut __u32 outpps; / current out packet rate,
    pub /: *mut *mut __u32 inbps; / current in byte rate,
    pub /: *mut *mut __u32 outbps; / current out byte rate,
}

// The argument to IP_VS_SO_GET_INFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_getinfo {
// version number
    pub version: c_uint,
// size of connection hash table
    pub size: c_uint,
// number of virtual services
    pub num_services: c_uint,
}

// The argument to IP_VS_SO_GET_SERVICE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_service_entry {
// which service: user fills in these
    pub protocol: __u16,
    pub /: *mut *mut __be32 addr; / virtual address,
    pub port: __be16,
    pub /: *mut *mut __u32 fwmark; / firwall mark of service,
// service options
    pub sched_name: [c_char; IP_VS_SCHEDNAME_MAXLEN],
    pub /: *mut *mut unsigned int flags; / virtual service flags,
    pub /: *mut *mut unsigned int timeout; / persistent timeout,
    pub /: *mut *mut __be32 netmask; / persistent netmask,
// number of real servers
    pub num_dests: c_uint,
// statistics
    pub stats: ip_vs_stats_user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_dest_entry {
    pub /: *mut *mut __be32 addr; / destination address,
    pub port: __be16,
    pub /: *mut *mut unsigned int conn_flags; / connection flags,
    pub /: *mut *mut int weight; / destination weight,
    pub /: *mut *mut __u32 u_threshold; / upper threshold,
    pub /: *mut *mut __u32 l_threshold; / lower threshold,
    pub /: *mut *mut __u32 activeconns; / active connections,
    pub /: *mut *mut __u32 inactconns; / inactive connections,
    pub /: *mut *mut __u32 persistconns; / persistent connections,
// statistics
    pub stats: ip_vs_stats_user,
}

// The argument to IP_VS_SO_GET_DESTS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_get_dests {
// which service: user fills in these
    pub protocol: __u16,
    pub /: *mut *mut __be32 addr; / virtual address,
    pub port: __be16,
    pub /: *mut *mut __u32 fwmark; / firwall mark of service,
// number of real servers
    pub num_dests: c_uint,
// the real servers
    pub entrytable: [ip_vs_dest_entry; ],
}

// The argument to IP_VS_SO_GET_SERVICES
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_get_services {
// number of virtual services
    pub num_services: c_uint,
// service table
    pub entrytable: [ip_vs_service_entry; ],
}

// The argument to IP_VS_SO_GET_TIMEOUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_timeout_user {
    pub tcp_timeout: c_int,
    pub tcp_fin_timeout: c_int,
    pub udp_timeout: c_int,
}

// The argument to IP_VS_SO_GET_DAEMON
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_daemon_user {
// sync daemon state (master/backup)
    pub state: c_int,
// multicast interface name
    pub mcast_ifn: [c_char; IP_VS_IFNAME_MAXLEN],
// SyncID we belong to
    pub syncid: c_int,
}

//
// IPVS Generic Netlink interface definitions
//
// Generic Netlink family info

pub const IPVS_GENL_VERSION: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_vs_flags {
    pub flags: __u32,
    pub mask: __u32,
}

// Generic Netlink command attributes

// Attributes used in the first level of commands

//
// Attributes used to describe a service
//
// Used inside nested attribute IPVS_CMD_ATTR_SERVICE
//

//
// Attributes used to describe a destination (real server)
//
// Used inside nested attribute IPVS_CMD_ATTR_DEST
//

//
// Attributes describing a sync daemon
//
// Used inside nested attribute IPVS_CMD_ATTR_DAEMON
//

//
// Attributes used to describe service or destination entry statistics
//
// Used inside nested attributes IPVS_SVC_ATTR_STATS, IPVS_DEST_ATTR_STATS,
// IPVS_SVC_ATTR_STATS64 and IPVS_DEST_ATTR_STATS64.
//

// Attributes used in response to IPVS_CMD_GET_INFO command

