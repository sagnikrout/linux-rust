//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx4/cq.h
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
// Copyright (c) 2007 Cisco Systems, Inc.  All rights reserved.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_cqe {
    pub vlan_my_qpn: __be32,
    pub immed_rss_invalid: __be32,
    pub g_mlpath_rqpn: __be32,
    pub sl_vid: __be16,
    pub rlid: __be16,
    pub status: __be16,
    pub ipv6_ext_mask: u8,
    pub badfcs_enc: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_err_cqe {
    pub my_qpn: __be32,
    pub reserved1: [u32; 5],
    pub wqe_index: __be16,
    pub vendor_err_syndrome: u8,
    pub syndrome: u8,
    pub reserved2: [u8; 3],
    pub owner_sr_opcode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ts_cqe {
    pub vlan_my_qpn: __be32,
    pub immed_rss_invalid: __be32,
    pub g_mlpath_rqpn: __be32,
    pub timestamp_hi: __be32,
    pub status: __be16,
    pub ipv6_ext_mask: u8,
    pub badfcs_enc: u8,
    pub byte_cnt: __be32,
    pub wqe_index: __be16,
    pub checksum: __be16,
    pub reserved: u8,
    pub timestamp_lo: __be16,
    pub owner_sr_opcode: u8,
    pub __packed: },
}

// L4_CSUM is logically part of status, but has to checked against badfcs_enc

// cq->arm_db = cpu_to_be32(sn << 28 | cmd | ci);
//
// Make sure that the doorbell record in host memory is
// written before ringing the doorbell via PCI MMIO.
//
// cq->set_ci_db = cpu_to_be32(cq->cons_index & 0xffffff);
