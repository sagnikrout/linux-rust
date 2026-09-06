//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/unsolicited_frame_control.h
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

pub const SCU_UNSOLICITED_FRAME_HEADER_DATA_DWORDS: c_int = 15;
//
// struct scu_unsolicited_frame_header -
//
// This structure delineates the format of an unsolicited frame header. The
// first DWORD are UF attributes defined by the silicon architecture. The data
// depicts actual header information received on the link.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_unsolicited_frame_header {
//
// This field indicates if there is an Initiator Index Table entry with
// which this header is associated.
//
    pub iit_exists:1: u32,
//
// This field simply indicates the protocol type (i.e. SSP, STP, SMP).
//
    pub protocol_type:3: u32,
//
// This field indicates if the frame is an address frame (IAF or OAF)
// or if it is a information unit frame.
//
    pub is_address_frame:1: u32,
//
// This field simply indicates the connection rate at which the frame
// was received.
//
    pub connection_rate:4: u32,
    pub reserved:23: u32,
//
// This field represents the actual header data received on the link.
//
    pub data: [u32; SCU_UNSOLICITED_FRAME_HEADER_DATA_DWORDS],
}

//
// enum unsolicited_frame_state -
//
// This enumeration represents the current unsolicited frame state.  The
// controller object can not updtate the hardware unsolicited frame put pointer
// unless it has already processed the priror unsolicited frames.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum unsolicited_frame_state {
//
// This state is when the frame is empty and not in use.  It is
// different from the released state in that the hardware could DMA
// data to this frame buffer.
//
    UNSOLICITED_FRAME_EMPTY,

//
// This state is set when the frame buffer is in use by by some
// object in the system.
//
    UNSOLICITED_FRAME_IN_USE,

//
// This state is set when the frame is returned to the free pool
// but one or more frames prior to this one are still in use.
// Once all of the frame before this one are freed it will go to
// the empty state.
//
    UNSOLICITED_FRAME_RELEASED,

    UNSOLICITED_FRAME_MAX_STATES
}

//
// struct sci_unsolicited_frame -
//
// This is the unsolicited frame data structure it acts as the container for
// the current frame state, frame header and frame buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_unsolicited_frame {
//
// This field contains the current frame state
//
    pub state: unsolicited_frame_state,
//
// This field points to the frame header data.
//
    pub header: *mut scu_unsolicited_frame_header,
//
// This field points to the frame buffer data.
//
    pub buffer: *mut c_void,
}

//
// struct sci_uf_header_array -
//
// This structure contains all of the unsolicited frame header information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_uf_header_array {
//
// This field is represents a virtual pointer to the start
// address of the UF address table.  The table contains
// 64-bit pointers as required by the hardware.
//
    pub array: *mut scu_unsolicited_frame_header,
//
// This field specifies the physical address location for the UF
// buffer array.
//
    pub physical_address: dma_addr_t,
}

//
// struct sci_uf_buffer_array -
//
// This structure contains all of the unsolicited frame buffer (actual payload)
// information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_uf_buffer_array {
//
// This field is the unsolicited frame data its used to manage
// the data for the unsolicited frame requests.  It also represents
// the virtual address location that corresponds to the
// physical_address field.
//
    pub array: [sci_unsolicited_frame; SCU_MAX_UNSOLICITED_FRAMES],
//
// This field specifies the physical address location for the UF
// buffer array.
//
    pub physical_address: dma_addr_t,
}

//
// struct sci_uf_address_table_array -
//
// This object maintains all of the unsolicited frame address table specific
// data.  The address table is a collection of 64-bit pointers that point to
// 1KB buffers into which the silicon will DMA unsolicited frames.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_uf_address_table_array {
//
// This field represents a virtual pointer that refers to the
// starting address of the UF address table.
// 64-bit pointers are required by the hardware.
//
    pub array: *mut u64,
//
// This field specifies the physical address location for the UF
// address table.
//
    pub physical_address: dma_addr_t,
}

//
// struct sci_unsolicited_frame_control -
//
// This object contains all of the data necessary to handle unsolicited frames.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_unsolicited_frame_control {
//
// This field is the software copy of the unsolicited frame queue
// get pointer.  The controller object writes this value to the
// hardware to let the hardware put more unsolicited frame entries.
//
    pub get: u32,
//
// This field contains all of the unsolicited frame header
// specific fields.
//
    pub headers: sci_uf_header_array,
//
// This field contains all of the unsolicited frame buffer
// specific fields.
//
    pub buffers: sci_uf_buffer_array,
//
// This field contains all of the unsolicited frame address table
// specific fields.
//
    pub address_table: sci_uf_address_table_array,
}

extern "C" {
    pub fn sci_unsolicited_frame_control_construct(ihost: *mut isci_host);
}
