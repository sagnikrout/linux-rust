//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb3/firmware_exports.h
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
// Copyright (c) 2004-2008 Chelsio, Inc. All rights reserved.
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
// WR OPCODES supported by the firmware.
//
pub const FW_WROPCODE_FORWARD: c_uint = 0x01;
pub const FW_WROPCODE_BYPASS: c_uint = 0x05;
pub const FW_WROPCODE_TUNNEL_TX_PKT: c_uint = 0x03;
pub const FW_WROPOCDE_ULPTX_DATA_SGL: c_uint = 0x00;
pub const FW_WROPCODE_ULPTX_MEM_READ: c_uint = 0x02;
pub const FW_WROPCODE_ULPTX_PKT: c_uint = 0x04;
pub const FW_WROPCODE_ULPTX_INVALIDATE: c_uint = 0x06;
pub const FW_WROPCODE_TUNNEL_RX_PKT: c_uint = 0x07;
pub const FW_WROPCODE_OFLD_GETTCB_RPL: c_uint = 0x08;
pub const FW_WROPCODE_OFLD_CLOSE_CON: c_uint = 0x09;
pub const FW_WROPCODE_OFLD_TP_ABORT_CON_REQ: c_uint = 0x0A;
pub const FW_WROPCODE_OFLD_HOST_ABORT_CON_RPL: c_uint = 0x0F;
pub const FW_WROPCODE_OFLD_HOST_ABORT_CON_REQ: c_uint = 0x0B;
pub const FW_WROPCODE_OFLD_TP_ABORT_CON_RPL: c_uint = 0x0C;
pub const FW_WROPCODE_OFLD_TX_DATA: c_uint = 0x0D;
pub const FW_WROPCODE_OFLD_TX_DATA_ACK: c_uint = 0x0E;
pub const FW_WROPCODE_RI_RDMA_INIT: c_uint = 0x10;
pub const FW_WROPCODE_RI_RDMA_WRITE: c_uint = 0x11;
pub const FW_WROPCODE_RI_RDMA_READ_REQ: c_uint = 0x12;
pub const FW_WROPCODE_RI_RDMA_READ_RESP: c_uint = 0x13;
pub const FW_WROPCODE_RI_SEND: c_uint = 0x14;
pub const FW_WROPCODE_RI_TERMINATE: c_uint = 0x15;
pub const FW_WROPCODE_RI_RDMA_READ: c_uint = 0x16;
pub const FW_WROPCODE_RI_RECEIVE: c_uint = 0x17;
pub const FW_WROPCODE_RI_BIND_MW: c_uint = 0x18;
pub const FW_WROPCODE_RI_FASTREGISTER_MR: c_uint = 0x19;
pub const FW_WROPCODE_RI_LOCAL_INV: c_uint = 0x1A;
pub const FW_WROPCODE_RI_MODIFY_QP: c_uint = 0x1B;
pub const FW_WROPCODE_RI_BYPASS: c_uint = 0x1C;
pub const FW_WROPOCDE_RSVD: c_uint = 0x1E;
pub const FW_WROPCODE_SGE_EGRESSCONTEXT_RR: c_uint = 0x1F;
pub const FW_WROPCODE_MNGT: c_uint = 0x1D;
pub const FW_MNGTOPCODE_PKTSCHED_SET: c_uint = 0x00;
// Maximum size of a WR sent from the host, limited by the SGE.
//
// Note: WR coming from ULP or TP are only limited by CIM.
//
pub const FW_WR_SIZE: c_int = 128;
// Maximum number of outstanding WRs sent from the host. Value must be
// programmed in the CTRL/TUNNEL/QP SGE Egress Context and used by
// offload modules to limit the number of WRs per connection.
//
pub const FW_T3_WR_NUM: c_int = 16;
pub const FW_N3_WR_NUM: c_int = 7;

// FW_TUNNEL_NUM corresponds to the number of supported TUNNEL Queues. These
// queues must start at SGE Egress Context FW_TUNNEL_SGEEC_START and must
// start at 'TID' (or 'uP Token') FW_TUNNEL_TID_START.
//
// Ingress Traffic (e.g. DMA completion credit)  for TUNNEL Queue[i] is sent
// to RESP Queue[i].
//
pub const FW_TUNNEL_NUM: c_int = 8;
pub const FW_TUNNEL_SGEEC_START: c_int = 8;
pub const FW_TUNNEL_TID_START: c_int = 65544;
// FW_CTRL_NUM corresponds to the number of supported CTRL Queues. These queues
// must start at SGE Egress Context FW_CTRL_SGEEC_START and must start at 'TID'
// (or 'uP Token') FW_CTRL_TID_START.
//
// Ingress Traffic for CTRL Queue[i] is sent to RESP Queue[i].
//
pub const FW_CTRL_NUM: c_int = 8;
pub const FW_CTRL_SGEEC_START: c_int = 65528;
pub const FW_CTRL_TID_START: c_int = 65536;
// FW_OFLD_NUM corresponds to the number of supported OFFLOAD Queues. These
// queues must start at SGE Egress Context FW_OFLD_SGEEC_START.
//
// Note: the 'uP Token' in the SGE Egress Context fields is irrelevant for
// OFFLOAD Queues, as the host is responsible for providing the correct TID in
// every WR.
//
// Ingress Trafffic for OFFLOAD Queue[i] is sent to RESP Queue[i].
//
pub const FW_OFLD_NUM: c_int = 8;
pub const FW_OFLD_SGEEC_START: c_int = 0;
//
pub const FW_RI_NUM: c_int = 1;
pub const FW_RI_SGEEC_START: c_int = 65527;
pub const FW_RI_TID_START: c_int = 65552;
//
// The RX_PKT_TID
//
pub const FW_RX_PKT_NUM: c_int = 1;
pub const FW_RX_PKT_TID_START: c_int = 65553;
// FW_WRC_NUM corresponds to the number of Work Request Context that supported
// by the firmware.
//

//
// FW type and version.
//
pub const S_FW_VERSION_TYPE: c_int = 28;
pub const M_FW_VERSION_TYPE: c_uint = 0xF;

pub const S_FW_VERSION_MAJOR: c_int = 16;
pub const M_FW_VERSION_MAJOR: c_uint = 0xFFF;

pub const S_FW_VERSION_MINOR: c_int = 8;
pub const M_FW_VERSION_MINOR: c_uint = 0xFF;

pub const S_FW_VERSION_MICRO: c_int = 0;
pub const M_FW_VERSION_MICRO: c_uint = 0xFF;

