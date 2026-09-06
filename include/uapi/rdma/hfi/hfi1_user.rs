//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/hfi/hfi1_user.h
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
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// GPL LICENSE SUMMARY
//
// Copyright(c) 2015 - 2020 Intel Corporation.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of version 2 of the GNU General Public License as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// BSD LICENSE
//
// Copyright(c) 2015 Intel Corporation.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// - Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// - Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
// - Neither the name of Intel Corporation nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// This file contains defines, structures, etc. that are used
// to communicate between kernel and user code.
//

//
// This version number is given to the driver by the user code during
// initialization in the spu_userversion field of hfi1_user_info, so
// the driver can check for compatibility with user code.
//
// The major version changes when data structures change in an incompatible
// way. The driver must be the same for initialization to succeed.
//
pub const HFI1_USER_SWMAJOR: c_int = 6;
//
// Minor version differences are always compatible
// a within a major version, however if user software is larger
// than driver software, some new features and/or structure fields
// may not be implemented; the user code must deal with this if it
// cares, or it must abort after initialization reports the difference.
//
pub const HFI1_USER_SWMINOR: c_int = 3;
//
// We will encode the major/minor inside a single 32bit version number.
//
pub const HFI1_SWMAJOR_SHIFT: c_int = 16;
//
// Set of HW and driver capability/feature bits.
// These bit values are used to configure enabled/disabled HW and
// driver features. The same set of bits are communicated to user
// space.
//

pub const _HFI1_EVENT_FROZEN_BIT: c_int = 0;
pub const _HFI1_EVENT_LINKDOWN_BIT: c_int = 1;
pub const _HFI1_EVENT_LID_CHANGE_BIT: c_int = 2;
pub const _HFI1_EVENT_LMC_CHANGE_BIT: c_int = 3;
pub const _HFI1_EVENT_SL2VL_CHANGE_BIT: c_int = 4;
pub const _HFI1_EVENT_TID_MMU_NOTIFY_BIT: c_int = 5;

//
// These are the status bits readable (in ASCII form, 64bit value)
// from the "status" sysfs file.  For binary compatibility, values
// must remain as is; removed states can be reused for different
// purposes.
//
pub const HFI1_STATUS_INITTED: c_uint = 0x1    /* basic initialization done */;
// Chip has been found and initialized
pub const HFI1_STATUS_CHIP_PRESENT: c_uint = 0x20;
// IB link is at ACTIVE, usable for data traffic
pub const HFI1_STATUS_IB_READY: c_uint = 0x40;
// link is configured, LID, MTU, etc. have been set
pub const HFI1_STATUS_IB_CONF: c_uint = 0x80;
// A Fatal hardware error has occurred.
pub const HFI1_STATUS_HWERROR: c_uint = 0x200;
//
// Number of supported shared contexts.
// This is the maximum number of software contexts that can share
// a hardware send/receive context.
//
pub const HFI1_MAX_SHARED_CTXTS: c_int = 8;
//
// Poll types
//
pub const HFI1_POLL_TYPE_ANYRCV: c_uint = 0x0;
pub const HFI1_POLL_TYPE_URGENT: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi1_sdma_comp_state {
    FREE = 0,
    QUEUED,
    COMPLETE,
    ERROR
}

//
// SDMA completion ring entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_sdma_comp_entry {
    pub status: __u32,
    pub errcode: __u32,
}

//
// Device status and notifications from driver to user-space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_status {
    pub /: *mut *mut __aligned_u64 dev; / device/hw status bits,
    pub /: *mut *mut __aligned_u64 port; / port state and status bits,
    pub freezemsg: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdma_req_opcode {
    EXPECTED = 0,
    EAGER
}

pub const HFI1_SDMA_REQ_VERSION_MASK: c_uint = 0xF;
pub const HFI1_SDMA_REQ_VERSION_SHIFT: c_uint = 0x0;
pub const HFI1_SDMA_REQ_OPCODE_MASK: c_uint = 0xF;
pub const HFI1_SDMA_REQ_OPCODE_SHIFT: c_uint = 0x4;
pub const HFI1_SDMA_REQ_IOVCNT_MASK: c_uint = 0xFF;
pub const HFI1_SDMA_REQ_IOVCNT_SHIFT: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_req_info {
//
// bits 0-3 - version (currently unused)
// bits 4-7 - opcode (enum sdma_req_opcode)
// bits 8-15 - io vector count
//
    pub ctrl: __u16,
//
// Number of fragments contained in this request.
// User-space has already computed how many
// fragment-sized packet the user buffer will be
// split into.
//
    pub npkts: __u16,
//
// Size of each fragment the user buffer will be
// split into.
//
    pub fragsize: __u16,
//
// Index of the slot in the SDMA completion ring
// this request should be using. User-space is
// in charge of managing its own ring.
//
    pub comp_idx: __u16,
    pub __attribute__((__packed__)): },
//
// SW KDETH header.
// swdata is SW defined portion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_kdeth_header {
    pub ver_tid_offset: __le32,
    pub jkey: __le16,
    pub hcrc: __le16,
    pub swdata: [__le32; 7],
    pub __attribute__((__packed__)): },
//
// Structure describing the headers that User space uses. The
// structure above is a subset of this one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_pkt_header {
    pub pbc: [__le16; 4],
    pub lrh: [__be16; 4],
    pub bth: [__be32; 3],
    pub kdeth: hfi1_kdeth_header,
    pub __attribute__((__packed__)): },
//
// The list of usermode accessible registers.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi1_ureg {
// (RO)  DMA RcvHdr to be used next.
    ur_rcvhdrtail = 0,
// (RW)  RcvHdr entry to be processed next by host.
    ur_rcvhdrhead = 1,
// (RO)  Index of next Eager index to use.
    ur_rcvegrindextail = 2,
// (RW)  Eager TID to be processed next
    ur_rcvegrindexhead = 3,
// (RO)  Receive Eager Offset Tail
    ur_rcvegroffsettail = 4,
// For internal use only; max register number.
    ur_maxreg,
// (RW)  Receive TID flow table
    ur_rcvtidflowtable = 256
}
