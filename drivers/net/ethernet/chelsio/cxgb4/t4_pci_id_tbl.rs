//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/t4_pci_id_tbl.h
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
// This file is part of the Chelsio T4/T5 Ethernet driver for Linux.
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
// The code can defined cpp macros for creating a PCI Device ID Table. This is
// useful because it allows the PCI ID Table to be maintained in a single place.
//
// The macros are:
//
// CH_PCI_DEVICE_ID_TABLE_DEFINE_BEGIN
// -- Used to start the definition of the PCI ID Table.
//
// CH_PCI_DEVICE_ID_FUNCTION
// -- The PCI Function Number to use in the PCI Device ID Table.  "0"
// -- for drivers attaching to PF0-3, "4" for drivers attaching to PF4,
// -- "8" for drivers attaching to SR-IOV Virtual Functions, etc.
//
// CH_PCI_DEVICE_ID_FUNCTION2 [optional]
// -- If defined, create a PCI Device ID Table with both
// -- CH_PCI_DEVICE_ID_FUNCTION and CH_PCI_DEVICE_ID_FUNCTION2 populated.
//
// CH_PCI_ID_TABLE_ENTRY(DeviceID)
// -- Used for the individual PCI Device ID entries.  Note that we will
// -- be adding a trailing comma (",") after all of the entries (and
// -- between the pairs of entries if CH_PCI_DEVICE_ID_FUNCTION2 is defined).
//
// CH_PCI_DEVICE_ID_TABLE_DEFINE_END
// -- Used to finish the definition of the PCI ID Table.  Note that we
// -- will be adding a trailing semi-colon (";") here.
//

// T4 and later ASICs use a PCI Device ID scheme of 0xVFPP where:
//
// V  = "4" for T4; "5" for T5, etc.
// F  = "0" for PF 0..3; "4".."7" for PF4..7; and "8" for VFs
// PP = adapter product designation
//
// We use this consistency in order to create the proper PCI Device IDs
// for the specified CH_PCI_DEVICE_ID_FUNCTION.
//

// T4 adapters:
//
// T5 adapters:
//
// T6 adapters:
//
