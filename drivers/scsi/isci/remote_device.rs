//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/remote_device.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sci_remote_device_not_ready_reason_code {
    SCIC_REMOTE_DEVICE_NOT_READY_START_REQUESTED,
    SCIC_REMOTE_DEVICE_NOT_READY_STOP_REQUESTED,
    SCIC_REMOTE_DEVICE_NOT_READY_SATA_REQUEST_STARTED,
    SCIC_REMOTE_DEVICE_NOT_READY_SATA_SDB_ERROR_FIS_RECEIVED,
    SCIC_REMOTE_DEVICE_NOT_READY_SMP_REQUEST_STARTED,
    SCIC_REMOTE_DEVICE_NOT_READY_REASON_CODE_MAX
}

//
// isci_remote_device - isci representation of a sas expander / end point
// @device_port_width: hw setting for number of simultaneous connections
// @connection_rate: per-taskcontext connection rate for this device
// @working_request: SATA requests have no tag we for unaccelerated
// protocols we need a method to associate unsolicited
// frames with a pending request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isci_remote_device {
pub const IDEV_START_PENDING: c_int = 0;
pub const IDEV_STOP_PENDING: c_int = 1;
pub const IDEV_ALLOCATED: c_int = 2;
pub const IDEV_GONE: c_int = 3;
pub const IDEV_IO_READY: c_int = 4;
pub const IDEV_IO_NCQERROR: c_int = 5;
pub const IDEV_RNC_LLHANG_ENABLED: c_int = 6;
pub const IDEV_ABORT_PATH_ACTIVE: c_int = 7;
pub const IDEV_ABORT_PATH_RESUME_PENDING: c_int = 8;
    pub flags: c_ulong,
    pub kref: kref,
    pub isci_port: *mut isci_port,
    pub domain_dev: *mut domain_device,
    pub node: list_head,
    pub sm: sci_base_state_machine,
    pub device_port_width: u32,
    pub connection_rate: sas_linkrate,
    pub owning_port: *mut isci_port,
    pub rnc: sci_remote_node_context,
// XXX unify with device reference counting and delete
    pub started_request_count: u32,
    pub working_request: *mut isci_request,
    pub not_ready_reason: u32,
    pub abort_resume_cb: scics_sds_remote_node_context_callback,
    pub abort_resume_cbparam: *mut c_void,
}

pub const ISCI_REMOTE_DEVICE_START_TIMEOUT: c_int = 5000;
// device reference routines must be called under sci_lock
extern "C" {
    pub fn isci_remote_device_release(kref: *mut kref);
}
extern "C" {
    pub fn isci_remote_device_gone(domain_dev: *mut domain_device);
}
extern "C" {
    pub fn isci_remote_device_found(domain_dev: *mut domain_device) -> c_int;
}
//
// sci_remote_device_stop() - This method will stop both transmission and
// reception of link activity for the supplied remote device.  This method
// disables normal IO requests from flowing through to the remote device.
// @remote_device: This parameter specifies the device to be stopped.
// @timeout: This parameter specifies the number of milliseconds in which the
// stop operation should complete.
//
// An indication of whether the device was successfully stopped. SCI_SUCCESS
// This value is returned if the transmission and reception for the device was
// successfully stopped.
//
// enum sci_remote_device_states - This enumeration depicts all the states
// for the common remote device state machine.
// @SCI_DEV_INITIAL: Simply the initial state for the base remote device
// state machine.
//
// @SCI_DEV_STOPPED: This state indicates that the remote device has
// successfully been stopped.  In this state no new IO operations are
// permitted.  This state is entered from the INITIAL state.  This state
// is entered from the STOPPING state.
//
// @SCI_DEV_STARTING: This state indicates the the remote device is in
// the process of becoming ready (i.e. starting).  In this state no new
// IO operations are permitted.  This state is entered from the STOPPED
// state.
//
// @SCI_DEV_READY: This state indicates the remote device is now ready.
// Thus, the user is able to perform IO operations on the remote device.
// This state is entered from the STARTING state.
//
// @SCI_STP_DEV_IDLE: This is the idle substate for the stp remote
// device.  When there are no active IO for the device it is is in this
// state.
//
// @SCI_STP_DEV_CMD: This is the command state for the STP remote
// device.  This state is entered when the device is processing a
// non-NCQ command.  The device object will fail any new start IO
// requests until this command is complete.
//
// @SCI_STP_DEV_NCQ: This is the NCQ state for the STP remote device.
// This state is entered when the device is processing an NCQ reuqest.
// It will remain in this state so long as there is one or more NCQ
// requests being processed.
//
// @SCI_STP_DEV_NCQ_ERROR: This is the NCQ error state for the STP
// remote device.  This state is entered when an SDB error FIS is
// received by the device object while in the NCQ state.  The device
// object will only accept a READ LOG command while in this state.
//
// @SCI_STP_DEV_ATAPI_ERROR: This is the ATAPI error state for the STP
// ATAPI remote device.  This state is entered when ATAPI device sends
// error status FIS without data while the device object is in CMD
// state.  A suspension event is expected in this state.  The device
// object will resume right away.
//
// @SCI_STP_DEV_AWAIT_RESET: This is the READY substate indicates the
// device is waiting for the RESET task coming to be recovered from
// certain hardware specific error.
//
// @SCI_SMP_DEV_IDLE: This is the ready operational substate for the
// remote device.  This is the normal operational state for a remote
// device.
//
// @SCI_SMP_DEV_CMD: This is the suspended state for the remote device.
// This is the state that the device is placed in when a RNC suspend is
// received by the SCU hardware.
//
// @SCI_DEV_STOPPING: This state indicates that the remote device is in
// the process of stopping.  In this state no new IO operations are
// permitted, but existing IO operations are allowed to complete.  This
// state is entered from the READY state.  This state is entered from
// the FAILED state.
//
// @SCI_DEV_FAILED: This state indicates that the remote device has
// failed.  In this state no new IO operations are permitted.  This
// state is entered from the INITIALIZING state.  This state is entered
// from the READY state.
//
// @SCI_DEV_RESETTING: This state indicates the device is being reset.
// In this state no new IO operations are permitted.  This state is
// entered from the READY state.
//
// @SCI_DEV_FINAL: Simply the final state for the base remote device
// state machine.
//

// XXX delete this voodoo when converting to the top-level device
// reference count
//
// pass */;
extern "C" {
    pub fn isci_dev_set_hang_detection_timeout(idev: *mut isci_remote_device, timeout: u32);
}
