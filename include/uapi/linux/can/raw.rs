//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/can/raw.h
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
// linux/can/raw.h
//
// Definitions for raw CAN sockets
//
// Authors: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
// Urs Thuermann   <urs.thuermann@volkswagen.de>
// Copyright (c) 2002-2007 Volkswagen Group Electronic Research
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

// for socket options affecting the socket (not the global system)
// configuration for CAN XL virtual CAN identifier (VCID) handling
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_raw_vcid_options {
    pub /: *mut *mut __u8 flags; / flags for vcid (filter) behaviour,
    pub /: *mut *mut __u8 tx_vcid; / VCID value set into canxl_frame.prio,
    pub /: *mut *mut __u8 rx_vcid; / VCID value for VCID filter,
    pub /: *mut *mut __u8 rx_vcid_mask; / VCID mask for VCID filter,
}

// can_raw_vcid_options.flags for CAN XL virtual CAN identifier handling
pub const CAN_RAW_XL_VCID_TX_SET: c_uint = 0x01;
pub const CAN_RAW_XL_VCID_TX_PASS: c_uint = 0x02;
pub const CAN_RAW_XL_VCID_RX_FILTER: c_uint = 0x04;
