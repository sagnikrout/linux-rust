//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/ulp/srpt/ib_dm_mad.h
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


//
// Copyright (c) 2006 - 2009 Mellanox Technology Inc.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// See also section 13.4.7 Status Field, table 115 MAD Common Status
// Field Bit Values and also section 16.3.1.1 Status Field in the
// InfiniBand Architecture Specification.
//
// See also the Device Management chapter, section 16.3.3 Attributes,
// table 279 Device Management Attributes in the InfiniBand
// Architecture Specification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_dm_hdr {
    pub reserved: [u8; 28],
}

//
// Structure of management datagram sent by the SRP target implementation.
// Contains a management datagram header, reliable multi-packet transaction
// protocol (RMPP) header and ib_dm_hdr. Notes:
// - The SRP target implementation does not use RMPP or ib_dm_hdr when sending
// management datagrams.
// - The header size must be exactly 64 bytes (IB_MGMT_DEVICE_HDR), since this
// is the header size that is passed to ib_create_send_mad() in ib_srpt.c.
// - The maximum supported size for a management datagram when not using RMPP
// is 256 bytes -- 64 bytes header and 192 (IB_MGMT_DEVICE_DATA) bytes data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_dm_mad {
    pub mad_hdr: ib_mad_hdr,
    pub rmpp_hdr: ib_rmpp_hdr,
    pub dm_hdr: ib_dm_hdr,
    pub data: [u8; IB_MGMT_DEVICE_DATA],
}

//
// IOUnitInfo as defined in section 16.3.3.3 IOUnitInfo of the InfiniBand
// Architecture Specification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_dm_iou_info {
    pub change_id: __be16,
    pub max_controllers: u8,
    pub op_rom: u8,
    pub controller_list: [u8; 128],
}

//
// IOControllerprofile as defined in section 16.3.3.4 IOControllerProfile of
// the InfiniBand Architecture Specification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_dm_ioc_profile {
    pub guid: __be64,
    pub vendor_id: __be32,
    pub device_id: __be32,
    pub device_version: __be16,
    pub reserved1: __be16,
    pub subsys_vendor_id: __be32,
    pub subsys_device_id: __be32,
    pub io_class: __be16,
    pub io_subclass: __be16,
    pub protocol: __be16,
    pub protocol_version: __be16,
    pub service_conn: __be16,
    pub initiators_supported: __be16,
    pub send_queue_depth: __be16,
    pub reserved2: u8,
    pub rdma_read_depth: u8,
    pub send_size: __be32,
    pub rdma_size: __be32,
    pub op_cap_mask: u8,
    pub svc_cap_mask: u8,
    pub num_svc_entries: u8,
    pub reserved3: [u8; 9],
    pub id_string: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_dm_svc_entry {
    pub name: [u8; 40],
    pub id: __be64,
}

//
// See also section 16.3.3.5 ServiceEntries in the InfiniBand Architecture
// Specification. See also section B.7, table B.8 in the T10 SRP r16a document.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_dm_svc_entries {
    pub service_entries: [ib_dm_svc_entry; 4],
}
