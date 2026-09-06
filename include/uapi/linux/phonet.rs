//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/phonet.h
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
// file phonet.h
//
// Phonet sockets kernel interface
//
// Copyright (C) 2008 Nokia Corporation. All rights reserved.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// version 2 as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA
// 02110-1301 USA
//

// Automatic protocol selection
pub const PN_PROTO_TRANSPORT: c_int = 0;
// Phonet datagram socket
pub const PN_PROTO_PHONET: c_int = 1;
// Phonet pipe
pub const PN_PROTO_PIPE: c_int = 2;
pub const PHONET_NPROTO: c_int = 3;
// Socket options for SOL_PNPIPE level
pub const PNPIPE_ENCAP: c_int = 1;
pub const PNPIPE_IFINDEX: c_int = 2;
pub const PNPIPE_HANDLE: c_int = 3;
pub const PNPIPE_INITSTATE: c_int = 4;
pub const PNADDR_ANY: c_int = 0;
pub const PNADDR_BROADCAST: c_uint = 0xFC;
pub const PNPORT_RESOURCE_ROUTING: c_int = 0;
// Values for PNPIPE_ENCAP option
pub const PNPIPE_ENCAP_NONE: c_int = 0;
pub const PNPIPE_ENCAP_IP: c_int = 1;
// ioctls

// Phonet protocol header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phonethdr {
    pub pn_rdev: __u8,
    pub pn_sdev: __u8,
    pub pn_res: __u8,
    pub pn_length: __be16,
    pub pn_robj: __u8,
    pub pn_sobj: __u8,
    pub __attribute__((packed)): },
// Common Phonet payload header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phonetmsg {
    pub /: *mut *mut __u8 pn_trans_id; / transaction ID,
    pub /: *mut *mut __u8 pn_msg_id; / message type,
    pub /: *mut *mut __u8 pn_submsg_id; / message subtype,
    pub pn_data: [__u8; 5],
    pub base: },
    pub /: *mut *mut __u16 pn_e_res_id; / extended resource ID,
    pub /: *mut *mut __u8 pn_e_submsg_id; / message subtype,
    pub pn_e_data: [__u8; 3],
    pub ext: },
    pub pn_msg_u: },
}

pub const PN_COMMON_MESSAGE: c_uint = 0xF0;
pub const PN_COMMGR: c_uint = 0x10;
pub const PN_PREFIX: c_uint = 0xE0 /* resource for extended messages */;

// data for unreachable errors
pub const PN_COMM_SERVICE_NOT_IDENTIFIED_RESP: c_uint = 0x01;
pub const PN_COMM_ISA_ENTITY_NOT_REACHABLE_RESP: c_uint = 0x14;

// Phonet socket address structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_pn {
    pub spn_family: __kernel_sa_family_t,
    pub spn_obj: __u8,
    pub spn_dev: __u8,
    pub spn_resource: __u8,
    pub 3]: __u8 spn_zero[sizeof(struct sockaddr) - sizeof(__kernel_sa_family_t) -,
    pub __attribute__((packed)): },
// Well known address
pub const PN_DEV_PC: c_uint = 0x10;
    pub 0x3ff): return (addr << 8) | (port &,
    pub 0xff: return handle &,
    pub 8: return handle >>,
    pub 0x3ff: return handle &,
    pub 0xfc: return (handle >> 8) &,
    pub 0x03: spn->spn_dev &=,
    pub 0xfc: spn->spn_dev |= addr &,
    pub 0xfc: spn->spn_dev &=,
    pub 0x03: spn->spn_dev |= (port >> 8) &,
    pub 0xff: spn->spn_obj = port &,
    pub pn_dev(handle): spn->spn_dev =,
    pub pn_obj(handle): spn->spn_obj =,
    pub resource: spn->spn_resource =,
    pub 0xfc: return spn->spn_dev &,
    pub spn->spn_obj: return ((spn->spn_dev & 0x03) << 8) |,
    pub spn->spn_obj): return pn_object(spn->spn_dev,,
    pub spn->spn_resource: return,
// Phonet device ioctl requests
