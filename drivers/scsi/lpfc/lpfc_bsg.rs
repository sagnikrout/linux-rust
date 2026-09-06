//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_bsg.h
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
// This file is part of the Emulex Linux Device Driver for
// Fibre Channel Host Bus Adapters.
// Copyright (C) 2017-2024 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2010-2015 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General
// Public License as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful.
// ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
// WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT, ARE
// DISCLAIMED, EXCEPT TO THE EXTENT THAT SUCH DISCLAIMERS ARE HELD
// TO BE LEGALLY INVALID.  See the GNU General Public License for
// more details, a copy of which can be found in the file COPYING
// included with this package.
//
// bsg definitions
// No pointers to user data are allowed, all application buffers and sizes will
// derived through the bsg interface.
//
// These are the vendor unique structures passed in using the bsg
// FC_BSG_HST_VENDOR message code type.
//
pub const LPFC_BSG_VENDOR_SET_CT_EVENT: c_int = 1;
pub const LPFC_BSG_VENDOR_GET_CT_EVENT: c_int = 2;
pub const LPFC_BSG_VENDOR_SEND_MGMT_RESP: c_int = 3;
pub const LPFC_BSG_VENDOR_DIAG_MODE: c_int = 4;
pub const LPFC_BSG_VENDOR_DIAG_RUN_LOOPBACK: c_int = 5;
pub const LPFC_BSG_VENDOR_GET_MGMT_REV: c_int = 6;
pub const LPFC_BSG_VENDOR_MBOX: c_int = 7;
pub const LPFC_BSG_VENDOR_DIAG_MODE_END: c_int = 10;
pub const LPFC_BSG_VENDOR_LINK_DIAG_TEST: c_int = 11;
pub const LPFC_BSG_VENDOR_FORCED_LINK_SPEED: c_int = 14;
pub const LPFC_BSG_VENDOR_RAS_GET_LWPD: c_int = 16;
pub const LPFC_BSG_VENDOR_RAS_GET_FWLOG: c_int = 17;
pub const LPFC_BSG_VENDOR_RAS_GET_CONFIG: c_int = 18;
pub const LPFC_BSG_VENDOR_RAS_SET_CONFIG: c_int = 19;
pub const LPFC_BSG_VENDOR_GET_TRUNK_INFO: c_int = 20;
pub const LPFC_BSG_VENDOR_GET_CGNBUF_INFO: c_int = 21;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_ct_event {
    pub command: u32,
    pub type_mask: u32,
    pub ev_req_id: u32,
    pub ev_reg_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_ct_event {
    pub command: u32,
    pub ev_reg_id: u32,
    pub ev_req_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_ct_event_reply {
    pub immed_data: u32,
    pub type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct send_mgmt_resp {
    pub command: u32,
    pub tag: u32,
}

pub const DISABLE_LOOP_BACK: c_uint = 0x0 /* disables loop back */;
pub const INTERNAL_LOOP_BACK: c_uint = 0x1 /* adapter short cuts the loop internally */;
pub const EXTERNAL_LOOP_BACK: c_uint = 0x2 /* requires an external loopback plug */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag_mode_set {
    pub command: u32,
    pub type: u32,
    pub timeout: u32,
    pub physical_link: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sli4_link_diag {
    pub command: u32,
    pub timeout: u32,
    pub test_id: u32,
    pub loops: u32,
    pub test_version: u32,
    pub error_action: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag_mode_test {
    pub command: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag_status {
    pub mbox_status: u32,
    pub shdr_status: u32,
    pub shdr_add_status: u32,
}

pub const LPFC_WWNN_TYPE: c_int = 0;
pub const LPFC_WWPN_TYPE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_mgmt_rev {
    pub command: u32,
}

pub const MANAGEMENT_MAJOR_REV: c_int = 1;
pub const MANAGEMENT_MINOR_REV: c_int = 1;
// the MgmtRevInfo structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MgmtRevInfo {
    pub a_Major: u32,
    pub a_Minor: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_mgmt_rev_reply {
    pub info: MgmtRevInfo,
}

// BSG mailbox request header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfc_mbox_req {
    pub command: u32,
    pub mbOffset: u32,
    pub inExtWLen: u32,
    pub outExtWLen: u32,
    pub extMboxTag: u32,
    pub extSeqNum: u32,
}

//
// macros and data structures for handling sli-config mailbox command
// pass-through support, this header file is shared between user and
// kernel spaces, note the set of macros are duplicates from lpfc_hw4.h,
// with macro names prefixed with bsg_, as the macros defined in
// lpfc_hw4.h are not accessible from user space.
//
// Macros to deal with bit fields. Each bit field must have 3 #defines
// associated with it (_SHIFT, _MASK, and _WORD).
// EG. For a bit field that is in the 7th bit of the "field4" field of a
// structure and is 2 bits in size the following #defines must exist:
// struct temp {
// uint32_t        field1;
// uint32_t        field2;
// uint32_t        field3;
// uint32_t        field4;
// #define example_bit_field_SHIFT         7
// #define example_bit_field_MASK          0x03
// #define example_bit_field_WORD          field4
// uint32_t        field5;
// };
// Then the macros below may be used to get or set the value of that field.
// EG. To get the value of the bit field from the above example:
// struct temp t1;
// value = bsg_bf_get(example_bit_field, &t1);
// And then to set that bit field:
// bsg_bf_set(example_bit_field, &t1, 2);
// Or clear that bit field:
// bsg_bf_set(example_bit_field, &t1, 0);
//

//
// The sli_config structure specified here is based on the following
// restriction:
//
// -- SLI_CONFIG EMB=0, carrying MSEs, will carry subcommands without
// carrying HBD.
// -- SLI_CONFIG EMB=1, not carrying MSE, will carry subcommands with or
// without carrying HBDs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_config_mse {
    pub pa_lo: u32,
    pub pa_hi: u32,
    pub buf_len: u32,
pub const lpfc_mbox_sli_config_mse_len_SHIFT: c_int = 0;
pub const lpfc_mbox_sli_config_mse_len_MASK: c_uint = 0xffffff;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_config_hbd {
    pub buf_len: u32,
pub const lpfc_mbox_sli_config_ecmn_hbd_len_SHIFT: c_int = 0;
pub const lpfc_mbox_sli_config_ecmn_hbd_len_MASK: c_uint = 0xffffff;

    pub pa_lo: u32,
    pub pa_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_config_hdr {
    pub word1: u32,
pub const lpfc_mbox_hdr_emb_SHIFT: c_int = 0;
pub const lpfc_mbox_hdr_emb_MASK: c_uint = 0x00000001;

pub const lpfc_mbox_hdr_mse_cnt_SHIFT: c_int = 3;
pub const lpfc_mbox_hdr_mse_cnt_MASK: c_uint = 0x0000001f;

    pub payload_length: u32,
    pub tag_lo: u32,
    pub tag_hi: u32,
    pub reserved5: u32,
}

pub const LPFC_CSF_BOOT_DEV: c_uint = 0x1D;
pub const LPFC_CSF_QUERY: c_int = 0;
pub const LPFC_CSF_SAVE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_config_emb0_subsys {
    pub sli_config_hdr: lpfc_sli_config_hdr,
pub const LPFC_MBX_SLI_CONFIG_MAX_MSE: c_int = 19;
    pub mse: [lpfc_sli_config_mse; LPFC_MBX_SLI_CONFIG_MAX_MSE],
    pub padding: u32,
    pub word64: u32,
pub const lpfc_emb0_subcmnd_opcode_SHIFT: c_int = 0;
pub const lpfc_emb0_subcmnd_opcode_MASK: c_uint = 0xff;

pub const lpfc_emb0_subcmnd_subsys_SHIFT: c_int = 8;
pub const lpfc_emb0_subcmnd_subsys_MASK: c_uint = 0xff;

// Subsystem FCOE (0x0C) OpCodes
pub const SLI_CONFIG_SUBSYS_FCOE: c_uint = 0x0C;
pub const FCOE_OPCODE_READ_FCF: c_uint = 0x08;
pub const FCOE_OPCODE_ADD_FCF: c_uint = 0x09;
pub const FCOE_OPCODE_SET_DPORT_MODE: c_uint = 0x27;
pub const FCOE_OPCODE_GET_DPORT_RESULTS: c_uint = 0x28;
    pub /: *mut *mut uint32_t timeout; / comn_set_feature timeout,
    pub /: *mut *mut uint32_t request_length; / comn_set_feature request len,
    pub /: *mut *mut uint32_t version; / comn_set_feature version,
    pub /: *mut *mut uint32_t word68; / comn_set_feature feature,
pub const lpfc_emb0_subcmnd_csf_feat_SHIFT: c_int = 0;
pub const lpfc_emb0_subcmnd_csf_feat_MASK: c_uint = 0xffffffff;

pub const lpfc_emb0_subcmnd_rd_obj_des_rd_len_SHIFT: c_int = 0;
pub const lpfc_emb0_subcmnd_rd_obj_des_rd_len_MASK: c_uint = 0x00ffffff;

    pub /: *mut *mut uint32_t word69; / comn_set_feature parameter len,
    pub /: *mut *mut uint32_t word70; / comn_set_feature parameter val0,
pub const lpfc_emb0_subcmnd_csf_p0_SHIFT: c_int = 0;
pub const lpfc_emb0_subcmnd_csf_p0_MASK: c_uint = 0x3;
    pub reserved71: [u32; 25],
    pub /: *mut *mut uint32_t word96; / rd_obj hbd_count,
pub const lpfc_emb0_subcmnd_rd_obj_hbd_cnt_SHIFT: c_int = 0;
pub const lpfc_emb0_subcmnd_rd_obj_hbd_cnt_MASK: c_uint = 0xffffffff;

pub const LPFC_EMB0_MAX_RD_OBJ_HBD_CNT: c_int = 31;
    pub hbd: [lpfc_sli_config_hbd; LPFC_EMB0_MAX_RD_OBJ_HBD_CNT],
    pub word190: u32,
    pub word191: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_config_emb1_subsys {
    pub sli_config_hdr: lpfc_sli_config_hdr,
    pub word6: u32,
pub const lpfc_emb1_subcmnd_opcode_SHIFT: c_int = 0;
pub const lpfc_emb1_subcmnd_opcode_MASK: c_uint = 0xff;

pub const lpfc_emb1_subcmnd_subsys_SHIFT: c_int = 8;
pub const lpfc_emb1_subcmnd_subsys_MASK: c_uint = 0xff;

// Subsystem COMN (0x01) OpCodes
pub const SLI_CONFIG_SUBSYS_COMN: c_uint = 0x01;
pub const COMN_OPCODE_GET_PROFILE_CONFIG: c_uint = 0xA4;
pub const COMN_OPCODE_READ_OBJECT: c_uint = 0xAB;
pub const COMN_OPCODE_WRITE_OBJECT: c_uint = 0xAC;
pub const COMN_OPCODE_READ_OBJECT_LIST: c_uint = 0xAD;
pub const COMN_OPCODE_DELETE_OBJECT: c_uint = 0xAE;
pub const COMN_OPCODE_SET_FEATURES: c_uint = 0xBF;
pub const COMN_OPCODE_GET_CNTL_ADDL_ATTRIBUTES: c_uint = 0x79;
pub const COMN_OPCODE_GET_CNTL_ATTRIBUTES: c_uint = 0x20;
    pub timeout: u32,
    pub request_length: u32,
    pub word9: u32,
pub const lpfc_subcmnd_version_SHIFT: c_int = 0;
pub const lpfc_subcmnd_version_MASK: c_uint = 0xff;

    pub word10: u32,
pub const lpfc_subcmnd_ask_rd_len_SHIFT: c_int = 0;
pub const lpfc_subcmnd_ask_rd_len_MASK: c_uint = 0xffffff;

    pub rd_offset: u32,
    pub obj_name: [u32; 26],
    pub hbd_count: u32,
pub const LPFC_MBX_SLI_CONFIG_MAX_HBD: c_int = 8;
    pub hbd: [lpfc_sli_config_hbd; LPFC_MBX_SLI_CONFIG_MAX_HBD],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli_config_mbox {
    pub word0: u32,
pub const lpfc_mqe_status_SHIFT: c_int = 16;
pub const lpfc_mqe_status_MASK: c_uint = 0x0000FFFF;

pub const lpfc_mqe_command_SHIFT: c_int = 8;
pub const lpfc_mqe_command_MASK: c_uint = 0x000000FF;

    pub sli_config_emb0_subsys: lpfc_sli_config_emb0_subsys,
    pub sli_config_emb1_subsys: lpfc_sli_config_emb1_subsys,
    pub un: },
}

pub const LPFC_FORCED_LINK_SPEED_NOT_SUPPORTED: c_int = 0;
pub const LPFC_FORCED_LINK_SPEED_SUPPORTED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_forced_link_speed_support {
    pub command: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct forced_link_speed_support_reply {
    pub supported: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_bsg_ras_req {
    pub command: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_bsg_get_fwlog_req {
    pub command: u32,
    pub read_size: u32,
    pub read_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_bsg_get_ras_lwpd {
    pub offset: u32,
    pub wrap_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_bsg_set_ras_config_req {
    pub command: u32,
    pub action: u8,
pub const LPFC_RASACTION_STOP_LOGGING: c_uint = 0x00;
pub const LPFC_RASACTION_START_LOGGING: c_uint = 0x01;
    pub log_level: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_bsg_get_ras_config_reply {
    pub state: u8,
pub const LPFC_RASLOG_STATE_STOPPED: c_uint = 0x00;
pub const LPFC_RASLOG_STATE_RUNNING: c_uint = 0x01;
    pub log_level: u8,
    pub log_buff_sz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_trunk_info {
    pub word0: u32,
pub const lpfc_trunk_info_link_status_SHIFT: c_int = 0;
pub const lpfc_trunk_info_link_status_MASK: c_int = 1;

pub const lpfc_trunk_info_trunk_active0_SHIFT: c_int = 8;
pub const lpfc_trunk_info_trunk_active0_MASK: c_int = 1;

pub const lpfc_trunk_info_trunk_active1_SHIFT: c_int = 9;
pub const lpfc_trunk_info_trunk_active1_MASK: c_int = 1;

pub const lpfc_trunk_info_trunk_active2_SHIFT: c_int = 10;
pub const lpfc_trunk_info_trunk_active2_MASK: c_int = 1;

pub const lpfc_trunk_info_trunk_active3_SHIFT: c_int = 11;
pub const lpfc_trunk_info_trunk_active3_MASK: c_int = 1;

pub const lpfc_trunk_info_trunk_config0_SHIFT: c_int = 12;
pub const lpfc_trunk_info_trunk_config0_MASK: c_int = 1;

pub const lpfc_trunk_info_trunk_config1_SHIFT: c_int = 13;
pub const lpfc_trunk_info_trunk_config1_MASK: c_int = 1;

pub const lpfc_trunk_info_trunk_config2_SHIFT: c_int = 14;
pub const lpfc_trunk_info_trunk_config2_MASK: c_int = 1;

pub const lpfc_trunk_info_trunk_config3_SHIFT: c_int = 15;
pub const lpfc_trunk_info_trunk_config3_MASK: c_int = 1;

    pub port_speed: u16,
    pub logical_speed: u16,
    pub reserved3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_trunk_info_req {
    pub command: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_cgnbuf_info_req {
    pub command: u32,
    pub read_size: u32,
    pub reset: u32,
pub const LPFC_BSG_CGN_RESET_STAT: c_int = 1;
}

// driver only
pub const SLI_CONFIG_NOT_HANDLED: c_int = 0;
pub const SLI_CONFIG_HANDLED: c_int = 1;
