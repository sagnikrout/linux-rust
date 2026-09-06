//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/dpseci_cmd.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright 2013-2016 Freescale Semiconductor Inc.
// Copyright 2017-2018 NXP
//
// DPSECI Version
pub const DPSECI_VER_MAJOR: c_int = 5;
pub const DPSECI_VER_MINOR: c_int = 3;

// Command versioning
pub const DPSECI_CMD_BASE_VERSION: c_int = 1;
pub const DPSECI_CMD_BASE_VERSION_V2: c_int = 2;
pub const DPSECI_CMD_ID_OFFSET: c_int = 4;

// Command IDs

// Macros for accessing command fields smaller than 1 byte

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_cmd_open {
    pub dpseci_id: __le32,
}

pub const DPSECI_ENABLE_SHIFT: c_int = 0;
pub const DPSECI_ENABLE_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_rsp_is_enabled {
    pub is_enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_rsp_get_attributes {
    pub id: __le32,
    pub pad0: __le32,
    pub num_tx_queues: u8,
    pub num_rx_queues: u8,
    pub pad1: [u8; 6],
    pub options: __le32,
}

pub const DPSECI_DEST_TYPE_SHIFT: c_int = 0;
pub const DPSECI_DEST_TYPE_SIZE: c_int = 4;
pub const DPSECI_ORDER_PRESERVATION_SHIFT: c_int = 0;
pub const DPSECI_ORDER_PRESERVATION_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_cmd_queue {
    pub dest_id: __le32,
    pub priority: u8,
    pub queue: u8,
    pub dest_type: u8,
    pub pad: u8,
    pub user_ctx: __le64,
    pub options: __le32,
    pub fqid: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_rsp_get_tx_queue {
    pub pad: __le32,
    pub fqid: __le32,
    pub priority: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_rsp_get_sec_attr {
    pub ip_id: __le16,
    pub major_rev: u8,
    pub minor_rev: u8,
    pub era: u8,
    pub pad0: [u8; 3],
    pub deco_num: u8,
    pub zuc_auth_acc_num: u8,
    pub zuc_enc_acc_num: u8,
    pub pad1: u8,
    pub snow_f8_acc_num: u8,
    pub snow_f9_acc_num: u8,
    pub crc_acc_num: u8,
    pub pad2: u8,
    pub pk_acc_num: u8,
    pub kasumi_acc_num: u8,
    pub rng_acc_num: u8,
    pub pad3: u8,
    pub md_acc_num: u8,
    pub arc4_acc_num: u8,
    pub des_acc_num: u8,
    pub aes_acc_num: u8,
    pub ccha_acc_num: u8,
    pub ptha_acc_num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_rsp_get_api_version {
    pub major: __le16,
    pub minor: __le16,
}

pub const DPSECI_CGN_DEST_TYPE_SHIFT: c_int = 0;
pub const DPSECI_CGN_DEST_TYPE_SIZE: c_int = 4;
pub const DPSECI_CGN_UNITS_SHIFT: c_int = 4;
pub const DPSECI_CGN_UNITS_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_cmd_congestion_notification {
    pub dest_id: __le32,
    pub notification_mode: __le16,
    pub priority: u8,
    pub options: u8,
    pub message_iova: __le64,
    pub message_ctx: __le64,
    pub threshold_entry: __le32,
    pub threshold_exit: __le32,
}
