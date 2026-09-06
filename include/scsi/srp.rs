//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/srp.h
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
// Copyright (c) 2005 Cisco Systems.  All rights reserved.
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
// $Id$
//
// Structures and constants for the SCSI RDMA Protocol (SRP) as
// defined by the INCITS T10 committee.  This file was written using
// draft Revision 16a of the SRP standard.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_login_rej_reason {
    SRP_LOGIN_REJ_UNABLE_ESTABLISH_CHANNEL		= 0x00010000,
    SRP_LOGIN_REJ_INSUFFICIENT_RESOURCES		= 0x00010001,
    SRP_LOGIN_REJ_REQ_IT_IU_LENGTH_TOO_LARGE	= 0x00010002,
    SRP_LOGIN_REJ_UNABLE_ASSOCIATE_CHANNEL		= 0x00010003,
    SRP_LOGIN_REJ_UNSUPPORTED_DESCRIPTOR_FMT	= 0x00010004,
    SRP_LOGIN_REJ_MULTI_CHANNEL_UNSUPPORTED		= 0x00010005,
    SRP_LOGIN_REJ_CHANNEL_LIMIT_REACHED		= 0x00010006
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_direct_buf {
    pub va: __be64,
    pub key: __be32,
    pub len: __be32,
}

//
// We need the packed attribute because the SRP spec puts the list of
// descriptors at an offset of 20, which is not aligned to the size of
// struct srp_direct_buf.  The whole structure must be packed to avoid
// having the 20-byte structure padded to 24 bytes on 64-bit architectures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_indirect_buf {
    pub __aligned(4): srp_direct_buf table_desc __packed,
    pub len: __be32,
    pub __aligned(4): srp_direct_buf desc_list[] __packed,
}

// Immediate data buffer descriptor as defined in SRP2.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_imm_buf {
    pub len: __be32,
}

// srp_login_req.flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_login_req {
    pub opcode: u8,
    pub reserved1: [u8; 7],
    pub tag: u64,
    pub req_it_iu_len: __be32,
    pub reserved2: [u8; 4],
    pub req_buf_fmt: __be16,
    pub req_flags: u8,
    pub reserved3: [u8; 1],
    pub /: *mut *mut __be16 imm_data_offset; / new in SRP2,
    pub reserved4: [u8; 2],
    pub initiator_port_id: [u8; 16],
    pub target_port_id: [u8; 16],
}

//
// struct srp_login_req_rdma - RDMA/CM login parameters.
//
// RDMA/CM over InfiniBand can only carry 92 - 36 = 56 bytes of private
// data. The %srp_login_req_rdma structure contains the same information as
// %srp_login_req but with the reserved data removed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_login_req_rdma {
    pub tag: u64,
    pub req_buf_fmt: __be16,
    pub req_flags: u8,
    pub opcode: u8,
    pub req_it_iu_len: __be32,
    pub initiator_port_id: [u8; 16],
    pub target_port_id: [u8; 16],
    pub imm_data_offset: __be16,
    pub reserved: [u8; 6],
}

// srp_login_rsp.rsp_flags
//
// The SRP spec defines the size of the LOGIN_RSP structure to be 52
// bytes, so it needs to be packed to avoid having it padded to 56
// bytes on 64-bit architectures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_login_rsp {
    pub opcode: u8,
    pub reserved1: [u8; 3],
    pub req_lim_delta: __be32,
    pub __aligned(4): u64 tag __packed,
    pub max_it_iu_len: __be32,
    pub max_ti_iu_len: __be32,
    pub buf_fmt: __be16,
    pub rsp_flags: u8,
    pub reserved2: [u8; 25],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_login_rej {
    pub opcode: u8,
    pub reserved1: [u8; 3],
    pub reason: __be32,
    pub tag: u64,
    pub reserved2: [u8; 8],
    pub buf_fmt: __be16,
    pub reserved3: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_i_logout {
    pub opcode: u8,
    pub reserved: [u8; 7],
    pub tag: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_t_logout {
    pub opcode: u8,
    pub sol_not: u8,
    pub reserved: [u8; 2],
    pub reason: __be32,
    pub tag: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_tsk_mgmt {
    pub opcode: u8,
    pub sol_not: u8,
    pub reserved1: [u8; 6],
    pub tag: u64,
    pub reserved2: [u8; 4],
    pub lun: scsi_lun,
    pub reserved3: [u8; 2],
    pub tsk_mgmt_func: u8,
    pub reserved4: u8,
    pub task_tag: u64,
    pub reserved5: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_cmd {
    pub opcode: u8,
    pub sol_not: u8,
    pub reserved1: [u8; 3],
    pub buf_fmt: u8,
    pub data_out_desc_cnt: u8,
    pub data_in_desc_cnt: u8,
    pub tag: u64,
    pub reserved2: [u8; 4],
    pub lun: scsi_lun,
    pub reserved3: u8,
    pub task_attr: u8,
    pub reserved4: u8,
    pub add_cdb_len: u8,
    pub cdb: [u8; 16],
    pub add_data: [u8; ],
}

//
// The SRP spec defines the size of the RSP structure to be 36 bytes,
// so it needs to be packed to avoid having it padded to 40 bytes on
// 64-bit architectures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_rsp {
    pub opcode: u8,
    pub sol_not: u8,
    pub reserved1: [u8; 2],
    pub req_lim_delta: __be32,
    pub __aligned(4): u64 tag __packed,
    pub reserved2: [u8; 2],
    pub flags: u8,
    pub status: u8,
    pub data_out_res_cnt: __be32,
    pub data_in_res_cnt: __be32,
    pub sense_data_len: __be32,
    pub resp_data_len: __be32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_cred_req {
    pub opcode: u8,
    pub sol_not: u8,
    pub reserved: [u8; 2],
    pub req_lim_delta: __be32,
    pub tag: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_cred_rsp {
    pub opcode: u8,
    pub reserved: [u8; 7],
    pub tag: u64,
}

//
// The SRP spec defines the fixed portion of the AER_REQ structure to be
// 36 bytes, so it needs to be packed to avoid having it padded to 40 bytes
// on 64-bit architectures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_aer_req {
    pub opcode: u8,
    pub sol_not: u8,
    pub reserved: [u8; 2],
    pub req_lim_delta: __be32,
    pub __aligned(4): u64 tag __packed,
    pub reserved2: u32,
    pub lun: scsi_lun,
    pub sense_data_len: __be32,
    pub reserved3: u32,
    pub sense_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_aer_rsp {
    pub opcode: u8,
    pub reserved: [u8; 7],
    pub tag: u64,
}
