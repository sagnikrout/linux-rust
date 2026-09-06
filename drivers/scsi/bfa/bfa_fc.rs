//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_fc.h
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
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014- QLogic Corporation.
// All rights reserved
// www.qlogic.com
//
// Linux driver for QLogic BR-series Fibre Channel Host Bus Adapter.
//

pub type wwn_t = u64;

pub const FC_ALPA_MAX: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_s {

//
// generic SCSI cdb definition
//
pub const SCSI_MAX_CDBLEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_cdb_s {
    pub scsi_cdb: [u8; SCSI_MAX_CDBLEN],
}

pub const SCSI_MAX_ALLOC_LEN: c_uint = 0xFF    /* maximum allocarion length */;
//
// Fibre Channel Header Structure (FCHS) definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fchs_s {

    pub /: *mut *mut u32 routing:4; / routing bits,
    pub /: *mut *mut u32 cat_info:4; / category info,

    pub /: *mut *mut u32 cat_info:4; / category info,
    pub /: *mut *mut u32 routing:4; / routing bits,

    pub /: *mut *mut u32 d_id:24; / destination identifier,
    pub /: *mut *mut u32 cs_ctl:8; / class specific control,
    pub /: *mut *mut u32 s_id:24; / source identifier,
    pub /: *mut *mut u32 type:8; / data structure type,
    pub /: *mut *mut u32 f_ctl:24; / initial frame control,
    pub /: *mut *mut u8 seq_id; / sequence identifier,
    pub /: *mut *mut u8 df_ctl; / data field control,
    pub /: *mut *mut u16 seq_cnt; / sequence count,
    pub /: *mut *mut __be16 ox_id; / originator exchange ID,
    pub /: *mut *mut u16 rx_id; / responder exchange ID,
    pub /: *mut *mut u32 ro; / relative offset,
}

//
// routing bit definitions
//
// information category for extended link data and FC-4 Link Data
//
// information category for extended headers (VFT, IFR or encapsulation)
//
// information category for FC-4 device data
//
// Type Field Definitions. FC-PH Section 18.5 pg. 165
//
// Frame Control Definitions. FC-PH Table-45. pg. 168
//
// Fabric Well Known Addresses
//
// domain/area/port defines
//
pub const FC_DOMAIN_MASK: c_uint = 0xFF0000;
pub const FC_DOMAIN_SHIFT: c_int = 16;
pub const FC_AREA_MASK: c_uint = 0x00FF00;
pub const FC_AREA_SHIFT: c_int = 8;
pub const FC_PORT_MASK: c_uint = 0x0000FF;
pub const FC_PORT_SHIFT: c_int = 0;

//
// generic ELS command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_els_cmd_s {
    pub /: *mut *mut u32 els_code:8; / ELS Command Code,
    pub reserved:24: u32,
}

//
// ELS Command Codes. FC-PH Table-75. pg. 223
//
// End-to-End Link Beacon
// FC-SP
//
// Version numbers for FC-PH standards,
// used in login to indicate what port
// supports. See FC-PH-X table 158.
//
// PDU size defines
//
// N_Port PLOGI Common Service Parameters.
// FC-PH-x. Figure-76. pg. 308.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_plogi_csp_s {
    pub /: *mut *mut u8 verhi; / FC-PH high version,
    pub /: *mut *mut u8 verlo; / FC-PH low version,
    pub /: *mut *mut __be16 bbcred; / BB_Credit,

    pub /: *mut *mut ciro:1; / continuously increasing RO,

    pub /: *mut *mut __be16 rxsz; / receive data_field size,
    pub conseq: __be16,
    pub ro_bitmap: __be16,
    pub e_d_tov: __be32,
}

//
// N_Port PLOGI Class Specific Parameters.
// FC-PH-x. Figure 78. pg. 318.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_plogi_clp_s {

    pub class_valid:1: u32,
    pub =1.: *mut *mut u32 intermix:1; / class intermix supported if set,
// valid only for class1. Reserved for
// class2 & class3
    pub reserved1:2: u32,
    pub sequential:1: u32,
    pub reserved2:3: u32,

    pub reserved2:3: u32,
    pub sequential:1: u32,
    pub reserved1:2: u32,
    pub =1.: *mut *mut u32 intermix:1; / class intermix supported if set,
// valid only for class1. Reserved for
// class2 & class3
    pub class_valid:1: u32,

    pub reserved3:24: u32,
    pub reserved4:16: u32,
    pub /: *mut *mut u32 rxsz:16; / Receive data_field size,
    pub reserved5:8: u32,
    pub conseq:8: u32,
    pub /: *mut *mut u32 e2e_credit:16; / end to end credit,
    pub reserved7:8: u32,
    pub ospx:8: u32,
    pub reserved8:16: u32,
}

// ASCII value for each character in string "BRCD"
pub const FLOGI_VVL_BRCD: c_uint = 0x42524344;
//
// PLOGI els command and reply payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_logi_s {
    pub /: *mut *mut fc_els_cmd_s els_cmd; / ELS command code,
    pub /: *mut *mut fc_plogi_csp_s csp; / common service params,
    pub port_name: wwn_t,
    pub node_name: wwn_t,
    pub /: *mut *mut fc_plogi_clp_s class1; / class 1 service parameters,
    pub /: *mut *mut fc_plogi_clp_s class2; / class 2 service parameters,
    pub /: *mut *mut fc_plogi_clp_s class3; / class 3 service parameters,
    pub /: *mut *mut fc_plogi_clp_s class4; / class 4 service parameters,
    pub /: *mut *mut u8 vvl[16]; / vendor version level,
}

//
// LOGO els command payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_logo_s {
    pub /: *mut *mut fc_els_cmd_s els_cmd; / ELS command code,
    pub res1:8: u32,
    pub /: *mut *mut u32 nport_id:24; / N_Port identifier of source,
    pub /: *mut *mut wwn_t orig_port_name; / Port name of the LOGO originator,
}

//
// ADISC els command payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_adisc_s {
    pub /: *mut *mut fc_els_cmd_s els_cmd; / ELS command code,
    pub res1:8: u32,
    pub /: *mut *mut u32 orig_HA:24; / originator hard address,
    pub /: *mut *mut wwn_t orig_port_name; / originator port name,
    pub /: *mut *mut wwn_t orig_node_name; / originator node name,
    pub res2:8: u32,
    pub /: *mut *mut u32 nport_id:24; / originator NPortID,
}

//
// Exchange status block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_exch_status_blk_s {
    pub oxid:16: u32,
    pub rxid:16: u32,
    pub res1:8: u32,
    pub /: *mut *mut u32 orig_np:24; / originator NPortID,
    pub res2:8: u32,
    pub /: *mut *mut u32 resp_np:24; / responder NPortID,
    pub es_bits: u32,
    pub res3: u32,
//
// un modified section of the fields
//
}

//
// RES els command payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_res_s {
    pub /: *mut *mut fc_els_cmd_s els_cmd; / ELS command code,
    pub res1:8: u32,
    pub /: *mut *mut u32 nport_id:24; / N_Port identifier of source,
    pub oxid:16: u32,
    pub rxid:16: u32,
    pub assoc_hdr: [u8; 32],
}

//
// RES els accept payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_res_acc_s {
    pub /: *mut *mut fc_els_cmd_s els_cmd; / ELS command code,
    pub /: *mut *mut fc_exch_status_blk_s fc_exch_blk; / Exchange status block,
}

//
// REC els command payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rec_s {
    pub /: *mut *mut fc_els_cmd_s els_cmd; / ELS command code,
    pub res1:8: u32,
    pub /: *mut *mut u32 nport_id:24; / N_Port identifier of source,
    pub oxid:16: u32,
    pub rxid:16: u32,
}

pub const FC_REC_ESB_OWN_RSP: c_uint = 0x80000000	/* responder owns */;
pub const FC_REC_ESB_SI: c_uint = 0x40000000	/* SI is owned	*/;
pub const FC_REC_ESB_COMP: c_uint = 0x20000000	/* exchange is complete	*/;
pub const FC_REC_ESB_ENDCOND_ABN: c_uint = 0x10000000	/* abnormal ending	*/;
pub const FC_REC_ESB_RQACT: c_uint = 0x04000000	/* recovery qual active	*/;
pub const FC_REC_ESB_ERRP_MSK: c_uint = 0x03000000;
pub const FC_REC_ESB_OXID_INV: c_uint = 0x00800000	/* invalid OXID		*/;
pub const FC_REC_ESB_RXID_INV: c_uint = 0x00400000	/* invalid RXID		*/;
pub const FC_REC_ESB_PRIO_INUSE: c_uint = 0x00200000;
//
// REC els accept payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rec_acc_s {
    pub /: *mut *mut fc_els_cmd_s els_cmd; / ELS command code,
    pub oxid:16: u32,
    pub rxid:16: u32,
    pub res1:8: u32,
    pub /: *mut *mut u32 orig_id:24; / N_Port id of exchange originator,
    pub res2:8: u32,
    pub /: *mut *mut u32 resp_id:24; / N_Port id of exchange responder,
    pub /: *mut *mut u32 count; / data transfer count,
    pub /: *mut *mut u32 e_stat; / exchange status,
}

//
// RSI els payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rsi_s {
    pub els_cmd: fc_els_cmd_s,
    pub res1:8: u32,
    pub orig_sid:24: u32,
    pub oxid:16: u32,
    pub rxid:16: u32,
}

//
// structure for PRLI paramater pages, both request & response
// see FC-PH-X table 113 & 115 for explanation also FCP table 8
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_prli_params_s {
    pub reserved:16: u32,

    pub reserved1:5: u32,
    pub rec_support:1: u32,
    pub task_retry_id:1: u32,
    pub retry:1: u32,
    pub confirm:1: u32,
    pub doverlay:1: u32,
    pub initiator:1: u32,
    pub target:1: u32,
    pub cdmix:1: u32,
    pub drmix:1: u32,
    pub rxrdisab:1: u32,
    pub wxrdisab:1: u32,

    pub retry:1: u32,
    pub task_retry_id:1: u32,
    pub rec_support:1: u32,
    pub reserved1:5: u32,
    pub wxrdisab:1: u32,
    pub rxrdisab:1: u32,
    pub drmix:1: u32,
    pub cdmix:1: u32,
    pub target:1: u32,
    pub initiator:1: u32,
    pub doverlay:1: u32,
    pub confirm:1: u32,

}

//
// valid values for rspcode in PRLI ACC payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_prli_params_page_s {
    pub type:8: u32,
    pub codext:8: u32,

    pub origprocasv:1: u32,
    pub rsppav:1: u32,
    pub imagepair:1: u32,
    pub reserved1:1: u32,
    pub rspcode:4: u32,

    pub rspcode:4: u32,
    pub reserved1:1: u32,
    pub imagepair:1: u32,
    pub rsppav:1: u32,
    pub origprocasv:1: u32,

    pub reserved2:8: u32,
    pub origprocas: u32,
    pub rspprocas: u32,
    pub servparams: fc_prli_params_s,
}

//
// PRLI request and accept payload, FC-PH-X tables 112 & 114
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_prli_s {
    pub command:8: u32,
    pub pglen:8: u32,
    pub pagebytes:16: u32,
    pub parampage: fc_prli_params_page_s,
}

//
// PRLO logout params page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_prlo_params_page_s {
    pub type:8: u32,
    pub type_ext:8: u32,

    pub /: *mut *mut u32 opa_valid:1; / originator process associator valid,
    pub /: *mut *mut u32 rpa_valid:1; / responder process associator valid,
    pub res1:14: u32,

    pub res1:14: u32,
    pub /: *mut *mut u32 rpa_valid:1; / responder process associator valid,
    pub /: *mut *mut u32 opa_valid:1; / originator process associator valid,

    pub orig_process_assc: u32,
    pub resp_process_assc: u32,
    pub res2: u32,
}

//
// PRLO els command payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_prlo_s {
    pub command:8: u32,
    pub page_len:8: u32,
    pub payload_len:16: u32,
    pub prlo_params: [fc_prlo_params_page_s; 1],
}

//
// PRLO Logout response parameter page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_prlo_acc_params_page_s {
    pub type:8: u32,
    pub type_ext:8: u32,

    pub /: *mut *mut u32 opa_valid:1; / originator process associator valid,
    pub /: *mut *mut u32 rpa_valid:1; / responder process associator valid,
    pub res1:14: u32,

    pub res1:14: u32,
    pub /: *mut *mut u32 rpa_valid:1; / responder process associator valid,
    pub /: *mut *mut u32 opa_valid:1; / originator process associator valid,

    pub orig_process_assc: u32,
    pub resp_process_assc: u32,
    pub fc4type_csp: u32,
}

//
// PRLO els command ACC payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_prlo_acc_s {
    pub command:8: u32,
    pub page_len:8: u32,
    pub payload_len:16: u32,
    pub prlo_acc_params: [fc_prlo_acc_params_page_s; 1],
}

//
// SCR els command payload
//
// SCR VU registrations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_scr_s {
    pub command:8: u32,
    pub res:24: u32,
    pub /: *mut *mut u32 vu_reg_func:8; / Vendor Unique Registrations,
    pub res1:16: u32,
    pub reg_func:8: u32,
}

//
// Information category for Basic link data
//
// LS_RJT els reply payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ls_rjt_s {
    pub /: *mut *mut fc_els_cmd_s els_cmd; / ELS command code,
    pub res1:8: u32,
    pub /: *mut *mut u32 reason_code:8; / Reason code for reject,
    pub /: *mut *mut u32 reason_code_expl:8; / Reason code explanation,
    pub /: *mut *mut u32 vendor_unique:8; / Vendor specific,
}

//
// LS_RJT reason codes
//
// LS_RJT reason code explanation
//
// RRQ els command payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rrq_s {
    pub /: *mut *mut fc_els_cmd_s els_cmd; / ELS command code,
    pub res1:8: u32,
    pub /: *mut *mut u32 s_id:24; / exchange originator S_ID,
    pub /: *mut *mut u32 ox_id:16; / originator exchange ID,
    pub /: *mut *mut u32 rx_id:16; / responder exchange ID,
    pub /: *mut *mut u32 res2[8]; / optional association header,
}

//
// ABTS BA_ACC reply payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ba_acc_s {
    pub /: *mut *mut u32 seq_id_valid:8; / set to 0x00 for Abort Exchange,
    pub /: *mut *mut u32 seq_id:8; / invalid for Abort Exchange,
    pub res2:16: u32,
    pub /: *mut *mut u32 ox_id:16; / OX_ID from ABTS frame,
    pub /: *mut *mut u32 rx_id:16; / RX_ID from ABTS frame,
    pub /: *mut *mut u32 low_seq_cnt:16; / set to 0x0000 for Abort Exchange,
    pub /: *mut *mut u32 high_seq_cnt:16; / set to 0xFFFF for Abort Exchange,
}

//
// ABTS BA_RJT reject payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ba_rjt_s {
    pub /: *mut *mut u32 res1:8; / Reserved,
    pub /: *mut *mut u32 reason_code:8; / reason code for reject,
    pub /: *mut *mut u32 reason_expl:8; / reason code explanation,
    pub /: *mut *mut u32 vendor_unique:8; / vendor unique reason code,set to 0,
}

//
// TPRLO logout parameter page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_tprlo_params_page_s {
    pub type:8: u32,
    pub type_ext:8: u32,

    pub opa_valid:1: u32,
    pub rpa_valid:1: u32,
    pub tpo_nport_valid:1: u32,
    pub global_process_logout:1: u32,
    pub res1:12: u32,

    pub res1:12: u32,
    pub global_process_logout:1: u32,
    pub tpo_nport_valid:1: u32,
    pub rpa_valid:1: u32,
    pub opa_valid:1: u32,

    pub orig_process_assc: u32,
    pub resp_process_assc: u32,
    pub res2:8: u32,
    pub tpo_nport_id: u32,
}

//
// TPRLO ELS command payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_tprlo_s {
    pub command:8: u32,
    pub page_len:8: u32,
    pub payload_len:16: u32,
    pub tprlo_params: [fc_tprlo_params_page_s; 1],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_tprlo_type {
    FC_GLOBAL_LOGO = 1,
    FC_TPR_LOGO
}

//
// TPRLO els command ACC payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_tprlo_acc_s {
    pub command:8: u32,
    pub page_len:8: u32,
    pub payload_len:16: u32,
    pub tprlo_acc_params: [fc_prlo_acc_params_page_s; 1],
}

//
// RSCN els command req payload
//
pub const FC_RSCN_PGLEN: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_rscn_format {
    FC_RSCN_FORMAT_PORTID	= 0x0,
    FC_RSCN_FORMAT_AREA	= 0x1,
    FC_RSCN_FORMAT_DOMAIN	= 0x2,
    FC_RSCN_FORMAT_FABRIC	= 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rscn_event_s {
    pub format:2: u32,
    pub qualifier:4: u32,
    pub resvd:2: u32,
    pub portid:24: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rscn_pl_s {
    pub command: u8,
    pub pagelen: u8,
    pub payldlen: __be16,
    pub event: [fc_rscn_event_s; ],
}

//
// ECHO els command req payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_echo_s {
    pub els_cmd: fc_els_cmd_s,
}

//
// RNID els command
//
pub const RNID_NODEID_DATA_FORMAT_COMMON: c_uint = 0x00;
pub const RNID_NODEID_DATA_FORMAT_FCP3: c_uint = 0x08;
pub const RNID_NODEID_DATA_FORMAT_DISCOVERY: c_uint = 0xDF;
pub const RNID_ASSOCIATED_TYPE_UNKNOWN: c_uint = 0x00000001;
pub const RNID_ASSOCIATED_TYPE_OTHER: c_uint = 0x00000002;
pub const RNID_ASSOCIATED_TYPE_HUB: c_uint = 0x00000003;
pub const RNID_ASSOCIATED_TYPE_SWITCH: c_uint = 0x00000004;
pub const RNID_ASSOCIATED_TYPE_GATEWAY: c_uint = 0x00000005;
pub const RNID_ASSOCIATED_TYPE_STORAGE_DEVICE: c_uint = 0x00000009;
pub const RNID_ASSOCIATED_TYPE_HOST: c_uint = 0x0000000A;
pub const RNID_ASSOCIATED_TYPE_STORAGE_SUBSYSTEM: c_uint = 0x0000000B;
pub const RNID_ASSOCIATED_TYPE_STORAGE_ACCESS_DEVICE: c_uint = 0x0000000E;
pub const RNID_ASSOCIATED_TYPE_NAS_SERVER: c_uint = 0x00000011;
pub const RNID_ASSOCIATED_TYPE_BRIDGE: c_uint = 0x00000002;
pub const RNID_ASSOCIATED_TYPE_VIRTUALIZATION_DEVICE: c_uint = 0x00000003;
pub const RNID_ASSOCIATED_TYPE_MULTI_FUNCTION_DEVICE: c_uint = 0x000000FF;
//
// RNID els command payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rnid_cmd_s {
    pub els_cmd: fc_els_cmd_s,
    pub node_id_data_format:8: u32,
    pub reserved:24: u32,
}

//
// RNID els response payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rnid_common_id_data_s {
    pub port_name: wwn_t,
    pub node_name: wwn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rnid_general_topology_data_s {
    pub vendor_unique: [u32; 4],
    pub asso_type: __be32,
    pub phy_port_num: u32,
    pub num_attached_nodes: __be32,
    pub node_mgmt:8: u32,
    pub ip_version:8: u32,
    pub udp_tcp_port_num:16: u32,
    pub ip_address: [u32; 4],
    pub reserved:16: u32,
    pub vendor_specific:16: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rnid_acc_s {
    pub els_cmd: fc_els_cmd_s,
    pub node_id_data_format:8: u32,
    pub common_id_data_length:8: u32,
    pub reserved:8: u32,
    pub specific_id_data_length:8: u32,
    pub common_id_data: fc_rnid_common_id_data_s,
    pub gen_topology_data: fc_rnid_general_topology_data_s,
}

pub const RNID_ASSOCIATED_TYPE_UNKNOWN: c_uint = 0x00000001;
pub const RNID_ASSOCIATED_TYPE_OTHER: c_uint = 0x00000002;
pub const RNID_ASSOCIATED_TYPE_HUB: c_uint = 0x00000003;
pub const RNID_ASSOCIATED_TYPE_SWITCH: c_uint = 0x00000004;
pub const RNID_ASSOCIATED_TYPE_GATEWAY: c_uint = 0x00000005;
pub const RNID_ASSOCIATED_TYPE_STORAGE_DEVICE: c_uint = 0x00000009;
pub const RNID_ASSOCIATED_TYPE_HOST: c_uint = 0x0000000A;
pub const RNID_ASSOCIATED_TYPE_STORAGE_SUBSYSTEM: c_uint = 0x0000000B;
pub const RNID_ASSOCIATED_TYPE_STORAGE_ACCESS_DEVICE: c_uint = 0x0000000E;
pub const RNID_ASSOCIATED_TYPE_NAS_SERVER: c_uint = 0x00000011;
pub const RNID_ASSOCIATED_TYPE_BRIDGE: c_uint = 0x00000002;
pub const RNID_ASSOCIATED_TYPE_VIRTUALIZATION_DEVICE: c_uint = 0x00000003;
pub const RNID_ASSOCIATED_TYPE_MULTI_FUNCTION_DEVICE: c_uint = 0x000000FF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_rpsc_speed_cap {
    RPSC_SPEED_CAP_1G = 0x8000,
    RPSC_SPEED_CAP_2G = 0x4000,
    RPSC_SPEED_CAP_4G = 0x2000,
    RPSC_SPEED_CAP_10G = 0x1000,
    RPSC_SPEED_CAP_8G = 0x0800,
    RPSC_SPEED_CAP_16G = 0x0400,

    RPSC_SPEED_CAP_UNKNOWN = 0x0001,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_rpsc_op_speed {
    RPSC_OP_SPEED_1G = 0x8000,
    RPSC_OP_SPEED_2G = 0x4000,
    RPSC_OP_SPEED_4G = 0x2000,
    RPSC_OP_SPEED_10G = 0x1000,
    RPSC_OP_SPEED_8G = 0x0800,
    RPSC_OP_SPEED_16G = 0x0400,

    RPSC_OP_SPEED_NOT_EST = 0x0001,	/* speed not established */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rpsc_speed_info_s {
    pub /: *mut *mut __be16 port_speed_cap; / see enum fc_rpsc_speed_cap,
    pub /: *mut *mut __be16 port_op_speed; / see enum fc_rpsc_op_speed,
}

//
// If RPSC request is sent to the Domain Controller, the request is for
// all the ports within that domain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rpsc_cmd_s {
    pub els_cmd: fc_els_cmd_s,
}

//
// RPSC Acc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rpsc_acc_s {
    pub command:8: u32,
    pub rsvd:8: u32,
    pub num_entries:16: u32,
    pub speed_info: [fc_rpsc_speed_info_s; 1],
}

//
// If RPSC2 request is sent to the Domain Controller,
//
pub const FC_BRCD_TOKEN: c_uint = 0x42524344;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rpsc2_cmd_s {
    pub els_cmd: fc_els_cmd_s,
    pub token: __be32,
    pub resvd: u16,
    pub /: *mut *mut __be16 num_pids; / Number of pids in the request,
    pub rsvd1:8: u32,
    pub /: *mut *mut u32 pid:24; / port identifier,
    pub pid_list: [}; 1],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_rpsc2_port_type {
    RPSC2_PORT_TYPE_UNKNOWN = 0,
    RPSC2_PORT_TYPE_NPORT   = 1,
    RPSC2_PORT_TYPE_NLPORT  = 2,
    RPSC2_PORT_TYPE_NPIV_PORT  = 0x5f,
    RPSC2_PORT_TYPE_NPORT_TRUNK  = 0x6f,
}

//
// RPSC2 portInfo entry structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rpsc2_port_info_s {
    pub /: *mut *mut __be32 pid; / PID,
    pub resvd1: u16,
    pub /: *mut *mut __be16 index; / port number / index,
    pub resvd2: u8,
    pub /: *mut *mut u8 type; / port type N/NL/...,
    pub /: *mut *mut __be16 speed; / port Operating Speed,
}

//
// RPSC2 Accept payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rpsc2_acc_s {
    pub els_cmd: u8,
    pub resvd: u8,
    pub /: *mut *mut __be16 num_pids; / Number of pids in the request,
    pub /: *mut *mut fc_rpsc2_port_info_s port_info[1]; / port information,
}

//
// bit fields so that multiple classes can be specified
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_cos {
    FC_CLASS_2	= 0x04,
    FC_CLASS_3	= 0x08,
    FC_CLASS_2_3	= 0x0C,
}

//
// symbolic name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_symname_s {
    pub symname: [u8; FC_SYMNAME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_alpabm_s {
    pub 8]: u8 alpa_bm[FC_ALPA_MAX /,
}

//
// protocol default timeout values
//
pub const FC_ED_TOV: c_int = 2;

pub const FC_RA_TOV: c_int = 10;

//
// virtual fabric related defines
//

pub const FC_VF_ID_MIN: c_int = 1;
pub const FC_VF_ID_MAX: c_uint = 0xEFF;
pub const FC_VF_ID_CTL: c_uint = 0xFEF	/*  control VF_ID */;
//
// Virtual Fabric Tagging header format
// @caution This is defined only in BIG ENDIAN format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_vft_s {
    pub r_ctl:8: u32,
    pub ver:2: u32,
    pub type:4: u32,
    pub res_a:2: u32,
    pub priority:3: u32,
    pub vf_id:12: u32,
    pub res_b:1: u32,
    pub hopct:8: u32,
    pub res_c:24: u32,
}

//
// FCP_CMND definitions
//
pub const FCP_CMND_CDB_LEN: c_int = 16;
pub const FCP_CMND_LUN_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_cmnd_s {
    pub /: *mut *mut scsi_lun lun; / 64-bit LU number,
    pub /: *mut *mut u8 crn; / command reference number,

    pub /: *mut *mut taskattr:3; / scsi task attribute,

    pub /: *mut *mut u8 tm_flags; / task management flags,

    pub /: *mut *mut iodir:2; / read/write FCP_DATA IUs,

    pub /: *mut *mut addl_cdb_len:6; / additional CDB length,

    pub cdb: scsi_cdb_s,
    pub /: *mut *mut __be32 fcp_dl; / bytes to be transferred,
}

//
// struct fcp_cmnd_s .iodir field values
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcp_iodir {
    FCP_IODIR_NONE  = 0,
    FCP_IODIR_WRITE = 1,
    FCP_IODIR_READ  = 2,
    FCP_IODIR_RW    = 3,
}

//
// Task management flags field - only one bit shall be set
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcp_tm_cmnd {
    FCP_TM_ABORT_TASK_SET	= BIT(1),
    FCP_TM_CLEAR_TASK_SET	= BIT(2),
    FCP_TM_LUN_RESET	= BIT(4),
    FCP_TM_TARGET_RESET	= BIT(5),	/* obsolete in FCP-3 */
    FCP_TM_CLEAR_ACA	= BIT(6),
}

//
// FCP_RSP residue flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcp_residue {
    FCP_NO_RESIDUE = 0,     /* no residue */
    FCP_RESID_OVER = 1,     /* more data left that was not sent */
    FCP_RESID_UNDER = 2,    /* less data than requested */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_rspinfo_s {
    pub res0:24: u32,
    pub /: *mut *mut u32 rsp_code:8; / response code (as above),
    pub res1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcp_resp_s {
    pub /: *mut *mut u32 reserved[2]; / 2 words reserved,
    pub reserved2: u16,

    pub reserved3:3: u8,
    pub /: *mut *mut u8 fcp_conf_req:1; / FCP_CONF is requested,
    pub /: *mut *mut u8 resid_flags:2; / underflow/overflow,
    pub /: *mut *mut u8 sns_len_valid:1; / sense len is valid,
    pub /: *mut *mut u8 rsp_len_valid:1; / response len is valid,

    pub /: *mut *mut u8 rsp_len_valid:1; / response len is valid,
    pub /: *mut *mut u8 sns_len_valid:1; / sense len is valid,
    pub /: *mut *mut u8 resid_flags:2; / underflow/overflow,
    pub /: *mut *mut u8 fcp_conf_req:1; / FCP_CONF is requested,
    pub reserved3:3: u8,

    pub /: *mut *mut u8 scsi_status; / one byte SCSI status,
    pub /: *mut *mut u32 residue; / residual data bytes,
    pub /: *mut *mut u32 sns_len; / length od sense info,
    pub /: *mut *mut u32 rsp_len; / length of response info,
}

//
// CT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_hdr_s {
    pub /: *mut *mut u32 rev_id:8; / Revision of the CT,
    pub /: *mut *mut u32 in_id:24; / Initiator Id,
    pub /: *mut *mut u32 gs_type:8; / Generic service Type,
    pub /: *mut *mut u32 gs_sub_type:8; / Generic service sub type,
    pub /: *mut *mut u32 options:8; / options,
    pub /: *mut *mut u32 rsvrd:8; / reserved,
    pub /: *mut *mut u32 cmd_rsp_code:16;/ ct command/response code,
    pub /: *mut *mut u32 max_res_size:16;/ maximum/residual size,
    pub /: *mut *mut u32 frag_id:8; / fragment ID,
    pub /: *mut *mut u32 reason_code:8; / reason code,
    pub /: *mut *mut u32 exp_code:8; / explanation code,
    pub /: *mut *mut u32 vendor_unq:8; / vendor unique,
}

//
// defines for the Revision
//
// defines for gs_type
//
// defines for gs_sub_type for gs type directory service
//
// defines for gs_sub_type for gs type management service
//
// defines for CT response code field
//
// definitions for CT reason code
//
// definitions for explanations code for Name server
//
// definitions for the explanation code for all servers
//
// Command codes for Name server
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_id_req_s {
    pub rsvd:8: u32,
    pub /: *mut *mut u32 dap:24; / port identifier,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_gidpn_req_s {
    pub /: *mut *mut wwn_t port_name; / port wwn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_gidpn_resp_s {
    pub rsvd:8: u32,
    pub /: *mut *mut u32 dap:24; / port identifier,
}

//
// RFT_ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_rftid_req_s {
    pub rsvd:8: u32,
    pub /: *mut *mut u32 dap:24; / port identifier,
    pub /: *mut *mut __be32 fc4_type[8]; / fc4 types,
}

//
// RFF_ID : Register FC4 features.
//
pub const FC_GS_FCP_FC4_FEATURE_INITIATOR: c_uint = 0x02;
pub const FC_GS_FCP_FC4_FEATURE_TARGET: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_rffid_req_s {
    pub rsvd:8: u32,
    pub /: *mut *mut u32 dap:24; / port identifier,
    pub rsvd1:16: u32,
    pub /: *mut *mut u32 fc4ftr_bits:8; / fc4 feature bits,
    pub /: *mut *mut u32 fc4_type:8; / corresponding FC4 Type,
}

//
// GID_FT Request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_gidft_req_s {
    pub reserved: u8,
    pub /: *mut *mut u8 domain_id; / domain, 0 - all fabric,
    pub /: *mut *mut u8 area_id; / area, 0 - whole domain,
    pub /: *mut *mut u8 fc4_type; / FC_TYPE_FCP for SCSI devices,
}

//
// GID_FT Response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_gidft_resp_s {
    pub /: *mut *mut u8 last:1; / last port identifier flag,
    pub reserved:7: u8,
    pub /: *mut *mut u32 pid:24; / port identifier,
}

//
// RSPN_ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_rspnid_req_s {
    pub rsvd:8: u32,
    pub /: *mut *mut u32 dap:24; / port identifier,
    pub /: *mut *mut u8 spn_len; / symbolic port name length,
    pub /: *mut *mut u8 spn[256]; / symbolic port name,
}

//
// RSNN_NN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_rsnn_nn_req_s {
    pub /: *mut *mut wwn_t node_name; / Node name,
    pub /: *mut *mut u8 snn_len; / symbolic node name length,
    pub /: *mut *mut u8 snn[256]; / symbolic node name,
}

//
// RPN_ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_rpnid_req_s {
    pub rsvd:8: u32,
    pub port_id:24: u32,
    pub port_name: wwn_t,
}

//
// RNN_ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_rnnid_req_s {
    pub rsvd:8: u32,
    pub port_id:24: u32,
    pub node_name: wwn_t,
}

//
// RCS_ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_rcsid_req_s {
    pub rsvd:8: u32,
    pub port_id:24: u32,
    pub cos: u32,
}

//
// RPT_ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_rptid_req_s {
    pub rsvd:8: u32,
    pub port_id:24: u32,
    pub port_type:8: u32,
    pub rsvd1:24: u32,
}

//
// GA_NXT Request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_ganxt_req_s {
    pub rsvd:8: u32,
    pub port_id:24: u32,
}

//
// GA_NXT Response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_ganxt_rsp_s {
    pub /: *mut *mut u32 port_type:8; / Port Type,
    pub /: *mut *mut u32 port_id:24; / Port Identifier,
    pub /: *mut *mut wwn_t port_name; / Port Name,
    pub /: *mut *mut u8 spn_len; / Length of Symbolic Port Name,
    pub /: *mut *mut char spn[255]; / Symbolic Port Name,
    pub /: *mut *mut wwn_t node_name; / Node Name,
    pub /: *mut *mut u8 snn_len; / Length of Symbolic Node Name,
    pub /: *mut *mut char snn[255]; / Symbolic Node Name,
    pub /: *mut *mut u8 ipa[8]; / Initial Process Associator,
    pub /: *mut *mut u8 ip[16]; / IP Address,
    pub /: *mut *mut u32 cos; / Class of Service,
    pub /: *mut *mut u32 fc4types[8]; / FC-4 TYPEs,
    pub /: *mut *mut wwn_t fabric_port_name; / Fabric Port Name,
    pub /: *mut *mut u32 rsvd:8; / Reserved,
    pub /: *mut *mut u32 hard_addr:24; / Hard Address,
}

//
// Command codes for Fabric Configuration Server
//
// GMAL Command ( Get ( interconnect Element) Management Address List)
// To retrieve the IP Address of a Switch.
//

// GMAL/GFN request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_req_s {
    pub /: *mut *mut wwn_t wwn; / PWWN/NWWN,
}

// Accept Response to GMAL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_gmal_resp_s {
    pub /: *mut *mut __be32 ms_len; / Num of entries,
    pub ms_ma: [u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcgs_gmal_entry_s {
    pub len: u8,
    pub /: *mut *mut u8 prefix[7]; / like "http://",
    pub ip_addr: [u8; 248],
}

//
// FDMI Command Codes
//
pub const FDMI_GRHL: c_uint = 0x0100;
pub const FDMI_GHAT: c_uint = 0x0101;
pub const FDMI_GRPL: c_uint = 0x0102;
pub const FDMI_GPAT: c_uint = 0x0110;
pub const FDMI_RHBA: c_uint = 0x0200;
pub const FDMI_RHAT: c_uint = 0x0201;
pub const FDMI_RPRT: c_uint = 0x0210;
pub const FDMI_RPA: c_uint = 0x0211;
pub const FDMI_DHBA: c_uint = 0x0300;
pub const FDMI_DPRT: c_uint = 0x0310;
//
// FDMI reason codes
//
pub const FDMI_NO_ADDITIONAL_EXP: c_uint = 0x00;
pub const FDMI_HBA_ALREADY_REG: c_uint = 0x10;
pub const FDMI_HBA_ATTRIB_NOT_REG: c_uint = 0x11;
pub const FDMI_HBA_ATTRIB_MULTIPLE: c_uint = 0x12;
pub const FDMI_HBA_ATTRIB_LENGTH_INVALID: c_uint = 0x13;
pub const FDMI_HBA_ATTRIB_NOT_PRESENT: c_uint = 0x14;
pub const FDMI_PORT_ORIG_NOT_IN_LIST: c_uint = 0x15;
pub const FDMI_PORT_HBA_NOT_IN_LIST: c_uint = 0x16;
pub const FDMI_PORT_ATTRIB_NOT_REG: c_uint = 0x20;
pub const FDMI_PORT_NOT_REG: c_uint = 0x21;
pub const FDMI_PORT_ATTRIB_MULTIPLE: c_uint = 0x22;
pub const FDMI_PORT_ATTRIB_LENGTH_INVALID: c_uint = 0x23;
pub const FDMI_PORT_ALREADY_REGISTEREED: c_uint = 0x24;
//
// FDMI Transmission Speed Mask values
//
pub const FDMI_TRANS_SPEED_1G: c_uint = 0x00000001;
pub const FDMI_TRANS_SPEED_2G: c_uint = 0x00000002;
pub const FDMI_TRANS_SPEED_10G: c_uint = 0x00000004;
pub const FDMI_TRANS_SPEED_4G: c_uint = 0x00000008;
pub const FDMI_TRANS_SPEED_8G: c_uint = 0x00000010;
pub const FDMI_TRANS_SPEED_16G: c_uint = 0x00000020;
pub const FDMI_TRANS_SPEED_UNKNOWN: c_uint = 0x00008000;
//
// FDMI HBA attribute types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fdmi_hba_attribute_type {
    FDMI_HBA_ATTRIB_NODENAME = 1,	/* 0x0001 */
    FDMI_HBA_ATTRIB_MANUFACTURER,	/* 0x0002 */
    FDMI_HBA_ATTRIB_SERIALNUM,	/* 0x0003 */
    FDMI_HBA_ATTRIB_MODEL,		/* 0x0004 */
    FDMI_HBA_ATTRIB_MODEL_DESC,	/* 0x0005 */
    FDMI_HBA_ATTRIB_HW_VERSION,	/* 0x0006 */
    FDMI_HBA_ATTRIB_DRIVER_VERSION,	/* 0x0007 */
    FDMI_HBA_ATTRIB_ROM_VERSION,	/* 0x0008 */
    FDMI_HBA_ATTRIB_FW_VERSION,	/* 0x0009 */
    FDMI_HBA_ATTRIB_OS_NAME,	/* 0x000A */
    FDMI_HBA_ATTRIB_MAX_CT,		/* 0x000B */
    FDMI_HBA_ATTRIB_NODE_SYM_NAME,  /* 0x000C */
    FDMI_HBA_ATTRIB_VENDOR_INFO,    /* 0x000D */
    FDMI_HBA_ATTRIB_NUM_PORTS,  /* 0x000E */
    FDMI_HBA_ATTRIB_FABRIC_NAME,    /* 0x000F */
    FDMI_HBA_ATTRIB_BIOS_VER,   /* 0x0010 */
    FDMI_HBA_ATTRIB_VENDOR_ID = 0x00E0,

    FDMI_HBA_ATTRIB_MAX_TYPE
}

//
// FDMI Port attribute types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fdmi_port_attribute_type {
    FDMI_PORT_ATTRIB_FC4_TYPES = 1,	/* 0x0001 */
    FDMI_PORT_ATTRIB_SUPP_SPEED,	/* 0x0002 */
    FDMI_PORT_ATTRIB_PORT_SPEED,	/* 0x0003 */
    FDMI_PORT_ATTRIB_FRAME_SIZE,	/* 0x0004 */
    FDMI_PORT_ATTRIB_DEV_NAME,	/* 0x0005 */
    FDMI_PORT_ATTRIB_HOST_NAME,	/* 0x0006 */
    FDMI_PORT_ATTRIB_NODE_NAME,     /* 0x0007 */
    FDMI_PORT_ATTRIB_PORT_NAME,     /* 0x0008 */
    FDMI_PORT_ATTRIB_PORT_SYM_NAME, /* 0x0009 */
    FDMI_PORT_ATTRIB_PORT_TYPE,     /* 0x000A */
    FDMI_PORT_ATTRIB_SUPP_COS,      /* 0x000B */
    FDMI_PORT_ATTRIB_PORT_FAB_NAME, /* 0x000C */
    FDMI_PORT_ATTRIB_PORT_FC4_TYPE, /* 0x000D */
    FDMI_PORT_ATTRIB_PORT_STATE = 0x101,    /* 0x0101 */
    FDMI_PORT_ATTRIB_PORT_NUM_RPRT = 0x102, /* 0x0102 */

    FDMI_PORT_ATTR_MAX_TYPE
}

//
// FDMI attribute
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdmi_attr_s {
    pub type: __be16,
    pub len: __be16,
    pub value: [u8; ],
}

//
// HBA Attribute Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdmi_hba_attr_s {
    pub /: *mut *mut __be32 attr_count; / # of attributes,
    pub /: *mut *mut fdmi_attr_s hba_attr; / n attributes,
}

//
// Registered Port List
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdmi_port_list_s {
    pub /: *mut *mut __be32 num_ports; / number Of Port Entries,
    pub /: *mut *mut wwn_t port_entry; / one or more,
}

//
// Port Attribute Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdmi_port_attr_s {
    pub /: *mut *mut __be32 attr_count; / # of attributes,
    pub /: *mut *mut fdmi_attr_s port_attr; / n attributes,
}

//
// FDMI Register HBA Attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdmi_rhba_s {
    pub /: *mut *mut wwn_t hba_id; / HBA Identifier,
    pub /: *mut *mut fdmi_port_list_s port_list; / Registered Port List,
    pub /: *mut *mut fdmi_hba_attr_s hba_attr_blk; / HBA attribute block,
}

//
// FDMI Register Port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdmi_rprt_s {
    pub /: *mut *mut wwn_t hba_id; / HBA Identifier,
    pub /: *mut *mut wwn_t port_name; / Port wwn,
    pub /: *mut *mut fdmi_port_attr_s port_attr_blk; / Port Attr Block,
}

//
// FDMI Register Port Attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdmi_rpa_s {
    pub /: *mut *mut wwn_t port_name; / port wwn,
    pub /: *mut *mut fdmi_port_attr_s port_attr_blk; / Port Attr Block,
}

