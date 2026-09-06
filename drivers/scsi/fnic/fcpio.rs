//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/fcpio.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

//
// This header file includes all of the data structures used for
// communication by the host driver to the fcp firmware.
//
// Exchange and sequence id space allocated to the host driver
//
pub const FCPIO_HOST_EXCH_RANGE_START: c_uint = 0x1000;
pub const FCPIO_HOST_EXCH_RANGE_END: c_uint = 0x1fff;
pub const FCPIO_HOST_SEQ_ID_RANGE_START: c_uint = 0x80;
pub const FCPIO_HOST_SEQ_ID_RANGE_END: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_nvme_io_event {
    pub links: list_head,
    pub io_work: work_struct,
    pub arg1: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_io_event_s {
    pub links: list_head,
    pub io_work: work_struct,
    pub arg1: *mut c_void,
}

//
// Command entry type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcpio_type {
//
// Initiator request types
//
    FCPIO_ICMND_16 = 0x1,
    FCPIO_ICMND_32,
    FCPIO_ICMND_CMPL,
    FCPIO_ITMF,
    FCPIO_ITMF_CMPL,
    FCPIO_NVME_CMD,
    FCPIO_NVME_ERSP_HW_CMPL,
    FCPIO_NVME_ERSP_FW_CMPL,

//
// Target request types
//
    FCPIO_TCMND_16 = 0x11,
    FCPIO_TCMND_32,
    FCPIO_TDATA,
    FCPIO_TXRDY,
    FCPIO_TRSP,
    FCPIO_TDRSP_CMPL,
    FCPIO_TTMF,
    FCPIO_TTMF_ACK,
    FCPIO_TABORT,
    FCPIO_TABORT_CMPL,

//
// Misc request types
//
    FCPIO_ACK = 0x20,
    FCPIO_RESET,
    FCPIO_RESET_CMPL,
    FCPIO_FLOGI_REG,
    FCPIO_FLOGI_REG_CMPL,
    FCPIO_ECHO,
    FCPIO_ECHO_CMPL,
    FCPIO_LUNMAP_CHNG,
    FCPIO_LUNMAP_REQ,
    FCPIO_LUNMAP_REQ_CMPL,
    FCPIO_FLOGI_FIP_REG,
    FCPIO_FLOGI_FIP_REG_CMPL,
}

//
// Header status codes from the firmware
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcpio_status {
    FCPIO_SUCCESS = 0,              /* request was successful */

//
// If a request to the firmware is rejected, the original request
// header will be returned with the status set to one of the following:
//
    FCPIO_INVALID_HEADER,    /* header contains invalid data */
    FCPIO_OUT_OF_RESOURCE,   /* out of resources to complete request */
    FCPIO_INVALID_PARAM,     /* some parameter in request is invalid */
    FCPIO_REQ_NOT_SUPPORTED, /* request type is not supported */
    FCPIO_IO_NOT_FOUND,      /* requested I/O was not found */

//
// Once a request is processed, the firmware will usually return
// a cmpl message type.  In cases where errors occurred,
// the header status field would be filled in with one of the following:
//
    FCPIO_ABORTED = 0x41,     /* request was aborted */
    FCPIO_TIMEOUT,            /* request was timed out */
    FCPIO_SGL_INVALID,        /* request was aborted due to sgl error */
    FCPIO_MSS_INVALID,        /* request was aborted due to mss error */
    FCPIO_DATA_CNT_MISMATCH,  /* recv/sent more/less data than exp. */
    FCPIO_FW_ERR,             /* request was terminated due to fw error */
    FCPIO_ITMF_REJECTED,      /* itmf req was rejected by remote node */
    FCPIO_ITMF_FAILED,        /* itmf req was failed by remote node */
    FCPIO_ITMF_INCORRECT_LUN, /* itmf req targeted incorrect LUN */
    FCPIO_CMND_REJECTED,      /* request was invalid and rejected */
    FCPIO_NO_PATH_AVAIL,      /* no paths to the lun was available */
    FCPIO_PATH_FAILED,        /* i/o sent to current path failed */
    FCPIO_LUNMAP_CHNG_PEND,   /* i/o rejected due to lunmap change */
}

//
// The header command tag.  All host requests will use the "tag" field
// to mark commands with a unique tag.  When the firmware responds to
// a host request, it will copy the tag field into the response.
//
// The only firmware requests that will use the rx_id/ox_id fields instead
// of the tag field will be the target command and target task management
// requests.  These two requests do not have corresponding host requests
// since they come directly from the FC initiator on the network.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_tag {
    pub req_id: u32,
    pub rx_id: u16,
    pub ox_id: u16,
    pub ex_id: },
    pub u: },
}

// id = tag->u.req_id;
// rx_id = tag->u.ex_id.rx_id;
// ox_id = tag->u.ex_id.ox_id;
//
// The header for an fcpio request, whether from the firmware or from the
// host driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_header {
    pub /: *mut *mut u8 type; / enum fcpio_type,
    pub /: *mut *mut u8 status; / header status entry,
    pub /: *mut *mut u16 _resvd; / reserved,
    pub /: *mut *mut fcpio_tag tag; / header tag,
}

// type = hdr->type;
// status = hdr->status;
// tag = hdr->tag;
pub const NVME_CMD_SZ: c_int = 96;
pub const CDB_16: c_int = 16;
pub const CDB_32: c_int = 32;
pub const LUN_ADDRESS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_nvme_cmnd {
    pub d_id: [u8; 3],
    pub /: *mut *mut u_int8_t flags; / command flags,
    pub /: *mut *mut u_int32_t sgl_cnt; / scatter-gather list count,
    pub /: *mut *mut u_int64_t sgl_addr; / scatter-gather list addr,
    pub cmd_len: u_int16_t,
    pub /: *mut *mut u_int16_t _resvd1; / reserved: should be 0,
    pub /: *mut *mut u_int32_t data_len; / length of data expected,
    pub /: *mut *mut u_int8_t nvme_cmnd[NVME_CMD_SZ]; / NVME command,
}

//
// fcpio_icmnd_16: host -> firmware request
//
// used for sending out an initiator SCSI 16-byte command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_icmnd_16 {
    pub /: *mut *mut u32 lunmap_id; / index into lunmap table,
    pub /: *mut *mut u8 special_req_flags; / special exchange request flags,
    pub /: *mut *mut u8 _resvd0[3]; / reserved,
    pub /: *mut *mut u32 sgl_cnt; / scatter-gather list count,
    pub /: *mut *mut u32 sense_len; / sense buffer length,
    pub /: *mut *mut u64 sgl_addr; / scatter-gather list addr,
    pub /: *mut *mut u64 sense_addr; / sense buffer address,
    pub /: *mut *mut u8 crn; / SCSI Command Reference No.,
    pub /: *mut *mut u8 pri_ta; / SCSI Priority and Task attribute,
    pub /: *mut *mut u8 _resvd1; / reserved: should be 0,
    pub /: *mut *mut u8 flags; / command flags,
    pub /: *mut *mut u8 scsi_cdb[CDB_16]; / SCSI Cmnd Descriptor Block,
    pub /: *mut *mut u32 data_len; / length of data expected,
    pub /: *mut *mut u8 lun[LUN_ADDRESS]; / FC vNIC only: LUN address,
    pub /: *mut *mut u8 _resvd2; / reserved,
    pub /: *mut *mut u8 d_id[3]; / FC vNIC only: Target D_ID,
    pub /: *mut *mut u16 mss; / FC vNIC only: max burst,
    pub /: *mut *mut u16 _resvd3; / reserved,
    pub /: *mut *mut u32 r_a_tov; / FC vNIC only: Res. Alloc Timeout,
    pub /: *mut *mut u32 e_d_tov; / FC vNIC only: Err Detect Timeout,
}

//
// Special request flags
//
pub const FCPIO_ICMND_SRFLAG_RETRY: c_uint = 0x01   /* Enable Retry handling on exchange */;
//
// Priority/Task Attribute settings
//

//
// Command flags
//
pub const FCPIO_ICMND_RDDATA: c_uint = 0x02    /* read data */;
pub const FCPIO_ICMND_WRDATA: c_uint = 0x01    /* write data */;
//
// fcpio_icmnd_32: host -> firmware request
//
// used for sending out an initiator SCSI 32-byte command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_icmnd_32 {
    pub /: *mut *mut u32 lunmap_id; / index into lunmap table,
    pub /: *mut *mut u8 special_req_flags; / special exchange request flags,
    pub /: *mut *mut u8 _resvd0[3]; / reserved,
    pub /: *mut *mut u32 sgl_cnt; / scatter-gather list count,
    pub /: *mut *mut u32 sense_len; / sense buffer length,
    pub /: *mut *mut u64 sgl_addr; / scatter-gather list addr,
    pub /: *mut *mut u64 sense_addr; / sense buffer address,
    pub /: *mut *mut u8 crn; / SCSI Command Reference No.,
    pub /: *mut *mut u8 pri_ta; / SCSI Priority and Task attribute,
    pub /: *mut *mut u8 _resvd1; / reserved: should be 0,
    pub /: *mut *mut u8 flags; / command flags,
    pub /: *mut *mut u8 scsi_cdb[CDB_32]; / SCSI Cmnd Descriptor Block,
    pub /: *mut *mut u32 data_len; / length of data expected,
    pub /: *mut *mut u8 lun[LUN_ADDRESS]; / FC vNIC only: LUN address,
    pub /: *mut *mut u8 _resvd2; / reserved,
    pub /: *mut *mut u8 d_id[3]; / FC vNIC only: Target D_ID,
    pub /: *mut *mut u16 mss; / FC vNIC only: max burst,
    pub /: *mut *mut u16 _resvd3; / reserved,
    pub /: *mut *mut u32 r_a_tov; / FC vNIC only: Res. Alloc Timeout,
    pub /: *mut *mut u32 e_d_tov; / FC vNIC only: Error Detect Timeout,
}

//
// fcpio_itmf: host -> firmware request
//
// used for requesting the firmware to abort a request and/or send out
// a task management function
//
// The t_tag field is only needed when the request type is ABT_TASK.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_itmf {
    pub /: *mut *mut u32 lunmap_id; / index into lunmap table,
    pub /: *mut *mut u32 tm_req; / SCSI Task Management request,
    pub /: *mut *mut u32 t_tag; / header tag of fcpio to be aborted,
    pub /: *mut *mut u32 _resvd; / _reserved,
    pub /: *mut *mut u8 lun[LUN_ADDRESS]; / FC vNIC only: LUN address,
    pub /: *mut *mut u8 _resvd1; / reserved,
    pub /: *mut *mut u8 d_id[3]; / FC vNIC only: Target D_ID,
    pub /: *mut *mut u32 r_a_tov; / FC vNIC only: R_A_TOV in msec,
    pub /: *mut *mut u32 e_d_tov; / FC vNIC only: E_D_TOV in msec,
}

//
// Task Management request
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcpio_itmf_tm_req_type {
    FCPIO_ITMF_ABT_TASK_TERM = 0x01,    /* abort task and terminate */
    FCPIO_ITMF_ABT_TASK,                /* abort task and issue abts */
    FCPIO_ITMF_ABT_TASK_SET,            /* abort task set */
    FCPIO_ITMF_CLR_TASK_SET,            /* clear task set */
    FCPIO_ITMF_LUN_RESET,               /* logical unit reset task mgmt */
    FCPIO_ITMF_CLR_ACA,                 /* Clear ACA condition */
}

//
// fcpio_tdata: host -> firmware request
//
// used for requesting the firmware to send out a read data transfer for a
// target command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_tdata {
    pub /: *mut *mut u16 rx_id; / FC rx_id of target command,
    pub /: *mut *mut u16 flags; / command flags,
    pub /: *mut *mut u32 rel_offset; / data sequence relative offset,
    pub /: *mut *mut u32 sgl_cnt; / scatter-gather list count,
    pub /: *mut *mut u32 data_len; / length of data expected to send,
    pub /: *mut *mut u64 sgl_addr; / scatter-gather list address,
}

//
// Command flags
//
pub const FCPIO_TDATA_SCSI_RSP: c_uint = 0x01    /* send a scsi resp. after last frame */;
//
// fcpio_txrdy: host -> firmware request
//
// used for requesting the firmware to send out a write data transfer for a
// target command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_txrdy {
    pub /: *mut *mut u16 rx_id; / FC rx_id of target command,
    pub /: *mut *mut u16 _resvd0; / reserved,
    pub /: *mut *mut u32 rel_offset; / data sequence relative offset,
    pub /: *mut *mut u32 sgl_cnt; / scatter-gather list count,
    pub /: *mut *mut u32 data_len; / length of data expected to send,
    pub /: *mut *mut u64 sgl_addr; / scatter-gather list address,
}

//
// fcpio_trsp: host -> firmware request
//
// used for requesting the firmware to send out a response for a target
// command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_trsp {
    pub /: *mut *mut u16 rx_id; / FC rx_id of target command,
    pub /: *mut *mut u16 _resvd0; / reserved,
    pub /: *mut *mut u32 sense_len; / sense data buffer length,
    pub /: *mut *mut u64 sense_addr; / sense data buffer address,
    pub /: *mut *mut u16 _resvd1; / reserved,
    pub /: *mut *mut u8 flags; / response request flags,
    pub /: *mut *mut u8 scsi_status; / SCSI status,
    pub /: *mut *mut u32 residual; / SCSI data residual value of I/O,
}

//
// resposnse request flags
//
pub const FCPIO_TRSP_RESID_UNDER: c_uint = 0x08   /* residual is valid and is underflow */;
pub const FCPIO_TRSP_RESID_OVER: c_uint = 0x04   /* residual is valid and is overflow */;
//
// fcpio_ttmf_ack: host -> firmware response
//
// used by the host to indicate to the firmware it has received and processed
// the target tmf request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_ttmf_ack {
    pub /: *mut *mut u16 rx_id; / FC rx_id of target command,
    pub /: *mut *mut u16 _resvd0; / reserved,
    pub /: *mut *mut u32 tmf_status; / SCSI task management status,
}

//
// fcpio_tabort: host -> firmware request
//
// used by the host to request the firmware to abort a target request that was
// received by the firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_tabort {
    pub /: *mut *mut u16 rx_id; / rx_id of the target request,
}

//
// fcpio_reset: host -> firmware request
//
// used by the host to signal a reset of the driver to the firmware
// and to request firmware to clean up all outstanding I/O
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_reset {
    pub _resvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcpio_flogi_reg_format_type {
    FCPIO_FLOGI_REG_DEF_DEST = 0,    /* Use the oui | s_id mac format */
    FCPIO_FLOGI_REG_GW_DEST,         /* Use the fixed gateway mac */
}

//
// fcpio_flogi_reg: host -> firmware request
//
// fc vnic only
// used by the host to notify the firmware of the lif's s_id
// and destination mac address format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_flogi_reg {
    pub format: u8,
    pub /: *mut *mut u8 s_id[3]; / FC vNIC only: Source S_ID,
    pub /: *mut *mut u8 gateway_mac[ETH_ALEN]; / Destination gateway mac,
    pub _resvd: u16,
    pub /: *mut *mut u32 r_a_tov; / R_A_TOV in msec,
    pub /: *mut *mut u32 e_d_tov; / E_D_TOV in msec,
}

//
// fcpio_echo: host -> firmware request
//
// sends a heartbeat echo request to the firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_echo {
    pub _resvd: u32,
}

//
// fcpio_lunmap_req: host -> firmware request
//
// scsi vnic only
// sends a request to retrieve the lunmap table for scsi vnics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_lunmap_req {
    pub /: *mut *mut u64 addr; / address of the buffer,
    pub /: *mut *mut u32 len; / len of the buffer,
}

//
// fcpio_flogi_fip_reg: host -> firmware request
//
// fc vnic only
// used by the host to notify the firmware of the lif's s_id
// and destination mac address format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_flogi_fip_reg {
    pub _resvd0: u8,
    pub /: *mut *mut u8 s_id[3]; / FC vNIC only: Source S_ID,
    pub /: *mut *mut u8 fcf_mac[ETH_ALEN]; / FCF Target destination mac,
    pub _resvd1: u16,
    pub /: *mut *mut u32 r_a_tov; / R_A_TOV in msec,
    pub /: *mut *mut u32 e_d_tov; / E_D_TOV in msec,
    pub /: *mut *mut u8 ha_mac[ETH_ALEN]; / Host adapter source mac,
    pub _resvd2: u16,
}

//
// Basic structure for all fcpio structures that are sent from the host to the
// firmware.  They are 128 bytes per structure.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_host_req {
    pub hdr: fcpio_header,
//
// Defines space needed for request
//
    pub fcpio_header)]: u8 buf[FCPIO_HOST_REQ_LEN - sizeof(struct,
//
// Initiator host requests
//
    pub nvcmnd: fcpio_nvme_cmnd,
    pub icmnd_16: fcpio_icmnd_16,
    pub icmnd_32: fcpio_icmnd_32,
    pub itmf: fcpio_itmf,
//
// Target host requests
//
    pub tdata: fcpio_tdata,
    pub txrdy: fcpio_txrdy,
    pub trsp: fcpio_trsp,
    pub ttmf_ack: fcpio_ttmf_ack,
    pub tabort: fcpio_tabort,
//
// Misc requests
//
    pub reset: fcpio_reset,
    pub flogi_reg: fcpio_flogi_reg,
    pub echo: fcpio_echo,
    pub lunmap_req: fcpio_lunmap_req,
    pub flogi_fip_reg: fcpio_flogi_fip_reg,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_nvme_cmpl {
    pub resp_size: u8,
    pub resvd: [u8; 3],
    pub resp_bytes: [u8; 32],
}

//
// fcpio_icmnd_cmpl: firmware -> host response
//
// used for sending the host a response to an initiator command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_icmnd_cmpl {
    pub /: *mut *mut u8 _resvd0[6]; / reserved,
    pub /: *mut *mut u8 flags; / response flags,
    pub /: *mut *mut u8 scsi_status; / SCSI status,
    pub /: *mut *mut u32 residual; / SCSI data residual length,
    pub /: *mut *mut u32 sense_len; / SCSI sense length,
}

//
// response flags
//
pub const FCPIO_ICMND_CMPL_RESID_UNDER: c_uint = 0x08    /* resid under and valid */;
pub const FCPIO_ICMND_CMPL_RESID_OVER: c_uint = 0x04    /* resid over and valid */;
//
// fcpio_itmf_cmpl: firmware -> host response
//
// used for sending the host a response for a itmf request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_itmf_cmpl {
    pub /: *mut *mut u32 _resvd; / reserved,
}

//
// fcpio_tcmnd_16: firmware -> host request
//
// used by the firmware to notify the host of an incoming target SCSI 16-Byte
// request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_tcmnd_16 {
    pub /: *mut *mut u8 lun[LUN_ADDRESS]; / FC vNIC only: LUN address,
    pub /: *mut *mut u8 crn; / SCSI Command Reference No.,
    pub /: *mut *mut u8 pri_ta; / SCSI Priority and Task attribute,
    pub /: *mut *mut u8 _resvd2; / reserved: should be 0,
    pub /: *mut *mut u8 flags; / command flags,
    pub /: *mut *mut u8 scsi_cdb[CDB_16]; / SCSI Cmnd Descriptor Block,
    pub /: *mut *mut u32 data_len; / length of data expected,
    pub /: *mut *mut u8 _resvd1; / reserved,
    pub /: *mut *mut u8 s_id[3]; / FC vNIC only: Source S_ID,
}

//
// Priority/Task Attribute settings
//

//
// Command flags
//
pub const FCPIO_TCMND_RDDATA: c_uint = 0x02    /* read data */;
pub const FCPIO_TCMND_WRDATA: c_uint = 0x01    /* write data */;
//
// fcpio_tcmnd_32: firmware -> host request
//
// used by the firmware to notify the host of an incoming target SCSI 32-Byte
// request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_tcmnd_32 {
    pub /: *mut *mut u8 lun[LUN_ADDRESS]; / FC vNIC only: LUN address,
    pub /: *mut *mut u8 crn; / SCSI Command Reference No.,
    pub /: *mut *mut u8 pri_ta; / SCSI Priority and Task attribute,
    pub /: *mut *mut u8 _resvd2; / reserved: should be 0,
    pub /: *mut *mut u8 flags; / command flags,
    pub /: *mut *mut u8 scsi_cdb[CDB_32]; / SCSI Cmnd Descriptor Block,
    pub /: *mut *mut u32 data_len; / length of data expected,
    pub /: *mut *mut u8 _resvd0; / reserved,
    pub /: *mut *mut u8 s_id[3]; / FC vNIC only: Source S_ID,
}

//
// fcpio_tdrsp_cmpl: firmware -> host response
//
// used by the firmware to notify the host of a response to a host target
// command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_tdrsp_cmpl {
    pub /: *mut *mut u16 rx_id; / rx_id of the target request,
    pub /: *mut *mut u16 _resvd0; / reserved,
}

//
// fcpio_ttmf: firmware -> host request
//
// used by the firmware to notify the host of an incoming task management
// function request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_ttmf {
    pub /: *mut *mut u8 _resvd0; / reserved,
    pub /: *mut *mut u8 s_id[3]; / FC vNIC only: Source S_ID,
    pub /: *mut *mut u8 lun[LUN_ADDRESS]; / FC vNIC only: LUN address,
    pub /: *mut *mut u8 crn; / SCSI Command Reference No.,
    pub /: *mut *mut u8 _resvd2[3]; / reserved,
    pub /: *mut *mut u32 tmf_type; / task management request type,
}

//
// Task Management request
//
pub const FCPIO_TTMF_CLR_ACA: c_uint = 0x40    /* Clear ACA condition */;
pub const FCPIO_TTMF_LUN_RESET: c_uint = 0x10    /* logical unit reset task mgmt */;
pub const FCPIO_TTMF_CLR_TASK_SET: c_uint = 0x04    /* clear task set */;
pub const FCPIO_TTMF_ABT_TASK_SET: c_uint = 0x02    /* abort task set */;
pub const FCPIO_TTMF_ABT_TASK: c_uint = 0x01    /* abort task */;
//
// fcpio_tabort_cmpl: firmware -> host response
//
// used by the firmware to respond to a host's tabort request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_tabort_cmpl {
    pub /: *mut *mut u16 rx_id; / rx_id of the target request,
    pub /: *mut *mut u16 _resvd0; / reserved,
}

//
// fcpio_ack: firmware -> host response
//
// used by firmware to notify the host of the last work request received
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_ack {
    pub /: *mut *mut u16 request_out; / last host entry received,
    pub _resvd: u16,
}

//
// fcpio_reset_cmpl: firmware -> host response
//
// use by firmware to respond to the host's reset request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_reset_cmpl {
    pub vnic_id: u16,
}

//
// fcpio_flogi_reg_cmpl: firmware -> host response
//
// fc vnic only
// response to the fcpio_flogi_reg request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_flogi_reg_cmpl {
    pub _resvd: u32,
}

//
// fcpio_echo_cmpl: firmware -> host response
//
// response to the fcpio_echo request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_echo_cmpl {
    pub _resvd: u32,
}

//
// fcpio_lunmap_chng: firmware -> host notification
//
// scsi vnic only
// notifies the host that the lunmap tables have changed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_lunmap_chng {
    pub _resvd: u32,
}

//
// fcpio_lunmap_req_cmpl: firmware -> host response
//
// scsi vnic only
// response for lunmap table request from the host
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_lunmap_req_cmpl {
    pub _resvd: u32,
}

//
// Basic structure for all fcpio structures that are sent from the firmware to
// the host.  They are 64 bytes per structure.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_fw_req {
    pub hdr: fcpio_header,
//
// Defines space needed for request
//
    pub fcpio_header)]: u8 buf[FCPIO_FW_REQ_LEN - sizeof(struct,
//
// Initiator firmware responses
//
    pub icmnd_cmpl: fcpio_icmnd_cmpl,
    pub nvme_cmpl: fcpio_nvme_cmpl,
    pub itmf_cmpl: fcpio_itmf_cmpl,
//
// Target firmware new requests
//
    pub tcmnd_16: fcpio_tcmnd_16,
    pub tcmnd_32: fcpio_tcmnd_32,
//
// Target firmware responses
//
    pub tdrsp_cmpl: fcpio_tdrsp_cmpl,
    pub ttmf: fcpio_ttmf,
    pub tabort_cmpl: fcpio_tabort_cmpl,
//
// Firmware response to work received
//
    pub ack: fcpio_ack,
//
// Misc requests
//
    pub reset_cmpl: fcpio_reset_cmpl,
    pub flogi_reg_cmpl: fcpio_flogi_reg_cmpl,
    pub echo_cmpl: fcpio_echo_cmpl,
    pub lunmap_chng: fcpio_lunmap_chng,
    pub lunmap_req_cmpl: fcpio_lunmap_req_cmpl,
    pub u: },
}

//
// Access routines to encode and decode the color bit, which is the most
// significant bit of the MSB of the structure
//
// c |= 0x80;
// c &= ~0x80;
// color = *c >> 7;
//
// Make sure color bit is read from desc *before* other fields
// are read from desc.  Hardware guarantees color bit is last
// bit (byte) written.  Adding the rmb() prevents the compiler
// and/or CPU from reordering the reads which would potentially
// result in reading stale values.
//
// Lunmap table entry for scsi vnics
//
pub const FCPIO_LUNMAP_TABLE_SIZE: c_int = 256;
pub const FCPIO_FLAGS_LUNMAP_VALID: c_uint = 0x80;
pub const FCPIO_FLAGS_BOOT: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_lunmap_entry {
    pub bus: u8,
    pub target: u8,
    pub lun: u8,
    pub path_cnt: u8,
    pub flags: u16,
    pub update_cnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcpio_lunmap_tbl {
    pub update_cnt: u32,
    pub lunmaps: [fcpio_lunmap_entry; FCPIO_LUNMAP_TABLE_SIZE],
}
