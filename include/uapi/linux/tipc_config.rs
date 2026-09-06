//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/tipc_config.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// include/uapi/linux/tipc_config.h: Header for TIPC configuration interface
//
// Copyright (c) 2003-2006, Ericsson AB
// Copyright (c) 2005-2007, 2010-2011, Wind River Systems
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the names of the copyright holders nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//

//
// Configuration
//
// All configuration management messaging involves sending a request message
// to the TIPC configuration service on a node, which sends a reply message
// back.  (In the future multi-message replies may be supported.)
//
// Both request and reply messages consist of a transport header and payload.
// The transport header contains info about the desired operation;
// the payload consists of zero or more type/length/value (TLV) items
// which specify parameters or results for the operation.
//
// For many operations, the request and reply messages have a fixed number
// of TLVs (usually zero or one); however, some reply messages may return
// a variable number of TLVs.  A failed request is denoted by the presence
// of an "error string" TLV in the reply message instead of the TLV(s) the
// reply should contain if the request succeeds.
//
// Public commands:
// May be issued by any process.
// Accepted by own node, or by remote node only if remote management enabled.
//
pub const TIPC_CMD_NOOP: c_uint = 0x0000    /* tx none, rx none */;
pub const TIPC_CMD_GET_NODES: c_uint = 0x0001    /* tx net_addr, rx node_info(s) */;
pub const TIPC_CMD_GET_MEDIA_NAMES: c_uint = 0x0002    /* tx none, rx media_name(s) */;
pub const TIPC_CMD_GET_BEARER_NAMES: c_uint = 0x0003    /* tx none, rx bearer_name(s) */;
pub const TIPC_CMD_GET_LINKS: c_uint = 0x0004    /* tx net_addr, rx link_info(s) */;
pub const TIPC_CMD_SHOW_NAME_TABLE: c_uint = 0x0005    /* tx name_tbl_query, rx ultra_string */;
pub const TIPC_CMD_SHOW_PORTS: c_uint = 0x0006    /* tx none, rx ultra_string */;
pub const TIPC_CMD_SHOW_LINK_STATS: c_uint = 0x000B    /* tx link_name, rx ultra_string */;
pub const TIPC_CMD_SHOW_STATS: c_uint = 0x000F    /* tx unsigned, rx ultra_string */;
//
// Protected commands:
// May only be issued by "network administration capable" process.
// Accepted by own node, or by remote node only if remote management enabled
// and this node is zone manager.
//
pub const TIPC_CMD_GET_REMOTE_MNG: c_uint = 0x4003    /* tx none, rx unsigned */;
pub const TIPC_CMD_GET_MAX_PORTS: c_uint = 0x4004    /* tx none, rx unsigned */;
pub const TIPC_CMD_GET_MAX_PUBL: c_uint = 0x4005    /* obsoleted */;
pub const TIPC_CMD_GET_MAX_SUBSCR: c_uint = 0x4006    /* obsoleted */;
pub const TIPC_CMD_GET_MAX_ZONES: c_uint = 0x4007    /* obsoleted */;
pub const TIPC_CMD_GET_MAX_CLUSTERS: c_uint = 0x4008    /* obsoleted */;
pub const TIPC_CMD_GET_MAX_NODES: c_uint = 0x4009    /* obsoleted */;
pub const TIPC_CMD_GET_MAX_SLAVES: c_uint = 0x400A    /* obsoleted */;
pub const TIPC_CMD_GET_NETID: c_uint = 0x400B    /* tx none, rx unsigned */;
pub const TIPC_CMD_ENABLE_BEARER: c_uint = 0x4101    /* tx bearer_config, rx none */;
pub const TIPC_CMD_DISABLE_BEARER: c_uint = 0x4102    /* tx bearer_name, rx none */;
pub const TIPC_CMD_SET_LINK_TOL: c_uint = 0x4107    /* tx link_config, rx none */;
pub const TIPC_CMD_SET_LINK_PRI: c_uint = 0x4108    /* tx link_config, rx none */;
pub const TIPC_CMD_SET_LINK_WINDOW: c_uint = 0x4109    /* tx link_config, rx none */;
pub const TIPC_CMD_SET_LOG_SIZE: c_uint = 0x410A    /* obsoleted */;
pub const TIPC_CMD_DUMP_LOG: c_uint = 0x410B    /* obsoleted */;
pub const TIPC_CMD_RESET_LINK_STATS: c_uint = 0x410C    /* tx link_name, rx none */;
//
// Private commands:
// May only be issued by "network administration capable" process.
// Accepted by own node only; cannot be used on a remote node.
//
pub const TIPC_CMD_SET_NODE_ADDR: c_uint = 0x8001    /* tx net_addr, rx none */;
pub const TIPC_CMD_SET_REMOTE_MNG: c_uint = 0x8003    /* tx unsigned, rx none */;
pub const TIPC_CMD_SET_MAX_PORTS: c_uint = 0x8004    /* tx unsigned, rx none */;
pub const TIPC_CMD_SET_MAX_PUBL: c_uint = 0x8005    /* obsoleted */;
pub const TIPC_CMD_SET_MAX_SUBSCR: c_uint = 0x8006    /* obsoleted */;
pub const TIPC_CMD_SET_MAX_ZONES: c_uint = 0x8007    /* obsoleted */;
pub const TIPC_CMD_SET_MAX_CLUSTERS: c_uint = 0x8008    /* obsoleted */;
pub const TIPC_CMD_SET_MAX_NODES: c_uint = 0x8009    /* obsoleted */;
pub const TIPC_CMD_SET_MAX_SLAVES: c_uint = 0x800A    /* obsoleted */;
pub const TIPC_CMD_SET_NETID: c_uint = 0x800B    /* tx unsigned, rx none */;
//
// Reserved commands:
// May not be issued by any process.
// Used internally by TIPC.
//
pub const TIPC_CMD_NOT_NET_ADMIN: c_uint = 0xC001    /* tx none, rx none */;
//
// TLV types defined for TIPC
//

//
// Link priority limits (min, default, max, media default)
//
pub const TIPC_MIN_LINK_PRI: c_int = 0;
pub const TIPC_DEF_LINK_PRI: c_int = 10;
pub const TIPC_MAX_LINK_PRI: c_int = 31;

//
// Link tolerance limits (min, default, max), in ms
//
pub const TIPC_MIN_LINK_TOL: c_int = 50;
pub const TIPC_DEF_LINK_TOL: c_int = 1500;
pub const TIPC_MAX_LINK_TOL: c_int = 30000;

//
// Link window limits (min, default, max), in packets
//
pub const TIPC_MIN_LINK_WIN: c_int = 16;
pub const TIPC_DEF_LINK_WIN: c_int = 50;
pub const TIPC_MAX_LINK_WIN: c_int = 8191;
//
// Default MTU for UDP media
//
pub const TIPC_DEF_LINK_UDP_MTU: c_int = 14000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_node_info {
    pub /: *mut *mut __be32 addr; / network address of node,
    pub /: *mut *mut __be32 up; / 0=down, 1= up,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_link_info {
    pub /: *mut *mut __be32 dest; / network address of peer node,
    pub /: *mut *mut __be32 up; / 0=down, 1=up,
    pub /: *mut *mut char str[TIPC_MAX_LINK_NAME]; / link name,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_bearer_config {
    pub /: *mut *mut __be32 priority; / Range [1,31]. Override per link,
    pub /: *mut *mut __be32 disc_domain; / <Z.C.N> describing desired nodes,
    pub name: [c_char; TIPC_MAX_BEARER_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_link_config {
    pub value: __be32,
    pub name: [c_char; TIPC_MAX_LINK_NAME],
}

pub const TIPC_NTQ_ALLTYPES: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_name_table_query {
    pub /: *mut *mut __be32 depth; / 1:type, 2:+name info, 3:+port info, 4+:+debug info,
    pub /: *mut *mut __be32 type; / {t,l,u} info ignored if high bit of "depth" is set,
    pub /: *mut *mut __be32 lowbound; / (i.e. displays all entries of name table),
    pub upbound: __be32,
}

//
// The error string TLV is a null-terminated string describing the cause
// of the request failure.  To simplify error processing (and to save space)
// the first character of the string can be a special error code character
// (lying by the range 0x80 to 0xFF) which represents a pre-defined reason.
//

//
// A TLV consists of a descriptor, followed by the TLV value.
// TLV descriptor fields are stored in network byte order;
// TLV values must also be stored in network byte order (where applicable).
// TLV descriptors must be aligned to addresses which are multiple of 4,
// so up to 3 bytes of padding may exist at the end of the TLV value area.
// There must not be any padding between the TLV descriptor and its value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlv_desc {
    pub /: *mut *mut __be16 tlv_len; / TLV length (descriptor + value),
    pub /: *mut *mut __be16 tlv_type; / TLV identifier,
}

pub const TLV_ALIGNTO: c_int = 4;

//
// Would also like to check that "tlv" is a multiple of 4,
// but don't know how to do this in a portable way.
// - Tried doing (!(tlv & (TLV_ALIGNTO-1))), but GCC compiler
// won't allow binary "&" with a pointer.
// - Tried casting "tlv" to integer type, but causes warning about size
// mismatch when pointer is bigger than chosen type (int, long, ...).
//
extern "C" {
    pub fn __be16_to_cpu(_arg: tlv->tlv_len) -> return;
}
extern "C" {
    pub fn TLV_SPACE(_arg: len) -> return;
}
//
// A TLV list descriptor simplifies processing of messages
// containing multiple TLVs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlv_list_desc {
    pub /: *mut *mut *mut tlv_desc tlv_ptr; / ptr to current TLV,
    pub /: *mut *mut __u32 tlv_space; / # bytes from curr TLV to list end,
}

extern "C" {
    pub fn TLV_CHECK(_arg: list->tlv_ptr, _arg: list->tlv_space, _arg: exp_type) -> return;
}
extern "C" {
    pub fn TLV_DATA(_arg: list->tlv_ptr) -> return;
}
//
// Configuration messages exchanged via NETLINK_GENERIC use the following
// family id, name, version and command.
//

pub const TIPC_GENL_VERSION: c_uint = 0x1;
pub const TIPC_GENL_CMD: c_uint = 0x1;
//
// TIPC specific header used in NETLINK_GENERIC requests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_genlmsghdr {
    pub /: *mut *mut __u32 dest; / Destination address,
    pub /: *mut *mut __u16 cmd; / Command,
    pub /: *mut *mut __u16 reserved; / Unused,
}

//
// Configuration messages exchanged via TIPC sockets use the TIPC configuration
// message header, which is defined below.  This structure is analogous
// to the Netlink message header, but fields are stored in network byte order
// and no padding is permitted between the header and the message data
// that follows.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_cfg_msg_hdr {
    pub /: *mut *mut __be32 tcm_len; / Message length (including header),
    pub /: *mut *mut __be16 tcm_type; / Command type,
    pub /: *mut *mut __be16 tcm_flags; / Additional flags,
    pub /: *mut *mut char tcm_reserved[8]; / Unused,
}

pub const TCM_F_REQUEST: c_uint = 0x1	/* Flag: Request message */;
pub const TCM_F_MORE: c_uint = 0x2	/* Flag: Message to be continued */;

extern "C" {
    pub fn TCM_SPACE(_arg: data_len) -> return;
}
