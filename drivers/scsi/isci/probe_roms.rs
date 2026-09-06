//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/probe_roms.h
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

pub const SCIC_SDS_PARM_NO_SPEED: c_int = 0;
// generation 1 (i.e. 1.5 Gb/s)
pub const SCIC_SDS_PARM_GEN1_SPEED: c_int = 1;
// generation 2 (i.e. 3.0 Gb/s)
pub const SCIC_SDS_PARM_GEN2_SPEED: c_int = 2;
// generation 3 (i.e. 6.0 Gb/s)
pub const SCIC_SDS_PARM_GEN3_SPEED: c_int = 3;

// parameters that can be set by module parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_user_parameters {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_phy_user_params {
//
// This field specifies the NOTIFY (ENABLE SPIN UP) primitive
// insertion frequency for this phy index.
//
    pub notify_enable_spin_up_insertion_frequency: u32,
//
// This method specifies the number of transmitted DWORDs within which
// to transmit a single ALIGN primitive.  This value applies regardless
// of what type of device is attached or connection state.  A value of
// 0 indicates that no ALIGN primitives will be inserted.
//
    pub align_insertion_frequency: u16,
//
// This method specifies the number of transmitted DWORDs within which
// to transmit 2 ALIGN primitives.  This applies for SAS connections
// only.  A minimum value of 3 is required for this field.
//
    pub in_connection_align_insertion_frequency: u16,
//
// This field indicates the maximum speed generation to be utilized
// by phys in the supplied port.
// - A value of 1 indicates generation 1 (i.e. 1.5 Gb/s).
// - A value of 2 indicates generation 2 (i.e. 3.0 Gb/s).
// - A value of 3 indicates generation 3 (i.e. 6.0 Gb/s).
//
    pub max_speed_generation: u8,
    pub phys: [}; SCI_MAX_PHYS],
//
// This field specifies the maximum number of direct attached devices
// that can have power supplied to them simultaneously.
//
    pub max_concurr_spinup: u8,
//
// This field specifies the number of seconds to allow a phy to consume
// power before yielding to another phy.
//
    pub phy_spin_up_delay_interval: u8,
//
// These timer values specifies how long a link will remain open with no
// activity in increments of a microsecond, it can be in increments of
// 100 microseconds if the upper most bit is set.
//
    pub stp_inactivity_timeout: u16,
    pub ssp_inactivity_timeout: u16,
//
// These timer values specifies how long a link will remain open in increments
// of 100 microseconds.
//
    pub stp_max_occupancy_timeout: u16,
    pub ssp_max_occupancy_timeout: u16,
//
// This timer value specifies how long a link will remain open with no
// outbound traffic in increments of a microsecond.
//
    pub no_outbound_task_timeout: u8,
}

pub const SCIC_SDS_PARM_PHY_MASK_MIN: c_uint = 0x0;
pub const SCIC_SDS_PARM_PHY_MASK_MAX: c_uint = 0xF;
pub const MAX_CONCURRENT_DEVICE_SPIN_UP_COUNT: c_int = 4;
extern "C" {
    pub fn sci_oem_parameters_validate(oem: *mut sci_oem_params, version: u8) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isci_oem_hdr {
    pub sig: [u8; 4],
    pub rev_major: u8,
    pub rev_minor: u8,
    pub len: u16,
    pub checksum: u8,
    pub reserved1: u8,
    pub reserved2: u16,
// C attribute field omitted

pub const SCI_MAX_PORTS: c_int = 4;
pub const SCI_MAX_PHYS: c_int = 4;
pub const SCI_MAX_CONTROLLERS: c_int = 2;

pub const ROMSIGNATURE: c_uint = 0xaa55;

pub const ISCI_OEM_SIG_SIZE: c_int = 4;

pub const ISCI_ROM_SIG_SIZE: c_int = 8;

pub const ISCI_ROM_VER_1_0: c_uint = 0x10;
pub const ISCI_ROM_VER_1_1: c_uint = 0x11;
pub const ISCI_ROM_VER_1_3: c_uint = 0x13;

// Allowed PORT configuration modes APC Automatic PORT configuration mode is
// defined by the OEM configuration parameters providing no PHY_MASK parameters
// for any PORT. i.e. There are no phys assigned to any of the ports at start.
// MPC Manual PORT configuration mode is defined by the OEM configuration
// parameters providing a PHY_MASK value for any PORT.  It is assumed that any
// PORT with no PHY_MASK is an invalid port and not all PHYs must be assigned.
// A PORT_PHY mask that assigns just a single PHY to a port and no other PHYs
// being assigned is sufficient to declare manual PORT configuration.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sci_port_configuration_mode {
    SCIC_PORT_MANUAL_CONFIGURATION_MODE = 0,
    SCIC_PORT_AUTOMATIC_CONFIGURATION_MODE = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_bios_oem_param_block_hdr {
    pub signature: [u8; ISCI_ROM_SIG_SIZE],
    pub total_block_length: u16,
    pub hdr_length: u8,
    pub version: u8,
    pub preboot_source: u8,
    pub num_elements: u8,
    pub element_length: u16,
    pub reserved: [u8; 8],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_oem_params {
    pub mode_type: u8,
    pub max_concurr_spin_up: u8,
//
// This bitfield indicates the OEM's desired default Tx
// Spread Spectrum Clocking (SSC) settings for SATA and SAS.
// NOTE: Default SSC Modulation Frequency is 31.5KHz.
//
// NOTE: Max spread for SATA is +0 / -5000 PPM.
// Down-spreading SSC (only method allowed for SATA):
// SATA SSC Tx Disabled                    = 0x0
// SATA SSC Tx at +0 / -1419 PPM Spread    = 0x2
// SATA SSC Tx at +0 / -2129 PPM Spread    = 0x3
// SATA SSC Tx at +0 / -4257 PPM Spread    = 0x6
// SATA SSC Tx at +0 / -4967 PPM Spread    = 0x7
//
    pub ssc_sata_tx_spread_level:4: u8,
//
// SAS SSC Tx Disabled                     = 0x0
//
// NOTE: Max spread for SAS down-spreading +0
// -2300 PPM
// Down-spreading SSC:
// SAS SSC Tx at +0 / -1419 PPM Spread     = 0x2
// SAS SSC Tx at +0 / -2129 PPM Spread     = 0x3
//
// NOTE: Max spread for SAS center-spreading +2300
// -2300 PPM
// Center-spreading SSC:
// SAS SSC Tx at +1064 / -1064 PPM Spread  = 0x3
// SAS SSC Tx at +2129 / -2129 PPM Spread  = 0x6
//
    pub ssc_sas_tx_spread_level:3: u8,
//
// NOTE: Refer to the SSC section of the SAS 2.x
// Specification for proper setting of this field.
// For standard SAS Initiator SAS PHY operation it
// should be 0 for Down-spreading.
// SAS SSC Tx spread type:
// Down-spreading SSC      = 0
// Center-spreading SSC    = 1
//
    pub ssc_sas_tx_type:1: u8,
}

//
// This field indicates length of the SAS/SATA cable between
// host and device.
// This field is used make relationship between analog
// parameters of the phy in the silicon and length of the cable.
// Supported cable attenuation levels:
// "short"- up to 3m, "medium"-3m to 6m, and "long"- more than
// 6m.
//
// This is bit mask field:
//
// BIT:      (MSB) 7     6     5     4
// ASSIGNMENT:   <phy3><phy2><phy1><phy0>  - Medium cable
// length assignment
// BIT:            3     2     1     0  (LSB)
// ASSIGNMENT:   <phy3><phy2><phy1><phy0>  - Long cable length
// assignment
//
// BITS 7-4 are set when the cable length is assigned to medium
// BITS 3-0 are set when the cable length is assigned to long
//
// The BIT positions are clear when the cable length is
// assigned to short.
//
// Setting the bits for both long and medium cable length is
// undefined.
//
// A value of 0x84 would assign
// phy3 - medium
// phy2 - long
// phy1 - short
// phy0 - short
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_phy_oem_params {
    pub high: u32,
    pub low: u32,
    pub sas_address: },
    pub afe_tx_amp_control0: u32,
    pub afe_tx_amp_control1: u32,
    pub afe_tx_amp_control2: u32,
    pub afe_tx_amp_control3: u32,
    pub phys: [}; SCI_MAX_PHYS],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isci_orom {
    pub hdr: sci_bios_oem_param_block_hdr,
    pub ctrl: [sci_oem_params; SCI_MAX_CONTROLLERS],
// C attribute field omitted
