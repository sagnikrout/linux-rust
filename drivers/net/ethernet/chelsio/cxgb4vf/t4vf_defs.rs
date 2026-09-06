//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4vf/t4vf_defs.h
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
// This file is part of the Chelsio T4 PCI-E SR-IOV Virtual Function Ethernet
// driver for Linux.
//
// Copyright (c) 2009-2010 Chelsio Communications, Inc. All rights reserved.
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
// The VF Register Map.
//
// The Scatter Gather Engine (SGE), Multiport Support module (MPS), PIO Local
// bus module (PL) and CPU Interface Module (CIM) components are mapped via
// the Slice to Module Map Table (see below) in the Physical Function Register
// Map.  The Mail Box Data (MBDATA) range is mapped via the PCI-E Mailbox Base
// and Offset registers in the PF Register Map.  The MBDATA base address is
// quite constrained as it determines the Mailbox Data addresses for both PFs
// and VFs, and therefore must fit in both the VF and PF Register Maps without
// overlapping other registers.
//
pub const T4VF_SGE_BASE_ADDR: c_uint = 0x0000;
pub const T4VF_MPS_BASE_ADDR: c_uint = 0x0100;
pub const T4VF_PL_BASE_ADDR: c_uint = 0x0200;
pub const T4VF_MBDATA_BASE_ADDR: c_uint = 0x0240;
pub const T6VF_MBDATA_BASE_ADDR: c_uint = 0x0280;
pub const T4VF_CIM_BASE_ADDR: c_uint = 0x0300;
pub const T4VF_REGMAP_START: c_uint = 0x0000;
pub const T4VF_REGMAP_SIZE: c_uint = 0x0400;
//
// There's no hardware limitation which requires that the addresses of the
// Mailbox Data in the fixed CIM PF map and the programmable VF map must
// match.  However, it's a useful convention ...
//

//
// Virtual Function "Slice to Module Map Table" definitions.
//
// This table allows us to map subsets of the various module register sets
// into the T4VF Register Map.  Each table entry identifies the index of the
// module whose registers are being mapped, the offset within the module's
// register set that the mapping should start at, the limit of the mapping,
// and the offset within the T4VF Register Map to which the module's registers
// are being mapped.  All addresses and qualtities are in terms of 32-bit
// words.  The "limit" value is also in terms of 32-bit words and is equal to
// the last address mapped in the T4VF Register Map 1 (i.e. it's a "<="
// relation rather than a "<").
//

pub const SGE_VF_KDOORBELL: c_uint = 0x0;
pub const SGE_VF_GTS: c_uint = 0x4;
pub const MPS_VF_CTL: c_uint = 0x0;
pub const MPS_VF_STAT_RX_VF_ERR_FRAMES_H: c_uint = 0xfc;
pub const PL_VF_WHOAMI: c_uint = 0x0;
pub const CIM_VF_EXT_MAILBOX_CTRL: c_uint = 0x0;
pub const CIM_VF_EXT_MAILBOX_STATUS: c_uint = 0x4;
//
// There isn't a Slice to Module Map Table entry for the Mailbox Data
// registers, but it's convenient to use similar names as above.  There are 8
// little-endian 64-bit Mailbox Data registers.  Note that the "instances"
// value below is in terms of 32-bit words which matches the "word" addressing
// space we use above for the Slice to Module Map Space.
//
pub const NUM_CIM_VF_MAILBOX_DATA_INSTANCES: c_int = 16;
pub const T4VF_MBDATA_FIRST: c_int = 0;

