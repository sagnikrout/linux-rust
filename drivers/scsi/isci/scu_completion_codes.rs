//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/scu_completion_codes.h
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
// This file contains the constants and macros for the SCU hardware completion
// codes.
//
pub const SCU_COMPLETION_TYPE_SHIFT: c_int = 28;
pub const SCU_COMPLETION_TYPE_MASK: c_uint = 0x70000000;
//
// SCU_COMPLETION_TYPE() -
//
// This macro constructs an SCU completion type
//

//
// SCU_COMPLETION_TYPE() -
//
// These macros contain the SCU completion types SCU_COMPLETION_TYPE
//

//
// These constants provide the shift and mask values for the various parts of
// an SCU completion code.
//
pub const SCU_COMPLETION_STATUS_MASK: c_uint = 0x0FFC0000;
pub const SCU_COMPLETION_TL_STATUS_MASK: c_uint = 0x0FC00000;
pub const SCU_COMPLETION_TL_STATUS_SHIFT: c_int = 22;
pub const SCU_COMPLETION_SDMA_STATUS_MASK: c_uint = 0x003C0000;
pub const SCU_COMPLETION_PEG_MASK: c_uint = 0x00010000;
pub const SCU_COMPLETION_PORT_MASK: c_uint = 0x00007000;

pub const SCU_COMPLETION_PE_SHIFT: c_int = 12;
pub const SCU_COMPLETION_INDEX_MASK: c_uint = 0x00000FFF;
//
// SCU_GET_COMPLETION_TYPE() -
//
// This macro returns the SCU completion type.
//

//
// SCU_GET_COMPLETION_STATUS() -
//
// This macro returns the SCU completion status.
//

//
// SCU_GET_COMPLETION_TL_STATUS() -
//
// This macro returns the transport layer completion status.
//

//
// SCU_MAKE_COMPLETION_STATUS() -
//
// This macro takes a completion code and performs the shift and mask
// operations to turn it into a completion code that can be compared to a
// SCU_GET_COMPLETION_TL_STATUS.
//

//
// SCU_NORMALIZE_COMPLETION_STATUS() -
//
// This macro takes a SCU_GET_COMPLETION_TL_STATUS and normalizes it for a
// return code.
//

//
// SCU_GET_COMPLETION_SDMA_STATUS() -
//
// This macro returns the SDMA completion status.
//

//
// SCU_GET_COMPLETION_PEG() -
//
// This macro returns the Protocol Engine Group from the completion code.
//

//
// SCU_GET_COMPLETION_PORT() -
//
// This macro reuturns the logical port index from the completion code.
//

//
// SCU_GET_PROTOCOL_ENGINE_INDEX() -
//
// This macro returns the PE index from the completion code.
//

//
// SCU_GET_COMPLETION_INDEX() -
//
// This macro returns the index of the completion which is either a TCi or an
// RNi depending on the completion type.
//

pub const SCU_UNSOLICITED_FRAME_MASK: c_uint = 0x0FFF0000;
pub const SCU_UNSOLICITED_FRAME_SHIFT: c_int = 16;
//
// SCU_GET_FRAME_INDEX() -
//
// This macro returns a normalized frame index from an unsolicited frame
// completion.
//

pub const SCU_UNSOLICITED_FRAME_ERROR_MASK: c_uint = 0x00008000;
//
// SCU_GET_FRAME_ERROR() -
//
// This macro returns a zero (0) value if there is no frame error otherwise it
// returns non-zero (!0).
//

//
// These constants represent normalized completion codes which must be shifted
// 18 bits to match it with the hardware completion code. In a 16-bit compiler,
// immediate constants are 16-bit values (the size of an int). If we shift
// those by 18 bits, we completely lose the value. To ensure the value is a
// 32-bit value like we want, each immediate value must be cast to a u32.
//

