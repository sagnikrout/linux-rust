//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/request.h
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
// isci_stp_request - extra request infrastructure to handle pio/atapi protocol
// @pio_len - number of bytes requested at PIO setup
// @status - pio setup ending status value to tell us if we need
// to wait for another fis or if the transfer is complete.  Upon
// receipt of a d2h fis this will be the status field of that fis.
// @sgl - track pio transfer progress as we iterate through the sgl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isci_stp_request {
    pub pio_len: u32,
    pub status: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isci_stp_pio_sgl {
    pub index: c_int,
    pub set: u8,
    pub offset: u32,
    pub sgl: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isci_request {
pub const IREQ_COMPLETE_IN_TARGET: c_int = 0;
pub const IREQ_TERMINATED: c_int = 1;
pub const IREQ_TMF: c_int = 2;
pub const IREQ_ACTIVE: c_int = 3;

pub const IREQ_TC_ABORT_POSTED: c_int = 5;
pub const IREQ_ABORT_PATH_ACTIVE: c_int = 6;

    pub flags: c_ulong,
// XXX kill ttype and ttype_ptr, allocate full sas_task
#[repr(C)]
#[derive(Copy, Clone)]
pub union ttype_ptr_union {
    pub /: *mut *mut *mut sas_task io_task_ptr; / When ttype==io_task,
    pub /: *mut *mut *mut isci_tmf tmf_task_ptr; / When ttype==tmf_task,
    pub ttype_ptr: },
    pub isci_host: *mut isci_host,
    pub request_daddr: dma_addr_t,
    pub zero_scatter_daddr: dma_addr_t,
    pub num_sg_entries: c_uint,
// Note: "io_request_completion" is completed in two different ways
// depending on whether this is a TMF or regular request.
// - TMF requests are completed in the thread that started them;
// - regular requests are completed in the request completion callback
// function.
// This difference in operation allows the aborter of a TMF request
// to be sure that once the TMF request completes, the I/O that the
// TMF was aborting is guaranteed to have completed.
//
// XXX kill io_request_completion
//
    pub io_request_completion: *mut completion,
    pub sm: sci_base_state_machine,
    pub owning_controller: *mut isci_host,
    pub target_device: *mut isci_remote_device,
    pub io_tag: u16,
    pub protocol: sas_protocol,
    pub /: *mut *mut u32 scu_status; / hardware result,
    pub /: *mut *mut u32 sci_status; / upper layer disposition,
    pub post_context: u32,
    pub tc: *mut scu_task_context,
// could be larger with sg chaining
// C attribute field omitted
// This field is a pointer to the stored rx frame data.  It is used in
// STP internal requests and SMP response frames.  If this field is
// non-NULL the saved frame must be released on IO request completion.
//
    pub saved_rx_frame_index: u32,
    pub cmd: ssp_cmd_iu,
    pub tmf: ssp_task_iu,
}

//
// enum sci_base_request_states - request state machine states
//
// @SCI_REQ_INIT: Simply the initial state for the base request state machine.
//
// @SCI_REQ_CONSTRUCTED: This state indicates that the request has been
// constructed.  This state is entered from the INITIAL state.
//
// @SCI_REQ_STARTED: This state indicates that the request has been started.
// This state is entered from the CONSTRUCTED state.
//
// @SCI_REQ_STP_UDMA_WAIT_TC_COMP:
// @SCI_REQ_STP_UDMA_WAIT_D2H:
// @SCI_REQ_STP_NON_DATA_WAIT_H2D:
// @SCI_REQ_STP_NON_DATA_WAIT_D2H:
//
// @SCI_REQ_STP_PIO_WAIT_H2D: While in this state the IO request object is
// waiting for the TC completion notification for the H2D Register FIS
//
// @SCI_REQ_STP_PIO_WAIT_FRAME: While in this state the IO request object is
// waiting for either a PIO Setup FIS or a D2H register FIS.  The type of frame
// received is based on the result of the prior frame and line conditions.
//
// @SCI_REQ_STP_PIO_DATA_IN: While in this state the IO request object is
// waiting for a DATA frame from the device.
//
// @SCI_REQ_STP_PIO_DATA_OUT: While in this state the IO request object is
// waiting to transmit the next data frame to the device.
//
// @SCI_REQ_ATAPI_WAIT_H2D: While in this state the IO request object is
// waiting for the TC completion notification for the H2D Register FIS
//
// @SCI_REQ_ATAPI_WAIT_PIO_SETUP: While in this state the IO request object is
// waiting for either a PIO Setup.
//
// @SCI_REQ_ATAPI_WAIT_D2H: The non-data IO transit to this state in this state
// after receiving TC completion. While in this state IO request object is
// waiting for D2H status frame as UF.
//
// @SCI_REQ_ATAPI_WAIT_TC_COMP: When transmitting raw frames hardware reports
// task context completion after every frame submission, so in the
// non-accelerated case we need to expect the completion for the "cdb" frame.
//
// @SCI_REQ_TASK_WAIT_TC_COMP: The AWAIT_TC_COMPLETION sub-state indicates that
// the started raw task management request is waiting for the transmission of
// the initial frame (i.e. command, task, etc.).
//
// @SCI_REQ_TASK_WAIT_TC_RESP: This sub-state indicates that the started task
// management request is waiting for the reception of an unsolicited frame
// (i.e.  response IU).
//
// @SCI_REQ_SMP_WAIT_RESP: This sub-state indicates that the started task
// management request is waiting for the reception of an unsolicited frame
// (i.e.  response IU).
//
// @SCI_REQ_SMP_WAIT_TC_COMP: The AWAIT_TC_COMPLETION sub-state indicates that
// the started SMP request is waiting for the transmission of the initial frame
// (i.e.  command, task, etc.).
//
// @SCI_REQ_COMPLETED: This state indicates that the request has completed.
// This state is entered from the STARTED state. This state is entered from the
// ABORTING state.
//
// @SCI_REQ_ABORTING: This state indicates that the request is in the process
// of being terminated/aborted.  This state is entered from the CONSTRUCTED
// state.  This state is entered from the STARTED state.
//
// @SCI_REQ_FINAL: Simply the final state for the base request state machine.
//

extern "C" {
    pub fn sci_request_start(ireq: *mut isci_request) -> sci_status;
}
extern "C" {
    pub fn sci_io_request_terminate(ireq: *mut isci_request) -> sci_status;
}
// XXX open code in caller

extern "C" {
    pub fn sci_task_request_construct_ssp(ireq: *mut isci_request);
}
extern "C" {
    pub fn sci_smp_request_copy_response(ireq: *mut isci_request);
}
