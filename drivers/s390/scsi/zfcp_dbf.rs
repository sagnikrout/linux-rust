//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/scsi/zfcp_dbf.h
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


// SPDX-License-Identifier: GPL-2.0
//
// zfcp device driver
// debug feature declarations
//
// Copyright IBM Corp. 2008, 2026
//

pub const ZFCP_DBF_TAG_LEN: c_int = 7;
pub const ZFCP_DBF_INVALID_WWPN: c_uint = 0x0000000000000000ull;
pub const ZFCP_DBF_INVALID_LUN: c_uint = 0xFFFFFFFFFFFFFFFFull;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zfcp_dbf_pseudo_erp_act_type {
    ZFCP_PSEUDO_ERP_ACTION_RPORT_ADD = 0xff,
    ZFCP_PSEUDO_ERP_ACTION_RPORT_DEL = 0xfe,
}

//
// struct zfcp_dbf_rec_trigger - trace record for triggered recovery action
// @ready: number of ready recovery actions
// @running: number of running recovery actions
// @want: wanted recovery action
// @need: needed recovery action
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_rec_trigger {
    pub ready: u32,
    pub running: u32,
    pub want: u8,
    pub need: u8,
    pub __packed: },
//
// struct zfcp_dbf_rec_running - trace record for running recovery
// @fsf_req_id: request id for fsf requests
// @rec_status: status of the fsf request
// @rec_step: current step of the recovery action
// @rec_action: ERP action type
// @rec_count: recoveries including retries for particular @rec_action
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_rec_running {
    pub fsf_req_id: u64,
    pub rec_status: u32,
    pub rec_step: u16,
    pub rec_action: u8,
    pub rec_count: u8,
    pub __packed: },
//
// enum zfcp_dbf_rec_id - recovery trace record id
// @ZFCP_DBF_REC_TRIG: triggered recovery identifier
// @ZFCP_DBF_REC_RUN: running recovery identifier
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zfcp_dbf_rec_id {
    ZFCP_DBF_REC_TRIG	= 1,
    ZFCP_DBF_REC_RUN	= 2,
}

//
// struct zfcp_dbf_rec - trace record for error recovery actions
// @id: unique number of recovery record type
// @tag: identifier string specifying the location of initiation
// @lun: logical unit number
// @wwpn: word wide port number
// @d_id: destination ID
// @adapter_status: current status of the adapter
// @port_status: current status of the port
// @lun_status: current status of the lun
// @u: record type specific data
// @u.trig: structure zfcp_dbf_rec_trigger
// @u.run: structure zfcp_dbf_rec_running
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_rec {
    pub id: u8,
    pub tag: [c_char; ZFCP_DBF_TAG_LEN],
    pub lun: u64,
    pub wwpn: u64,
    pub d_id: u32,
    pub adapter_status: u32,
    pub port_status: u32,
    pub lun_status: u32,
    pub trig: zfcp_dbf_rec_trigger,
    pub run: zfcp_dbf_rec_running,
    pub u: },
    pub __packed: },
//
// enum zfcp_dbf_san_id - SAN trace record identifier
// @ZFCP_DBF_SAN_REQ: request trace record id
// @ZFCP_DBF_SAN_RES: response trace record id
// @ZFCP_DBF_SAN_ELS: extended link service record id
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zfcp_dbf_san_id {
    ZFCP_DBF_SAN_REQ	= 1,
    ZFCP_DBF_SAN_RES	= 2,
    ZFCP_DBF_SAN_ELS	= 3,
}

// struct zfcp_dbf_san - trace record for SAN requests and responses
// @id: unique number of recovery record type
// @tag: identifier string specifying the location of initiation
// @fsf_req_id: request id for fsf requests
// @payload: unformatted information related to request/response
// @d_id: destination id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_san {
    pub id: u8,
    pub tag: [c_char; ZFCP_DBF_TAG_LEN],
    pub fsf_req_id: u64,
    pub d_id: u32,
    pub payload: [c_char; ZFCP_DBF_SAN_MAX_PAYLOAD],
    pub pl_len: u16,
    pub __packed: },
//
// struct zfcp_dbf_hba_res - trace record for hba responses
// @req_issued: timestamp when request was issued
// @prot_status: protocol status
// @prot_status_qual: protocol status qualifier
// @fsf_status: fsf status
// @fsf_status_qual: fsf status qualifier
// @port_handle: handle for port
// @lun_handle: handle for LUN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_hba_res {
    pub req_issued: u64,
    pub prot_status: u32,
    pub prot_status_qual: [u8; FSF_PROT_STATUS_QUAL_SIZE],
    pub fsf_status: u32,
    pub fsf_status_qual: [u8; FSF_STATUS_QUALIFIER_SIZE],
    pub port_handle: u32,
    pub lun_handle: u32,
    pub plogi_len: u32,
    pub prli_len: u32,
    pub __packed: },
//
// struct zfcp_dbf_hba_uss - trace record for unsolicited status
// @status_type: type of unsolicited status
// @status_subtype: subtype of unsolicited status
// @d_id: destination ID
// @lun: logical unit number
// @queue_designator: queue designator
// @length: buffer length
// @res1: reserved field 1
// @res2: reserved field 2
// @class: class of service
// @res3: reserved field 3
// @s_id: source ID
// @res4: reserved field 4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_hba_uss {
    pub status_type: u32,
    pub status_subtype: u32,
    pub d_id: u32,
    pub lun: u64,
    pub queue_designator: u64,
    pub length: u32,
    pub res1: u32,
    pub res2: u8,
    pub class: u32,
    pub res3: u8,
    pub s_id: u32,
    pub res4: [u8; 20],
    pub __packed: },
//
// struct zfcp_dbf_hba_uas - trace record for sysfs unit add store
// @wwpn: remote port wwn
// @fcp_lun: FCP LUN
// @ret: return value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_hba_uas {
    pub wwpn: u64,
    pub fcp_lun: u64,
    pub ret: u32,
    pub __packed: },
//
// struct zfcp_dbf_hba_fces - trace record for FC Endpoint Security
// @req_issued: timestamp when request was issued
// @fsf_status: fsf status
// @port_handle: handle for port
// @wwpn: remote FC port WWPN
// @fc_security_old: old FC Endpoint Security
// @fc_security_new: new FC Endpoint Security
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_hba_fces {
    pub req_issued: u64,
    pub fsf_status: u32,
    pub port_handle: u32,
    pub wwpn: u64,
    pub fc_security_old: u32,
    pub fc_security_new: u32,
    pub __packed: },
//
// enum zfcp_dbf_hba_id - HBA trace record identifier
// @ZFCP_DBF_HBA_RES: response trace record
// @ZFCP_DBF_HBA_USS: unsolicited status trace record
// @ZFCP_DBF_HBA_BIT: bit error trace record
// @ZFCP_DBF_HBA_BASIC: basic adapter event, only trace tag, no other data
// @ZFCP_DBF_HBA_FCES: FC Endpoint Security trace record
// @ZFCP_DBF_HBA_UAS: unit add store trace record
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zfcp_dbf_hba_id {
    ZFCP_DBF_HBA_RES	= 1,
    ZFCP_DBF_HBA_USS	= 2,
    ZFCP_DBF_HBA_BIT	= 3,
    ZFCP_DBF_HBA_BASIC	= 4,
    ZFCP_DBF_HBA_FCES	= 5,
    ZFCP_DBF_HBA_UAS        = 6,
}

//
// struct zfcp_dbf_hba - common trace record for HBA records
// @id: unique number of recovery record type
// @tag: identifier string specifying the location of initiation
// @fsf_req_id: request id for fsf requests
// @fsf_req_status: status of fsf request
// @fsf_cmd: fsf command
// @fsf_seq_no: fsf sequence number
// @pl_len: length of payload stored as zfcp_dbf_pay
// @u: record type specific data
// @u.res:  data for fsf responses
// @u.uss:  data for unsolicited status buffer
// @u.be:   data for bit error unsolicited status buffer
// @u.fces: data for FC Endpoint Security
// @u.uas:  data for unit add store
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_hba {
    pub id: u8,
    pub tag: [c_char; ZFCP_DBF_TAG_LEN],
    pub fsf_req_id: u64,
    pub fsf_req_status: u32,
    pub fsf_cmd: u32,
    pub fsf_seq_no: u32,
    pub pl_len: u16,
    pub res: zfcp_dbf_hba_res,
    pub uss: zfcp_dbf_hba_uss,
    pub be: fsf_bit_error_payload,
    pub fces: zfcp_dbf_hba_fces,
    pub uas: zfcp_dbf_hba_uas,
    pub u: },
    pub __packed: },
//
// enum zfcp_dbf_scsi_id - scsi trace record identifier
// @ZFCP_DBF_SCSI_CMND: scsi command trace record
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zfcp_dbf_scsi_id {
    ZFCP_DBF_SCSI_CMND	= 1,
}

//
// struct zfcp_dbf_scsi - common trace record for SCSI records
// @id: unique number of recovery record type
// @tag: identifier string specifying the location of initiation
// @scsi_id: scsi device id
// @scsi_lun: scsi device logical unit number, low part of 64 bit, old 32 bit
// @scsi_result: scsi result
// @scsi_retries: current retry number of scsi request
// @scsi_allowed: allowed retries
// @fcp_rsp_info: FCP response info code
// @scsi_opcode: scsi opcode
// @fsf_req_id: request id of fsf request
// @host_scribble: LLD specific data attached to SCSI request
// @pl_len: length of payload stored as zfcp_dbf_pay
// @fcp_rsp: response for FCP request
// @scsi_lun_64_hi: scsi device logical unit number, high part of 64 bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_scsi {
    pub id: u8,
    pub tag: [c_char; ZFCP_DBF_TAG_LEN],
    pub scsi_id: u32,
    pub scsi_lun: u32,
    pub scsi_result: u32,
    pub scsi_retries: u8,
    pub scsi_allowed: u8,
    pub fcp_rsp_info: u8,
pub const ZFCP_DBF_SCSI_OPCODE: c_int = 16;
    pub scsi_opcode: [u8; ZFCP_DBF_SCSI_OPCODE],
    pub fsf_req_id: u64,
    pub host_scribble: u64,
    pub pl_len: u16,
    pub fcp_rsp: fcp_resp_with_ext,
    pub scsi_lun_64_hi: u32,
    pub __packed: },
//
// struct zfcp_dbf_pay - trace record for unformatted payload information
// @area: area this record is originated from
// @counter: ascending record number
// @fsf_req_id: request id of fsf request
// @data: unformatted data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf_pay {
    pub counter: u8,
    pub area: [c_char; ZFCP_DBF_TAG_LEN],
    pub fsf_req_id: u64,
pub const ZFCP_DBF_PAY_MAX_REC: c_uint = 0x100;
    pub data: [c_char; ZFCP_DBF_PAY_MAX_REC],
    pub __packed: },
//
// struct zfcp_dbf - main dbf trace structure
// @pay: reference to payload trace area
// @rec: reference to recovery trace area
// @hba: reference to hba trace area
// @san: reference to san trace area
// @scsi: reference to scsi trace area
// @pay_lock: lock protecting payload trace buffer
// @rec_lock: lock protecting recovery trace buffer
// @hba_lock: lock protecting hba trace buffer
// @san_lock: lock protecting san trace buffer
// @scsi_lock: lock protecting scsi trace buffer
// @pay_buf: pre-allocated buffer for payload
// @rec_buf: pre-allocated buffer for recovery
// @hba_buf: pre-allocated buffer for hba
// @san_buf: pre-allocated buffer for san
// @scsi_buf: pre-allocated buffer for scsi
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zfcp_dbf {
    pub pay: *mut debug_info_t,
    pub rec: *mut debug_info_t,
    pub hba: *mut debug_info_t,
    pub san: *mut debug_info_t,
    pub scsi: *mut debug_info_t,
    pub pay_lock: spinlock_t,
    pub rec_lock: spinlock_t,
    pub hba_lock: spinlock_t,
    pub san_lock: spinlock_t,
    pub scsi_lock: spinlock_t,
    pub pay_buf: zfcp_dbf_pay,
    pub rec_buf: zfcp_dbf_rec,
    pub hba_buf: zfcp_dbf_hba,
    pub san_buf: zfcp_dbf_san,
    pub scsi_buf: zfcp_dbf_scsi,
}

//
// zfcp_dbf_hba_fsf_resp_suppress - true if we should not trace by default
// @req: request that has been completed
//
// Returns true if FCP response with only benign residual under count.
//
// zfcp_dbf_hba_fsf_response - trace event for request completion
// @req: request that has been completed
//
// zfcp_dbf_scsi_result - trace event for SCSI command completion
// @scmd: SCSI command pointer
// @req: FSF request used to issue SCSI command
//
// zfcp_dbf_scsi_fail_send - trace event for failure to send SCSI command
// @scmd: SCSI command pointer
//
// zfcp_dbf_scsi_abort - trace event for SCSI command abort
// @tag: tag indicating success or failure of abort operation
// @scmd: SCSI command to be aborted
// @fsf_req: request containing abort (might be NULL)
//
// zfcp_dbf_scsi_devreset() - Trace event for Logical Unit or Target Reset.
// @tag: Tag indicating success or failure of reset operation.
// @sdev: Pointer to SCSI device as context for this event.
// @flag: Indicates type of reset (Target Reset, Logical Unit Reset).
// @fsf_req: Pointer to FSF request representing the TMF, or NULL.
//
// zfcp_dbf_scsi_nullcmnd() - trace NULLify of SCSI command in dev/tgt-reset.
// @scmnd: SCSI command that was NULLified.
// @fsf_req: request that owned @scmnd.
//
