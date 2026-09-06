//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/snic/snic_fwint.h
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
// Copyright 2014 Cisco Systems, Inc.  All rights reserved.

pub const LUN_ADDR_LEN: c_int = 8;
//
// Command entry type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_io_type {
//
// Initiator request types
//
    SNIC_REQ_REPORT_TGTS = 0x2,	/* Report Targets */
    SNIC_REQ_ICMND,			/* Initiator command for SCSI IO */
    SNIC_REQ_ITMF,			/* Initiator command for Task Mgmt */
    SNIC_REQ_HBA_RESET,		/* SNIC Reset */
    SNIC_REQ_EXCH_VER,		/* Exchange Version Information */
    SNIC_REQ_TGT_INFO,		/* Backend/Target Information */
    SNIC_REQ_BOOT_LUNS,

//
// Response type
//
    SNIC_RSP_REPORT_TGTS_CMPL = 0x12,/* Report Targets Completion */
    SNIC_RSP_ICMND_CMPL,		/* SCSI IO Completion */
    SNIC_RSP_ITMF_CMPL,		/* Task Management Completion */
    SNIC_RSP_HBA_RESET_CMPL,	/* SNIC Reset Completion */
    SNIC_RSP_EXCH_VER_CMPL,		/* Exchange Version Completion*/
    SNIC_RSP_BOOT_LUNS_CMPL,

//
// Misc Request types
//
    SNIC_MSG_ACK = 0x80,		/* Ack: snic_notify_msg */
    SNIC_MSG_ASYNC_EVNOTIFY,	/* Asynchronous Event Notification */
}

//
// Header status codes from firmware
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_io_status {
    SNIC_STAT_IO_SUCCESS = 0,	/* request was successful */

//
// If a request to the fw is rejected, the original request header
// will be returned with the status set to one of the following:
//
    SNIC_STAT_INVALID_HDR,	/* header contains invalid data */
    SNIC_STAT_OUT_OF_RES,	/* out of resources to complete request */
    SNIC_STAT_INVALID_PARM,	/* some parameter in request is not valid */
    SNIC_STAT_REQ_NOT_SUP,	/* req type is not supported */
    SNIC_STAT_IO_NOT_FOUND,	/* requested IO was not found */

//
// Once a request is processed, the fw will usually return
// a cmpl message type. In cases where errors occurred,
// the header status would be filled in with one of the following:
//
    SNIC_STAT_ABORTED,		/* req was aborted */
    SNIC_STAT_TIMEOUT,		/* req was timed out */
    SNIC_STAT_SGL_INVALID,		/* req was aborted due to sgl error */
    SNIC_STAT_DATA_CNT_MISMATCH,	/*recv/sent more/less data than expec */
    SNIC_STAT_FW_ERR,		/* req was terminated due to fw error */
    SNIC_STAT_ITMF_REJECT,		/* itmf req was rejected by target */
    SNIC_STAT_ITMF_FAIL,		/* itmf req was failed */
    SNIC_STAT_ITMF_INCORRECT_LUN,	/* itmf req has incorrect LUN id*/
    SNIC_STAT_CMND_REJECT,		/* req was invalid and rejected */
    SNIC_STAT_DEV_OFFLINE,		/* req sent to offline device */
    SNIC_STAT_NO_BOOTLUN,
    SNIC_STAT_SCSI_ERR,		/* SCSI error returned by Target. */
    SNIC_STAT_NOT_READY,		/* sNIC Subsystem is not ready */
    SNIC_STAT_FATAL_ERROR,		/* sNIC is in unrecoverable state */
}

//
// snic_io_hdr : host <--> firmware
//
// for any other message that will be queued to firmware should
// have the following request header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_io_hdr {
    pub hid: __le32,
    pub /: *mut *mut __le32 cmnd_id; / tag here,
    pub /: *mut *mut ulong init_ctx; / initiator context,
    pub /: *mut *mut u8 type; / request/response type,
    pub /: *mut *mut u8 status; / header status entry,
    pub RoCE*/: *mut *mut u8 protocol; / Protocol specific, may needed for,
    pub flags: u8,
    pub sg_cnt: __le16,
    pub resvd: u16,
}

// auxillary funciton for encoding the snic_io_hdr
// auxillary funciton for decoding the snic_io_hdr
// typ = hdr->type;
// stat = hdr->status;
// hid = le32_to_cpu(hdr->hid);
// cmnd_id = le32_to_cpu(hdr->cmnd_id);
// ctx = hdr->init_ctx;
//
// snic_host_info: host -> firmware
//
// Used for sending host information to firmware, and request fw version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_exch_ver_req {
    pub /: *mut *mut __le32 drvr_ver; / for debugging, when fw dump captured,
    pub /: *mut *mut __le32 os_type; / for OS specific features,
}

//
// os_type flags
// Bit 0-7 : OS information
// Bit 8-31: Feature/Capability Information
//
pub const SNIC_OS_LINUX: c_uint = 0x1;
pub const SNIC_OS_WIN: c_uint = 0x2;
pub const SNIC_OS_ESX: c_uint = 0x3;
//
// HBA Capabilities
// Bit 1: Reserved.
// Bit 2: Dynamic Discovery of LUNs.
// Bit 3: Async event notifications on tgt online/offline events.
// Bit 4: IO timeout support in FW.
// Bit 5-31: Reserved.
//
pub const SNIC_HBA_CAP_DDL: c_uint = 0x02	/* Supports Dynamic Discovery of LUNs */;
pub const SNIC_HBA_CAP_AEN: c_uint = 0x04	/* Supports Async Event Noitifcation */;
pub const SNIC_HBA_CAP_TMO: c_uint = 0x08	/* Supports IO timeout in FW */;
//
// snic_exch_ver_rsp : firmware -> host
//
// Used by firmware to send response to version request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_exch_ver_rsp {
    pub version: __le32,
    pub hid: __le32,
    pub /: *mut *mut __le32 max_concur_ios; / max concurrent ios,
    pub /: *mut *mut __le32 max_sgs_per_cmd; / max sgls per IO,
    pub /: *mut *mut __le32 max_io_sz; / max io size supported,
    pub /: *mut *mut __le32 hba_cap; / hba capabilities,
    pub /: *mut *mut __le32 max_tgts; / max tgts supported,
    pub /: *mut *mut __le16 io_timeout; / FW extended timeout,
    pub rsvd: u16,
}

//
// snic_report_tgts : host -> firmware request
//
// Used by the host to request list of targets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_report_tgts {
    pub sg_cnt: __le16,
    pub /: *mut *mut __le16 flags; / specific flags from fw,
    pub _resvd: [u8; 4],
    pub /: *mut *mut __le64 sg_addr; / Points to SGL,
    pub sense_addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_type {
    SNIC_NONE = 0x0,
    SNIC_DAS,
    SNIC_SAN,
}

// Report Target Response
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_tgt_type {
    SNIC_TGT_NONE = 0x0,
    SNIC_TGT_DAS,	/* DAS Target */
    SNIC_TGT_SAN,	/* SAN Target */
}

// target id format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_tgt_id {
    pub /: *mut *mut __le32 tgt_id; / target id,
    pub /: *mut *mut __le16 tgt_type; / tgt type,
    pub /: *mut *mut __le16 vnic_id; / corresponding vnic id,
}

//
// snic_report_tgts_cmpl : firmware -> host response
//
// Used by firmware to send response to Report Targets request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_report_tgts_cmpl {
    pub /: *mut *mut __le32 tgt_cnt; / Number of Targets accessible,
    pub _resvd: u32,
}

//
// Command flags
//
// Bit 0: Read flags
// Bit 1: Write flag
// Bit 2: ESGL - sg/esg array contains extended sg
// ESGE - is a host buffer contains sg elements
// Bit 3-4: Task Attributes
// 00b - simple
// 01b - head of queue
// 10b - ordered
// Bit 5-7: Priority - future use
// Bit 8-15: Reserved
//
pub const SNIC_ICMND_WR: c_uint = 0x01	/* write command */;
pub const SNIC_ICMND_RD: c_uint = 0x02	/* read command */;
pub const SNIC_ICMND_ESGL: c_uint = 0x04	/* SGE/ESGE array contains valid data*/;
//
// Priority/Task Attribute settings
//

//
// snic_icmnd : host-> firmware request
//
// used for sending out an initiator SCSI 16/32-byte command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_icmnd {
    pub /: *mut *mut __le16 sg_cnt; / Number of SG Elements,
    pub /: *mut *mut __le16 flags; / flags,
    pub /: *mut *mut __le32 sense_len; / Sense buffer length,
    pub /: *mut *mut __le64 tgt_id; / Destination Target ID,
    pub /: *mut *mut __le64 lun_id; / Destination LUN ID,
    pub cdb_len: u8,
    pub _resvd: u8,
    pub io*/: *mut *mut __le16 time_out; / ms time for Res allocations fw to handle,
    pub /: *mut *mut __le32 data_len; / Total number of bytes to be transferred,
    pub cdb: [u8; SNIC_CDB_LEN],
    pub /: *mut *mut __le64 sg_addr; / Points to SG List,
    pub /: *mut *mut __le64 sense_addr; / Sense buffer address,
}

// Response flags
// Bit 0: Under run
// Bit 1: Over Run
// Bit 2-7: Reserved
//
pub const SNIC_ICMND_CMPL_UNDR_RUN: c_uint = 0x01	/* resid under and valid */;
pub const SNIC_ICMND_CMPL_OVER_RUN: c_uint = 0x02	/* resid over and valid */;
//
// snic_icmnd_cmpl: firmware -> host response
//
// Used for sending the host a response to an icmnd (initiator command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_icmnd_cmpl {
    pub /: *mut *mut u8 scsi_status; / value as per SAM,
    pub flags: u8,
    pub /: *mut *mut __le16 sense_len; / Sense Length,
    pub /: *mut *mut __le32 resid; / Residue : # bytes under or over run,
}

//
// snic_itmf: host->firmware request
//
// used for requesting the firmware to abort a request and/or send out
// a task management function
//
// the req_id field is valid in case of abort task and clear task
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_itmf {
    pub /: *mut *mut u8 tm_type; / SCSI Task Management request,
    pub resvd: u8,
    pub /: *mut *mut __le16 flags; / flags,
    pub /: *mut *mut __le32 req_id; / Command id of snic req to be aborted,
    pub /: *mut *mut __le64 tgt_id; / Target ID,
    pub /: *mut *mut __le64 lun_id; / Destination LUN ID,
    pub /: *mut *mut __le16 timeout; / in sec,
}

//
// Task Management Request
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_itmf_tm_type {
    SNIC_ITMF_ABTS_TASK = 0x01,	/* Abort Task */
    SNIC_ITMF_ABTS_TASK_SET,	/* Abort Task Set */
    SNIC_ITMF_CLR_TASK,		/* Clear Task */
    SNIC_ITMF_CLR_TASKSET,		/* Clear Task Set */
    SNIC_ITMF_LUN_RESET,		/* Lun Reset */
    SNIC_ITMF_ABTS_TASK_TERM,	/* Supported for SAN Targets */
}

//
// snic_itmf_cmpl: firmware -> host resposne
//
// used for sending the host a response for a itmf request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_itmf_cmpl {
    pub /: *mut *mut __le32 nterminated; / # IOs terminated as a result of tmf,
    pub /: *mut *mut u8 flags; / flags,
    pub _resvd: [u8; 3],
}

//
// itmfl_cmpl flags
// Bit 0 : 1 - Num terminated field valid
// Bit 1 - 7 : Reserved
//
pub const SNIC_NUM_TERM_VALID: c_uint = 0x01	/* Number of IOs terminated */;
//
// snic_hba_reset: host -> firmware request
//
// used for requesting firmware to reset snic
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_hba_reset {
    pub /: *mut *mut __le16 flags; / flags,
    pub _resvd: [u8; 6],
}

//
// snic_hba_reset_cmpl: firmware -> host response
//
// Used by firmware to respond to the host's hba reset request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_hba_reset_cmpl {
    pub added*/: *mut *mut u8 flags; / flags : more info needs to be,
    pub _resvd: [u8; 7],
}

//
// snic_notify_msg: firmware -> host response
//
// Used by firmware to notify host of the last work queue entry received
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_notify_msg {
    pub /: *mut *mut __le32 wqe_num; / wq entry number,
    pub /: *mut *mut u8 flags; / flags, macros,
    pub _resvd: [u8; 4],
}

// snic_async_evnotify: firmware -> host notification
//
// Used by firmware to notify the host about configuration/state changes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_async_evnotify {
    pub FLS_EVENT_DESC: u8,
    pub /: *mut *mut u8 vnic; / vnic id,
    pub _resvd: [u8; 2],
    pub /: *mut *mut __le32 ev_id; / Event ID,
    pub /: *mut *mut u8 ev_data[SNIC_EVDATA_LEN]; / Event Data,
    pub _resvd2: [u8; 4],
}

// async event flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snic_ev_type {
    SNIC_EV_TGT_OFFLINE = 0x01, /* Target Offline, PL contains TGT ID */
    SNIC_EV_TGT_ONLINE,	/* Target Online, PL contains TGT ID */
    SNIC_EV_LUN_OFFLINE,	/* LUN Offline, PL contains LUN ID */
    SNIC_EV_LUN_ONLINE,	/* LUN Online, PL contains LUN ID */
    SNIC_EV_CONF_CHG,	/* Dev Config/Attr Change Event */
    SNIC_EV_TGT_ADDED,	/* Target Added */
    SNIC_EV_TGT_DELTD,	/* Target Del'd, PL contains TGT ID */
    SNIC_EV_LUN_ADDED,	/* LUN Added */
    SNIC_EV_LUN_DELTD,	/* LUN Del'd, PL cont. TGT & LUN ID */

    SNIC_EV_DISC_CMPL = 0x10, /* Discovery Completed Event */
}

// Payload 88 bytes = 128 - 24 - 16

//
// snic_host_req: host -> firmware request
//
// Basic structure for all snic requests that are sent from the host to
// firmware. They are 128 bytes in size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_host_req {
    pub /: *mut *mut u64 ctrl_data[2]; /16 bytes - Control Data,
    pub hdr: snic_io_hdr,
//
// Entry specific space, last byte contains color
//
    pub buf: [u8; SNIC_HOST_REQ_PAYLOAD],
//
// Exchange firmware version
//
    pub exch_ver: snic_exch_ver_req,
// report targets
    pub rpt_tgts: snic_report_tgts,
// io request
    pub icmnd: snic_icmnd,
// task management request
    pub itmf: snic_itmf,
// hba reset
    pub reset: snic_hba_reset,
    pub u: },
    pub req_pa: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_fw_req {
    pub hdr: snic_io_hdr,
//
// Entry specific space, last byte contains color
//
    pub snic_io_hdr)]: u8 buf[SNIC_FW_REQ_LEN - sizeof(struct,
// Exchange Version Response
    pub exch_ver_cmpl: snic_exch_ver_rsp,
// Report Targets Response
    pub rpt_tgts_cmpl: snic_report_tgts_cmpl,
// scsi response
    pub icmnd_cmpl: snic_icmnd_cmpl,
// task management response
    pub itmf_cmpl: snic_itmf_cmpl,
// hba reset response
    pub reset_cmpl: snic_hba_reset_cmpl,
// notify message
    pub ack: snic_notify_msg,
// async notification event
    pub async_ev: snic_async_evnotify,
    pub u: },
}

//
// Auxillary macro to verify specific snic req/cmpl structures
// to ensure that it will be aligned to 64 bit, and not using
// color bit field
//
// Macro flag: #define VERIFY_REQ_SZ(x)
// Macro flag: #define VERIFY_CMPL_SZ(x)
//
// Access routines to encode and decode the color bit, which is the most
// significant bit of the structure.
//
// c |= 0x80;
// c &= ~0x80;
// color = *c >> 7;
// Make sure color bit is read from desc *before* other fields
// are read from desc. Hardware guarantees color bit is last
// bit (byte) written. Adding the rmb() prevents the compiler
// and/or CPU from reordering the reads which would potentially
// result in reading stale values.
//
