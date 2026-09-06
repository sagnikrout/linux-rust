//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/remote_node_context.h
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
// This file contains the structures, constants, and prototypes associated with
// the remote node context in the silicon.  It exists to model and manage
// the remote node context in the silicon.
//

//
// This constant represents an invalid remote device id, it is used to program
// the STPDARNI register so the driver knows when it has received a SIGNATURE
// FIS from the SCU.
//
pub const SCIC_SDS_REMOTE_NODE_CONTEXT_INVALID_INDEX: c_uint = 0x0FFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sci_remote_node_suspension_reasons {
    SCI_HW_SUSPEND,
    SCI_SW_SUSPEND_NORMAL,
    SCI_SW_SUSPEND_LINKHANG_DETECT
}

extern "C" {
    pub fn void(: *mut *mut scics_sds_remote_node_context_callback)(void) -> typedef;
}
//
// enum sci_remote_node_context_states
// @SCI_RNC_INITIAL initial state for a remote node context.  On a resume
// request the remote node context will transition to the posting state.
//
// @SCI_RNC_POSTING: transition state that posts the RNi to the hardware. Once
// the RNC is posted the remote node context will be made ready.
//
// @SCI_RNC_INVALIDATING: transition state that will post an RNC invalidate to
// the hardware.  Once the invalidate is complete the remote node context will
// transition to the posting state.
//
// @SCI_RNC_RESUMING: transition state that will post an RNC resume to the
// hardare.  Once the event notification of resume complete is received the
// remote node context will transition to the ready state.
//
// @SCI_RNC_READY: state that the remote node context must be in to accept io
// request operations.
//
// @SCI_RNC_TX_SUSPENDED: state that the remote node context transitions to when
// it gets a TX suspend notification from the hardware.
//
// @SCI_RNC_TX_RX_SUSPENDED: state that the remote node context transitions to
// when it gets a TX RX suspend notification from the hardware.
//
// @SCI_RNC_AWAIT_SUSPENSION: wait state for the remote node context that waits
// for a suspend notification from the hardware.  This state is entered when
// either there is a request to supend the remote node context or when there is
// a TC completion where the remote node will be suspended by the hardware.
//

//
// This enumeration is used to define the end destination state for the remote
// node context.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sci_remote_node_context_destination_state {
    RNC_DEST_UNSPECIFIED,
    RNC_DEST_READY,
    RNC_DEST_FINAL,
    RNC_DEST_SUSPENDED,       /* Set when suspend during post/invalidate */
    RNC_DEST_SUSPENDED_RESUME /* Set when a resume was done during posting
// or invalidating and already suspending.
//
}

//
// struct sci_remote_node_context - This structure contains the data
// associated with the remote node context object.  The remote node context
// (RNC) object models the the remote device information necessary to manage
// the silicon RNC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_remote_node_context {
//
// This field indicates the remote node index (RNI) associated with
// this RNC.
//
    pub remote_node_index: u16,
//
// This field is the recored suspension type of the remote node
// context suspension.
//
    pub suspend_type: u32,
    pub suspend_reason: sci_remote_node_suspension_reasons,
    pub suspend_count: u32,
//
// This field is true if the remote node context is resuming from its current
// state.  This can cause an automatic resume on receiving a suspension
// notification.
//
    pub destination_state: sci_remote_node_context_destination_state,
//
// This field contains the callback function that the user requested to be
// called when the requested state transition is complete.
//
    pub user_callback: scics_sds_remote_node_context_callback,
//
// This field contains the parameter that is called when the user requested
// state transition is completed.
//
    pub user_cookie: *mut c_void,
//
// This field contains the data for the object's state machine.
//
    pub sm: sci_base_state_machine,
}

extern "C" {
    pub fn sci_remote_node_context_is_suspended(sci_rnc: *mut sci_remote_node_context) -> bool;
}
