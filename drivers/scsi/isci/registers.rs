//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/registers.h
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
// This file contains the constants and structures for the SCU memory mapped
// registers.
//

//
// struct scu_viit_entry - This is the SCU Virtual Initiator Table Entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_viit_entry {
//
// This must be encoded as to the type of initiator that is being constructed
// for this port.
//
    pub status: u32,
//
// Virtual initiator high SAS Address
//
    pub initiator_sas_address_hi: u32,
//
// Virtual initiator low SAS Address
//
    pub initiator_sas_address_lo: u32,
//
// This must be 0
//
    pub reserved: u32,
}

// IIT Status Defines

// IIT Remote Initiator Defines

//
// struct scu_iit_entry - This will be implemented later when we support
// virtual functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_iit_entry {
    pub status: u32,
    pub remote_initiator_sas_address_hi: u32,
    pub remote_initiator_sas_address_lo: u32,
    pub remote_initiator: u32,
}

// Generate a value for an SCU register

//
// Generate a bit value for an SCU register
// Make sure that the register MASK is just a single bit

//
// Unions for bitfield definitions of SCU Registers
// SMU Post Context Port
// *****************************************************************************

// *****************************************************************************

// *****************************************************************************

// *****************************************************************************

// *****************************************************************************

// *****************************************************************************

// *****************************************************************************

// *****************************************************************************

// *****************************************************************************

// *****************************************************************************

// --------------------------------------------------------------------------

// --------------------------------------------------------------------------

//
// It seems to make sense that if you are going to reset the protocol
// engine group that you would also reset all of the protocol engines

// *****************************************************************************

// *****************************************************************************

// *****************************************************************************

//
// * SDMA Registers
// *****************************************************************************

// *****************************************************************************

// *****************************************************************************

//
// * SCU Link Layer Registers
// *****************************************************************************

// TODO: Where is the SATA_PSELTOV register?
//
// * SCU SAS Maximum Arbitration Wait Time Timeout Register
// *****************************************************************************

//
// TODO: Where is the SAS_LNKTOV register?
// TODO: Where is the SAS_PHYTOV register?

// SAS Identify Frame PHY Identifier Register

//
// * Port Task Scheduler registers shift and mask values
// *****************************************************************************

// *****************************************************************************

//
// * SMU Registers
// *****************************************************************************
//
// ----------------------------------------------------------------------------
// SMU Registers
// These registers are based off of BAR0
//
// To calculate the offset for other functions use
// BAR0 + FN# * SystemPageSize * 2
//
// The TCA is only accessable from FN#0 (Physical Function) and each
// is programmed by (BAR0 + SCU_SMU_TCA_OFFSET + (FN# * 0x04)) or
// TCA0 for FN#0 is at BAR0 + 0x0400
// TCA1 for FN#1 is at BAR0 + 0x0404
// etc.
// ----------------------------------------------------------------------------
// Accessable to all FN#s
pub const SCU_SMU_PCP_OFFSET: c_uint = 0x0000;
pub const SCU_SMU_AMR_OFFSET: c_uint = 0x0004;
pub const SCU_SMU_ISR_OFFSET: c_uint = 0x0010;
pub const SCU_SMU_IMR_OFFSET: c_uint = 0x0014;
pub const SCU_SMU_ICC_OFFSET: c_uint = 0x0018;
pub const SCU_SMU_HTTLBAR_OFFSET: c_uint = 0x0020;
pub const SCU_SMU_HTTUBAR_OFFSET: c_uint = 0x0024;
pub const SCU_SMU_TCR_OFFSET: c_uint = 0x0028;
pub const SCU_SMU_CQLBAR_OFFSET: c_uint = 0x0030;
pub const SCU_SMU_CQUBAR_OFFSET: c_uint = 0x0034;
pub const SCU_SMU_CQPR_OFFSET: c_uint = 0x0040;
pub const SCU_SMU_CQGR_OFFSET: c_uint = 0x0044;
pub const SCU_SMU_CQC_OFFSET: c_uint = 0x0048;
// Accessable to FN#0 only
pub const SCU_SMU_RNCLBAR_OFFSET: c_uint = 0x0080;
pub const SCU_SMU_RNCUBAR_OFFSET: c_uint = 0x0084;
pub const SCU_SMU_DCC_OFFSET: c_uint = 0x0090;
pub const SCU_SMU_DFC_OFFSET: c_uint = 0x0094;
pub const SCU_SMU_SMUCSR_OFFSET: c_uint = 0x0098;
pub const SCU_SMU_SCUSRCR_OFFSET: c_uint = 0x009C;
pub const SCU_SMU_SMAW_OFFSET: c_uint = 0x00A0;
pub const SCU_SMU_SMDW_OFFSET: c_uint = 0x00A4;
// Accessable to FN#0 only
pub const SCU_SMU_TCA_OFFSET: c_uint = 0x0400;
// Accessable to all FN#s
pub const SCU_SMU_MT_MLAR0_OFFSET: c_uint = 0x2000;
pub const SCU_SMU_MT_MUAR0_OFFSET: c_uint = 0x2004;
pub const SCU_SMU_MT_MDR0_OFFSET: c_uint = 0x2008;
pub const SCU_SMU_MT_VCR0_OFFSET: c_uint = 0x200C;
pub const SCU_SMU_MT_MLAR1_OFFSET: c_uint = 0x2010;
pub const SCU_SMU_MT_MUAR1_OFFSET: c_uint = 0x2014;
pub const SCU_SMU_MT_MDR1_OFFSET: c_uint = 0x2018;
pub const SCU_SMU_MT_VCR1_OFFSET: c_uint = 0x201C;
pub const SCU_SMU_MPBA_OFFSET: c_uint = 0x3000;
//
// struct smu_registers - These are the SMU registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_registers {
// 0x0000 PCP
    pub post_context_port: u32,
// 0x0004 AMR
    pub address_modifier: u32,
    pub reserved_08: u32,
    pub reserved_0C: u32,
// 0x0010 ISR
    pub interrupt_status: u32,
// 0x0014 IMR
    pub interrupt_mask: u32,
// 0x0018 ICC
    pub interrupt_coalesce_control: u32,
    pub reserved_1C: u32,
// 0x0020 HTTLBAR
    pub host_task_table_lower: u32,
// 0x0024 HTTUBAR
    pub host_task_table_upper: u32,
// 0x0028 TCR
    pub task_context_range: u32,
    pub reserved_2C: u32,
// 0x0030 CQLBAR
    pub completion_queue_lower: u32,
// 0x0034 CQUBAR
    pub completion_queue_upper: u32,
    pub reserved_38: u32,
    pub reserved_3C: u32,
// 0x0040 CQPR
    pub completion_queue_put: u32,
// 0x0044 CQGR
    pub completion_queue_get: u32,
// 0x0048 CQC
    pub completion_queue_control: u32,
    pub reserved_4C: u32,
    pub reserved_5x: [u32; 4],
    pub reserved_6x: [u32; 4],
    pub reserved_7x: [u32; 4],
//
// Accessable to FN#0 only
// 0x0080 RNCLBAR
    pub remote_node_context_lower: u32,
// 0x0084 RNCUBAR
    pub remote_node_context_upper: u32,
    pub reserved_88: u32,
    pub reserved_8C: u32,
// 0x0090 DCC
    pub device_context_capacity: u32,
// 0x0094 DFC
    pub device_function_capacity: u32,
// 0x0098 SMUCSR
    pub control_status: u32,
// 0x009C SCUSRCR
    pub soft_reset_control: u32,
// 0x00A0 SMAW
    pub mmr_address_window: u32,
// 0x00A4 SMDW
    pub mmr_data_window: u32,
// 0x00A8 CGUCR
    pub clock_gating_control: u32,
// 0x00AC CGUPC
    pub clock_gating_performance: u32,
// A whole bunch of reserved space
    pub reserved_Bx: [u32; 4],
    pub reserved_Cx: [u32; 4],
    pub reserved_Dx: [u32; 4],
    pub reserved_Ex: [u32; 4],
    pub reserved_Fx: [u32; 4],
    pub reserved_1xx: [u32; 64],
    pub reserved_2xx: [u32; 64],
    pub reserved_3xx: [u32; 64],
//
// Accessable to FN#0 only
// 0x0400 TCA
    pub task_context_assignment: [u32; 256],
// MSI-X registers not included
}

//
// SDMA Registers
// *****************************************************************************
pub const SCU_SDMA_BASE: c_uint = 0x6000;
pub const SCU_SDMA_PUFATLHAR_OFFSET: c_uint = 0x0000;
pub const SCU_SDMA_PUFATUHAR_OFFSET: c_uint = 0x0004;
pub const SCU_SDMA_UFLHBAR_OFFSET: c_uint = 0x0008;
pub const SCU_SDMA_UFUHBAR_OFFSET: c_uint = 0x000C;
pub const SCU_SDMA_UFQC_OFFSET: c_uint = 0x0010;
pub const SCU_SDMA_UFQPP_OFFSET: c_uint = 0x0014;
pub const SCU_SDMA_UFQGP_OFFSET: c_uint = 0x0018;
pub const SCU_SDMA_PDMACR_OFFSET: c_uint = 0x001C;
pub const SCU_SDMA_CDMACR_OFFSET: c_uint = 0x0080;
//
// struct scu_sdma_registers - These are the SCU SDMA Registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_sdma_registers {
// 0x0000 PUFATLHAR
    pub uf_address_table_lower: u32,
// 0x0004 PUFATUHAR
    pub uf_address_table_upper: u32,
// 0x0008 UFLHBAR
    pub uf_header_base_address_lower: u32,
// 0x000C UFUHBAR
    pub uf_header_base_address_upper: u32,
// 0x0010 UFQC
    pub unsolicited_frame_queue_control: u32,
// 0x0014 UFQPP
    pub unsolicited_frame_put_pointer: u32,
// 0x0018 UFQGP
    pub unsolicited_frame_get_pointer: u32,
// 0x001C PDMACR
    pub pdma_configuration: u32,
// Reserved until offset 0x80
    pub reserved_0020_007C: [u32; 0x18],
// 0x0080 CDMACR
    pub cdma_configuration: u32,
// Remainder SDMA register space
    pub reserved_0084_0400: [u32; 0xDF],
}

//
// * SCU Link Registers
// *****************************************************************************
pub const SCU_PEG0_OFFSET: c_uint = 0x0000;
pub const SCU_PEG1_OFFSET: c_uint = 0x8000;
pub const SCU_TL0_OFFSET: c_uint = 0x0000;
pub const SCU_TL1_OFFSET: c_uint = 0x0400;
pub const SCU_TL2_OFFSET: c_uint = 0x0800;
pub const SCU_TL3_OFFSET: c_uint = 0x0C00;
pub const SCU_LL_OFFSET: c_uint = 0x0080;

// Transport Layer Offsets (PEG + TL)
pub const SCU_TLCR_OFFSET: c_uint = 0x0000;
pub const SCU_TLADTR_OFFSET: c_uint = 0x0004;
pub const SCU_TLTTMR_OFFSET: c_uint = 0x0008;
pub const SCU_TLEECR0_OFFSET: c_uint = 0x000C;
pub const SCU_STPTLDARNI_OFFSET: c_uint = 0x0010;

//
// struct scu_transport_layer_registers - These are the SCU Transport Layer
// registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_transport_layer_registers {
// 0x0000 TLCR
    pub control: u32,
// 0x0004 TLADTR
    pub arbitration_delay_timer: u32,
// 0x0008 TLTTMR
    pub timer_test_mode: u32,
// 0x000C reserved
    pub reserved_0C: u32,
// 0x0010 STPTLDARNI
    pub stp_rni: u32,
// 0x0014 TLFEWPORCTRL
    pub tlfe_wpo_read_control: u32,
// 0x0018 TLFEWPORDATA
    pub tlfe_wpo_read_data: u32,
// 0x001C RXTLSSCSR1
    pub rxtl_single_step_control_status_1: u32,
// 0x0020 RXTLSSCSR2
    pub rxtl_single_step_control_status_2: u32,
// 0x0024 AWTRDDCR
    pub tlfe_awt_retry_delay_debug_control: u32,
// Remainder of TL memory space
    pub reserved_0028_007F: [u32; 0x16],
}

// Protocol Engine Group Registers
pub const SCU_SCUVZECRx_OFFSET: c_uint = 0x1080;
// Link Layer Offsets (PEG + TL + LL)
pub const SCU_SAS_SPDTOV_OFFSET: c_uint = 0x0000;
pub const SCU_SAS_LLSTA_OFFSET: c_uint = 0x0004;
pub const SCU_SATA_PSELTOV_OFFSET: c_uint = 0x0008;
pub const SCU_SAS_TIMETOV_OFFSET: c_uint = 0x0010;
pub const SCU_SAS_LOSTOT_OFFSET: c_uint = 0x0014;
pub const SCU_SAS_LNKTOV_OFFSET: c_uint = 0x0018;
pub const SCU_SAS_PHYTOV_OFFSET: c_uint = 0x001C;
pub const SCU_SAS_AFERCNT_OFFSET: c_uint = 0x0020;
pub const SCU_SAS_WERCNT_OFFSET: c_uint = 0x0024;
pub const SCU_SAS_TIID_OFFSET: c_uint = 0x0028;
pub const SCU_SAS_TIDNH_OFFSET: c_uint = 0x002C;
pub const SCU_SAS_TIDNL_OFFSET: c_uint = 0x0030;
pub const SCU_SAS_TISSAH_OFFSET: c_uint = 0x0034;
pub const SCU_SAS_TISSAL_OFFSET: c_uint = 0x0038;
pub const SCU_SAS_TIPID_OFFSET: c_uint = 0x003C;
pub const SCU_SAS_TIRES2_OFFSET: c_uint = 0x0040;
pub const SCU_SAS_ADRSTA_OFFSET: c_uint = 0x0044;
pub const SCU_SAS_MAWTTOV_OFFSET: c_uint = 0x0048;
pub const SCU_SAS_FRPLDFIL_OFFSET: c_uint = 0x0054;
pub const SCU_SAS_RFCNT_OFFSET: c_uint = 0x0060;
pub const SCU_SAS_TFCNT_OFFSET: c_uint = 0x0064;
pub const SCU_SAS_RFDCNT_OFFSET: c_uint = 0x0068;
pub const SCU_SAS_TFDCNT_OFFSET: c_uint = 0x006C;
pub const SCU_SAS_LERCNT_OFFSET: c_uint = 0x0070;
pub const SCU_SAS_RDISERRCNT_OFFSET: c_uint = 0x0074;
pub const SCU_SAS_CRERCNT_OFFSET: c_uint = 0x0078;
pub const SCU_STPCTL_OFFSET: c_uint = 0x007C;
pub const SCU_SAS_PCFG_OFFSET: c_uint = 0x0080;
pub const SCU_SAS_CLKSM_OFFSET: c_uint = 0x0084;
pub const SCU_SAS_TXCOMWAKE_OFFSET: c_uint = 0x0088;
pub const SCU_SAS_TXCOMINIT_OFFSET: c_uint = 0x008C;
pub const SCU_SAS_TXCOMSAS_OFFSET: c_uint = 0x0090;
pub const SCU_SAS_COMINIT_OFFSET: c_uint = 0x0094;
pub const SCU_SAS_COMWAKE_OFFSET: c_uint = 0x0098;
pub const SCU_SAS_COMSAS_OFFSET: c_uint = 0x009C;
pub const SCU_SAS_SFERCNT_OFFSET: c_uint = 0x00A0;
pub const SCU_SAS_CDFERCNT_OFFSET: c_uint = 0x00A4;
pub const SCU_SAS_DNFERCNT_OFFSET: c_uint = 0x00A8;
pub const SCU_SAS_PRSTERCNT_OFFSET: c_uint = 0x00AC;
pub const SCU_SAS_CNTCTL_OFFSET: c_uint = 0x00B0;
pub const SCU_SAS_SSPTOV_OFFSET: c_uint = 0x00B4;
pub const SCU_FTCTL_OFFSET: c_uint = 0x00B8;
pub const SCU_FRCTL_OFFSET: c_uint = 0x00BC;
pub const SCU_FTWMRK_OFFSET: c_uint = 0x00C0;
pub const SCU_ENSPINUP_OFFSET: c_uint = 0x00C4;
pub const SCU_SAS_TRNTOV_OFFSET: c_uint = 0x00C8;
pub const SCU_SAS_PHYCAP_OFFSET: c_uint = 0x00CC;
pub const SCU_SAS_PHYCTL_OFFSET: c_uint = 0x00D0;
pub const SCU_SAS_LLCTL_OFFSET: c_uint = 0x00D8;
pub const SCU_AFE_XCVRCR_OFFSET: c_uint = 0x00DC;
pub const SCU_AFE_LUTCR_OFFSET: c_uint = 0x00E0;

// #define SCU_FRXHECR_DCNT_OFFSET      0x00B0
pub const SCU_PSZGCR_OFFSET: c_uint = 0x00E4;
pub const SCU_SAS_RECPHYCAP_OFFSET: c_uint = 0x00E8;
// #define SCU_TX_LUTSEL_OFFSET         0x00B8
pub const SCU_SAS_PTxC_OFFSET: c_uint = 0x00D4 /* Same offset as SAS_TCTSTM */;
//
// struct scu_link_layer_registers - SCU Link Layer Registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_link_layer_registers {
// 0x0000 SAS_SPDTOV
    pub speed_negotiation_timers: u32,
// 0x0004 SAS_LLSTA
    pub link_layer_status: u32,
// 0x0008 SATA_PSELTOV
    pub port_selector_timeout: u32,
    pub reserved0C: u32,
// 0x0010 SAS_TIMETOV
    pub timeout_unit_value: u32,
// 0x0014 SAS_RCDTOV
    pub rcd_timeout: u32,
// 0x0018 SAS_LNKTOV
    pub link_timer_timeouts: u32,
// 0x001C SAS_PHYTOV
    pub sas_phy_timeouts: u32,
// 0x0020 SAS_AFERCNT
    pub received_address_frame_error_counter: u32,
// 0x0024 SAS_WERCNT
    pub invalid_dword_counter: u32,
// 0x0028 SAS_TIID
    pub transmit_identification: u32,
// 0x002C SAS_TIDNH
    pub sas_device_name_high: u32,
// 0x0030 SAS_TIDNL
    pub sas_device_name_low: u32,
// 0x0034 SAS_TISSAH
    pub source_sas_address_high: u32,
// 0x0038 SAS_TISSAL
    pub source_sas_address_low: u32,
// 0x003C SAS_TIPID
    pub identify_frame_phy_id: u32,
// 0x0040 SAS_TIRES2
    pub identify_frame_reserved: u32,
// 0x0044 SAS_ADRSTA
    pub received_address_frame: u32,
// 0x0048 SAS_MAWTTOV
    pub maximum_arbitration_wait_timer_timeout: u32,
// 0x004C SAS_PTxC
    pub transmit_primitive: u32,
// 0x0050 SAS_RORES
    pub error_counter_event_notification_control: u32,
// 0x0054 SAS_FRPLDFIL
    pub frxq_payload_fill_threshold: u32,
// 0x0058 SAS_LLHANG_TOT
    pub link_layer_hang_detection_timeout: u32,
    pub reserved_5C: u32,
// 0x0060 SAS_RFCNT
    pub received_frame_count: u32,
// 0x0064 SAS_TFCNT
    pub transmit_frame_count: u32,
// 0x0068 SAS_RFDCNT
    pub received_dword_count: u32,
// 0x006C SAS_TFDCNT
    pub transmit_dword_count: u32,
// 0x0070 SAS_LERCNT
    pub loss_of_sync_error_count: u32,
// 0x0074 SAS_RDISERRCNT
    pub running_disparity_error_count: u32,
// 0x0078 SAS_CRERCNT
    pub received_frame_crc_error_count: u32,
// 0x007C STPCTL
    pub stp_control: u32,
// 0x0080 SAS_PCFG
    pub phy_configuration: u32,
// 0x0084 SAS_CLKSM
    pub clock_skew_management: u32,
// 0x0088 SAS_TXCOMWAKE
    pub transmit_comwake_signal: u32,
// 0x008C SAS_TXCOMINIT
    pub transmit_cominit_signal: u32,
// 0x0090 SAS_TXCOMSAS
    pub transmit_comsas_signal: u32,
// 0x0094 SAS_COMINIT
    pub cominit_control: u32,
// 0x0098 SAS_COMWAKE
    pub comwake_control: u32,
// 0x009C SAS_COMSAS
    pub comsas_control: u32,
// 0x00A0 SAS_SFERCNT
    pub received_short_frame_count: u32,
// 0x00A4 SAS_CDFERCNT
    pub received_frame_without_credit_count: u32,
// 0x00A8 SAS_DNFERCNT
    pub received_frame_after_done_count: u32,
// 0x00AC SAS_PRSTERCNT
    pub phy_reset_problem_count: u32,
// 0x00B0 SAS_CNTCTL
    pub counter_control: u32,
// 0x00B4 SAS_SSPTOV
    pub ssp_timer_timeout_values: u32,
// 0x00B8 FTCTL
    pub ftx_control: u32,
// 0x00BC FRCTL
    pub frx_control: u32,
// 0x00C0 FTWMRK
    pub ftx_watermark: u32,
// 0x00C4 ENSPINUP
    pub notify_enable_spinup_control: u32,
// 0x00C8 SAS_TRNTOV
    pub sas_training_sequence_timer_values: u32,
// 0x00CC SAS_PHYCAP
    pub phy_capabilities: u32,
// 0x00D0 SAS_PHYCTL
    pub phy_control: u32,
    pub reserved_d4: u32,
// 0x00D8 LLCTL
    pub link_layer_control: u32,
// 0x00DC AFE_XCVRCR
    pub afe_xcvr_control: u32,
// 0x00E0 AFE_LUTCR
    pub afe_lookup_table_control: u32,
// 0x00E4 PSZGCR
    pub phy_source_zone_group_control: u32,
// 0x00E8 SAS_RECPHYCAP
    pub receive_phycap: u32,
    pub reserved_ec: u32,
// 0x00F0 SNAFERXRSTCTL
    pub speed_negotiation_afe_rx_reset_control: u32,
// 0x00F4 SAS_SSIPMCTL
    pub power_management_control: u32,
// 0x00F8 SAS_PSPREQ_PRIM
    pub sas_pm_partial_request_primitive: u32,
// 0x00FC SAS_PSSREQ_PRIM
    pub sas_pm_slumber_request_primitive: u32,
// 0x0100 SAS_PPSACK_PRIM
    pub sas_pm_ack_primitive_register: u32,
// 0x0104 SAS_PSNAK_PRIM
    pub sas_pm_nak_primitive_register: u32,
// 0x0108 SAS_SSIPMTOV
    pub sas_primitive_timeout: u32,
    pub reserved_10c: u32,
// 0x0110 - 0x011C PLAPRDCTRLxREG
    pub pla_product_control: [u32; 4],
// 0x0120 PLAPRDSUMREG
    pub pla_product_sum: u32,
// 0x0124 PLACONTROLREG
    pub pla_control: u32,
// Remainder of memory space 896 bytes
    pub reserved_0128_037f: [u32; 0x96],
}

//
// 0x00D4 // Same offset as SAS_TCTSTM SAS_PTxC
// u32   primitive_transmit_control;
//
// ----------------------------------------------------------------------------
// SGPIO
// ----------------------------------------------------------------------------
pub const SCU_SGPIO_OFFSET: c_uint = 0x1400;
// #define SCU_SGPIO_OFFSET         0x6000   // later moves to 0x1400 see HSD 652625
pub const SCU_SGPIO_SGICR_OFFSET: c_uint = 0x0000;
pub const SCU_SGPIO_SGPBR_OFFSET: c_uint = 0x0004;
pub const SCU_SGPIO_SGSDLR_OFFSET: c_uint = 0x0008;
pub const SCU_SGPIO_SGSDUR_OFFSET: c_uint = 0x000C;
pub const SCU_SGPIO_SGSIDLR_OFFSET: c_uint = 0x0010;
pub const SCU_SGPIO_SGSIDUR_OFFSET: c_uint = 0x0014;
pub const SCU_SGPIO_SGVSCR_OFFSET: c_uint = 0x0018;
// Address from 0x0820 to 0x083C
pub const SCU_SGPIO_SGODSR_OFFSET: c_uint = 0x0020;
//
// struct scu_sgpio_registers - SCU SGPIO Registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_sgpio_registers {
// 0x0000 SGPIO_SGICR
    pub interface_control: u32,
// 0x0004 SGPIO_SGPBR
    pub blink_rate: u32,
// 0x0008 SGPIO_SGSDLR
    pub start_drive_lower: u32,
// 0x000C SGPIO_SGSDUR
    pub start_drive_upper: u32,
// 0x0010 SGPIO_SGSIDLR
    pub serial_input_lower: u32,
// 0x0014 SGPIO_SGSIDUR
    pub serial_input_upper: u32,
// 0x0018 SGPIO_SGVSCR
    pub vendor_specific_code: u32,
// 0x001C Reserved
    pub reserved_001c: u32,
// 0x0020 SGPIO_SGODSR
    pub output_data_select: [u32; 8],
// Remainder of memory space 256 bytes
    pub reserved_1444_14ff: [u32; 0x30],
}

//
// * Defines for VIIT entry offsets
// * Access additional entries by SCU_VIIT_BASE + index * 0x10
// *****************************************************************************
pub const SCU_VIIT_BASE: c_uint = 0x1c00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_viit_registers {
    pub registers: [u32; 256],
}

//
// * SCU PORT TASK SCHEDULER REGISTERS
// *****************************************************************************
pub const SCU_PTSG_BASE: c_uint = 0x1000;
pub const SCU_PTSG_PTSGCR_OFFSET: c_uint = 0x0000;
pub const SCU_PTSG_RTCR_OFFSET: c_uint = 0x0004;
pub const SCU_PTSG_RTCCR_OFFSET: c_uint = 0x0008;
pub const SCU_PTSG_PTS0CR_OFFSET: c_uint = 0x0010;
pub const SCU_PTSG_PTS0SR_OFFSET: c_uint = 0x0014;
pub const SCU_PTSG_PTS1CR_OFFSET: c_uint = 0x0018;
pub const SCU_PTSG_PTS1SR_OFFSET: c_uint = 0x001C;
pub const SCU_PTSG_PTS2CR_OFFSET: c_uint = 0x0020;
pub const SCU_PTSG_PTS2SR_OFFSET: c_uint = 0x0024;
pub const SCU_PTSG_PTS3CR_OFFSET: c_uint = 0x0028;
pub const SCU_PTSG_PTS3SR_OFFSET: c_uint = 0x002C;
pub const SCU_PTSG_PCSPE0CR_OFFSET: c_uint = 0x0030;
pub const SCU_PTSG_PCSPE1CR_OFFSET: c_uint = 0x0034;
pub const SCU_PTSG_PCSPE2CR_OFFSET: c_uint = 0x0038;
pub const SCU_PTSG_PCSPE3CR_OFFSET: c_uint = 0x003C;
pub const SCU_PTSG_ETMTSCCR_OFFSET: c_uint = 0x0040;
pub const SCU_PTSG_ETMRNSCCR_OFFSET: c_uint = 0x0044;
//
// struct scu_port_task_scheduler_registers - These are the control/stats pairs
// for each Port Task Scheduler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_port_task_scheduler_registers {
    pub control: u32,
    pub status: u32,
}

//
// struct scu_port_task_scheduler_group_registers - These are the PORT Task
// Scheduler registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_port_task_scheduler_group_registers {
// 0x0000 PTSGCR
    pub control: u32,
// 0x0004 RTCR
    pub real_time_clock: u32,
// 0x0008 RTCCR
    pub real_time_clock_control: u32,
// 0x000C
    pub reserved_0C: u32,
//
// 0x0010 PTS0CR
// 0x0014 PTS0SR
// 0x0018 PTS1CR
// 0x001C PTS1SR
// 0x0020 PTS2CR
// 0x0024 PTS2SR
// 0x0028 PTS3CR
// 0x002C PTS3SR
    pub port: [scu_port_task_scheduler_registers; 4],
//
// 0x0030 PCSPE0CR
// 0x0034 PCSPE1CR
// 0x0038 PCSPE2CR
// 0x003C PCSPE3CR
    pub protocol_engine: [u32; 4],
// 0x0040 ETMTSCCR
    pub tc_scanning_interval_control: u32,
// 0x0044 ETMRNSCCR
    pub rnc_scanning_interval_control: u32,
// Remainder of memory space 128 bytes
    pub reserved_1048_107f: [u32; 0x0E],
}

pub const SCU_PTSG_SCUVZECR_OFFSET: c_uint = 0x003C;
//
// * AFE REGISTERS
// *****************************************************************************
pub const SCU_AFE_MMR_BASE: c_uint = 0xE000;
//
// AFE 0 is at offset 0x0800
// AFE 1 is at offset 0x0900
// AFE 2 is at offset 0x0a00
// AFE 3 is at offset 0x0b00
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_afe_transceiver {
// 0x0000 AFE_XCVR_CTRL0
    pub afe_xcvr_control0: u32,
// 0x0004 AFE_XCVR_CTRL1
    pub afe_xcvr_control1: u32,
// 0x0008
    pub reserved_0008: u32,
// 0x000c afe_dfx_rx_control0
    pub afe_dfx_rx_control0: u32,
// 0x0010 AFE_DFX_RX_CTRL1
    pub afe_dfx_rx_control1: u32,
// 0x0014
    pub reserved_0014: u32,
// 0x0018 AFE_DFX_RX_STS0
    pub afe_dfx_rx_status0: u32,
// 0x001c AFE_DFX_RX_STS1
    pub afe_dfx_rx_status1: u32,
// 0x0020
    pub reserved_0020: u32,
// 0x0024 AFE_TX_CTRL
    pub afe_tx_control: u32,
// 0x0028 AFE_TX_AMP_CTRL0
    pub afe_tx_amp_control0: u32,
// 0x002c AFE_TX_AMP_CTRL1
    pub afe_tx_amp_control1: u32,
// 0x0030 AFE_TX_AMP_CTRL2
    pub afe_tx_amp_control2: u32,
// 0x0034 AFE_TX_AMP_CTRL3
    pub afe_tx_amp_control3: u32,
// 0x0038 afe_tx_ssc_control
    pub afe_tx_ssc_control: u32,
// 0x003c
    pub reserved_003c: u32,
// 0x0040 AFE_RX_SSC_CTRL0
    pub afe_rx_ssc_control0: u32,
// 0x0044 AFE_RX_SSC_CTRL1
    pub afe_rx_ssc_control1: u32,
// 0x0048 AFE_RX_SSC_CTRL2
    pub afe_rx_ssc_control2: u32,
// 0x004c AFE_RX_EQ_STS0
    pub afe_rx_eq_status0: u32,
// 0x0050 AFE_RX_EQ_STS1
    pub afe_rx_eq_status1: u32,
// 0x0054 AFE_RX_CDR_STS
    pub afe_rx_cdr_status: u32,
// 0x0058
    pub reserved_0058: u32,
// 0x005c AFE_CHAN_CTRL
    pub afe_channel_control: u32,
// 0x0060-0x006c
    pub reserved_0060_006c: [u32; 0x04],
// 0x0070 AFE_XCVR_EC_STS0
    pub afe_xcvr_error_capture_status0: u32,
// 0x0074 AFE_XCVR_EC_STS1
    pub afe_xcvr_error_capture_status1: u32,
// 0x0078 AFE_XCVR_EC_STS2
    pub afe_xcvr_error_capture_status2: u32,
// 0x007c afe_xcvr_ec_status3
    pub afe_xcvr_error_capture_status3: u32,
// 0x0080 AFE_XCVR_EC_STS4
    pub afe_xcvr_error_capture_status4: u32,
// 0x0084 AFE_XCVR_EC_STS5
    pub afe_xcvr_error_capture_status5: u32,
// 0x0088-0x00fc
    pub reserved_008c_00fc: [u32; 0x1e],
}

//
// struct scu_afe_registers - AFE Regsiters
//
// Uaoa AFE registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_afe_registers {
// 0Xe000 AFE_BIAS_CTRL
    pub afe_bias_control: u32,
    pub reserved_0004: u32,
// 0x0008 AFE_PLL_CTRL0
    pub afe_pll_control0: u32,
// 0x000c AFE_PLL_CTRL1
    pub afe_pll_control1: u32,
// 0x0010 AFE_PLL_CTRL2
    pub afe_pll_control2: u32,
// 0x0014 AFE_CB_STS
    pub afe_common_block_status: u32,
// 0x0018-0x007c
    pub reserved_18_7c: [u32; 0x1a],
// 0x0080 AFE_PMSN_MCTRL0
    pub afe_pmsn_master_control0: u32,
// 0x0084 AFE_PMSN_MCTRL1
    pub afe_pmsn_master_control1: u32,
// 0x0088 AFE_PMSN_MCTRL2
    pub afe_pmsn_master_control2: u32,
// 0x008C-0x00fc
    pub reserved_008c_00fc: [u32; 0x1D],
// 0x0100 AFE_DFX_MST_CTRL0
    pub afe_dfx_master_control0: u32,
// 0x0104 AFE_DFX_MST_CTRL1
    pub afe_dfx_master_control1: u32,
// 0x0108 AFE_DFX_DCL_CTRL
    pub afe_dfx_dcl_control: u32,
// 0x010c AFE_DFX_DMON_CTRL
    pub afe_dfx_digital_monitor_control: u32,
// 0x0110 AFE_DFX_AMONP_CTRL
    pub afe_dfx_analog_p_monitor_control: u32,
// 0x0114 AFE_DFX_AMONN_CTRL
    pub afe_dfx_analog_n_monitor_control: u32,
// 0x0118 AFE_DFX_NTL_STS
    pub afe_dfx_ntl_status: u32,
// 0x011c AFE_DFX_FIFO_STS0
    pub afe_dfx_fifo_status0: u32,
// 0x0120 AFE_DFX_FIFO_STS1
    pub afe_dfx_fifo_status1: u32,
// 0x0124 AFE_DFX_MPAT_CTRL
    pub afe_dfx_master_pattern_control: u32,
// 0x0128 AFE_DFX_P0_CTRL
    pub afe_dfx_p0_control: u32,
// 0x012c-0x01a8 AFE_DFX_P0_DRx
    pub afe_dfx_p0_data: [u32; 32],
// 0x01ac
    pub reserved_01ac: u32,
// 0x01b0-0x020c AFE_DFX_P0_IRx
    pub afe_dfx_p0_instruction: [u32; 24],
// 0x0210
    pub reserved_0210: u32,
// 0x0214 AFE_DFX_P1_CTRL
    pub afe_dfx_p1_control: u32,
// 0x0218-0x245 AFE_DFX_P1_DRx
    pub afe_dfx_p1_data: [u32; 16],
// 0x0258-0x029c
    pub reserved_0258_029c: [u32; 0x12],
// 0x02a0-0x02bc AFE_DFX_P1_IRx
    pub afe_dfx_p1_instruction: [u32; 8],
// 0x02c0-0x2fc
    pub reserved_02c0_02fc: [u32; 0x10],
// 0x0300 AFE_DFX_TX_PMSN_CTRL
    pub afe_dfx_tx_pmsn_control: u32,
// 0x0304 AFE_DFX_RX_PMSN_CTRL
    pub afe_dfx_rx_pmsn_control: u32,
    pub reserved_0308: u32,
// 0x030c AFE_DFX_NOA_CTRL0
    pub afe_dfx_noa_control0: u32,
// 0x0310 AFE_DFX_NOA_CTRL1
    pub afe_dfx_noa_control1: u32,
// 0x0314 AFE_DFX_NOA_CTRL2
    pub afe_dfx_noa_control2: u32,
// 0x0318 AFE_DFX_NOA_CTRL3
    pub afe_dfx_noa_control3: u32,
// 0x031c AFE_DFX_NOA_CTRL4
    pub afe_dfx_noa_control4: u32,
// 0x0320 AFE_DFX_NOA_CTRL5
    pub afe_dfx_noa_control5: u32,
// 0x0324 AFE_DFX_NOA_CTRL6
    pub afe_dfx_noa_control6: u32,
// 0x0328 AFE_DFX_NOA_CTRL7
    pub afe_dfx_noa_control7: u32,
// 0x032c-0x07fc
    pub reserved_032c_07fc: [u32; 0x135],
// 0x0800-0x0bfc
    pub scu_afe_xcvr: [scu_afe_transceiver; 4],
// 0x0c00-0x0ffc
    pub reserved_0c00_0ffc: [u32; 0x0100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_protocol_engine_group_registers {
    pub table: [u32; 0xE0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_viit_iit {
    pub table: [u32; 256],
}

//
// Placeholder for the ZONE Partition Table information ZONING will not be
// included in the 1.1 release.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_zone_partition_table {
    pub table: [u32; 2048],
}

//
// Placeholder for the CRAM register since I am not sure if we need to
// read/write to these registers as yet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_completion_ram {
    pub ram: [u32; 128],
}

//
// Placeholder for the FBRAM registers since I am not sure if we need to
// read/write to these registers as yet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_frame_buffer_ram {
    pub ram: [u32; 128],
}

pub const scu_scratch_ram_SIZE_IN_DWORDS: c_int = 256;
//
// Placeholder for the scratch RAM registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_scratch_ram {
    pub ram: [u32; scu_scratch_ram_SIZE_IN_DWORDS],
}

//
// Placeholder since I am not yet sure what these registers are here for.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct noa_protocol_engine_partition {
    pub reserved: [u32; 64],
}

//
// Placeholder since I am not yet sure what these registers are here for.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct noa_hub_partition {
    pub reserved: [u32; 64],
}

//
// Placeholder since I am not yet sure what these registers are here for.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct noa_host_interface_partition {
    pub reserved: [u32; 64],
}

//
// struct transport_link_layer_pair - The SCU Hardware pairs up the TL
// registers with the LL registers so we must place them adjcent to make the
// array of registers in the PEG.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct transport_link_layer_pair {
    pub tl: scu_transport_layer_registers,
    pub ll: scu_link_layer_registers,
}

//
// struct scu_peg_registers - SCU Protocol Engine Memory mapped register space.
// These registers are unique to each protocol engine group.  There can be
// at most two PEG for a single SCU part.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_peg_registers {
    pub pe: [transport_link_layer_pair; 4],
    pub ptsg: scu_port_task_scheduler_group_registers,
    pub peg: scu_protocol_engine_group_registers,
    pub sgpio: scu_sgpio_registers,
    pub reserved_01500_1BFF: [u32; 0x1C0],
    pub viit: [scu_viit_entry; 64],
    pub zpt0: scu_zone_partition_table,
    pub zpt1: scu_zone_partition_table,
}

//
// struct scu_registers - SCU registers including both PEG registers if we turn
// on that compile option. All of these registers are in the memory mapped
// space returned from BAR1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_registers {
// 0x0000 - PEG 0
    pub peg0: scu_peg_registers,
// 0x6000 - SDMA and Miscellaneous
    pub sdma: scu_sdma_registers,
    pub cram: scu_completion_ram,
    pub fbram: scu_frame_buffer_ram,
    pub reserved_6800_69FF: [u32; 0x80],
    pub noa_pe: noa_protocol_engine_partition,
    pub noa_hub: noa_hub_partition,
    pub noa_if: noa_host_interface_partition,
    pub reserved_6d00_7fff: [u32; 0x4c0],
// 0x8000 - PEG 1
    pub peg1: scu_peg_registers,
// 0xE000 - AFE Registers
    pub afe: scu_afe_registers,
// 0xF000 - reserved
    pub reserved_f000_211fff: [u32; 0x80c00],
// 0x212000 - scratch RAM
    pub scratch_ram: scu_scratch_ram,
}
