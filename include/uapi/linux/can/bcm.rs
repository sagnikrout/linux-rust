//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/can/bcm.h
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
// linux/can/bcm.h
//
// Definitions for CAN Broadcast Manager (BCM)
//
// Author: Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

//
// struct bcm_msg_head - head of messages to/from the broadcast manager
// @opcode:    opcode, see enum below.
// @flags:     special flags, see below.
// @count:     number of frames to send before changing interval.
// @ival1:     interval for the first @count frames.
// @ival2:     interval for the following frames.
// @can_id:    CAN ID of frames to be sent or received.
// @nframes:   number of frames appended to the message head.
// @frames:    array of CAN frames.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_msg_head {
    pub opcode: __u32,
    pub flags: __u32,
    pub count: __u32,
    pub ival2: bcm_timeval ival1,,
    pub can_id: canid_t,
    pub nframes: __u32,
    pub frames: [can_frame; ],
}

pub const SETTIMER: c_uint = 0x0001;
pub const STARTTIMER: c_uint = 0x0002;
pub const TX_COUNTEVT: c_uint = 0x0004;
pub const TX_ANNOUNCE: c_uint = 0x0008;
pub const TX_CP_CAN_ID: c_uint = 0x0010;
pub const RX_FILTER_ID: c_uint = 0x0020;
pub const RX_CHECK_DLC: c_uint = 0x0040;
pub const RX_NO_AUTOTIMER: c_uint = 0x0080;
pub const RX_ANNOUNCE_RESUME: c_uint = 0x0100;
pub const TX_RESET_MULTI_IDX: c_uint = 0x0200;
pub const RX_RTR_FRAME: c_uint = 0x0400;
pub const CAN_FD_FRAME: c_uint = 0x0800;
