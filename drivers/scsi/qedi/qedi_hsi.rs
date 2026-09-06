//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedi/qedi_hsi.h
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
// QLogic iSCSI Offload Driver
// Copyright (c) 2016 Cavium Inc.
//
// Add include to common target
//

//
// Add include to common storage target
//

//
// Add include to common TCP target
//

//
// Add include to common iSCSI target for both eCore and protocol driver
//

//
// iSCSI CMDQ element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cmdqe {
    pub conn_id: __le16,
    pub invalid_command: u8,
    pub cmd_hdr_type: u8,
    pub reserved1: [__le32; 2],
    pub cmd_payload: [__le32; 13],
}

//
// iSCSI CMD header type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iscsi_cmd_hdr_type {
    ISCSI_CMD_HDR_TYPE_BHS_ONLY /* iSCSI BHS with no expected AHS */,
    ISCSI_CMD_HDR_TYPE_BHS_W_AHS /* iSCSI BHS with expected AHS */,
    ISCSI_CMD_HDR_TYPE_AHS /* iSCSI AHS */,
    MAX_ISCSI_CMD_HDR_TYPE
}
