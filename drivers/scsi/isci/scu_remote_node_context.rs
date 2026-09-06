//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/scu_remote_node_context.h
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
// This file contains the structures and constatns used by the SCU hardware to
// describe a remote node context.
//
// struct ssp_remote_node_context - This structure contains the SCU hardware
// definition for an SSP remote node.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_remote_node_context {
// WORD 0
//
// This field is the remote node index assigned for this remote node. All
// remote nodes must have a unique remote node index. The value of the remote
// node index can not exceed the maximum number of remote nodes reported in
// the SCU device context capacity register.
//
    pub remote_node_index:12: u32,
    pub reserved0_1:4: u32,
//
// This field tells the SCU hardware how many simultaneous connections that
// this remote node will support.
//
    pub remote_node_port_width:4: u32,
//
// This field tells the SCU hardware which logical port to associate with this
// remote node.
//
    pub logical_port_index:3: u32,
    pub reserved0_2:5: u32,
//
// This field will enable the I_T nexus loss timer for this remote node.
//
    pub nexus_loss_timer_enable:1: u32,
//
// This field is the for driver debug only and is not used.
//
    pub check_bit:1: u32,
//
// This field must be set to true when the hardware DMAs the remote node
// context to the hardware SRAM.  When the remote node is being invalidated
// this field must be set to false.
//
    pub is_valid:1: u32,
//
// This field must be set to true.
//
    pub is_remote_node_context:1: u32,
// WORD 1 - 2
//
// This is the low word of the remote device SAS Address
//
    pub remote_sas_address_lo: u32,
//
// This field is the high word of the remote device SAS Address
//
    pub remote_sas_address_hi: u32,
// WORD 3
//
// This field reprensets the function number assigned to this remote device.
// This value must match the virtual function number that is being used to
// communicate to the device.
//
    pub function_number:8: u32,
    pub reserved3_1:8: u32,
//
// This field provides the driver a way to cheat on the arbitration wait time
// for this remote node.
//
    pub arbitration_wait_time:16: u32,
// WORD 4
//
// This field tells the SCU hardware how long this device may occupy the
// connection before it must be closed.
//
    pub connection_occupancy_timeout:16: u32,
//
// This field tells the SCU hardware how long to maintain a connection when
// there are no frames being transmitted on the link.
//
    pub connection_inactivity_timeout:16: u32,
// WORD  5
//
// This field allows the driver to cheat on the arbitration wait time for this
// remote node.
//
    pub initial_arbitration_wait_time:16: u32,
//
// This field is tells the hardware what to program for the connection rate in
// the open address frame.  See the SAS spec for valid values.
//
    pub oaf_connection_rate:4: u32,
//
// This field tells the SCU hardware what to program for the features in the
// open address frame.  See the SAS spec for valid values.
//
    pub oaf_features:4: u32,
//
// This field tells the SCU hardware what to use for the source zone group in
// the open address frame.  See the SAS spec for more details on zoning.
//
    pub oaf_source_zone_group:8: u32,
// WORD 6
//
// This field tells the SCU hardware what to use as the more capibilities in
// the open address frame. See the SAS Spec for details.
//
    pub oaf_more_compatibility_features: u32,
// WORD 7
    pub reserved7: u32,
}

//
// struct stp_remote_node_context - This structure contains the SCU hardware
// definition for a STP remote node.
//
// STP Targets are not yet supported so this definition is a placeholder until
// we do support them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_remote_node_context {
//
// Placeholder data for the STP remote node.
//
    pub data: [u32; 8],
}

//
// This union combines the SAS and SATA remote node definitions.
//
// union scu_remote_node_context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union scu_remote_node_context {
//
// SSP Remote Node
//
    pub ssp: ssp_remote_node_context,
//
// STP Remote Node
//
    pub stp: stp_remote_node_context,
}
