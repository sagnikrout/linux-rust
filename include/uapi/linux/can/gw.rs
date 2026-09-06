//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/can/gw.h
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


// SPDX-License-Identifier: ((GPL-2.0-only WITH Linux-syscall-note) OR BSD-3-Clause)
//
// linux/can/gw.h
//
// Definitions for CAN frame Gateway/Router/Bridge
//
// Author: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
// Copyright (c) 2011 Volkswagen Group Electronic Research
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of Volkswagen nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
//
// Alternatively, provided that this notice is retained in full, this
// software may be distributed under the terms of the GNU General
// Public License ("GPL") version 2, in which case the provisions of the
// GPL apply INSTEAD OF those given above.
//
// The provided data structures and external interfaces from this code
// are not restricted to be used by modules with a GPL compatible license.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
// DAMAGE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtcanmsg {
    pub can_family: __u8,
    pub gwtype: __u8,
    pub flags: __u16,
}

// CAN gateway types

// CAN rtnetlink attribute definitions

pub const CGW_FLAGS_CAN_ECHO: c_uint = 0x01;
pub const CGW_FLAGS_CAN_SRC_TSTAMP: c_uint = 0x02;
pub const CGW_FLAGS_CAN_IIF_TX_OK: c_uint = 0x04;
pub const CGW_FLAGS_CAN_FD: c_uint = 0x08;

// CAN frame elements that are affected by curr. 3 CAN frame modifications
pub const CGW_MOD_ID: c_uint = 0x01;
pub const CGW_MOD_DLC: c_uint = 0x02		/* Classical CAN data length code */;

pub const CGW_MOD_DATA: c_uint = 0x04;
pub const CGW_MOD_FLAGS: c_uint = 0x08		/* CAN FD flags */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgw_frame_mod {
    pub cf: can_frame,
    pub modtype: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgw_fdframe_mod {
    pub cf: canfd_frame,
    pub modtype: __u8,
    pub __attribute__((packed)): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgw_csum_xor {
    pub from_idx: __s8,
    pub to_idx: __s8,
    pub result_idx: __s8,
    pub init_xor_val: __u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgw_csum_crc8 {
    pub from_idx: __s8,
    pub to_idx: __s8,
    pub result_idx: __s8,
    pub init_crc_val: __u8,
    pub final_xor_val: __u8,
    pub crctab: [__u8; 256],
    pub profile: __u8,
    pub profile_data: [__u8; 20],
    pub __attribute__((packed)): },
// length of checksum operation parameters. idx = index in CAN frame data[]

// CRC8 profiles (compute CRC for additional data elements - see below)
}

//
// CAN rtnetlink attribute contents in detail
//
// CGW_XXX_IF (length 4 bytes):
// Sets an interface index for source/destination network interfaces.
// For the CAN->CAN gwtype the indices of _two_ CAN interfaces are mandatory.
//
// CGW_FILTER (length 8 bytes):
// Sets a CAN receive filter for the gateway job specified by the
// struct can_filter described in include/linux/can.h
//
// CGW_MOD_(AND|OR|XOR|SET) (length 17 bytes):
// Specifies a modification that's done to a received CAN frame before it is
// send out to the destination interface.
//
// <struct can_frame> data used as operator
// <u8> affected CAN frame elements
//
// CGW_LIM_HOPS (length 1 byte):
// Limit the number of hops of this specific rule. Usually the received CAN
// frame can be processed as much as 'max_hops' times (which is given at module
// load time of the can-gw module). This value is used to reduce the number of
// possible hops for this gateway rule to a value smaller then max_hops.
//
// CGW_MOD_UID (length 4 bytes):
// Optional non-zero user defined routing job identifier to alter existing
// modification settings at runtime.
//
// CGW_CS_XOR (length 4 bytes):
// Set a simple XOR checksum starting with an initial value into
// data[result-idx] using data[start-idx] .. data[end-idx]
//
// The XOR checksum is calculated like this:
//
// xor = init_xor_val
//
// for (i = from_idx .. to_idx)
// xor ^= can_frame.data[i]
//
// can_frame.data[ result_idx ] = xor
//
// CGW_CS_CRC8 (length 282 bytes):
// Set a CRC8 value into data[result-idx] using a given 256 byte CRC8 table,
// a given initial value and a defined input data[start-idx] .. data[end-idx].
// Finally the result value is XOR'ed with the final_xor_val.
//
// The CRC8 checksum is calculated like this:
//
// crc = init_crc_val
//
// for (i = from_idx .. to_idx)
// crc = crctab[ crc ^ can_frame.data[i] ]
//
// can_frame.data[ result_idx ] = crc ^ final_xor_val
//
// The calculated CRC may contain additional source data elements that can be
// defined in the handling of 'checksum profiles' e.g. shown in AUTOSAR specs
// like http://www.autosar.org/download/R4.0/AUTOSAR_SWS_E2ELibrary.pdf
// E.g. the profile_data[] may contain additional u8 values (called DATA_IDs)
// that are used depending on counter values inside the CAN frame data[].
// So far only three profiles have been implemented for illustration.
//
// Remark: In general the attribute data is a linear buffer.
// Beware of sending unpacked or aligned structs!
//
