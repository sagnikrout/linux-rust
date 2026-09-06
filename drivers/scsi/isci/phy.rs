//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/phy.h
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

// This is the timeout value for the SATA phy to wait for a SIGNATURE FIS
// before restarting the starting state machine.  Technically, the old parallel
// ATA specification required up to 30 seconds for a device to issue its
// signature FIS as a result of a soft reset.  Now we see that devices respond
// generally within 15 seconds, but we'll use 25 for now.
//
pub const SCIC_SDS_SIGNATURE_FIS_TIMEOUT: c_int = 25000;
// This is the timeout for the SATA OOB/SN because the hardware does not
// recognize a hot plug after OOB signal but before the SN signals.  We need to
// make sure after a hotplug timeout if we have not received the speed event
// notification from the hardware that we restart the hardware OOB state
// machine.
//
pub const SCIC_SDS_SATA_LINK_TRAINING_TIMEOUT: c_int = 250;
//
// isci_phy - hba local phy infrastructure
// @sm:
// @protocol: attached device protocol
// @phy_index: physical index relative to the controller (0-3)
// @bcn_received_while_port_unassigned: bcn to report after port association
// @sata_timer: timeout SATA signature FIS arrival
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isci_phy {
    pub sm: sci_base_state_machine,
    pub owning_port: *mut isci_port,
    pub max_negotiated_speed: sas_linkrate,
    pub protocol: sas_protocol,
    pub phy_index: u8,
    pub bcn_received_while_port_unassigned: bool,
    pub is_in_link_training: bool,
    pub sata_timer: sci_timer,
    pub transport_layer_registers: *mut scu_transport_layer_registers __iomem,
    pub link_layer_registers: *mut scu_link_layer_registers __iomem,
    pub sas_phy: asd_sas_phy,
    pub sas_addr: [u8; SAS_ADDR_SIZE],
    pub iaf: sas_identify_frame,
    pub fis: dev_to_host_fis,
    pub frame_rcvd: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_phy_cap {
//
// The SAS specification indicates the start bit shall
// always be set to
// 1.  This implementation will have the start bit set
// to 0 if the PHY CAPABILITIES were either not
// received or speed negotiation failed.
//
    pub start:1: u8,
    pub tx_ssc_type:1: u8,
    pub res1:2: u8,
    pub req_logical_linkrate:4: u8,
    pub gen1_no_ssc:1: u32,
    pub gen1_ssc:1: u32,
    pub gen2_no_ssc:1: u32,
    pub gen2_ssc:1: u32,
    pub gen3_no_ssc:1: u32,
    pub gen3_ssc:1: u32,
    pub res2:17: u32,
    pub parity:1: u32,
}

// this data structure reflects the link layer transmit identification reg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_phy_proto {
    pub _r_a:1: u16,
    pub smp_iport:1: u16,
    pub stp_iport:1: u16,
    pub ssp_iport:1: u16,
    pub _r_b:4: u16,
    pub _r_c:1: u16,
    pub smp_tport:1: u16,
    pub stp_tport:1: u16,
    pub ssp_tport:1: u16,
    pub _r_d:4: u16,
}

//
// struct sci_phy_properties - This structure defines the properties common to
// all phys that can be retrieved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_phy_properties {
//
// This field specifies the port that currently contains the
// supplied phy.  This field may be set to NULL
// if the phy is not currently contained in a port.
//
    pub iport: *mut isci_port,
//
// This field specifies the link rate at which the phy is
// currently operating.
//
    pub negotiated_link_rate: sas_linkrate,
//
// This field specifies the index of the phy in relation to other
// phys within the controller.  This index is zero relative.
//
    pub index: u8,
}

//
// struct sci_sas_phy_properties - This structure defines the properties,
// specific to a SAS phy, that can be retrieved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_sas_phy_properties {
//
// This field delineates the Identify Address Frame received
// from the remote end point.
//
    pub rcvd_iaf: sas_identify_frame,
//
// This field delineates the Phy capabilities structure received
// from the remote end point.
//
    pub rcvd_cap: sci_phy_cap,
}

//
// struct sci_sata_phy_properties - This structure defines the properties,
// specific to a SATA phy, that can be retrieved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_sata_phy_properties {
//
// This field delineates the signature FIS received from the
// attached target.
//
    pub signature_fis: dev_to_host_fis,
//
// This field specifies to the user if a port selector is connected
// on the specified phy.
//
    pub is_port_selector_present: bool,
}

//
// enum sci_phy_counter_id - This enumeration depicts the various pieces of
// optional information that can be retrieved for a specific phy.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sci_phy_counter_id {
//
// This PHY information field tracks the number of frames received.
//
    SCIC_PHY_COUNTER_RECEIVED_FRAME,

//
// This PHY information field tracks the number of frames transmitted.
//
    SCIC_PHY_COUNTER_TRANSMITTED_FRAME,

//
// This PHY information field tracks the number of DWORDs received.
//
    SCIC_PHY_COUNTER_RECEIVED_FRAME_WORD,

//
// This PHY information field tracks the number of DWORDs transmitted.
//
    SCIC_PHY_COUNTER_TRANSMITTED_FRAME_DWORD,

//
// This PHY information field tracks the number of times DWORD
// synchronization was lost.
//
    SCIC_PHY_COUNTER_LOSS_OF_SYNC_ERROR,

//
// This PHY information field tracks the number of received DWORDs with
// running disparity errors.
//
    SCIC_PHY_COUNTER_RECEIVED_DISPARITY_ERROR,

//
// This PHY information field tracks the number of received frames with a
// CRC error (not including short or truncated frames).
//
    SCIC_PHY_COUNTER_RECEIVED_FRAME_CRC_ERROR,

//
// This PHY information field tracks the number of DONE (ACK/NAK TIMEOUT)
// primitives received.
//
    SCIC_PHY_COUNTER_RECEIVED_DONE_ACK_NAK_TIMEOUT,

//
// This PHY information field tracks the number of DONE (ACK/NAK TIMEOUT)
// primitives transmitted.
//
    SCIC_PHY_COUNTER_TRANSMITTED_DONE_ACK_NAK_TIMEOUT,

//
// This PHY information field tracks the number of times the inactivity
// timer for connections on the phy has been utilized.
//
    SCIC_PHY_COUNTER_INACTIVITY_TIMER_EXPIRED,

//
// This PHY information field tracks the number of DONE (CREDIT TIMEOUT)
// primitives received.
//
    SCIC_PHY_COUNTER_RECEIVED_DONE_CREDIT_TIMEOUT,

//
// This PHY information field tracks the number of DONE (CREDIT TIMEOUT)
// primitives transmitted.
//
    SCIC_PHY_COUNTER_TRANSMITTED_DONE_CREDIT_TIMEOUT,

//
// This PHY information field tracks the number of CREDIT BLOCKED
// primitives received.
// @note Depending on remote device implementation, credit blocks
// may occur regularly.
//
    SCIC_PHY_COUNTER_RECEIVED_CREDIT_BLOCKED,

//
// This PHY information field contains the number of short frames
// received.  A short frame is simply a frame smaller then what is
// allowed by either the SAS or SATA specification.
//
    SCIC_PHY_COUNTER_RECEIVED_SHORT_FRAME,

//
// This PHY information field contains the number of frames received after
// credit has been exhausted.
//
    SCIC_PHY_COUNTER_RECEIVED_FRAME_WITHOUT_CREDIT,

//
// This PHY information field contains the number of frames received after
// a DONE has been received.
//
    SCIC_PHY_COUNTER_RECEIVED_FRAME_AFTER_DONE,

//
// This PHY information field contains the number of times the phy
// failed to achieve DWORD synchronization during speed negotiation.
//
    SCIC_PHY_COUNTER_SN_DWORD_SYNC_ERROR
}

//
// enum sci_phy_states - phy state machine states
// @SCI_PHY_INITIAL: Simply the initial state for the base domain state
// machine.
// @SCI_PHY_STOPPED: phy has successfully been stopped.  In this state
// no new IO operations are permitted on this phy.
// @SCI_PHY_STARTING: the phy is in the process of becomming ready.  In
// this state no new IO operations are permitted on
// this phy.
// @SCI_PHY_SUB_INITIAL: Initial state
// @SCI_PHY_SUB_AWAIT_OSSP_EN: Wait state for the hardware OSSP event
// type notification
// @SCI_PHY_SUB_AWAIT_SAS_SPEED_EN: Wait state for the PHY speed
// notification
// @SCI_PHY_SUB_AWAIT_IAF_UF: Wait state for the IAF Unsolicited frame
// notification
// @SCI_PHY_SUB_AWAIT_SAS_POWER: Wait state for the request to consume
// power
// @SCI_PHY_SUB_AWAIT_SATA_POWER: Wait state for request to consume
// power
// @SCI_PHY_SUB_AWAIT_SATA_PHY_EN: Wait state for the SATA PHY
// notification
// @SCI_PHY_SUB_AWAIT_SATA_SPEED_EN: Wait for the SATA PHY speed
// notification
// @SCI_PHY_SUB_AWAIT_SIG_FIS_UF: Wait state for the SIGNATURE FIS
// unsolicited frame notification
// @SCI_PHY_SUB_FINAL: Exit state for this state machine
// @SCI_PHY_READY: phy is now ready.  Thus, the user is able to perform
// IO operations utilizing this phy as long as it is
// currently part of a valid port.  This state is
// entered from the STARTING state.
// @SCI_PHY_RESETTING: phy is in the process of being reset.  In this
// state no new IO operations are permitted on this
// phy.  This state is entered from the READY state.
// @SCI_PHY_FINAL: Simply the final state for the base phy state
// machine.
//

extern "C" {
    pub fn sci_phy_linkrate(iphy: *mut isci_phy) -> sas_linkrate;
}
extern "C" {
    pub fn isci_phy_init(iphy: *mut isci_phy, ihost: *mut isci_host, index: c_int);
}
extern "C" {
    pub fn isci_phy_control(phy: *mut asd_sas_phy, func: phy_func, buf: *mut c_void) -> c_int;
}
