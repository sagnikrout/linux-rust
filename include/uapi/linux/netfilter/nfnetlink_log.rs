//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nfnetlink_log.h
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
// This file describes the netlink messages (i.e. 'protocol packets'),
// and not any kind of function definitions.  It is shared between kernel and
// userspace.  Don't put kernel specific stuff in here

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfulnl_msg_types {
    NFULNL_MSG_PACKET,		/* packet from kernel to userspace */
    NFULNL_MSG_CONFIG,		/* connect to a particular queue */

    NFULNL_MSG_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfulnl_msg_packet_hdr {
    pub /: *mut *mut __be16 hw_protocol; / hw protocol (network order),
    pub /: *mut *mut __u8 hook; / netfilter hook,
    pub _pad: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfulnl_msg_packet_hw {
    pub hw_addrlen: __be16,
    pub _pad: __u16,
    pub hw_addr: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfulnl_msg_packet_timestamp {
    pub sec: __aligned_be64,
    pub usec: __aligned_be64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfulnl_vlan_attr {
    NFULA_VLAN_UNSPEC,
    NFULA_VLAN_PROTO,		/* __be16 skb vlan_proto */
    NFULA_VLAN_TCI,			/* __be16 skb htons(vlan_tci) */
    __NFULA_VLAN_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfulnl_attr_type {
    NFULA_UNSPEC,
    NFULA_PACKET_HDR,
    NFULA_MARK,			/* __u32 nfmark */
    NFULA_TIMESTAMP,		/* nfulnl_msg_packet_timestamp */
    NFULA_IFINDEX_INDEV,		/* __u32 ifindex */
    NFULA_IFINDEX_OUTDEV,		/* __u32 ifindex */
    NFULA_IFINDEX_PHYSINDEV,	/* __u32 ifindex */
    NFULA_IFINDEX_PHYSOUTDEV,	/* __u32 ifindex */
    NFULA_HWADDR,			/* nfulnl_msg_packet_hw */
    NFULA_PAYLOAD,			/* opaque data payload */
    NFULA_PREFIX,			/* string prefix */
    NFULA_UID,			/* user id of socket */
    NFULA_SEQ,			/* instance-local sequence number */
    NFULA_SEQ_GLOBAL,		/* global sequence number */
    NFULA_GID,			/* group id of socket */
    NFULA_HWTYPE,			/* hardware type */
    NFULA_HWHEADER,			/* hardware header */
    NFULA_HWLEN,			/* hardware header length */
    NFULA_CT,                       /* nfnetlink_conntrack.h */
    NFULA_CT_INFO,                  /* enum ip_conntrack_info */
    NFULA_VLAN,			/* nested attribute: packet vlan info */
    NFULA_L2HDR,			/* full L2 header */

    __NFULA_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfulnl_msg_config_cmds {
    NFULNL_CFG_CMD_NONE,
    NFULNL_CFG_CMD_BIND,
    NFULNL_CFG_CMD_UNBIND,
    NFULNL_CFG_CMD_PF_BIND,
    NFULNL_CFG_CMD_PF_UNBIND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfulnl_msg_config_cmd {
    pub /: *mut *mut __u8 command; / nfulnl_msg_config_cmds,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfulnl_msg_config_mode {
    pub copy_range: __be32,
    pub copy_mode: __u8,
    pub _pad: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfulnl_attr_config {
    NFULA_CFG_UNSPEC,
    NFULA_CFG_CMD,			/* nfulnl_msg_config_cmd */
    NFULA_CFG_MODE,			/* nfulnl_msg_config_mode */
    NFULA_CFG_NLBUFSIZ,		/* __u32 buffer size */
    NFULA_CFG_TIMEOUT,		/* __u32 in 1/100 s */
    NFULA_CFG_QTHRESH,		/* __u32 */
    NFULA_CFG_FLAGS,		/* __u16 */
    __NFULA_CFG_MAX
}

pub const NFULNL_COPY_NONE: c_uint = 0x00;
pub const NFULNL_COPY_META: c_uint = 0x01;
pub const NFULNL_COPY_PACKET: c_uint = 0x02;
// 0xff is reserved, don't use it for new copy modes.
pub const NFULNL_CFG_F_SEQ: c_uint = 0x0001;
pub const NFULNL_CFG_F_SEQ_GLOBAL: c_uint = 0x0002;
pub const NFULNL_CFG_F_CONNTRACK: c_uint = 0x0004;
