//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb3/cxgb3_ctl_defs.h
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
// Copyright (c) 2003-2008 Chelsio, Inc. All rights reserved.
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
// Structure used to describe a TID range.  Valid TIDs are [base, base+num).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_range {
    pub /: *mut *mut unsigned int base; / first TID,
    pub /: *mut *mut unsigned int num; / number of TIDs in range,
}

//
// Structure used to request the size and contents of the MTU table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtutab {
    pub /: *mut *mut unsigned int size; / # of entries in the MTU table,
    pub /: *const *const *const unsigned short mtus; / the MTU table values,
}

//
// Structure used to request the adapter net_device owning a given MAC address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iff_mac {
    pub /: *mut *mut *mut net_device dev; / the net_device,
    pub /: *const *const *const unsigned char mac_addr; / MAC address to lookup,
    pub vlan_tag: u16,
}

// Structure used to request a port's iSCSI IPv4 address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_ipv4addr {
    pub /: *mut *mut *mut net_device dev; / the net_device,
    pub /: *mut *mut __be32 ipv4addr; / the return iSCSI IPv4 address,
}

//
// Structure used to request the TCP DDP parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddp_params {
    pub /: *mut *mut unsigned int llimit; / TDDP region start address,
    pub /: *mut *mut unsigned int ulimit; / TDDP region end address,
    pub /: *mut *mut unsigned int tag_mask; / TDDP tag mask,
    pub pdev: *mut pci_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adap_ports {
    pub /: *mut *mut unsigned int nports; / number of ports on this adapter,
    pub lldevs: [*mut net_device; 2],
}

//
// Structure used to return information to the iscsi layer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulp_iscsi_info {
    pub offset: c_uint,
    pub llimit: c_uint,
    pub ulimit: c_uint,
    pub tagmask: c_uint,
    pub pgsz_factor: [u8; 4],
    pub max_rxsz: c_uint,
    pub max_txsz: c_uint,
    pub pdev: *mut pci_dev,
}

//
// Structure used to return information to the RDMA layer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_info {
    pub /: *mut *mut unsigned int tpt_base; / TPT base address,
    pub /: *mut *mut unsigned int tpt_top; / TPT last entry address,
    pub /: *mut *mut unsigned int pbl_base; / PBL base address,
    pub /: *mut *mut unsigned int pbl_top; / PBL last entry address,
    pub /: *mut *mut unsigned int rqt_base; / RQT base address,
    pub /: *mut *mut unsigned int rqt_top; / RQT last entry address,
    pub /: *mut *mut unsigned int udbell_len; / user doorbell region length,
    pub /: *mut *mut unsigned long udbell_physbase; / user doorbell physical start addr,
    pub /: *mut *mut *mut void __iomem kdb_addr; / kernel doorbell register address,
    pub /: *mut *mut *mut pci_dev pdev; / associated PCI device,
}

//
// Structure used to request an operation on an RDMA completion queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cq_op {
    pub id: c_uint,
    pub op: c_uint,
    pub credits: c_uint,
}

//
// Structure used to setup RDMA completion queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cq_setup {
    pub id: c_uint,
    pub base_addr: c_ulonglong,
    pub size: c_uint,
    pub credits: c_uint,
    pub credit_thres: c_uint,
    pub ovfl_mode: c_uint,
}

//
// Structure used to setup the RDMA control egress context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ctrlqp_setup {
    pub base_addr: c_ulonglong,
    pub size: c_uint,
}

//
// Offload TX/RX page information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ofld_page_info {
    pub /: *mut *mut unsigned int page_size; / Page size, should be a power of 2,
    pub /: *mut *mut unsigned int num; / Number of pages,
}

//
// Structure used to get firmware and protocol engine versions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_embedded_info {
    pub fw_vers: u32,
    pub tp_vers: u32,
}
