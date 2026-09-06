//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/pm8001/pm80xx_hwi.h
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
// PMC-Sierra SPCv/ve 8088/8089 SAS/SATA based host adapters driver
//
// Copyright (c) 2008-2009 USI Co., Ltd.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// substantially similar to the "NO WARRANTY" disclaimer below
// ("Disclaimer") and any redistribution must be conditioned upon
// including a substantially similar Disclaimer requirement for further
// binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGES.
//

// for Request Opcode of IOMB

// 0x8 RESV IN SPCv

// 0xC, 0xD, 0xE removed in SPCv

// 0x13 SMP_RESPONSE is removed in SPCv

// 0x16 RESV IN SPCv

// 0x1A RESV IN SPCv

// 0x25 RESV IN SPCv

// 0x2D RESV IN SPCv

// for Response Opcode of IOMB

// 0x11 - SMP_RECEIVED Notification removed in SPCv

// spcv specific commands

// for phy start

// phy_stop
pub const PHY_STOP_SUCCESS: c_uint = 0x00;
pub const PHY_STOP_ERR_DEVICE_ATTACHED: c_uint = 0x1046;
// phy_profile
pub const SAS_PHY_ANALOG_SETTINGS_PAGE: c_uint = 0x04;
pub const PHY_DWORD_LENGTH: c_uint = 0xC;
// Thermal related
pub const THERMAL_ENABLE: c_uint = 0x1;
pub const THERMAL_LOG_ENABLE: c_uint = 0x1;
pub const THERMAL_PAGE_CODE_7H: c_uint = 0x6;
pub const THERMAL_PAGE_CODE_8H: c_uint = 0x7;
pub const LTEMPHIL: c_int = 70;
pub const RTEMPHIL: c_int = 100;
// Encryption info
pub const SCRATCH_PAD3_ENC_DISABLED: c_uint = 0x00000000;
pub const SCRATCH_PAD3_ENC_DIS_ERR: c_uint = 0x00000001;
pub const SCRATCH_PAD3_ENC_ENA_ERR: c_uint = 0x00000002;
pub const SCRATCH_PAD3_ENC_READY: c_uint = 0x00000003;

pub const SCRATCH_PAD3_SMF_ENABLED: c_int = 0;
pub const SCRATCH_PAD3_SM_MASK: c_uint = 0x000000F0;
pub const SCRATCH_PAD3_ERR_CODE: c_uint = 0x00FF0000;
pub const SEC_MODE_SMF: c_uint = 0x0;
pub const SEC_MODE_SMA: c_uint = 0x100;
pub const SEC_MODE_SMB: c_uint = 0x200;
pub const CIPHER_MODE_ECB: c_uint = 0x00000001;
pub const CIPHER_MODE_XTS: c_uint = 0x00000002;
pub const KEK_MGMT_SUBOP_KEYCARDUPDATE: c_uint = 0x4;
// SAS protocol timer configuration page
pub const SAS_PROTOCOL_TIMER_CONFIG_PAGE: c_uint = 0x04;
pub const STP_MCT_TMO: c_int = 32;
pub const SSP_MCT_TMO: c_int = 32;
pub const SAS_MAX_OPEN_TIME: c_int = 5;
pub const SMP_MAX_CONN_TIMER: c_uint = 0xFF;
pub const STP_FRM_TIMER: c_int = 0;

pub const SAS_MFD: c_int = 0;
pub const SAS_OPNRJT_RTRY_INTVL: c_int = 2;
pub const SAS_DOPNRJT_RTRY_TMO: c_int = 128;
pub const SAS_COPNRJT_RTRY_TMO: c_int = 128;

//
pub const SAS_DOPNRJT_RTRY_THR: c_int = 23438;
pub const SAS_COPNRJT_RTRY_THR: c_int = 23438;
pub const SAS_MAX_AIP: c_uint = 0x200000;
pub const IT_NEXUS_TIMEOUT: c_uint = 0x7D0;

// Port recovery timeout, 10000 ms for PM8006 controller
pub const CHIP_8006_PORT_RECOVERY_TIMEOUT: c_uint = 0x640000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_identify_frame_local {
// Byte 0
    pub frame_type:4: u8,
    pub dev_type:3: u8,
    pub _un0:1: u8,
// Byte 1
    pub _un1: u8,
// Byte 2
    pub _un20:1: u8,
    pub smp_iport:1: u8,
    pub stp_iport:1: u8,
    pub ssp_iport:1: u8,
    pub _un247:4: u8,
}

// Byte 3
// Byte 4 - 11
// Byte 12 - 19
// Byte 20

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_identify_frame_local {
// Byte 0
    pub _un0:1: u8,
    pub dev_type:3: u8,
    pub frame_type:4: u8,
// Byte 1
    pub _un1: u8,
// Byte 2
    pub _un247:4: u8,
    pub ssp_iport:1: u8,
    pub stp_iport:1: u8,
    pub smp_iport:1: u8,
    pub _un20:1: u8,
}

// Byte 3
// Byte 4 - 11
// Byte 12 - 19
// Byte 20

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi_msg_hdr {
    pub /: *mut *mut __le32 header; / Bits [11:0] - Message operation code,
// Bits [15:12] - Message Category
// Bits [21:16] - Outboundqueue ID for the
// Bits [23:22] - Reserved
// Bits [28:24] - Buffer Count, indicates how
// Bits [30:29] - Reserved
// Bits [31] - Message Valid bit
// C attribute field omitted
//
// brief the data structure of PHY Start Command
// use to describe enable the phy (128 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_start_req {
    pub tag: __le32,
    pub ase_sh_lm_slr_phyid: __le32,
    pub /: *mut *mut sas_identify_frame_local sas_identify; / 28 Bytes,
    pub spasti: __le32,
    pub reserved: [u32; 21],
// C attribute field omitted
//
// brief the data structure of PHY Start Command
// use to disable the phy (128 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_stop_req {
    pub tag: __le32,
    pub phy_id: __le32,
    pub reserved: [u32; 29],
// C attribute field omitted
// set device bits fis - device to host
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_dev_bits_fis {
    pub 0xA1*/: *mut *mut u8 fis_type; /,
    pub n_i_pmport: u8,
// b7 : n Bit. Notification bit. If set device needs attention.
// b6 : i Bit. Interrupt Bit
// b5-b4: reserved2
// b3-b0: PM Port
    pub status: u8,
    pub error: u8,
    pub _r_a: u32,
// C attribute field omitted
// PIO setup FIS - device to host
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pio_setup_fis {
    pub /: *mut *mut u8 fis_type; / 0x5f,
    pub i_d_pmPort: u8,
// b7 : reserved
// b6 : i bit. Interrupt bit
// b5 : d bit. data transfer direction. set to 1 for device to host
// b4 : reserved
// b3-b0: PM Port
    pub status: u8,
    pub error: u8,
    pub lbal: u8,
    pub lbam: u8,
    pub lbah: u8,
    pub device: u8,
    pub lbal_exp: u8,
    pub lbam_exp: u8,
    pub lbah_exp: u8,
    pub _r_a: u8,
    pub sector_count: u8,
    pub sector_count_exp: u8,
    pub _r_b: u8,
    pub e_status: u8,
    pub _r_c: [u8; 2],
    pub transfer_count: u8,
// C attribute field omitted
//
// brief the data structure of SATA Completion Response
// use to describe the sata task response (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sata_completion_resp {
    pub tag: __le32,
    pub status: __le32,
    pub param: __le32,
    pub sata_resp: [u32; 12],
// C attribute field omitted
//
// brief the data structure of SAS HW Event Notification
// use to alert the host about the hardware event(64 bytes)
//
// updated outbound struct for spcv
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_event_resp {
    pub lr_status_evt_portid: __le32,
    pub evt_param: __le32,
    pub phyid_npip_portstate: __le32,
    pub sas_identify: sas_identify_frame,
    pub sata_fis: dev_to_host_fis,
// C attribute field omitted
//
// brief the data structure for thermal event notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_hw_event {
    pub thermal_event: __le32,
    pub rht_lht: __le32,
// C attribute field omitted
//
// brief the data structure of REGISTER DEVICE Command
// use to describe MPI REGISTER DEVICE Command (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_dev_req {
    pub tag: __le32,
    pub phyid_portid: __le32,
    pub dtype_dlr_mcn_ir_retry: __le32,
    pub firstburstsize_ITNexustimeout: __le32,
    pub sas_addr: [u8; SAS_ADDR_SIZE],
    pub upper_device_id: __le32,
    pub reserved: [u32; 24],
// C attribute field omitted
//
// brief the data structure of DEREGISTER DEVICE Command
// use to request spc to remove all internal resources associated
// with the device id (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dereg_dev_req {
    pub tag: __le32,
    pub device_id: __le32,
    pub reserved: [u32; 29],
// C attribute field omitted
//
// brief the data structure of DEVICE_REGISTRATION Response
// use to notify the completion of the device registration (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_reg_resp {
    pub tag: __le32,
    pub status: __le32,
    pub device_id: __le32,
    pub reserved: [u32; 12],
// C attribute field omitted
//
// brief the data structure of Local PHY Control Command
// use to issue PHY CONTROL to local phy (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct local_phy_ctl_req {
    pub tag: __le32,
    pub phyop_phyid: __le32,
    pub reserved1: [u32; 29],
// C attribute field omitted
//
// brief the data structure of Local Phy Control Response
// use to describe MPI Local Phy Control Response (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct local_phy_ctl_resp {
    pub tag: __le32,
    pub phyop_phyid: __le32,
    pub status: __le32,
    pub reserved: [u32; 12],
// C attribute field omitted
pub const OP_BITS: c_uint = 0x0000FF00;
pub const ID_BITS: c_uint = 0x000000FF;
//
// brief the data structure of PORT Control Command
// use to control port properties (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_ctl_req {
    pub tag: __le32,
    pub portop_portid: __le32,
    pub param0: __le32,
    pub param1: __le32,
    pub reserved1: [u32; 27],
// C attribute field omitted
//
// brief the data structure of HW Event Ack Command
// use to acknowledge receive HW event (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_event_ack_req {
    pub tag: __le32,
    pub phyid_sea_portid: __le32,
    pub param0: __le32,
    pub param1: __le32,
    pub reserved1: [u32; 27],
// C attribute field omitted
//
// brief the data structure of PHY_START Response Command
// indicates the completion of PHY_START command (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_start_resp {
    pub tag: __le32,
    pub status: __le32,
    pub phyid: __le32,
    pub reserved: [u32; 12],
// C attribute field omitted
//
// brief the data structure of PHY_STOP Response Command
// indicates the completion of PHY_STOP command (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_stop_resp {
    pub tag: __le32,
    pub status: __le32,
    pub phyid: __le32,
    pub reserved: [u32; 12],
// C attribute field omitted
//
// brief the data structure of SSP Completion Response
// use to indicate a SSP Completion (n bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_completion_resp {
    pub tag: __le32,
    pub status: __le32,
    pub param: __le32,
    pub ssptag_rescv_rescpad: __le32,
// Must be last --ends in a flexible-array member.
    pub ssp_resp_iu: ssp_response_iu,
// __le32  residual_count;
// C attribute field omitted
pub const SSP_RESCV_BIT: c_uint = 0x00010000;
//
// brief the data structure of SATA EVNET response
// use to indicate a SATA Completion (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sata_event_resp {
    pub tag: __le32,
    pub event: __le32,
    pub port_id: __le32,
    pub device_id: __le32,
    pub reserved: u32,
    pub event_param0: __le32,
    pub event_param1: __le32,
    pub sata_addr_h32: __le32,
    pub sata_addr_l32: __le32,
    pub e_udt1_udt0_crc: __le32,
    pub e_udt5_udt4_udt3_udt2: __le32,
    pub a_udt1_udt0_crc: __le32,
    pub a_udt5_udt4_udt3_udt2: __le32,
    pub hwdevid_diferr: __le32,
    pub err_framelen_byteoffset: __le32,
    pub err_dataframe: __le32,
// C attribute field omitted
//
// brief the data structure of SSP EVNET esponse
// use to indicate a SSP Completion (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_event_resp {
    pub tag: __le32,
    pub event: __le32,
    pub port_id: __le32,
    pub device_id: __le32,
    pub ssp_tag: __le32,
    pub event_param0: __le32,
    pub event_param1: __le32,
    pub sas_addr_h32: __le32,
    pub sas_addr_l32: __le32,
    pub e_udt1_udt0_crc: __le32,
    pub e_udt5_udt4_udt3_udt2: __le32,
    pub a_udt1_udt0_crc: __le32,
    pub a_udt5_udt4_udt3_udt2: __le32,
    pub hwdevid_diferr: __le32,
    pub err_framelen_byteoffset: __le32,
    pub err_dataframe: __le32,
// C attribute field omitted
//
// brief the data structure of General Event Notification Response
// use to describe MPI General Event Notification Response (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct general_event_resp {
    pub status: __le32,
    pub inb_IOMB_payload: [__le32; 14],
// C attribute field omitted
pub const GENERAL_EVENT_PAYLOAD: c_int = 14;
pub const OPCODE_BITS: c_uint = 0x00000fff;
//
// brief the data structure of SMP Request Command
// use to describe MPI SMP REQUEST Command (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_req {
    pub tag: __le32,
    pub device_id: __le32,
    pub len_ip_ir: __le32,
// Bits [0] - Indirect response
// Bits [1] - Indirect Payload
// Bits [15:2] - Reserved
// Bits [23:16] - direct payload Len
// Bits [31:24] - Reserved
    pub smp_req16: [u8; 16],
    pub smp_req: [u8; 32],
    pub /: *mut *mut __le64 long_req_addr;/ sg dma address, LE,
    pub /: *mut *mut __le32 long_req_size;/ LE,
    pub _r_a: u32,
    pub /: *mut *mut __le64 long_resp_addr;/ sg dma address, LE,
    pub /: *mut *mut __le32 long_resp_size;/ LE,
    pub _r_b: u32,
    pub /: *mut *mut } long_smp_req;/ sequencer extension,
}

//
// brief the data structure of SMP Completion Response
// use to describe MPI SMP Completion Response (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_completion_resp {
    pub tag: __le32,
    pub status: __le32,
    pub param: __le32,
    pub _r_a: [u8; 252],
// C attribute field omitted
//
// brief the data structure of SSP SMP SATA Abort Command
// use to describe MPI SSP SMP & SATA Abort Command (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_abort_req {
    pub tag: __le32,
    pub device_id: __le32,
    pub tag_to_abort: __le32,
    pub abort_all: __le32,
    pub reserved: [u32; 27],
// C attribute field omitted
//
// brief the data structure of SSP SATA SMP Abort Response
// use to describe SSP SMP & SATA Abort Response ( 64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_abort_resp {
    pub tag: __le32,
    pub status: __le32,
    pub scp: __le32,
    pub reserved: [u32; 12],
// C attribute field omitted
//
// brief the data structure of SAS Diagnostic Start/End Command
// use to describe MPI SAS Diagnostic Start/End Command (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_diag_start_end_req {
    pub tag: __le32,
    pub operation_phyid: __le32,
    pub reserved: [u32; 29],
// C attribute field omitted
//
// brief the data structure of SAS Diagnostic Execute Command
// use to describe MPI SAS Diagnostic Execute Command (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_diag_execute_req {
    pub tag: __le32,
    pub cmdtype_cmddesc_phyid: __le32,
    pub pat1_pat2: __le32,
    pub threshold: __le32,
    pub codepat_errmsk: __le32,
    pub pmon: __le32,
    pub pERF1CTL: __le32,
    pub reserved: [u32; 24],
// C attribute field omitted
pub const SAS_DIAG_PARAM_BYTES: c_int = 24;
//
// brief the data structure of Set Device State Command
// use to describe MPI Set Device State Command (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_dev_state_req {
    pub tag: __le32,
    pub device_id: __le32,
    pub nds: __le32,
    pub reserved: [u32; 28],
// C attribute field omitted
//
// brief the data structure of SATA Start Command
// use to describe MPI SATA IO Start Command (64 bytes)
// Note: This structure is common for normal / encryption I/O
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sata_start_req {
    pub tag: __le32,
    pub device_id: __le32,
    pub data_len: __le32,
    pub retfis_ncqtag_atap_dir_m_dad: __le32,
    pub sata_fis: host_to_dev_fis,
    pub reserved1: u32,
    pub /: *mut *mut u32 reserved2; / dword 11. rsvd for normal I/O.,
// EPLE Descl for enc I/O
    pub /: *mut *mut u32 addr_low; / dword 12. rsvd for enc I/O,
    pub /: *mut *mut u32 addr_high; / dword 13. reserved for enc I/O,
    pub /: *mut *mut __le32 len; / dword 14: length for normal I/O.,
// EPLE Desch for enc I/O
    pub /: *mut *mut __le32 esgl; / dword 15. rsvd for enc I/O,
    pub /: *mut *mut __le32 atapi_scsi_cdb[4]; / dword 16-19. rsvd for enc I/O,
// The below fields are reserved for normal I/O
    pub /: *mut *mut __le32 key_index_mode; / dword 20,
    pub /: *mut *mut __le32 sector_cnt_enss;/ dword 21,
    pub /: *mut *mut __le32 keytagl; / dword 22,
    pub /: *mut *mut __le32 keytagh; / dword 23,
    pub /: *mut *mut __le32 twk_val0; / dword 24,
    pub /: *mut *mut __le32 twk_val1; / dword 25,
    pub /: *mut *mut __le32 twk_val2; / dword 26,
    pub /: *mut *mut __le32 twk_val3; / dword 27,
    pub /: *mut *mut __le32 enc_addr_low; / dword 28. Encryption SGL address high,
    pub /: *mut *mut __le32 enc_addr_high; / dword 29. Encryption SGL address low,
    pub /: *mut *mut __le32 enc_len; / dword 30. Encryption length,
    pub /: *mut *mut __le32 enc_esgl; / dword 31. Encryption esgl bit,
// C attribute field omitted
//
// brief the data structure of SSP INI TM Start Command
// use to describe MPI SSP INI TM Start Command (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_ini_tm_start_req {
    pub tag: __le32,
    pub device_id: __le32,
    pub relate_tag: __le32,
    pub tmf: __le32,
    pub lun: [u8; 8],
    pub ds_ads_m: __le32,
    pub reserved: [u32; 24],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_info_unit {
    pub /: *mut *mut u8 lun[8];/ SCSI Logical Unit Number,
    pub /: *mut *mut u8 reserved1;/ reserved,
    pub efb_prio_attr: u8,
// B7 : enabledFirstBurst
// B6-3 : taskPriority
// B2-0 : taskAttribute
    pub /: *mut *mut u8 reserved2; / reserved,
    pub additional_cdb_len: u8,
// B7-2 : additional_cdb_len
// B1-0 : reserved
    pub /: *mut *mut u8 cdb[16];/ The SCSI CDB up to 16 bytes length,
// C attribute field omitted
//
// brief the data structure of SSP INI IO Start Command
// use to describe MPI SSP INI IO Start Command (64 bytes)
// Note: This structure is common for normal / encryption I/O
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_ini_io_start_req {
    pub tag: __le32,
    pub device_id: __le32,
    pub data_len: __le32,
    pub dad_dir_m_tlr: __le32,
    pub ssp_iu: ssp_info_unit,
    pub /: *mut *mut __le32 addr_low; / dword 12: sgl low for normal I/O.,
// epl_descl for encryption I/O
    pub /: *mut *mut __le32 addr_high; / dword 13: sgl hi for normal I/O,
// dpl_descl for encryption I/O
    pub /: *mut *mut __le32 len; / dword 14: len for normal I/O.,
// edpl_desch for encryption I/O
    pub /: *mut *mut __le32 esgl; / dword 15: ESGL bit for normal I/O.,
// user defined tag mask for enc I/O
// The below fields are reserved for normal I/O
    pub /: *mut *mut u8 udt[12]; / dword 16-18,
    pub /: *mut *mut __le32 sectcnt_ios; / dword 19,
    pub /: *mut *mut __le32 key_cmode; / dword 20,
    pub /: *mut *mut __le32 ks_enss; / dword 21,
    pub /: *mut *mut __le32 keytagl; / dword 22,
    pub /: *mut *mut __le32 keytagh; / dword 23,
    pub /: *mut *mut __le32 twk_val0; / dword 24,
    pub /: *mut *mut __le32 twk_val1; / dword 25,
    pub /: *mut *mut __le32 twk_val2; / dword 26,
    pub /: *mut *mut __le32 twk_val3; / dword 27,
    pub /: *mut *mut __le32 enc_addr_low; / dword 28: Encryption sgl addr low,
    pub /: *mut *mut __le32 enc_addr_high; / dword 29: Encryption sgl addr hi,
    pub /: *mut *mut __le32 enc_len; / dword 30: Encryption length,
    pub /: *mut *mut __le32 enc_esgl; / dword 31: ESGL bit for encryption,
// C attribute field omitted
//
// brief the data structure for SSP_INI_DIF_ENC_IO COMMAND
// use to initiate SSP I/O operation with optional DIF/ENC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_dif_enc_io_req {
    pub tag: __le32,
    pub device_id: __le32,
    pub data_len: __le32,
    pub dirMTlr: __le32,
    pub sspiu0: __le32,
    pub sspiu1: __le32,
    pub sspiu2: __le32,
    pub sspiu3: __le32,
    pub sspiu4: __le32,
    pub sspiu5: __le32,
    pub sspiu6: __le32,
    pub epl_des: __le32,
    pub dpl_desl_ndplr: __le32,
    pub dpl_desh: __le32,
    pub uum_uuv_bss_difbits: __le32,
    pub udt: [u8; 12],
    pub sectcnt_ios: __le32,
    pub key_cmode: __le32,
    pub ks_enss: __le32,
    pub keytagl: __le32,
    pub keytagh: __le32,
    pub twk_val0: __le32,
    pub twk_val1: __le32,
    pub twk_val2: __le32,
    pub twk_val3: __le32,
    pub addr_low: __le32,
    pub addr_high: __le32,
    pub len: __le32,
    pub esgl: __le32,
// C attribute field omitted
//
// brief the data structure of Firmware download
// use to describe MPI FW DOWNLOAD Command (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_flash_Update_req {
    pub tag: __le32,
    pub cur_image_offset: __le32,
    pub cur_image_len: __le32,
    pub total_image_len: __le32,
    pub reserved0: [u32; 7],
    pub sgl_addr_lo: __le32,
    pub sgl_addr_hi: __le32,
    pub len: __le32,
    pub ext_reserved: __le32,
    pub reserved1: [u32; 16],
// C attribute field omitted
pub const FWFLASH_IOMB_RESERVED_LEN: c_uint = 0x07;
//
// brief the data structure of FW_FLASH_UPDATE Response
// use to describe MPI FW_FLASH_UPDATE Response (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_flash_Update_resp {
    pub tag: __le32,
    pub status: __le32,
    pub reserved: [u32; 13],
// C attribute field omitted
//
// brief the data structure of Get NVM Data Command
// use to get data from NVM in HBA(64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_nvm_data_req {
    pub tag: __le32,
    pub len_ir_vpdd: __le32,
    pub vpd_offset: __le32,
    pub reserved: [u32; 8],
    pub resp_addr_lo: __le32,
    pub resp_addr_hi: __le32,
    pub resp_len: __le32,
    pub reserved1: [u32; 17],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_nvm_data_req {
    pub tag: __le32,
    pub len_ir_vpdd: __le32,
    pub vpd_offset: __le32,
    pub reserved: [u32; 8],
    pub resp_addr_lo: __le32,
    pub resp_addr_hi: __le32,
    pub resp_len: __le32,
    pub reserved1: [u32; 17],
// C attribute field omitted
//
// brief the data structure for SET CONTROLLER CONFIG COMMAND
// use to modify controller configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_ctrl_cfg_req {
    pub tag: __le32,
    pub cfg_pg: [__le32; 14],
    pub reserved: [u32; 16],
// C attribute field omitted
//
// brief the data structure for GET CONTROLLER CONFIG COMMAND
// use to get controller configuration page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_ctrl_cfg_req {
    pub tag: __le32,
    pub pgcd: __le32,
    pub int_vec: __le32,
    pub reserved: [u32; 28],
// C attribute field omitted
//
// brief the data structure for KEK_MANAGEMENT COMMAND
// use for KEK management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kek_mgmt_req {
    pub tag: __le32,
    pub new_curidx_ksop: __le32,
    pub reserved: u32,
    pub kblob: [__le32; 12],
    pub reserved1: [u32; 16],
// C attribute field omitted
//
// brief the data structure for DEK_MANAGEMENT COMMAND
// use for DEK management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dek_mgmt_req {
    pub tag: __le32,
    pub kidx_dsop: __le32,
    pub dekidx: __le32,
    pub addr_l: __le32,
    pub addr_h: __le32,
    pub nent: __le32,
    pub dbf_tblsize: __le32,
    pub reserved: [u32; 24],
// C attribute field omitted
//
// brief the data structure for SET PHY PROFILE COMMAND
// use to retrive phy specific information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_phy_profile_req {
    pub tag: __le32,
    pub ppc_phyid: __le32,
    pub reserved: [__le32; 29],
// C attribute field omitted
//
// brief the data structure for GET PHY PROFILE COMMAND
// use to retrive phy specific information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_phy_profile_req {
    pub tag: __le32,
    pub ppc_phyid: __le32,
    pub profile: [__le32; 29],
// C attribute field omitted
//
// brief the data structure for EXT FLASH PARTITION
// use to manage ext flash partition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_flash_partition_req {
    pub tag: __le32,
    pub cmd: __le32,
    pub offset: __le32,
    pub len: __le32,
    pub reserved: [u32; 7],
    pub addr_low: __le32,
    pub addr_high: __le32,
    pub len1: __le32,
    pub ext: __le32,
    pub reserved1: [u32; 16],
// C attribute field omitted
pub const TWI_DEVICE: c_uint = 0x0;
pub const C_SEEPROM: c_uint = 0x1;
pub const VPD_FLASH: c_uint = 0x4;
pub const AAP1_RDUMP: c_uint = 0x5;
pub const IOP_RDUMP: c_uint = 0x6;
pub const EXPAN_ROM: c_uint = 0x7;
pub const IPMode: c_uint = 0x80000000;
pub const NVMD_TYPE: c_uint = 0x0000000F;
pub const NVMD_STAT: c_uint = 0x0000FFFF;
pub const NVMD_LEN: c_uint = 0xFF000000;
//
// brief the data structure of Get NVMD Data Response
// use to describe MPI Get NVMD Data Response (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_nvm_data_resp {
    pub tag: __le32,
    pub ir_tda_bn_dps_das_nvm: __le32,
    pub dlen_status: __le32,
    pub nvm_data: [__le32; 12],
// C attribute field omitted
//
// brief the data structure of SAS Diagnostic Start/End Response
// use to describe MPI SAS Diagnostic Start/End Response (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_diag_start_end_resp {
    pub tag: __le32,
    pub status: __le32,
    pub reserved: [u32; 13],
// C attribute field omitted
//
// brief the data structure of SAS Diagnostic Execute Response
// use to describe MPI SAS Diagnostic Execute Response (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_diag_execute_resp {
    pub tag: __le32,
    pub cmdtype_cmddesc_phyid: __le32,
    pub Status: __le32,
    pub ReportData: __le32,
    pub reserved: [u32; 11],
// C attribute field omitted
//
// brief the data structure of Set Device State Response
// use to describe MPI Set Device State Response (64 bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_dev_state_resp {
    pub tag: __le32,
    pub status: __le32,
    pub device_id: __le32,
    pub pds_nds: __le32,
    pub reserved: [u32; 11],
// C attribute field omitted
// new outbound structure for spcv - begins
//
// brief the data structure for SET CONTROLLER CONFIG COMMAND
// use to modify controller configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_ctrl_cfg_resp {
    pub tag: __le32,
    pub status: __le32,
    pub err_qlfr_pgcd: __le32,
    pub reserved: [u32; 12],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_ctrl_cfg_resp {
    pub tag: __le32,
    pub status: __le32,
    pub err_qlfr: __le32,
    pub confg_page: [__le32; 12],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kek_mgmt_resp {
    pub tag: __le32,
    pub status: __le32,
    pub kidx_new_curr_ksop: __le32,
    pub err_qlfr: __le32,
    pub reserved: [u32; 11],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dek_mgmt_resp {
    pub tag: __le32,
    pub status: __le32,
    pub kekidx_tbls_dsop: __le32,
    pub dekidx: __le32,
    pub err_qlfr: __le32,
    pub reserved: [u32; 10],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_phy_profile_resp {
    pub tag: __le32,
    pub status: __le32,
    pub ppc_phyid: __le32,
    pub ppc_specific_rsp: [__le32; 12],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_op_ext_resp {
    pub tag: __le32,
    pub cmd: __le32,
    pub status: __le32,
    pub epart_size: __le32,
    pub epart_sect_size: __le32,
    pub reserved: [u32; 10],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_phy_profile_resp {
    pub tag: __le32,
    pub status: __le32,
    pub ppc_phyid: __le32,
    pub ppc_specific_rsp: [__le32; 12],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_coalesced_comp_resp {
    pub coal_cnt: __le32,
    pub tag0: __le32,
    pub ssp_tag0: __le32,
    pub tag1: __le32,
    pub ssp_tag1: __le32,
    pub add_tag_ssp_tag: [__le32; 10],
// C attribute field omitted
// new outbound structure for spcv - ends
// brief data structure for SAS protocol timer configuration page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SASProtocolTimerConfig {
    pub /: *mut *mut __le32 pageCode; / 0,
    pub /: *mut *mut __le32 MST_MSI; / 1,
    pub /: *mut *mut __le32 STP_SSP_MCT_TMO; / 2,
    pub /: *mut *mut __le32 STP_FRM_TMO; / 3,
    pub /: *mut *mut __le32 STP_IDLE_TMO; / 4,
    pub /: *mut *mut __le32 OPNRJT_RTRY_INTVL; / 5,
    pub /: *mut *mut __le32 Data_Cmd_OPNRJT_RTRY_TMO; / 6,
    pub /: *mut *mut __le32 Data_Cmd_OPNRJT_RTRY_THR; / 7,
    pub /: *mut *mut __le32 MAX_AIP; / 8,
// C attribute field omitted
pub type SASProtocolTimerConfig_t = SASProtocolTimerConfig;
pub const NDS_BITS: c_uint = 0x0F;
pub const PDS_BITS: c_uint = 0xF0;
//
// HW Events type
//
pub const HW_EVENT_RESET_START: c_uint = 0x01;
pub const HW_EVENT_CHIP_RESET_COMPLETE: c_uint = 0x02;
pub const HW_EVENT_PHY_STOP_STATUS: c_uint = 0x03;
pub const HW_EVENT_SAS_PHY_UP: c_uint = 0x04;
pub const HW_EVENT_SATA_PHY_UP: c_uint = 0x05;
pub const HW_EVENT_SATA_SPINUP_HOLD: c_uint = 0x06;
pub const HW_EVENT_PHY_DOWN: c_uint = 0x07;
pub const HW_EVENT_PORT_INVALID: c_uint = 0x08;
pub const HW_EVENT_BROADCAST_CHANGE: c_uint = 0x09;
pub const HW_EVENT_PHY_ERROR: c_uint = 0x0A;
pub const HW_EVENT_BROADCAST_SES: c_uint = 0x0B;
pub const HW_EVENT_INBOUND_CRC_ERROR: c_uint = 0x0C;
pub const HW_EVENT_HARD_RESET_RECEIVED: c_uint = 0x0D;
pub const HW_EVENT_MALFUNCTION: c_uint = 0x0E;
pub const HW_EVENT_ID_FRAME_TIMEOUT: c_uint = 0x0F;
pub const HW_EVENT_BROADCAST_EXP: c_uint = 0x10;
pub const HW_EVENT_PHY_START_STATUS: c_uint = 0x11;
pub const HW_EVENT_LINK_ERR_INVALID_DWORD: c_uint = 0x12;
pub const HW_EVENT_LINK_ERR_DISPARITY_ERROR: c_uint = 0x13;
pub const HW_EVENT_LINK_ERR_CODE_VIOLATION: c_uint = 0x14;
pub const HW_EVENT_LINK_ERR_LOSS_OF_DWORD_SYNCH: c_uint = 0x15;
pub const HW_EVENT_LINK_ERR_PHY_RESET_FAILED: c_uint = 0x16;
pub const HW_EVENT_PORT_RECOVERY_TIMER_TMO: c_uint = 0x17;
pub const HW_EVENT_PORT_RECOVER: c_uint = 0x18;
pub const HW_EVENT_PORT_RESET_TIMER_TMO: c_uint = 0x19;
pub const HW_EVENT_PORT_RESET_COMPLETE: c_uint = 0x20;
pub const EVENT_BROADCAST_ASYNCH_EVENT: c_uint = 0x21;
// port state
pub const PORT_NOT_ESTABLISHED: c_uint = 0x00;
pub const PORT_VALID: c_uint = 0x01;
pub const PORT_LOSTCOMM: c_uint = 0x02;
pub const PORT_IN_RESET: c_uint = 0x04;
pub const PORT_3RD_PARTY_RESET: c_uint = 0x07;
pub const PORT_INVALID: c_uint = 0x08;
//
// SSP/SMP/SATA IO Completion Status values
//
pub const IO_SUCCESS: c_uint = 0x00;
pub const IO_ABORTED: c_uint = 0x01;
pub const IO_OVERFLOW: c_uint = 0x02;
pub const IO_UNDERFLOW: c_uint = 0x03;
pub const IO_FAILED: c_uint = 0x04;
pub const IO_ABORT_RESET: c_uint = 0x05;
pub const IO_NOT_VALID: c_uint = 0x06;
pub const IO_NO_DEVICE: c_uint = 0x07;
pub const IO_ILLEGAL_PARAMETER: c_uint = 0x08;
pub const IO_LINK_FAILURE: c_uint = 0x09;
pub const IO_PROG_ERROR: c_uint = 0x0A;
pub const IO_EDC_IN_ERROR: c_uint = 0x0B;
pub const IO_EDC_OUT_ERROR: c_uint = 0x0C;
pub const IO_ERROR_HW_TIMEOUT: c_uint = 0x0D;
pub const IO_XFER_ERROR_BREAK: c_uint = 0x0E;
pub const IO_XFER_ERROR_PHY_NOT_READY: c_uint = 0x0F;
pub const IO_OPEN_CNX_ERROR_PROTOCOL_NOT_SUPPORTED: c_uint = 0x10;
pub const IO_OPEN_CNX_ERROR_ZONE_VIOLATION: c_uint = 0x11;
pub const IO_OPEN_CNX_ERROR_BREAK: c_uint = 0x12;
pub const IO_OPEN_CNX_ERROR_IT_NEXUS_LOSS: c_uint = 0x13;
pub const IO_OPEN_CNX_ERROR_BAD_DESTINATION: c_uint = 0x14;
pub const IO_OPEN_CNX_ERROR_CONNECTION_RATE_NOT_SUPPORTED: c_uint = 0x15;
pub const IO_OPEN_CNX_ERROR_STP_RESOURCES_BUSY: c_uint = 0x16;
pub const IO_OPEN_CNX_ERROR_WRONG_DESTINATION: c_uint = 0x17;
// This error code 0x18 is not used on SPCv
pub const IO_OPEN_CNX_ERROR_UNKNOWN_ERROR: c_uint = 0x18;
pub const IO_XFER_ERROR_NAK_RECEIVED: c_uint = 0x19;
pub const IO_XFER_ERROR_ACK_NAK_TIMEOUT: c_uint = 0x1A;
pub const IO_XFER_ERROR_PEER_ABORTED: c_uint = 0x1B;
pub const IO_XFER_ERROR_RX_FRAME: c_uint = 0x1C;
pub const IO_XFER_ERROR_DMA: c_uint = 0x1D;
pub const IO_XFER_ERROR_CREDIT_TIMEOUT: c_uint = 0x1E;
pub const IO_XFER_ERROR_SATA_LINK_TIMEOUT: c_uint = 0x1F;
pub const IO_XFER_ERROR_SATA: c_uint = 0x20;
// This error code 0x22 is not used on SPCv
pub const IO_XFER_ERROR_ABORTED_DUE_TO_SRST: c_uint = 0x22;
pub const IO_XFER_ERROR_REJECTED_NCQ_MODE: c_uint = 0x21;
pub const IO_XFER_ERROR_ABORTED_NCQ_MODE: c_uint = 0x23;
pub const IO_XFER_OPEN_RETRY_TIMEOUT: c_uint = 0x24;
// This error code 0x25 is not used on SPCv
pub const IO_XFER_SMP_RESP_CONNECTION_ERROR: c_uint = 0x25;
pub const IO_XFER_ERROR_UNEXPECTED_PHASE: c_uint = 0x26;
pub const IO_XFER_ERROR_XFER_RDY_OVERRUN: c_uint = 0x27;
pub const IO_XFER_ERROR_XFER_RDY_NOT_EXPECTED: c_uint = 0x28;
pub const IO_XFER_ERROR_CMD_ISSUE_ACK_NAK_TIMEOUT: c_uint = 0x30;
// The following error code 0x31 and 0x32 are not using (obsolete)
pub const IO_XFER_ERROR_CMD_ISSUE_BREAK_BEFORE_ACK_NAK: c_uint = 0x31;
pub const IO_XFER_ERROR_CMD_ISSUE_PHY_DOWN_BEFORE_ACK_NAK: c_uint = 0x32;
pub const IO_XFER_ERROR_OFFSET_MISMATCH: c_uint = 0x34;
pub const IO_XFER_ERROR_XFER_ZERO_DATA_LEN: c_uint = 0x35;
pub const IO_XFER_CMD_FRAME_ISSUED: c_uint = 0x36;
pub const IO_ERROR_INTERNAL_SMP_RESOURCE: c_uint = 0x37;
pub const IO_PORT_IN_RESET: c_uint = 0x38;
pub const IO_DS_NON_OPERATIONAL: c_uint = 0x39;
pub const IO_DS_IN_RECOVERY: c_uint = 0x3A;
pub const IO_TM_TAG_NOT_FOUND: c_uint = 0x3B;
pub const IO_XFER_PIO_SETUP_ERROR: c_uint = 0x3C;
pub const IO_SSP_EXT_IU_ZERO_LEN_ERROR: c_uint = 0x3D;
pub const IO_DS_IN_ERROR: c_uint = 0x3E;
pub const IO_OPEN_CNX_ERROR_HW_RESOURCE_BUSY: c_uint = 0x3F;
pub const IO_ABORT_IN_PROGRESS: c_uint = 0x40;
pub const IO_ABORT_DELAYED: c_uint = 0x41;
pub const IO_INVALID_LENGTH: c_uint = 0x42;
// additional response event values
pub const IO_OPEN_CNX_ERROR_HW_RESOURCE_BUSY_ALT: c_uint = 0x43;
pub const IO_XFER_OPEN_RETRY_BACKOFF_THRESHOLD_REACHED: c_uint = 0x44;
pub const IO_OPEN_CNX_ERROR_IT_NEXUS_LOSS_OPEN_TMO: c_uint = 0x45;
pub const IO_OPEN_CNX_ERROR_IT_NEXUS_LOSS_NO_DEST: c_uint = 0x46;
pub const IO_OPEN_CNX_ERROR_IT_NEXUS_LOSS_OPEN_COLLIDE: c_uint = 0x47;
pub const IO_OPEN_CNX_ERROR_IT_NEXUS_LOSS_PATHWAY_BLOCKED: c_uint = 0x48;
pub const IO_DS_INVALID: c_uint = 0x49;
pub const IO_FATAL_ERROR: c_uint = 0x51;
// WARNING: the value is not contiguous from here
pub const IO_XFER_ERR_LAST_PIO_DATAIN_CRC_ERR: c_uint = 0x52;
pub const IO_XFER_DMA_ACTIVATE_TIMEOUT: c_uint = 0x53;
pub const IO_XFER_ERROR_INTERNAL_CRC_ERROR: c_uint = 0x54;
pub const MPI_IO_RQE_BUSY_FULL: c_uint = 0x55;
pub const IO_XFER_ERR_EOB_DATA_OVERRUN: c_uint = 0x56;
pub const IO_XFER_ERROR_INVALID_SSP_RSP_FRAME: c_uint = 0x57;
pub const IO_OPEN_CNX_ERROR_OPEN_PREEMPTED: c_uint = 0x58;
pub const MPI_ERR_IO_RESOURCE_UNAVAILABLE: c_uint = 0x1004;
pub const MPI_ERR_ATAPI_DEVICE_BUSY: c_uint = 0x1024;
pub const IO_XFR_ERROR_DEK_KEY_CACHE_MISS: c_uint = 0x2040;
//
// An encryption IO request failed due to DEK Key Tag mismatch.
// The key tag supplied in the encryption IOMB does not match with
// the Key Tag in the referenced DEK Entry.
//
pub const IO_XFR_ERROR_DEK_KEY_TAG_MISMATCH: c_uint = 0x2041;
pub const IO_XFR_ERROR_CIPHER_MODE_INVALID: c_uint = 0x2042;
//
// An encryption I/O request failed because the initial value (IV)
// in the unwrapped DEK blob didn't match the IV used to unwrap it.
//
pub const IO_XFR_ERROR_DEK_IV_MISMATCH: c_uint = 0x2043;
// An encryption I/O request failed due to an internal RAM ECC or
// interface error while unwrapping the DEK.
pub const IO_XFR_ERROR_DEK_RAM_INTERFACE_ERROR: c_uint = 0x2044;
// An encryption I/O request failed due to an internal RAM ECC or
// interface error while unwrapping the DEK.
pub const IO_XFR_ERROR_INTERNAL_RAM: c_uint = 0x2045;
//
// An encryption I/O request failed
// because the DEK index specified in the I/O was outside the bounds of
// the total number of entries in the host DEK table.
//
// Macro flag: #define IO_XFR_ERROR_DEK_INDEX_OUT_OF_BOUNDS0x2046
// define DIF IO response error status code
pub const IO_XFR_ERROR_DIF_MISMATCH: c_uint = 0x3000;
pub const IO_XFR_ERROR_DIF_APPLICATION_TAG_MISMATCH: c_uint = 0x3001;
pub const IO_XFR_ERROR_DIF_REFERENCE_TAG_MISMATCH: c_uint = 0x3002;
pub const IO_XFR_ERROR_DIF_CRC_MISMATCH: c_uint = 0x3003;
// define operator management response status and error qualifier code
pub const OPR_MGMT_OP_NOT_SUPPORTED: c_uint = 0x2060;
pub const OPR_MGMT_MPI_ENC_ERR_OPR_PARAM_ILLEGAL: c_uint = 0x2061;
pub const OPR_MGMT_MPI_ENC_ERR_OPR_ID_NOT_FOUND: c_uint = 0x2062;
pub const OPR_MGMT_MPI_ENC_ERR_OPR_ROLE_NOT_MATCH: c_uint = 0x2063;
pub const OPR_MGMT_MPI_ENC_ERR_OPR_MAX_NUM_EXCEEDED: c_uint = 0x2064;
pub const OPR_MGMT_MPI_ENC_ERR_KEK_UNWRAP_FAIL: c_uint = 0x2022;
pub const OPR_MGMT_MPI_ENC_ERR_NVRAM_OPERATION_FAILURE: c_uint = 0x2023;
// additional response event values
// WARNING: This error code must always be the last number.
// If you add error code, modify this code also
// It is used as an index
//
pub const IO_ERROR_UNKNOWN_GENERIC: c_uint = 0x2023;
// MSGU CONFIGURATION TABLE
pub const SPCv_MSGU_CFG_TABLE_UPDATE: c_uint = 0x001;
pub const SPCv_MSGU_CFG_TABLE_RESET: c_uint = 0x002;
pub const SPCv_MSGU_CFG_TABLE_FREEZE: c_uint = 0x004;
pub const SPCv_MSGU_CFG_TABLE_UNFREEZE: c_uint = 0x008;
pub const MSGU_IBDB_SET: c_uint = 0x00;
pub const MSGU_HOST_INT_STATUS: c_uint = 0x08;
pub const MSGU_HOST_INT_MASK: c_uint = 0x0C;
pub const MSGU_IOPIB_INT_STATUS: c_uint = 0x18;
pub const MSGU_IOPIB_INT_MASK: c_uint = 0x1C;
pub const MSGU_IBDB_CLEAR: c_uint = 0x20;
pub const MSGU_MSGU_CONTROL: c_uint = 0x24;
pub const MSGU_ODR: c_uint = 0x20;
pub const MSGU_ODCR: c_uint = 0x28;
pub const MSGU_ODMR: c_uint = 0x30;
pub const MSGU_ODMR_U: c_uint = 0x34;
pub const MSGU_ODMR_CLR: c_uint = 0x38;
pub const MSGU_ODMR_CLR_U: c_uint = 0x3C;
pub const MSGU_OD_RSVD: c_uint = 0x40;
pub const MSGU_SCRATCH_PAD_0: c_uint = 0x44;
pub const MSGU_SCRATCH_PAD_1: c_uint = 0x48;
pub const MSGU_SCRATCH_PAD_2: c_uint = 0x4C;
pub const MSGU_SCRATCH_PAD_3: c_uint = 0x50;
pub const MSGU_HOST_SCRATCH_PAD_0: c_uint = 0x54;
pub const MSGU_HOST_SCRATCH_PAD_1: c_uint = 0x58;
pub const MSGU_HOST_SCRATCH_PAD_2: c_uint = 0x5C;
pub const MSGU_HOST_SCRATCH_PAD_3: c_uint = 0x60;
pub const MSGU_HOST_SCRATCH_PAD_4: c_uint = 0x64;
pub const MSGU_HOST_SCRATCH_PAD_5: c_uint = 0x68;
pub const MSGU_SCRATCH_PAD_RSVD_0: c_uint = 0x6C;
pub const MSGU_SCRATCH_PAD_RSVD_1: c_uint = 0x70;

// bit definition for ODMR register
pub const ODMR_MASK_ALL: c_uint = 0xFFFFFFFF/* mask all;

// bit definition for ODCR register
pub const ODCR_CLEAR_ALL: c_uint = 0xFFFFFFFF /* mask all;
// MSIX Interupts
pub const MSIX_TABLE_OFFSET: c_uint = 0x2000;
pub const MSIX_TABLE_ELEMENT_SIZE: c_uint = 0x10;
pub const MSIX_INTERRUPT_CONTROL_OFFSET: c_uint = 0xC;

pub const MSIX_INTERRUPT_DISABLE: c_uint = 0x1;
pub const MSIX_INTERRUPT_ENABLE: c_uint = 0x0;
// state definition for Scratch Pad1 register
pub const SCRATCH_PAD_RAAE_READY: c_uint = 0x3;
pub const SCRATCH_PAD_ILA_READY: c_uint = 0xC;
pub const SCRATCH_PAD_BOOT_LOAD_SUCCESS: c_uint = 0x0;
pub const SCRATCH_PAD_IOP0_READY: c_uint = 0xC00;
pub const SCRATCH_PAD_IOP1_READY: c_uint = 0x3000;

// boot loader state
pub const SCRATCH_PAD1_BOOTSTATE_MASK: c_uint = 0x70	/* Bit 4-6 */;
pub const SCRATCH_PAD1_BOOTSTATE_SUCESS: c_uint = 0x0	/* Load successful */;
pub const SCRATCH_PAD1_BOOTSTATE_HDA_SEEPROM: c_uint = 0x10	/* HDA SEEPROM */;
pub const SCRATCH_PAD1_BOOTSTATE_HDA_BOOTSTRAP: c_uint = 0x20	/* HDA BootStrap Pins */;
pub const SCRATCH_PAD1_BOOTSTATE_HDA_SOFTRESET: c_uint = 0x30	/* HDA Soft Reset */;
pub const SCRATCH_PAD1_BOOTSTATE_CRIT_ERROR: c_uint = 0x40	/* HDA critical error */;
pub const SCRATCH_PAD1_BOOTSTATE_R1: c_uint = 0x50	/* Reserved */;
pub const SCRATCH_PAD1_BOOTSTATE_R2: c_uint = 0x60	/* Reserved */;
pub const SCRATCH_PAD1_BOOTSTATE_FATAL: c_uint = 0x70	/* Fatal Error */;
// state definition for Scratch Pad2 register
pub const SCRATCH_PAD2_POR: c_uint = 0x00	/* power on state */;
pub const SCRATCH_PAD2_SFR: c_uint = 0x01	/* soft reset state */;
pub const SCRATCH_PAD2_ERR: c_uint = 0x02	/* error state */;
pub const SCRATCH_PAD2_RDY: c_uint = 0x03	/* ready state */;
pub const SCRATCH_PAD2_FWRDY_RST: c_uint = 0x04	/* FW rdy for soft reset flag */;
pub const SCRATCH_PAD2_IOPRDY_RST: c_uint = 0x08	/* IOP ready for soft reset */;
pub const SCRATCH_PAD2_STATE_MASK: c_uint = 0xFFFFFFF4 /* ScratchPad 2;
pub const SCRATCH_PAD2_RESERVED: c_uint = 0x000003FC/* Scratch Pad1;
pub const SCRATCH_PAD_ERROR_MASK: c_uint = 0xFFFFFC00 /* Error mask bits */;
pub const SCRATCH_PAD_STATE_MASK: c_uint = 0x00000003 /* State Mask bits */;
// state definition for Scratchpad Rsvd 0, Offset 0x6C, Non-fatal
pub const NON_FATAL_SPBC_LBUS_ECC_ERR: c_uint = 0x70000001;
pub const NON_FATAL_BDMA_ERR: c_uint = 0xE0000001;
pub const NON_FATAL_THERM_OVERTEMP_ERR: c_uint = 0x80000001;
// main configuration offset - byte offset
pub const MAIN_SIGNATURE_OFFSET: c_uint = 0x00 /* DWORD 0x00 */;
pub const MAIN_INTERFACE_REVISION: c_uint = 0x04 /* DWORD 0x01 */;
pub const MAIN_FW_REVISION: c_uint = 0x08 /* DWORD 0x02 */;
pub const MAIN_MAX_OUTSTANDING_IO_OFFSET: c_uint = 0x0C /* DWORD 0x03 */;
pub const MAIN_MAX_SGL_OFFSET: c_uint = 0x10 /* DWORD 0x04 */;
pub const MAIN_CNTRL_CAP_OFFSET: c_uint = 0x14 /* DWORD 0x05 */;
pub const MAIN_GST_OFFSET: c_uint = 0x18 /* DWORD 0x06 */;
pub const MAIN_IBQ_OFFSET: c_uint = 0x1C /* DWORD 0x07 */;
pub const MAIN_OBQ_OFFSET: c_uint = 0x20 /* DWORD 0x08 */;
pub const MAIN_IQNPPD_HPPD_OFFSET: c_uint = 0x24 /* DWORD 0x09 */;
// 0x28 - 0x4C - RSVD
pub const MAIN_EVENT_CRC_CHECK: c_uint = 0x48 /* DWORD 0x12 */;
pub const MAIN_EVENT_LOG_ADDR_HI: c_uint = 0x50 /* DWORD 0x14 */;
pub const MAIN_EVENT_LOG_ADDR_LO: c_uint = 0x54 /* DWORD 0x15 */;
pub const MAIN_EVENT_LOG_BUFF_SIZE: c_uint = 0x58 /* DWORD 0x16 */;
pub const MAIN_EVENT_LOG_OPTION: c_uint = 0x5C /* DWORD 0x17 */;
pub const MAIN_PCS_EVENT_LOG_ADDR_HI: c_uint = 0x60 /* DWORD 0x18 */;
pub const MAIN_PCS_EVENT_LOG_ADDR_LO: c_uint = 0x64 /* DWORD 0x19 */;
pub const MAIN_PCS_EVENT_LOG_BUFF_SIZE: c_uint = 0x68 /* DWORD 0x1A */;
pub const MAIN_PCS_EVENT_LOG_OPTION: c_uint = 0x6C /* DWORD 0x1B */;
pub const MAIN_FATAL_ERROR_INTERRUPT: c_uint = 0x70 /* DWORD 0x1C */;
pub const MAIN_FATAL_ERROR_RDUMP0_OFFSET: c_uint = 0x74 /* DWORD 0x1D */;
pub const MAIN_FATAL_ERROR_RDUMP0_LENGTH: c_uint = 0x78 /* DWORD 0x1E */;
pub const MAIN_FATAL_ERROR_RDUMP1_OFFSET: c_uint = 0x7C /* DWORD 0x1F */;
pub const MAIN_FATAL_ERROR_RDUMP1_LENGTH: c_uint = 0x80 /* DWORD 0x20 */;
pub const MAIN_GPIO_LED_FLAGS_OFFSET: c_uint = 0x84 /* DWORD 0x21 */;
pub const MAIN_ANALOG_SETUP_OFFSET: c_uint = 0x88 /* DWORD 0x22 */;
pub const MAIN_INT_VECTOR_TABLE_OFFSET: c_uint = 0x8C /* DWORD 0x23 */;
pub const MAIN_SAS_PHY_ATTR_TABLE_OFFSET: c_uint = 0x90 /* DWORD 0x24 */;
pub const MAIN_PORT_RECOVERY_TIMER: c_uint = 0x94 /* DWORD 0x25 */;
pub const MAIN_INT_REASSERTION_DELAY: c_uint = 0x98 /* DWORD 0x26 */;
pub const MAIN_MPI_ILA_RELEASE_TYPE: c_uint = 0xA4 /* DWORD 0x29 */;

// Gereral Status Table offset - byte offset
pub const GST_GSTLEN_MPIS_OFFSET: c_uint = 0x00;
pub const GST_IQ_FREEZE_STATE0_OFFSET: c_uint = 0x04;
pub const GST_IQ_FREEZE_STATE1_OFFSET: c_uint = 0x08;
pub const GST_MSGUTCNT_OFFSET: c_uint = 0x0C;
pub const GST_IOPTCNT_OFFSET: c_uint = 0x10;
// 0x14 - 0x34 - RSVD
pub const GST_GPIO_INPUT_VAL: c_uint = 0x38;
// 0x3c - 0x40 - RSVD
pub const GST_RERRINFO_OFFSET0: c_uint = 0x44;
pub const GST_RERRINFO_OFFSET1: c_uint = 0x48;
pub const GST_RERRINFO_OFFSET2: c_uint = 0x4c;
pub const GST_RERRINFO_OFFSET3: c_uint = 0x50;
pub const GST_RERRINFO_OFFSET4: c_uint = 0x54;
pub const GST_RERRINFO_OFFSET5: c_uint = 0x58;
pub const GST_RERRINFO_OFFSET6: c_uint = 0x5c;
pub const GST_RERRINFO_OFFSET7: c_uint = 0x60;
// General Status Table - MPI state
pub const GST_MPI_STATE_UNINIT: c_uint = 0x00;
pub const GST_MPI_STATE_INIT: c_uint = 0x01;
pub const GST_MPI_STATE_TERMINATION: c_uint = 0x02;
pub const GST_MPI_STATE_ERROR: c_uint = 0x03;
pub const GST_MPI_STATE_MASK: c_uint = 0x07;
// Per SAS PHY Attributes
pub const PSPA_PHYSTATE0_OFFSET: c_uint = 0x00 /* Dword V */;
pub const PSPA_OB_HW_EVENT_PID0_OFFSET: c_uint = 0x04 /* DWORD V+1 */;
pub const PSPA_PHYSTATE1_OFFSET: c_uint = 0x08 /* Dword V+2 */;
pub const PSPA_OB_HW_EVENT_PID1_OFFSET: c_uint = 0x0C /* DWORD V+3 */;
pub const PSPA_PHYSTATE2_OFFSET: c_uint = 0x10 /* Dword V+4 */;
pub const PSPA_OB_HW_EVENT_PID2_OFFSET: c_uint = 0x14 /* DWORD V+5 */;
pub const PSPA_PHYSTATE3_OFFSET: c_uint = 0x18 /* Dword V+6 */;
pub const PSPA_OB_HW_EVENT_PID3_OFFSET: c_uint = 0x1C /* DWORD V+7 */;
pub const PSPA_PHYSTATE4_OFFSET: c_uint = 0x20 /* Dword V+8 */;
pub const PSPA_OB_HW_EVENT_PID4_OFFSET: c_uint = 0x24 /* DWORD V+9 */;
pub const PSPA_PHYSTATE5_OFFSET: c_uint = 0x28 /* Dword V+10 */;
pub const PSPA_OB_HW_EVENT_PID5_OFFSET: c_uint = 0x2C /* DWORD V+11 */;
pub const PSPA_PHYSTATE6_OFFSET: c_uint = 0x30 /* Dword V+12 */;
pub const PSPA_OB_HW_EVENT_PID6_OFFSET: c_uint = 0x34 /* DWORD V+13 */;
pub const PSPA_PHYSTATE7_OFFSET: c_uint = 0x38 /* Dword V+14 */;
pub const PSPA_OB_HW_EVENT_PID7_OFFSET: c_uint = 0x3C /* DWORD V+15 */;
pub const PSPA_PHYSTATE8_OFFSET: c_uint = 0x40 /* DWORD V+16 */;
pub const PSPA_OB_HW_EVENT_PID8_OFFSET: c_uint = 0x44 /* DWORD V+17 */;
pub const PSPA_PHYSTATE9_OFFSET: c_uint = 0x48 /* DWORD V+18 */;
pub const PSPA_OB_HW_EVENT_PID9_OFFSET: c_uint = 0x4C /* DWORD V+19 */;
pub const PSPA_PHYSTATE10_OFFSET: c_uint = 0x50 /* DWORD V+20 */;
pub const PSPA_OB_HW_EVENT_PID10_OFFSET: c_uint = 0x54 /* DWORD V+21 */;
pub const PSPA_PHYSTATE11_OFFSET: c_uint = 0x58 /* DWORD V+22 */;
pub const PSPA_OB_HW_EVENT_PID11_OFFSET: c_uint = 0x5C /* DWORD V+23 */;
pub const PSPA_PHYSTATE12_OFFSET: c_uint = 0x60 /* DWORD V+24 */;
pub const PSPA_OB_HW_EVENT_PID12_OFFSET: c_uint = 0x64 /* DWORD V+25 */;
pub const PSPA_PHYSTATE13_OFFSET: c_uint = 0x68 /* DWORD V+26 */;
pub const PSPA_OB_HW_EVENT_PID13_OFFSET: c_uint = 0x6c /* DWORD V+27 */;
pub const PSPA_PHYSTATE14_OFFSET: c_uint = 0x70 /* DWORD V+28 */;
pub const PSPA_OB_HW_EVENT_PID14_OFFSET: c_uint = 0x74 /* DWORD V+29 */;
pub const PSPA_PHYSTATE15_OFFSET: c_uint = 0x78 /* DWORD V+30 */;
pub const PSPA_OB_HW_EVENT_PID15_OFFSET: c_uint = 0x7c /* DWORD V+31 */;
// end PSPA
// inbound queue configuration offset - byte offset
pub const IB_PROPERITY_OFFSET: c_uint = 0x00;
pub const IB_BASE_ADDR_HI_OFFSET: c_uint = 0x04;
pub const IB_BASE_ADDR_LO_OFFSET: c_uint = 0x08;
pub const IB_CI_BASE_ADDR_HI_OFFSET: c_uint = 0x0C;
pub const IB_CI_BASE_ADDR_LO_OFFSET: c_uint = 0x10;
pub const IB_PIPCI_BAR: c_uint = 0x14;
pub const IB_PIPCI_BAR_OFFSET: c_uint = 0x18;
pub const IB_RESERVED_OFFSET: c_uint = 0x1C;
// outbound queue configuration offset - byte offset
pub const OB_PROPERITY_OFFSET: c_uint = 0x00;
pub const OB_BASE_ADDR_HI_OFFSET: c_uint = 0x04;
pub const OB_BASE_ADDR_LO_OFFSET: c_uint = 0x08;
pub const OB_PI_BASE_ADDR_HI_OFFSET: c_uint = 0x0C;
pub const OB_PI_BASE_ADDR_LO_OFFSET: c_uint = 0x10;
pub const OB_CIPCI_BAR: c_uint = 0x14;
pub const OB_CIPCI_BAR_OFFSET: c_uint = 0x18;
pub const OB_INTERRUPT_COALES_OFFSET: c_uint = 0x1C;
pub const OB_DYNAMIC_COALES_OFFSET: c_uint = 0x20;
pub const OB_PROPERTY_INT_ENABLE: c_uint = 0x40000000;
pub const MBIC_NMI_ENABLE_VPE0_IOP: c_uint = 0x000418;
pub const MBIC_NMI_ENABLE_VPE0_AAP1: c_uint = 0x000418;
// PCIE registers - BAR2(0x18), BAR1(win) 0x010000
pub const PCIE_EVENT_INTERRUPT_ENABLE: c_uint = 0x003040;
pub const PCIE_EVENT_INTERRUPT: c_uint = 0x003044;
pub const PCIE_ERROR_INTERRUPT_ENABLE: c_uint = 0x003048;
pub const PCIE_ERROR_INTERRUPT: c_uint = 0x00304C;
// SPCV soft reset
pub const SPC_REG_SOFT_RESET: c_uint = 0x00001000;
pub const SPCv_NORMAL_RESET_VALUE: c_uint = 0x1;
pub const SPCv_SOFT_RESET_READ_MASK: c_uint = 0xC0;
pub const SPCv_SOFT_RESET_NO_RESET: c_uint = 0x0;
pub const SPCv_SOFT_RESET_NORMAL_RESET_OCCURED: c_uint = 0x40;
pub const SPCv_SOFT_RESET_HDA_MODE_OCCURED: c_uint = 0x80;
pub const SPCv_SOFT_RESET_CHIP_RESET_OCCURED: c_uint = 0xC0;
// signature definition for host scratch pad0 register
pub const SPC_SOFT_RESET_SIGNATURE: c_uint = 0x252acbcd;
// Signature for Soft Reset
// SPC Reset register - BAR4(0x20), BAR2(win) (need dynamic mapping)
pub const SPC_REG_RESET: c_uint = 0x000000/* reset register */;
// bit definition for SPC_RESET register
pub const SPC_REG_RESET_OSSP: c_uint = 0x00000001;
pub const SPC_REG_RESET_RAAE: c_uint = 0x00000002;
pub const SPC_REG_RESET_PCS_SPBC: c_uint = 0x00000004;
pub const SPC_REG_RESET_PCS_IOP_SS: c_uint = 0x00000008;
pub const SPC_REG_RESET_PCS_AAP1_SS: c_uint = 0x00000010;
pub const SPC_REG_RESET_PCS_AAP2_SS: c_uint = 0x00000020;
pub const SPC_REG_RESET_PCS_LM: c_uint = 0x00000040;
pub const SPC_REG_RESET_PCS: c_uint = 0x00000080;
pub const SPC_REG_RESET_GSM: c_uint = 0x00000100;
pub const SPC_REG_RESET_DDR2: c_uint = 0x00010000;
pub const SPC_REG_RESET_BDMA_CORE: c_uint = 0x00020000;
pub const SPC_REG_RESET_BDMA_SXCBI: c_uint = 0x00040000;
pub const SPC_REG_RESET_PCIE_AL_SXCBI: c_uint = 0x00080000;
pub const SPC_REG_RESET_PCIE_PWR: c_uint = 0x00100000;
pub const SPC_REG_RESET_PCIE_SFT: c_uint = 0x00200000;
pub const SPC_REG_RESET_PCS_SXCBI: c_uint = 0x00400000;
pub const SPC_REG_RESET_LMS_SXCBI: c_uint = 0x00800000;
pub const SPC_REG_RESET_PMIC_SXCBI: c_uint = 0x01000000;
pub const SPC_REG_RESET_PMIC_CORE: c_uint = 0x02000000;
pub const SPC_REG_RESET_PCIE_PC_SXCBI: c_uint = 0x04000000;
pub const SPC_REG_RESET_DEVICE: c_uint = 0x80000000;
// registers for BAR Shifting - BAR2(0x18), BAR1(win)
pub const SPCV_IBW_AXI_TRANSLATION_LOW: c_uint = 0x001010;
pub const MBIC_AAP1_ADDR_BASE: c_uint = 0x060000;
pub const MBIC_IOP_ADDR_BASE: c_uint = 0x070000;
pub const GSM_ADDR_BASE: c_uint = 0x0700000;
// Dynamic map through Bar4 - 0x00700000
pub const GSM_CONFIG_RESET: c_uint = 0x00000000;
pub const RAM_ECC_DB_ERR: c_uint = 0x00000018;
pub const GSM_READ_ADDR_PARITY_INDIC: c_uint = 0x00000058;
pub const GSM_WRITE_ADDR_PARITY_INDIC: c_uint = 0x00000060;
pub const GSM_WRITE_DATA_PARITY_INDIC: c_uint = 0x00000068;
pub const GSM_READ_ADDR_PARITY_CHECK: c_uint = 0x00000038;
pub const GSM_WRITE_ADDR_PARITY_CHECK: c_uint = 0x00000040;
pub const GSM_WRITE_DATA_PARITY_CHECK: c_uint = 0x00000048;
pub const RB6_ACCESS_REG: c_uint = 0x6A0000;
pub const HDAC_EXEC_CMD: c_uint = 0x0002;
pub const HDA_C_PA: c_uint = 0xcb;
pub const HDA_SEQ_ID_BITS: c_uint = 0x00ff0000;
pub const HDA_GSM_OFFSET_BITS: c_uint = 0x00FFFFFF;
pub const HDA_GSM_CMD_OFFSET_BITS: c_uint = 0x42C0;
pub const HDA_GSM_RSP_OFFSET_BITS: c_uint = 0x42E0;
pub const MBIC_AAP1_ADDR_BASE: c_uint = 0x060000;
pub const MBIC_IOP_ADDR_BASE: c_uint = 0x070000;
pub const GSM_ADDR_BASE: c_uint = 0x0700000;
pub const SPC_TOP_LEVEL_ADDR_BASE: c_uint = 0x000000;
pub const GSM_CONFIG_RESET_VALUE: c_uint = 0x00003b00;
pub const GPIO_ADDR_BASE: c_uint = 0x00090000;
pub const GPIO_GPIO_0_0UTPUT_CTL_OFFSET: c_uint = 0x0000010c;
// RB6 offset
pub const SPC_RB6_OFFSET: c_uint = 0x80C0;
// Magic number of soft reset for RB6
pub const RB6_MAGIC_NUMBER_RST: c_uint = 0x1234;
// Device Register status
pub const DEVREG_SUCCESS: c_uint = 0x00;
pub const DEVREG_FAILURE_OUT_OF_RESOURCE: c_uint = 0x01;
pub const DEVREG_FAILURE_DEVICE_ALREADY_REGISTERED: c_uint = 0x02;
pub const DEVREG_FAILURE_INVALID_PHY_ID: c_uint = 0x03;
pub const DEVREG_FAILURE_PHY_ID_ALREADY_REGISTERED: c_uint = 0x04;
pub const DEVREG_FAILURE_PORT_ID_OUT_OF_RANGE: c_uint = 0x05;
pub const DEVREG_FAILURE_PORT_NOT_VALID_STATE: c_uint = 0x06;
pub const DEVREG_FAILURE_DEVICE_TYPE_NOT_VALID: c_uint = 0x07;
pub const MEMBASE_II_SHIFT_REGISTER: c_uint = 0x1010;

//
// As we know sleep (1~20) ms may result in sleep longer than ~20 ms, hence we
// choose 20 ms interval.
//
pub const FW_READY_INTERVAL: c_int = 20;
