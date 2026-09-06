//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/t4_hw.h
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
// This file is part of the Chelsio T4 Ethernet driver for Linux.
//
// Copyright (c) 2003-2014 Chelsio Communications, Inc. All rights reserved.
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

// SGE context types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctxt_type {
    CTXT_EGRESS,
    CTXT_INGRESS,
    CTXT_FLM,
    CTXT_CNM,
}

// PCI-e memory window access
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcie_memwin {
    MEMWIN_NIC      = 0,
    MEMWIN_RSVD1    = 1,
    MEMWIN_RSVD2    = 2,
    MEMWIN_RDMA     = 3,
    MEMWIN_RSVD4    = 4,
    MEMWIN_FOISCSI  = 5,
    MEMWIN_CSIOSTOR = 6,
    MEMWIN_RSVD7    = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_qstat {
    pub qid: __be32,
    pub cidx: __be16,
    pub pidx: __be16,
}

//
// Structure for last 128 bits of response descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsp_ctrl {
    pub hdrbuflen_pidx: __be32,
    pub pldbuflen_qid: __be32,
    pub type_gen: u8,
    pub last_flit: __be64,
}

pub const RSPD_NEWBUF_S: c_int = 31;

pub const RSPD_LEN_S: c_int = 0;
pub const RSPD_LEN_M: c_uint = 0x7fffffff;

pub const RSPD_GEN_S: c_int = 7;
pub const RSPD_TYPE_S: c_int = 4;
pub const RSPD_TYPE_M: c_uint = 0x3;

// Rx queue interrupt deferral fields: counter enable and timer index
pub const QINTR_CNT_EN_S: c_int = 0;

pub const QINTR_TIMER_IDX_S: c_int = 1;
pub const QINTR_TIMER_IDX_M: c_uint = 0x7;

//
// Flash layout.
//

//
// Various Expansion-ROM boot images, etc.
//
// iSCSI Boot Firmware Table (iBFT) and other driver-related
// parameters ...
//
// Boot configuration data.
//
// Location of firmware image in FLASH.
//
// Location of bootstrap firmware image in FLASH.
//
// iSCSI persistent/crash information.
//
// FCoE persistent/crash information.
//
// Location of Firmware Configuration File in FLASH.  Since the FPGA
// "FLASH" is smaller we need to store the Configuration File in a
// different location -- which will overlap the end of the firmware
// image if firmware ever gets that large ...
//
// We don't support FLASH devices which can't support the full
// standard set of sections which we need for normal
// operations.
//
// Sectors 32-63 are reserved for FLASH failover.
//

pub const SGE_TIMESTAMP_S: c_int = 0;
pub const SGE_TIMESTAMP_M: c_uint = 0xfffffffffffffffULL;

pub const I2C_DEV_ADDR_A0: c_uint = 0xa0;
pub const I2C_DEV_ADDR_A2: c_uint = 0xa2;
pub const I2C_PAGE_SIZE: c_uint = 0x100;
pub const SFP_DIAG_TYPE_ADDR: c_uint = 0x5c;
pub const SFP_DIAG_TYPE_LEN: c_uint = 0x1;

pub const SFF_8472_COMP_ADDR: c_uint = 0x5e;
pub const SFF_8472_COMP_LEN: c_uint = 0x1;
pub const SFF_REV_ADDR: c_uint = 0x1;
pub const SFF_REV_LEN: c_uint = 0x1;
