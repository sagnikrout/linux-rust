//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/scu_event_codes.h
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
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// GPL LICENSE SUMMARY
//
// Copyright(c) 2008 - 2011 Intel Corporation. All rights reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of version 2 of the GNU General Public License as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St - Fifth Floor, Boston, MA 02110-1301 USA.
// The full GNU General Public License is included in this distribution
// in the file called LICENSE.GPL.
//
// BSD LICENSE
//
// Copyright(c) 2008 - 2011 Intel Corporation. All rights reserved.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
// * Neither the name of Intel Corporation nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
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
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// This file contains the constants and macros for the SCU event codes.
//
pub const SCU_EVENT_TYPE_CODE_SHIFT: c_int = 24;
pub const SCU_EVENT_TYPE_CODE_MASK: c_uint = 0x0F000000;
pub const SCU_EVENT_SPECIFIC_CODE_SHIFT: c_int = 18;
pub const SCU_EVENT_SPECIFIC_CODE_MASK: c_uint = 0x00FC0000;

//
// SCU_EVENT_TYPE() -
//
// This macro constructs an SCU event type from the type value.
//

//
// SCU_EVENT_SPECIFIC() -
//
// This macro constructs an SCU event specifier from the code value.
//

//
// SCU_EVENT_MESSAGE() -
//
// This macro constructs a combines an SCU event type and SCU event specifier
// from the type and code values.
//

//
// SCU_EVENT_TYPE() -
//
// SCU_EVENT_TYPES
//

//
// SCU_EVENT_SPECIFIERS
//
pub const SCU_EVENT_SPECIFIER_DRIVER_SUSPEND: c_uint = 0x20;
pub const SCU_EVENT_SPECIFIER_RNC_RELEASE: c_uint = 0x00;
//
// SMU_COMMAND_EVENTS
//

//
// SMU_PCQ_EVENTS
//

//
// SMU_EVENTS
//

//
// TRANSPORT_LEVEL_ERRORS
//

//
// BROADCAST_CHANGE_EVENTS
//

//
// OSSP_EVENTS
//

//
// FATAL_INTERNAL_MEMORY_ERROR_EVENTS
//

//
// REMOTE_NODE_SUSPEND_EVENTS
//

//
// REMOTE_NODE_MISC_EVENTS
//

//
// ERROR_COUNT_EVENT
//

//
// scu_get_event_type() -
//
// This macro returns the SCU event type from the event code.
//

//
// scu_get_event_specifier() -
//
// This macro returns the SCU event specifier from the event code.
//

//
// scu_get_event_code() -
//
// This macro returns the combined SCU event type and SCU event specifier from
// the event code.
//

//
// PTS_SCHEDULE_EVENT
//

