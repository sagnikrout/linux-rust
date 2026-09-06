//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/epapr_hcalls.h
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


// SPDX-License-Identifier: ((GPL-2.0+ WITH Linux-syscall-note) OR BSD-3-Clause)
//
// ePAPR hcall interface
//
// Copyright 2008-2011 Freescale Semiconductor, Inc.
//
// Author: Timur Tabi <timur@freescale.com>
//
// This file is provided under a dual BSD/GPL license.  When using or
// redistributing this file, you may do so under either license.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
pub const EV_BYTE_CHANNEL_SEND: c_int = 1;
pub const EV_BYTE_CHANNEL_RECEIVE: c_int = 2;
pub const EV_BYTE_CHANNEL_POLL: c_int = 3;
pub const EV_INT_SET_CONFIG: c_int = 4;
pub const EV_INT_GET_CONFIG: c_int = 5;
pub const EV_INT_SET_MASK: c_int = 6;
pub const EV_INT_GET_MASK: c_int = 7;
pub const EV_INT_IACK: c_int = 9;
pub const EV_INT_EOI: c_int = 10;
pub const EV_INT_SEND_IPI: c_int = 11;
pub const EV_INT_SET_TASK_PRIORITY: c_int = 12;
pub const EV_INT_GET_TASK_PRIORITY: c_int = 13;
pub const EV_DOORBELL_SEND: c_int = 14;
pub const EV_MSGSND: c_int = 15;
pub const EV_IDLE: c_int = 16;
// vendor ID: epapr

pub const EV_EPAPR_VENDOR_ID: c_int = 1;

// The max number of bytes that a byte channel can send or receive per call
pub const EV_BYTE_CHANNEL_MAX_BYTES: c_int = 16;

// epapr return codes
pub const EV_SUCCESS: c_int = 0;

// resources to complete and should be
// retried
//

// complete the operation

