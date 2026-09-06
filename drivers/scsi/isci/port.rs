//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/port.h
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

pub const SCIC_SDS_DUMMY_PORT: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isci_status {
    isci_freed        = 0x00,
    isci_starting     = 0x01,
    isci_ready        = 0x02,
    isci_ready_for_io = 0x03,
    isci_stopping     = 0x04,
    isci_stopped      = 0x05,
}

//
// struct isci_port - isci direct attached sas port object
// @ready_exit: several states constitute 'ready'. When exiting ready we
// need to take extra port-teardown actions that are
// skipped when exiting to another 'ready' state.
// @logical_port_index: software port index
// @physical_port_index: hardware port index
// @active_phy_mask: identifies phy members
// @enabled_phy_mask: phy mask for the port
// that are already part of the port
// @reserved_tag:
// @reserved_rni: reserver for port task scheduler workaround
// @started_request_count: reference count for outstanding commands
// @not_ready_reason: set during state transitions and notified
// @timer: timeout start/stop operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isci_port {
    pub isci_host: *mut isci_host,
    pub remote_dev_list: list_head,
pub const IPORT_RESET_PENDING: c_int = 0;
    pub state: c_ulong,
    pub hard_reset_status: sci_status,
    pub sm: sci_base_state_machine,
    pub ready_exit: bool,
    pub logical_port_index: u8,
    pub physical_port_index: u8,
    pub active_phy_mask: u8,
    pub enabled_phy_mask: u8,
    pub last_active_phy: u8,
    pub reserved_rni: u16,
    pub reserved_tag: u16,
    pub started_request_count: u32,
    pub assigned_device_count: u32,
    pub hang_detect_users: u32,
    pub not_ready_reason: u32,
    pub phy_table: [*mut isci_phy; SCI_MAX_PHYS],
    pub owning_controller: *mut isci_host,
    pub timer: sci_timer,
    pub port_task_scheduler_registers: *mut scu_port_task_scheduler_registers __iomem,
// XXX rework: only one register, no need to replicate per-port
    pub port_pe_configuration_register: *mut u32 __iomem,
    pub viit_registers: *mut scu_viit_entry __iomem,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sci_port_not_ready_reason_code {
    SCIC_PORT_NOT_READY_NO_ACTIVE_PHYS,
    SCIC_PORT_NOT_READY_HARD_RESET_REQUESTED,
    SCIC_PORT_NOT_READY_INVALID_PORT_CONFIGURATION,
    SCIC_PORT_NOT_READY_RECONFIGURING,

    SCIC_PORT_NOT_READY_REASON_CODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_port_end_point_properties {
    pub sas_address: sci_sas_address,
    pub protocols: sci_phy_proto,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_port_properties {
    pub index: u32,
    pub local: sci_port_end_point_properties,
    pub remote: sci_port_end_point_properties,
    pub phy_mask: u32,
}

//
// enum sci_port_states - port state machine states
// @SCI_PORT_STOPPED: port has successfully been stopped.  In this state
// no new IO operations are permitted.  This state is
// entered from the STOPPING state.
// @SCI_PORT_STOPPING: port is in the process of stopping.  In this
// state no new IO operations are permitted, but
// existing IO operations are allowed to complete.
// This state is entered from the READY state.
// @SCI_PORT_READY: port is now ready.  Thus, the user is able to
// perform IO operations on this port. This state is
// entered from the STARTING state.
// @SCI_PORT_SUB_WAITING: port is started and ready but has no active
// phys.
// @SCI_PORT_SUB_OPERATIONAL: port is started and ready and there is at
// least one phy operational.
// @SCI_PORT_SUB_CONFIGURING: port is started and there was an
// add/remove phy event.  This state is only
// used in Automatic Port Configuration Mode
// (APC)
// @SCI_PORT_RESETTING: port is in the process of performing a hard
// reset.  Thus, the user is unable to perform IO
// operations on this port.  This state is entered
// from the READY state.
// @SCI_PORT_FAILED: port has failed a reset request.  This state is
// entered when a port reset request times out. This
// state is entered from the RESETTING state.
//

// pass */;

extern "C" {
    pub fn sci_port_start(iport: *mut isci_port) -> sci_status;
}
extern "C" {
    pub fn sci_port_stop(iport: *mut isci_port) -> sci_status;
}
extern "C" {
    pub fn isci_port_bcn_enable(: *mut isci_host, : *mut isci_port);
}
extern "C" {
    pub fn isci_port_formed(: *mut asd_sas_phy);
}
extern "C" {
    pub fn isci_port_deformed(: *mut asd_sas_phy);
}
extern "C" {
    pub fn isci_ata_check_ready(dev: *mut domain_device) -> c_int;
}
