//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/t4_values.h
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
// This file contains definitions for various T4 register value hardware
// constants.  The types of values encoded here are predominantly those for
// register fields which control "modal" behavior.  For the most part, we do
// not include definitions for register fields which are simple numeric
// metrics, etc.
//
// SGE register field values.
//
// CONTROL1 register
pub const RXPKTCPLMODE_SPLIT_X: c_int = 1;
pub const INGPCIEBOUNDARY_SHIFT_X: c_int = 5;
pub const INGPCIEBOUNDARY_32B_X: c_int = 0;
pub const INGPADBOUNDARY_SHIFT_X: c_int = 5;
pub const T6_INGPADBOUNDARY_SHIFT_X: c_int = 3;
pub const T6_INGPADBOUNDARY_8B_X: c_int = 0;
pub const T6_INGPADBOUNDARY_32B_X: c_int = 2;
pub const INGPADBOUNDARY_32B_X: c_int = 0;
// CONTROL2 register
pub const INGPACKBOUNDARY_SHIFT_X: c_int = 5;
pub const INGPACKBOUNDARY_16B_X: c_int = 0;
pub const INGPACKBOUNDARY_64B_X: c_int = 1;
// GTS register
pub const SGE_TIMERREGS: c_int = 6;
pub const TIMERREG_COUNTER0_X: c_int = 0;
pub const FETCHBURSTMIN_64B_X: c_int = 2;
pub const FETCHBURSTMIN_128B_X: c_int = 3;
// T6 and later use a single-bit encoding for FetchBurstMin
pub const FETCHBURSTMIN_64B_T6_X: c_int = 0;
pub const FETCHBURSTMIN_128B_T6_X: c_int = 1;
pub const FETCHBURSTMAX_256B_X: c_int = 2;
pub const FETCHBURSTMAX_512B_X: c_int = 3;
pub const HOSTFCMODE_INGRESS_QUEUE_X: c_int = 1;
pub const HOSTFCMODE_STATUS_PAGE_X: c_int = 2;
pub const CIDXFLUSHTHRESH_32_X: c_int = 5;
pub const CIDXFLUSHTHRESH_128_X: c_int = 7;
pub const UPDATEDELIVERY_INTERRUPT_X: c_int = 1;
pub const RSPD_TYPE_FLBUF_X: c_int = 0;
pub const RSPD_TYPE_CPL_X: c_int = 1;
pub const RSPD_TYPE_INTR_X: c_int = 2;
// Congestion Manager Definitions.
//
pub const CONMCTXT_CNGTPMODE_S: c_int = 19;

pub const CONMCTXT_CNGCHMAP_S: c_int = 0;

pub const CONMCTXT_CNGTPMODE_CHANNEL_X: c_int = 2;
pub const CONMCTXT_CNGTPMODE_QUEUE_X: c_int = 1;
// T5 and later support a new BAR2-based doorbell mechanism for Egress Queues.
// The User Doorbells are each 128 bytes in length with a Simple Doorbell at
// offsets 8x and a Write Combining single 64-byte Egress Queue Unit
// (IDXSIZE_UNIT_X) Gather Buffer interface at offset 64.  For Ingress Queues,
// we have a Going To Sleep register at offsets 8x+4.
//
// As noted above, we have many instances of the Simple Doorbell and Going To
// Sleep registers at offsets 8x and 8x+4, respectively.  We want to use a
// non-64-byte aligned offset for the Simple Doorbell in order to attempt to
// avoid buffering of the writes to the Simple Doorbell and we want to use a
// non-contiguous offset for the Going To Sleep writes in order to avoid
// possible combining between them.
//
pub const SGE_UDB_SIZE: c_int = 128;
pub const SGE_UDB_KDOORBELL: c_int = 8;
pub const SGE_UDB_GTS: c_int = 20;
pub const SGE_UDB_WCDOORBELL: c_int = 64;
// CIM register field values.
//
pub const X_MBOWNER_FW: c_int = 1;
pub const X_MBOWNER_PL: c_int = 2;
// PCI-E definitions
pub const WINDOW_SHIFT_X: c_int = 10;
pub const PCIEOFST_SHIFT_X: c_int = 10;
// TP_VLAN_PRI_MAP controls which subset of fields will be present in the
// Compressed Filter Tuple for LE filters.  Each bit set in TP_VLAN_PRI_MAP
// selects for a particular field being present.  These fields, when present
// in the Compressed Filter Tuple, have the following widths in bits.
//
pub const FT_FCOE_W: c_int = 1;
pub const FT_PORT_W: c_int = 3;
pub const FT_VNIC_ID_W: c_int = 17;
pub const FT_VLAN_W: c_int = 17;
pub const FT_TOS_W: c_int = 8;
pub const FT_PROTOCOL_W: c_int = 8;
pub const FT_ETHERTYPE_W: c_int = 16;
pub const FT_MACMATCH_W: c_int = 9;
pub const FT_MPSHITTYPE_W: c_int = 3;
pub const FT_FRAGMENTATION_W: c_int = 1;
// Some of the Compressed Filter Tuple fields have internal structure.  These
// bit shifts/masks describe those structures.  All shifts are relative to the
// base position of the fields within the Compressed Filter Tuple
//
pub const FT_VLAN_VLD_S: c_int = 16;

pub const FT_VNID_ID_VF_S: c_int = 0;

pub const FT_VNID_ID_PF_S: c_int = 7;

pub const FT_VNID_ID_VLD_S: c_int = 16;

