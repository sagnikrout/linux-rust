//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/scsi/scsi_bsg_ufs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// UFS Transport SGIO v4 BSG Message Support
//
// Copyright (C) 2011-2013 Samsung India Software Operations
// Copyright (C) 2018 Western Digital Corporation
//

//
// This file intended to be included by both kernel and user space
//
pub const UFS_CDB_SIZE: c_int = 16;
// uic commands are 4DW long, per UFSHCI V2.1 paragraph 5.6.1

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_bsg_msg_code {
    UPIU_TRANSACTION_UIC_CMD = 0x1F,
    UPIU_TRANSACTION_ARPMB_CMD,
}

// UFS RPMB Request Message Types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_rpmb_op_type {
    UFS_RPMB_WRITE_KEY		= 0x01,
    UFS_RPMB_READ_CNT		= 0x02,
    UFS_RPMB_WRITE			= 0x03,
    UFS_RPMB_READ			= 0x04,
    UFS_RPMB_READ_RESP		= 0x05,
    UFS_RPMB_SEC_CONF_WRITE		= 0x06,
    UFS_RPMB_SEC_CONF_READ		= 0x07,
    UFS_RPMB_PURGE_ENABLE		= 0x08,
    UFS_RPMB_PURGE_STATUS_READ	= 0x09,
}

//
// struct utp_upiu_header - UPIU header structure
// @dword_0: UPIU header DW-0
// @dword_1: UPIU header DW-1
// @dword_2: UPIU header DW-2
//
// @transaction_code: Type of request or response. See also enum
// upiu_request_transaction and enum upiu_response_transaction.
// @flags: UPIU flags. The meaning of individual flags depends on the
// transaction code.
// @lun: Logical unit number.
// @task_tag: Task tag.
// @iid: Initiator ID.
// @command_set_type: 0 for SCSI command set; 1 for UFS specific.
// @tm_function: Task management function in case of a task management request
// UPIU.
// @query_function: Query function in case of a query request UPIU.
// @response: 0 for success; 1 for failure.
// @status: SCSI status if this is the header of a response to a SCSI command.
// @ehs_length: EHS length in units of 32 bytes.
// @device_information:
// @data_segment_length: data segment length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utp_upiu_header {
    pub dword_0: __be32,
    pub dword_1: __be32,
    pub dword_2: __be32,
}

//
// struct utp_upiu_query - QUERY REQUEST UPIU structure.
// @opcode: query function to perform B-0
// @idn: descriptor or attribute identification number B-1
// @index: Index that further identifies which data to access B-2
// @selector: Index that further identifies which data to access B-3
// @reserved_osf: spec reserved field B-4,5
// @length: number of descriptor bytes to read or write B-6,7
// @value: if @opcode == UPIU_QUERY_OPCODE_WRITE_ATTR, the value to be written B-6,7
// @reserved: reserved for future use DW-6,7
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utp_upiu_query {
    pub opcode: __u8,
    pub idn: __u8,
    pub index: __u8,
    pub selector: __u8,
    pub reserved_osf: __be16,
    pub length: __be16,
    pub value: __be32,
    pub reserved: [__be32; 2],
}

//
// struct utp_upiu_query_v4_0 - upiu request buffer structure for
// query request >= UFS 4.0 spec.
// @opcode: command to perform B-0
// @idn: a value that indicates the particular type of data B-1
// @index: Index to further identify data B-2
// @selector: Index to further identify data B-3
// @osf3: spec field B-4
// @osf4: spec field B-5
// @osf5: spec field B 6,7
// @osf6: spec field DW 8,9
// @osf7: spec field DW 10,11
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utp_upiu_query_v4_0 {
    pub opcode: __u8,
    pub idn: __u8,
    pub index: __u8,
    pub selector: __u8,
    pub osf3: __u8,
    pub osf4: __u8,
    pub osf5: __be16,
    pub osf6: __be32,
    pub osf7: __be32,
// private:
    pub reserved: __be32,
}

//
// struct utp_upiu_cmd - Command UPIU structure
// @exp_data_transfer_len: Data Transfer Length DW-3
// @cdb: Command Descriptor Block CDB DW-4 to DW-7
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utp_upiu_cmd {
    pub exp_data_transfer_len: __be32,
    pub cdb: [__u8; UFS_CDB_SIZE],
}

//
// struct utp_upiu_req - general upiu request structure
// @header:UPIU header structure DW-0 to DW-2
// @sc: fields structure for scsi command DW-3 to DW-7
// @qr: fields structure for query request DW-3 to DW-7
// @uc: use utp_upiu_query to host the 4 dwords of uic command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utp_upiu_req {
    pub header: utp_upiu_header,
    pub sc: utp_upiu_cmd,
    pub qr: utp_upiu_query,
    pub uc: utp_upiu_query,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_arpmb_meta {
    pub req_resp_type: __be16,
    pub nonce: [__u8; 16],
    pub write_counter: __be32,
    pub addr_lun: __be16,
    pub block_count: __be16,
    pub result: __be16,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_ehs {
    pub length: __u8,
    pub ehs_type: __u8,
    pub ehssub_type: __be16,
    pub meta: ufs_arpmb_meta,
    pub mac_key: [__u8; 32],
    pub __attribute__((__packed__)): },
// request (CDB) structure of the sg_io_v4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_bsg_request {
    pub msgcode: __u32,
    pub upiu_req: utp_upiu_req,
}

// response (request sense data) structure of the sg_io_v4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_bsg_reply {
//
// The completion result. Result exists in two forms:
// if negative, it is an -Exxx system errno value. There will
// be no further reply information supplied.
// else, it's the 4-byte scsi error result, with driver, host,
// msg and status fields. The per-msgcode reply structure
// will contain valid data.
//
    pub result: c_int,
// If there was reply_payload, how much was received?
    pub reply_payload_rcv_len: __u32,
    pub upiu_rsp: utp_upiu_req,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_rpmb_request {
    pub bsg_request: ufs_bsg_request,
    pub ehs_req: ufs_ehs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_rpmb_reply {
    pub bsg_reply: ufs_bsg_reply,
    pub ehs_rsp: ufs_ehs,
}
