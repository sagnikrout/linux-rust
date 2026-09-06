//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/sas.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// SAS structures and definitions header file
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//

pub const SAS_ADDR_SIZE: c_int = 8;
pub const HASHED_SAS_ADDR_SIZE: c_int = 3;

pub const SMP_REQUEST: c_uint = 0x40;
pub const SMP_RESPONSE: c_uint = 0x41;
pub const SSP_DATA: c_uint = 0x01;
pub const SSP_XFER_RDY: c_uint = 0x05;
pub const SSP_COMMAND: c_uint = 0x06;
pub const SSP_RESPONSE: c_uint = 0x07;
pub const SSP_TASK: c_uint = 0x16;
pub const SMP_REPORT_GENERAL: c_uint = 0x00;
pub const SMP_REPORT_MANUF_INFO: c_uint = 0x01;
pub const SMP_READ_GPIO_REG: c_uint = 0x02;
pub const SMP_DISCOVER: c_uint = 0x10;
pub const SMP_REPORT_PHY_ERR_LOG: c_uint = 0x11;
pub const SMP_REPORT_PHY_SATA: c_uint = 0x12;
pub const SMP_REPORT_ROUTE_INFO: c_uint = 0x13;
pub const SMP_WRITE_GPIO_REG: c_uint = 0x82;
pub const SMP_CONF_ROUTE_INFO: c_uint = 0x90;
pub const SMP_PHY_CONTROL: c_uint = 0x91;
pub const SMP_PHY_TEST_FUNCTION: c_uint = 0x92;
pub const SMP_RESP_FUNC_ACC: c_uint = 0x00;
pub const SMP_RESP_FUNC_UNK: c_uint = 0x01;
pub const SMP_RESP_FUNC_FAILED: c_uint = 0x02;
pub const SMP_RESP_INV_FRM_LEN: c_uint = 0x03;
pub const SMP_RESP_NO_PHY: c_uint = 0x10;
pub const SMP_RESP_NO_INDEX: c_uint = 0x11;
pub const SMP_RESP_PHY_NO_SATA: c_uint = 0x12;
pub const SMP_RESP_PHY_UNK_OP: c_uint = 0x13;
pub const SMP_RESP_PHY_UNK_TESTF: c_uint = 0x14;
pub const SMP_RESP_PHY_TEST_INPROG: c_uint = 0x15;
pub const SMP_RESP_PHY_VACANT: c_uint = 0x16;
// SAM TMFs
pub const TMF_ABORT_TASK: c_uint = 0x01;
pub const TMF_ABORT_TASK_SET: c_uint = 0x02;
pub const TMF_CLEAR_TASK_SET: c_uint = 0x04;
pub const TMF_LU_RESET: c_uint = 0x08;
pub const TMF_CLEAR_ACA: c_uint = 0x40;
pub const TMF_QUERY_TASK: c_uint = 0x80;
// SAS TMF responses
pub const TMF_RESP_FUNC_COMPLETE: c_uint = 0x00;
pub const TMF_RESP_INVALID_FRAME: c_uint = 0x02;
pub const TMF_RESP_FUNC_ESUPP: c_uint = 0x04;
pub const TMF_RESP_FUNC_FAILED: c_uint = 0x05;
pub const TMF_RESP_FUNC_SUCC: c_uint = 0x08;
pub const TMF_RESP_NO_LUN: c_uint = 0x09;
pub const TMF_RESP_OVERLAPPED_TAG: c_uint = 0x0A;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sas_oob_mode {
    OOB_NOT_CONNECTED,
    SATA_OOB_MODE,
    SAS_OOB_MODE
}

// See sas_discover.c if you plan on changing these
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sas_device_type {
// these are SAS protocol defined (attached device type field)
    SAS_PHY_UNUSED = 0,
    SAS_END_DEVICE = 1,
    SAS_EDGE_EXPANDER_DEVICE = 2,
    SAS_FANOUT_EXPANDER_DEVICE = 3,
// these are internal to libsas
    SAS_HA = 4,
    SAS_SATA_DEV = 5,
    SAS_SATA_PM = 7,
    SAS_SATA_PM_PORT = 8,
    SAS_SATA_PENDING = 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sas_protocol {
    SAS_PROTOCOL_NONE		= 0,
    SAS_PROTOCOL_SATA		= 0x01,
    SAS_PROTOCOL_SMP		= 0x02,
    SAS_PROTOCOL_STP		= 0x04,
    SAS_PROTOCOL_SSP		= 0x08,
    SAS_PROTOCOL_ALL		= 0x0E,
    SAS_PROTOCOL_STP_ALL		= SAS_PROTOCOL_STP|SAS_PROTOCOL_SATA,
// these are internal to libsas
    SAS_PROTOCOL_INTERNAL_ABORT	= 0x10,
}

// From the spec; local phys only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_func {
    PHY_FUNC_NOP,
    PHY_FUNC_LINK_RESET,		  /* Enables the phy */
    PHY_FUNC_HARD_RESET,
    PHY_FUNC_DISABLE,
    PHY_FUNC_CLEAR_ERROR_LOG = 5,
    PHY_FUNC_CLEAR_AFFIL,
    PHY_FUNC_TX_SATA_PS_SIGNAL,
    PHY_FUNC_RELEASE_SPINUP_HOLD = 0x10, /* LOCAL PORT ONLY! */
    PHY_FUNC_SET_LINK_RATE,
    PHY_FUNC_GET_EVENTS,
}

// SAS LLDD would need to report only _very_few_ of those, like BROADCAST.
// Most of those are here for completeness.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sas_prim {
    SAS_PRIM_AIP_NORMAL = 1,
    SAS_PRIM_AIP_R0     = 2,
    SAS_PRIM_AIP_R1     = 3,
    SAS_PRIM_AIP_R2     = 4,
    SAS_PRIM_AIP_WC     = 5,
    SAS_PRIM_AIP_WD     = 6,
    SAS_PRIM_AIP_WP     = 7,
    SAS_PRIM_AIP_RWP    = 8,

    SAS_PRIM_BC_CH      = 9,
    SAS_PRIM_BC_RCH0    = 10,
    SAS_PRIM_BC_RCH1    = 11,
    SAS_PRIM_BC_R0      = 12,
    SAS_PRIM_BC_R1      = 13,
    SAS_PRIM_BC_R2      = 14,
    SAS_PRIM_BC_R3      = 15,
    SAS_PRIM_BC_R4      = 16,

    SAS_PRIM_NOTIFY_ENSP= 17,
    SAS_PRIM_NOTIFY_R0  = 18,
    SAS_PRIM_NOTIFY_R1  = 19,
    SAS_PRIM_NOTIFY_R2  = 20,

    SAS_PRIM_CLOSE_CLAF = 21,
    SAS_PRIM_CLOSE_NORM = 22,
    SAS_PRIM_CLOSE_R0   = 23,
    SAS_PRIM_CLOSE_R1   = 24,

    SAS_PRIM_OPEN_RTRY  = 25,
    SAS_PRIM_OPEN_RJCT  = 26,
    SAS_PRIM_OPEN_ACPT  = 27,

    SAS_PRIM_DONE       = 28,
    SAS_PRIM_BREAK      = 29,

    SATA_PRIM_DMAT      = 33,
    SATA_PRIM_PMNAK     = 34,
    SATA_PRIM_PMACK     = 35,
    SATA_PRIM_PMREQ_S   = 36,
    SATA_PRIM_PMREQ_P   = 37,
    SATA_SATA_R_ERR     = 38,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sas_open_rej_reason {
// Abandon open
    SAS_OREJ_UNKNOWN   = 0,
    SAS_OREJ_BAD_DEST  = 1,
    SAS_OREJ_CONN_RATE = 2,
    SAS_OREJ_EPROTO    = 3,
    SAS_OREJ_RESV_AB0  = 4,
    SAS_OREJ_RESV_AB1  = 5,
    SAS_OREJ_RESV_AB2  = 6,
    SAS_OREJ_RESV_AB3  = 7,
    SAS_OREJ_WRONG_DEST= 8,
    SAS_OREJ_STP_NORES = 9,

// Retry open
    SAS_OREJ_NO_DEST   = 10,
    SAS_OREJ_PATH_BLOCKED = 11,
    SAS_OREJ_RSVD_CONT0 = 12,
    SAS_OREJ_RSVD_CONT1 = 13,
    SAS_OREJ_RSVD_INIT0 = 14,
    SAS_OREJ_RSVD_INIT1 = 15,
    SAS_OREJ_RSVD_STOP0 = 16,
    SAS_OREJ_RSVD_STOP1 = 17,
    SAS_OREJ_RSVD_RETRY = 18,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sas_gpio_reg_type {
    SAS_GPIO_REG_CFG   = 0,
    SAS_GPIO_REG_RX    = 1,
    SAS_GPIO_REG_RX_GP = 2,
    SAS_GPIO_REG_TX    = 3,
    SAS_GPIO_REG_TX_GP = 4,
}

// Response frame DATAPRES field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_to_host_fis {
    pub /: *mut *mut u8 fis_type; / 0x34,
    pub flags: u8,
    pub status: u8,
    pub error: u8,
    pub lbal: u8,
    pub }: { u8 lbam; u8 byte_count_low;,
    pub }: { u8 lbah; u8 byte_count_high;,
    pub device: u8,
    pub lbal_exp: u8,
    pub lbam_exp: u8,
    pub lbah_exp: u8,
    pub _r_a: u8,
    pub }: { u8 sector_count; u8 interrupt_reason;,
    pub sector_count_exp: u8,
    pub _r_b: u8,
    pub _r_c: u8,
    pub _r_d: u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_to_dev_fis {
    pub /: *mut *mut u8 fis_type; / 0x27,
    pub flags: u8,
    pub command: u8,
    pub features: u8,
    pub lbal: u8,
    pub }: { u8 lbam; u8 byte_count_low;,
    pub }: { u8 lbah; u8 byte_count_high;,
    pub device: u8,
    pub lbal_exp: u8,
    pub lbam_exp: u8,
    pub lbah_exp: u8,
    pub features_exp: u8,
    pub }: { u8 sector_count; u8 interrupt_reason;,
    pub sector_count_exp: u8,
    pub _r_a: u8,
    pub control: u8,
    pub _r_b: u32,
// C attribute field omitted
// Prefer to have code clarity over header file clarity.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_identify_frame {
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
pub struct ssp_frame_hdr {
    pub frame_type: u8,
    pub hashed_dest_addr: [u8; HASHED_SAS_ADDR_SIZE],
    pub _r_a: u8,
    pub hashed_src_addr: [u8; HASHED_SAS_ADDR_SIZE],
    pub _r_b: __be16,
    pub changing_data_ptr:1: u8,
    pub retransmit:1: u8,
    pub retry_data_frames:1: u8,
    pub _r_c:5: u8,
    pub num_fill_bytes:2: u8,
    pub _r_d:6: u8,
    pub _r_e: u32,
    pub tag: __be16,
    pub tptt: __be16,
    pub data_offs: __be32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_response_iu {
    pub _r_a: [u8; 10],
    pub datapres:2: u8,
    pub _r_b:6: u8,
    pub status: u8,
    pub _r_c: u32,
    pub sense_data_len: __be32,
    pub response_data_len: __be32,
    pub resp_data): DECLARE_FLEX_ARRAY(u8,,
    pub sense_data): DECLARE_FLEX_ARRAY(u8,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_command_iu {
    pub lun: [u8; 8],
    pub _r_a: u8,
    pub attr:3: u8,
    pub prio:4: u8,
    pub efb:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfer_rdy_iu {
    pub requested_offset: __be32,
    pub write_data_len: __be32,
    pub _r_a: __be32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_tmf_iu {
    pub lun: [u8; 8],
    pub _r_a: u16,
    pub tmf: u8,
    pub _r_b: u8,
    pub tag: __be16,
    pub _r_c: [u8; 14],
// C attribute field omitted
// ---------- SMP ----------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_general_resp {
    pub change_count: __be16,
    pub route_indexes: __be16,
    pub _r_a: u8,
    pub num_phys: u8,
    pub conf_route_table:1: u8,
    pub configuring:1: u8,
    pub config_others:1: u8,
    pub orej_retry_supp:1: u8,
    pub stp_cont_awt:1: u8,
    pub self_config:1: u8,
    pub zone_config:1: u8,
    pub t2t_supp:1: u8,
    pub _r_c: u8,
    pub enclosure_logical_id: [u8; 8],
    pub _r_d: [u8; 12],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct discover_resp {
    pub _r_a: [u8; 5],
    pub phy_id: u8,
    pub _r_b: __be16,
    pub _r_c:4: u8,
    pub attached_dev_type:3: u8,
    pub _r_d:1: u8,
    pub linkrate:4: u8,
    pub _r_e:4: u8,
    pub attached_sata_host:1: u8,
    pub iproto:3: u8,
    pub _r_f:4: u8,
    pub attached_sata_dev:1: u8,
    pub tproto:3: u8,
    pub _r_g:3: u8,
    pub attached_sata_ps:1: u8,
    pub sas_addr: [u8; 8],
    pub attached_sas_addr: [u8; 8],
    pub attached_phy_id: u8,
    pub _r_h: [u8; 7],
    pub hmin_linkrate:4: u8,
    pub pmin_linkrate:4: u8,
    pub hmax_linkrate:4: u8,
    pub pmax_linkrate:4: u8,
    pub change_count: u8,
    pub pptv:4: u8,
    pub _r_i:3: u8,
    pub virtual:1: u8,
    pub routing_attr:4: u8,
    pub _r_j:4: u8,
    pub conn_type: u8,
    pub conn_el_index: u8,
    pub conn_phy_link: u8,
    pub _r_k: [u8; 8],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_phy_sata_resp {
    pub _r_a: [u8; 5],
    pub phy_id: u8,
    pub _r_b: u8,
    pub affil_valid:1: u8,
    pub affil_supp:1: u8,
    pub _r_c:6: u8,
    pub _r_d: u32,
    pub stp_sas_addr: [u8; 8],
    pub fis: dev_to_host_fis,
    pub _r_e: u32,
    pub affil_stp_ini_addr: [u8; 8],
    pub crc: __be32,
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_identify_frame {
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
pub struct ssp_frame_hdr {
    pub frame_type: u8,
    pub hashed_dest_addr: [u8; HASHED_SAS_ADDR_SIZE],
    pub _r_a: u8,
    pub hashed_src_addr: [u8; HASHED_SAS_ADDR_SIZE],
    pub _r_b: __be16,
    pub _r_c:5: u8,
    pub retry_data_frames:1: u8,
    pub retransmit:1: u8,
    pub changing_data_ptr:1: u8,
    pub _r_d:6: u8,
    pub num_fill_bytes:2: u8,
    pub _r_e: u32,
    pub tag: __be16,
    pub tptt: __be16,
    pub data_offs: __be32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_response_iu {
    pub _r_a: [u8; 10],
    pub _r_b:6: u8,
    pub datapres:2: u8,
    pub status: u8,
    pub _r_c: u32,
    pub sense_data_len: __be32,
    pub response_data_len: __be32,
    pub resp_data): DECLARE_FLEX_ARRAY(u8,,
    pub sense_data): DECLARE_FLEX_ARRAY(u8,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_command_iu {
    pub lun: [u8; 8],
    pub _r_a: u8,
    pub efb:1: u8,
    pub prio:4: u8,
    pub attr:3: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfer_rdy_iu {
    pub requested_offset: __be32,
    pub write_data_len: __be32,
    pub _r_a: __be32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_tmf_iu {
    pub lun: [u8; 8],
    pub _r_a: u16,
    pub tmf: u8,
    pub _r_b: u8,
    pub tag: __be16,
    pub _r_c: [u8; 14],
// C attribute field omitted
// ---------- SMP ----------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_general_resp {
    pub change_count: __be16,
    pub route_indexes: __be16,
    pub _r_a: u8,
    pub num_phys: u8,
    pub t2t_supp:1: u8,
    pub zone_config:1: u8,
    pub self_config:1: u8,
    pub stp_cont_awt:1: u8,
    pub orej_retry_supp:1: u8,
    pub config_others:1: u8,
    pub configuring:1: u8,
    pub conf_route_table:1: u8,
    pub _r_c: u8,
    pub enclosure_logical_id: [u8; 8],
    pub _r_d: [u8; 12],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct discover_resp {
    pub _r_a: [u8; 5],
    pub phy_id: u8,
    pub _r_b: __be16,
    pub _r_d:1: u8,
    pub attached_dev_type:3: u8,
    pub _r_c:4: u8,
    pub _r_e:4: u8,
    pub linkrate:4: u8,
    pub _r_f:4: u8,
    pub iproto:3: u8,
    pub attached_sata_host:1: u8,
    pub attached_sata_ps:1: u8,
    pub _r_g:3: u8,
    pub tproto:3: u8,
    pub attached_sata_dev:1: u8,
    pub sas_addr: [u8; 8],
    pub attached_sas_addr: [u8; 8],
    pub attached_phy_id: u8,
    pub _r_h: [u8; 7],
    pub pmin_linkrate:4: u8,
    pub hmin_linkrate:4: u8,
    pub pmax_linkrate:4: u8,
    pub hmax_linkrate:4: u8,
    pub change_count: u8,
    pub virtual:1: u8,
    pub _r_i:3: u8,
    pub pptv:4: u8,
    pub _r_j:4: u8,
    pub routing_attr:4: u8,
    pub conn_type: u8,
    pub conn_el_index: u8,
    pub conn_phy_link: u8,
    pub _r_k: [u8; 8],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_phy_sata_resp {
    pub _r_a: [u8; 5],
    pub phy_id: u8,
    pub _r_b: u8,
    pub _r_c:6: u8,
    pub affil_supp:1: u8,
    pub affil_valid:1: u8,
    pub _r_d: u32,
    pub stp_sas_addr: [u8; 8],
    pub fis: dev_to_host_fis,
    pub _r_e: u32,
    pub affil_stp_ini_addr: [u8; 8],
    pub crc: __be32,
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_rg_resp {
    pub frame_type: u8,
    pub function: u8,
    pub result: u8,
    pub reserved: u8,
    pub rg: report_general_resp,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_disc_resp {
    pub frame_type: u8,
    pub function: u8,
    pub result: u8,
    pub reserved: u8,
    pub disc: discover_resp,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_rps_resp {
    pub frame_type: u8,
    pub function: u8,
    pub result: u8,
    pub reserved: u8,
    pub rps: report_phy_sata_resp,
// C attribute field omitted
