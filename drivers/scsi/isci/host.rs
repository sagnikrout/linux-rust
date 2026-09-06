//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/host.h
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

//
// struct sci_power_control -
//
// This structure defines the fields for managing power control for direct
// attached disk devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_power_control {
//
// This field is set when the power control timer is running and cleared when
// it is not.
//
    pub timer_started: bool,
//
// Timer to control when the directed attached disks can consume power.
//
    pub timer: sci_timer,
//
// This field is used to keep track of how many phys are put into the
// requesters field.
//
    pub phys_waiting: u8,
//
// This field is used to keep track of how many phys have been granted to consume power
//
    pub phys_granted_power: u8,
//
// This field is an array of phys that we are waiting on. The phys are direct
// mapped into requesters via struct sci_phy.phy_index
//
    pub requesters: [*mut isci_phy; SCI_MAX_PHYS],
}

extern "C" {
    pub fn is_port_config_apc(ihost: *mut isci_host) -> bool;
}
extern "C" {
    pub fn is_controller_start_complete(ihost: *mut isci_host) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_port_configuration_agent {
    pub phy_configured_mask: u16,
    pub phy_ready_mask: u16,
    pub min_index: u8,
    pub max_index: u8,
    pub phy_valid_port_range: [}; SCI_MAX_PHYS],
    pub timer_pending: bool,
    pub link_up_handler: port_config_fn,
    pub link_down_handler: port_config_fn,
    pub timer: sci_timer,
}

//
// isci_host - primary host/controller object
// @timer: timeout start/stop operations
// @device_table: rni (hw remote node index) to remote device lookup table
// @available_remote_nodes: rni allocator
// @power_control: manage device spin up
// @io_request_sequence: generation number for tci's (task contexts)
// @task_context_table: hw task context table
// @remote_node_context_table: hw remote node context table
// @completion_queue: hw-producer driver-consumer communication ring
// @completion_queue_get: tracks the driver 'head' of the ring to notify hw
// @logical_port_entries: min({driver|silicon}-supported-port-count)
// @remote_node_entries: min({driver|silicon}-supported-node-count)
// @task_context_entries: min({driver|silicon}-supported-task-count)
// @phy_timer: phy startup timer
// @invalid_phy_mask: if an invalid_link_up notification is reported a bit for
// the phy index is set so further notifications are not
// made.  Once the phy reports link up and is made part of a
// port then this bit is cleared.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isci_host {
    pub sm: sci_base_state_machine,
// XXX can we time this externally
    pub timer: sci_timer,
// XXX drop reference module params directly
    pub user_parameters: sci_user_parameters,
// XXX no need to be a union
    pub oem_parameters: sci_oem_params,
    pub port_agent: sci_port_configuration_agent,
    pub device_table: [*mut isci_remote_device; SCI_MAX_REMOTE_DEVICES],
    pub available_remote_nodes: sci_remote_node_table,
    pub power_control: sci_power_control,
    pub io_request_sequence: [u8; SCI_MAX_IO_REQUESTS],
    pub task_context_table: *mut scu_task_context,
    pub tc_dma: dma_addr_t,
    pub remote_node_context_table: *mut scu_remote_node_context,
    pub rnc_dma: dma_addr_t,
    pub completion_queue: *mut u32,
    pub cq_dma: dma_addr_t,
    pub completion_queue_get: u32,
    pub logical_port_entries: u32,
    pub remote_node_entries: u32,
    pub task_context_entries: u32,
    pub ufi_buf: *mut c_void,
    pub ufi_dma: dma_addr_t,
    pub uf_control: sci_unsolicited_frame_control,
// phy startup
    pub phy_timer: sci_timer,
// XXX kill
    pub phy_startup_timer_pending: bool,
    pub next_phy_to_start: u32,
// XXX convert to unsigned long and use bitops
    pub invalid_phy_mask: u8,
// TODO attempt dynamic interrupt coalescing scheme
    pub interrupt_coalesce_number: u16,
    pub interrupt_coalesce_timeout: u32,
    pub smu_registers: *mut smu_registers __iomem,
    pub scu_registers: *mut scu_registers __iomem,
    pub tci_head: u16,
    pub tci_tail: u16,
    pub tci_pool: [u16; SCI_MAX_IO_REQUESTS],
    pub /: *mut *mut int id; / unique within a given pci device,
    pub phys: [isci_phy; SCI_MAX_PHYS],
    pub /: *mut *mut isci_port ports[SCI_MAX_PORTS + 1]; / includes dummy port,
    pub sas_ports: [asd_sas_port; SCI_MAX_PORTS],
    pub sas_ha: sas_ha_struct,
    pub pdev: *mut pci_dev,
pub const IHOST_START_PENDING: c_int = 0;
pub const IHOST_STOP_PENDING: c_int = 1;
pub const IHOST_IRQ_ENABLED: c_int = 2;
    pub flags: c_ulong,
    pub eventq: wait_queue_head_t,
    pub completion_tasklet: tasklet_struct,
    pub scic_lock: spinlock_t,
    pub reqs: [*mut isci_request; SCI_MAX_IO_REQUESTS],
    pub devices: [isci_remote_device; SCI_MAX_REMOTE_DEVICES],
}

//
// enum sci_controller_states - This enumeration depicts all the states
// for the common controller state machine.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sci_controller_states {
//
// Simply the initial state for the base controller state machine.
//
    SCIC_INITIAL = 0,

//
// This state indicates that the controller is reset.  The memory for
// the controller is in it's initial state, but the controller requires
// initialization.
// This state is entered from the INITIAL state.
// This state is entered from the RESETTING state.
//
    SCIC_RESET,

//
// This state is typically an action state that indicates the controller
// is in the process of initialization.  In this state no new IO operations
// are permitted.
// This state is entered from the RESET state.
//
    SCIC_INITIALIZING,

//
// This state indicates that the controller has been successfully
// initialized.  In this state no new IO operations are permitted.
// This state is entered from the INITIALIZING state.
//
    SCIC_INITIALIZED,

//
// This state indicates the the controller is in the process of becoming
// ready (i.e. starting).  In this state no new IO operations are permitted.
// This state is entered from the INITIALIZED state.
//
    SCIC_STARTING,

//
// This state indicates the controller is now ready.  Thus, the user
// is able to perform IO operations on the controller.
// This state is entered from the STARTING state.
//
    SCIC_READY,

//
// This state is typically an action state that indicates the controller
// is in the process of resetting.  Thus, the user is unable to perform
// IO operations on the controller.  A reset is considered destructive in
// most cases.
// This state is entered from the READY state.
// This state is entered from the FAILED state.
// This state is entered from the STOPPED state.
//
    SCIC_RESETTING,

//
// This state indicates that the controller is in the process of stopping.
// In this state no new IO operations are permitted, but existing IO
// operations are allowed to complete.
// This state is entered from the READY state.
//
    SCIC_STOPPING,

//
// This state indicates that the controller could not successfully be
// initialized.  In this state no new IO operations are permitted.
// This state is entered from the INITIALIZING state.
// This state is entered from the STARTING state.
// This state is entered from the STOPPING state.
// This state is entered from the RESETTING state.
//
    SCIC_FAILED,
}

//
// struct isci_pci_info - This class represents the pci function containing the
// controllers. Depending on PCI SKU, there could be up to 2 controllers in
// the PCI function.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isci_pci_info {
    pub hosts: [*mut isci_host; SCI_MAX_CONTROLLERS],
    pub orom: *mut isci_orom,
}

extern "C" {
    pub fn pci_get_drvdata(_arg: pdev) -> return;
}

extern "C" {
    pub fn dev_to_ihost(_arg: idev->domain_dev) -> return;
}
// we always use protocol engine group zero
pub const ISCI_PEG: c_int = 0;
// see sci_controller_io_tag_allocate|free for how seq and tci are built

// these are returned by the hardware, so sanitize them

// interrupt coalescing baseline: 9 == 3 to 5us interrupt delay per command
pub const ISCI_COALESCE_BASE: c_int = 9;
// expander attached sata devices require 3 rnc slots
//
// sci_controller_clear_invalid_phy() -
//
// This macro will clear the bit in the invalid phy mask for this controller
// object.  This is used to control messages reported for invalid link up
// notifications.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cable_selections {
    short_cable     = 0,
    long_cable      = 1,
    medium_cable    = 2,
    undefined_cable = 3
}

extern "C" {
    pub fn decode_cable_selection(ihost: *mut isci_host, phy: c_int) -> cable_selections;
}
extern "C" {
    pub fn validate_cable_selections(ihost: *mut isci_host);
}
// set hw control for 'activity', even though active enclosures seem to drive
// the activity led on their own.  Skip setting FSENG control on 'status' due
// to unexpected operation and 'error' due to not being a supported automatic
// FSENG output
//
pub const SGPIO_HW_CONTROL: c_uint = 0x00000443;
extern "C" {
    pub fn ARRAY_SIZE(_arg: ihost->scu_registers->peg0.sgpio.output_data_select) -> return;
}
extern "C" {
    pub fn sci_controller_continue_io(ireq: *mut isci_request) -> sci_status;
}
extern "C" {
    pub fn isci_host_scan_finished(: *mut Scsi_Host, long: unsigned) -> c_int;
}
extern "C" {
    pub fn isci_host_start(: *mut Scsi_Host);
}
extern "C" {
    pub fn isci_alloc_tag(ihost: *mut isci_host) -> u16;
}
extern "C" {
    pub fn isci_free_tag(ihost: *mut isci_host, io_tag: u16) -> sci_status;
}
extern "C" {
    pub fn isci_tci_free(ihost: *mut isci_host, tci: u16);
}
extern "C" {
    pub fn ireq_done(ihost: *mut isci_host, ireq: *mut isci_request, task: *mut sas_task);
}
extern "C" {
    pub fn isci_host_init(: *mut isci_host) -> c_int;
}
extern "C" {
    pub fn isci_host_completion_routine(data: c_ulong);
}
extern "C" {
    pub fn isci_host_deinit(: *mut isci_host);
}
extern "C" {
    pub fn sci_controller_disable_interrupts(ihost: *mut isci_host);
}
extern "C" {
    pub fn sci_controller_has_remote_devices_stopping(ihost: *mut isci_host) -> bool;
}
extern "C" {
    pub fn sci_controller_transition_to_ready(ihost: *mut isci_host, status: sci_status);
}
