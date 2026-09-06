//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/aiclib.h
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
// Largely written by Julian Elischer (julian@tfs.com)
// for TRW Financial Systems.
//
// TRW Financial Systems, in accordance with their agreement with Carnegie
// Mellon University, makes this software available to CMU to distribute
// or use in any manner that they see fit as long as this message is kept with
// the software. For this reason TFS also grants any other persons or
// organisations permission to use or modify this software.
//
// TFS supplies this software to be publicly redistributed
// on the understanding that TFS is not responsible for the correct
// functioning of this software in any circumstances.
//
// Ported to run under 386BSD by Julian Elischer (julian@tfs.com) Sept 1992
//
// $FreeBSD: src/sys/cam/scsi/scsi_all.h,v 1.21 2002/10/08 17:12:44 ken Exp $
//
// Copyright (c) 2003 Adaptec Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// substantially similar to the "NO WARRANTY" disclaimer below
// ("Disclaimer") and any redistribution must be conditioned upon
// including a substantially similar Disclaimer requirement for further
// binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGES.
//
// $Id$
//
pub const SCSI_REV_0: c_int = 0;
pub const SCSI_REV_CCS: c_int = 1;
pub const SCSI_REV_2: c_int = 2;
pub const SCSI_REV_SPC: c_int = 3;
pub const SCSI_REV_SPC2: c_int = 4;
pub const SSD_ERRCODE: c_uint = 0x7F;
pub const SSD_CURRENT_ERROR: c_uint = 0x70;
pub const SSD_DEFERRED_ERROR: c_uint = 0x71;
pub const SSD_ERRCODE_VALID: c_uint = 0x80;
pub const SSD_KEY: c_uint = 0x0F;
pub const SSD_KEY_NO_SENSE: c_uint = 0x00;
pub const SSD_KEY_RECOVERED_ERROR: c_uint = 0x01;
pub const SSD_KEY_NOT_READY: c_uint = 0x02;
pub const SSD_KEY_MEDIUM_ERROR: c_uint = 0x03;
pub const SSD_KEY_HARDWARE_ERROR: c_uint = 0x04;
pub const SSD_KEY_ILLEGAL_REQUEST: c_uint = 0x05;
pub const SSD_KEY_UNIT_ATTENTION: c_uint = 0x06;
pub const SSD_KEY_DATA_PROTECT: c_uint = 0x07;
pub const SSD_KEY_BLANK_CHECK: c_uint = 0x08;
pub const SSD_KEY_Vendor_Specific: c_uint = 0x09;
pub const SSD_KEY_COPY_ABORTED: c_uint = 0x0a;
pub const SSD_KEY_ABORTED_COMMAND: c_uint = 0x0b;
pub const SSD_KEY_EQUAL: c_uint = 0x0c;
pub const SSD_KEY_VOLUME_OVERFLOW: c_uint = 0x0d;
pub const SSD_KEY_MISCOMPARE: c_uint = 0x0e;
pub const SSD_KEY_RESERVED: c_uint = 0x0f;
pub const SSD_ILI: c_uint = 0x20;
pub const SSD_EOM: c_uint = 0x40;
pub const SSD_FILEMARK: c_uint = 0x80;
pub const SSD_SCS_VALID: c_uint = 0x80;
pub const SSD_FIELDPTR_CMD: c_uint = 0x40;
pub const SSD_BITPTR_VALID: c_uint = 0x08;
pub const SSD_BITPTR_VALUE: c_uint = 0x07;
pub const SSD_MIN_SIZE: c_int = 18;

// Large Disk Handling
// ugly, ugly sector_div calling convention..
// Macros for generating the elements of the PCI ID tables.

// Generate IDs for all 16 possibilites.
// The argument has already masked out
// the 4 least significant bits of the device id.
// (e.g., mask: ID_9005_GENERIC_MASK).
//

