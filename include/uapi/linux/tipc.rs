//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/tipc.h
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
// include/uapi/linux/tipc.h: Header for TIPC socket interface
//
// Copyright (c) 2003-2006, 2015-2016 Ericsson AB
// Copyright (c) 2005, 2010-2011, Wind River Systems
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
// TIPC addressing primitives
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_socket_addr {
    pub ref: __u32,
    pub node: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_service_addr {
    pub type: __u32,
    pub instance: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_service_range {
    pub type: __u32,
    pub lower: __u32,
    pub upper: __u32,
}

//
// Application-accessible service types
//

//
// Publication scopes when binding service / service range
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tipc_scope {
    TIPC_CLUSTER_SCOPE = 2, /* 0 can also be used */
    TIPC_NODE_SCOPE    = 3
}

//
// Limiting values for messages
//

//
// Message importance levels
//
pub const TIPC_LOW_IMPORTANCE: c_int = 0;
pub const TIPC_MEDIUM_IMPORTANCE: c_int = 1;
pub const TIPC_HIGH_IMPORTANCE: c_int = 2;
pub const TIPC_CRITICAL_IMPORTANCE: c_int = 3;
//
// Msg rejection/connection shutdown reasons
//
pub const TIPC_OK: c_int = 0;
pub const TIPC_ERR_NO_NAME: c_int = 1;
pub const TIPC_ERR_NO_PORT: c_int = 2;
pub const TIPC_ERR_NO_NODE: c_int = 3;
pub const TIPC_ERR_OVERLOAD: c_int = 4;
pub const TIPC_CONN_SHUTDOWN: c_int = 5;
//
// TIPC topology subscription service definitions
//
pub const TIPC_SUB_PORTS: c_uint = 0x01    /* filter: evt at each match */;
pub const TIPC_SUB_SERVICE: c_uint = 0x02    /* filter: evt at first up/last down */;
pub const TIPC_SUB_CANCEL: c_uint = 0x04    /* filter: cancel a subscription */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_subscr {
    pub /: *mut *mut tipc_service_range seq; / range of interest,
    pub /: *mut *mut __u32 timeout; / subscription duration (in ms),
    pub /: *mut *mut __u32 filter; / bitmask of filter options,
    pub /: *mut *mut char usr_handle[8]; / available for subscriber use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_event {
    pub /: *mut *mut __u32 event; / event type,
    pub /: *mut *mut __u32 found_lower; / matching range,
    pub /: *mut *mut __u32 found_upper; / " ",
    pub /: *mut *mut tipc_socket_addr port; / associated socket,
    pub /: *mut *mut tipc_subscr s; / associated subscription,
}

//
// Socket API
//

pub const AF_TIPC: c_int = 30;

pub const SOL_TIPC: c_int = 271;

pub const TIPC_ADDR_MCAST: c_int = 1;
pub const TIPC_SERVICE_RANGE: c_int = 1;
pub const TIPC_SERVICE_ADDR: c_int = 2;
pub const TIPC_SOCKET_ADDR: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_tipc {
    pub family: c_ushort,
    pub addrtype: c_uchar,
    pub scope: signed char,
    pub id: tipc_socket_addr,
    pub nameseq: tipc_service_range,
    pub name: tipc_service_addr,
    pub domain: __u32,
    pub name: },
    pub addr: },
}

//
// Ancillary data objects supported by recvmsg()
//

//
// TIPC-specific socket option names
//

//
// Flag values
//
pub const TIPC_GROUP_LOOPBACK: c_uint = 0x1  /* Receive copy of sent msg when match */;
pub const TIPC_GROUP_MEMBER_EVTS: c_uint = 0x2  /* Receive membership events in socket */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_group_req {
    pub /: *mut *mut __u32 type; / group id,
    pub /: *mut *mut __u32 instance; / member id,
    pub /: *mut *mut __u32 scope; / cluster/node,
    pub flags: __u32,
}

//
// Maximum sizes of TIPC bearer-related names (including terminating NULL)
// The string formatting for each name element is:
// media: media
// interface: media:interface name
// link: node:interface-node:interface
//
pub const TIPC_NODEID_LEN: c_int = 16;
pub const TIPC_MAX_MEDIA_NAME: c_int = 16;
pub const TIPC_MAX_IF_NAME: c_int = 16;
pub const TIPC_MAX_BEARER_NAME: c_int = 32;
pub const TIPC_MAX_LINK_NAME: c_int = 68;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_sioc_ln_req {
    pub peer: __u32,
    pub bearer_id: __u32,
    pub linkname: [c_char; TIPC_MAX_LINK_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_sioc_nodeid_req {
    pub peer: __u32,
    pub node_id: [c_char; TIPC_NODEID_LEN],
}

//
// TIPC Crypto, AEAD
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_aead_key {
    pub alg_name: [c_char; TIPC_AEAD_ALG_NAME],
    pub /: *mut *mut unsigned int keylen; / in bytes,
    pub key: [c_char; ],
}

// The macros and functions below are deprecated:
//
pub const TIPC_CFG_SRV: c_int = 0;
pub const TIPC_ZONE_SCOPE: c_int = 1;
pub const TIPC_ADDR_NAMESEQ: c_int = 1;
pub const TIPC_ADDR_NAME: c_int = 2;
pub const TIPC_ADDR_ID: c_int = 3;
pub const TIPC_NODE_BITS: c_int = 12;
pub const TIPC_CLUSTER_BITS: c_int = 12;
pub const TIPC_ZONE_BITS: c_int = 8;
pub const TIPC_NODE_OFFSET: c_int = 0;

