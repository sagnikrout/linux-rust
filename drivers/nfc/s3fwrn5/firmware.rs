//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/s3fwrn5/firmware.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// NCI based driver for Samsung S3FWRN5 NFC chip
//
// Copyright (C) 2015 Samsung Electronics
// Robert Baldyga <r.baldyga@samsung.com>
//
// FW Message Types
pub const S3FWRN5_FW_MSG_CMD: c_uint = 0x00;
pub const S3FWRN5_FW_MSG_RSP: c_uint = 0x01;
pub const S3FWRN5_FW_MSG_DATA: c_uint = 0x02;
// FW Return Codes
pub const S3FWRN5_FW_RET_SUCCESS: c_uint = 0x00;
pub const S3FWRN5_FW_RET_MESSAGE_TYPE_INVALID: c_uint = 0x01;
pub const S3FWRN5_FW_RET_COMMAND_INVALID: c_uint = 0x02;
pub const S3FWRN5_FW_RET_PAGE_DATA_OVERFLOW: c_uint = 0x03;
pub const S3FWRN5_FW_RET_SECT_DATA_OVERFLOW: c_uint = 0x04;
pub const S3FWRN5_FW_RET_AUTHENTICATION_FAIL: c_uint = 0x05;
pub const S3FWRN5_FW_RET_FLASH_OPERATION_FAIL: c_uint = 0x06;
pub const S3FWRN5_FW_RET_ADDRESS_OUT_OF_RANGE: c_uint = 0x07;
pub const S3FWRN5_FW_RET_PARAMETER_INVALID: c_uint = 0x08;
// ---- FW Packet structures ----
pub const S3FWRN5_FW_HDR_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3fwrn5_fw_header {
    pub type: __u8,
    pub code: __u8,
    pub len: __u16,
}

pub const S3FWRN5_FW_CMD_RESET: c_uint = 0x00;
pub const S3FWRN5_FW_CMD_GET_BOOTINFO: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3fwrn5_fw_cmd_get_bootinfo_rsp {
    pub hw_version: [__u8; 4],
    pub sector_size: __u16,
    pub page_size: __u16,
    pub frame_max_size: __u16,
    pub hw_buffer_size: __u16,
}

pub const S3FWRN5_FW_CMD_ENTER_UPDATE_MODE: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3fwrn5_fw_cmd_enter_updatemode {
    pub hashcode_size: __u16,
    pub signature_size: __u16,
}

pub const S3FWRN5_FW_CMD_UPDATE_SECTOR: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3fwrn5_fw_cmd_update_sector {
    pub base_address: __u32,
}

pub const S3FWRN5_FW_CMD_COMPLETE_UPDATE_MODE: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3fwrn5_fw_image {
    pub fw: *const firmware,
    pub date: [c_char; 13],
    pub version: u32,
    pub sig: *const c_void,
    pub sig_size: u32,
    pub image: *const c_void,
    pub image_sectors: u32,
    pub custom_sig: *const c_void,
    pub custom_sig_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3fwrn5_fw_info {
    pub ndev: *mut nci_dev,
    pub fw: s3fwrn5_fw_image,
    pub 1]: char fw_name[NFC_FIRMWARE_NAME_MAXSIZE +,
    pub sig: *const c_void,
    pub sig_size: u32,
    pub sector_size: u32,
    pub base_addr: u32,
    pub completion: completion,
    pub rsp: *mut sk_buff,
    pub parity: c_char,
}

extern "C" {
    pub fn s3fwrn5_fw_request_firmware(fw_info: *mut s3fwrn5_fw_info) -> c_int;
}
extern "C" {
    pub fn s3fwrn5_fw_init(fw_info: *mut s3fwrn5_fw_info, fw_name: *const c_char);
}
extern "C" {
    pub fn s3fwrn5_fw_setup(fw_info: *mut s3fwrn5_fw_info) -> c_int;
}
extern "C" {
    pub fn s3fwrn5_fw_check_version(fw_info: *const s3fwrn5_fw_info, version: u32) -> bool;
}
extern "C" {
    pub fn s3fwrn5_fw_download(fw_info: *mut s3fwrn5_fw_info) -> c_int;
}
extern "C" {
    pub fn s3fwrn5_fw_cleanup(fw_info: *mut s3fwrn5_fw_info);
}
extern "C" {
    pub fn s3fwrn5_fw_recv_frame(ndev: *mut nci_dev, skb: *mut sk_buff) -> c_int;
}
