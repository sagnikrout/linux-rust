//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas_tf/if_usb.h
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
// Copyright (C) 2008, cozybit Inc.
// Copyright (C) 2003-2006, Marvell International Ltd.
//

//
// This file contains definition for USB interface.
//
pub const CMD_TYPE_REQUEST: c_uint = 0xF00DFACE;
pub const CMD_TYPE_DATA: c_uint = 0xBEADC0DE;
pub const CMD_TYPE_INDICATION: c_uint = 0xBEEFFACE;
pub const BOOT_CMD_FW_BY_USB: c_uint = 0x01;
pub const BOOT_CMD_FW_IN_EEPROM: c_uint = 0x02;
pub const BOOT_CMD_UPDATE_BOOT2: c_uint = 0x03;
pub const BOOT_CMD_UPDATE_FW: c_uint = 0x04;
pub const BOOT_CMD_MAGIC_NUMBER: c_uint = 0x4C56524D   /* LVRM */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootcmd {
    pub magic: __le32,
    pub cmd: u8,
    pub pad: [u8; 11],
}

pub const BOOT_CMD_RESP_OK: c_uint = 0x0001;
pub const BOOT_CMD_RESP_FAIL: c_uint = 0x0000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootcmdresp {
    pub magic: __le32,
    pub cmd: u8,
    pub result: u8,
    pub pad: [u8; 2],
}

// USB card description structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct if_usb_card {
    pub udev: *mut usb_device,
    pub cmd_urb: *mut *mut *mut urb rx_urb, tx_urb,,
    pub priv: *mut lbtf_private,
    pub rx_skb: *mut sk_buff,
    pub ep_in: u8,
    pub ep_out: u8,
    pub bootcmdresp: i8,
    pub ep_in_size: c_int,
    pub ep_out_buf: *mut c_void,
    pub ep_out_size: c_int,
    pub fw: *const firmware,
    pub fw_timeout: timer_list,
    pub fw_wq: wait_queue_head_t,
    pub fwseqnum: u32,
    pub totalbytes: u32,
    pub fwlastblksent: u32,
    pub CRC_OK: u8,
    pub fwdnldover: u8,
    pub fwfinalblk: u8,
    pub boot2_version: __le16,
}

// fwheader
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwheader {
    pub dnldcmd: __le32,
    pub baseaddr: __le32,
    pub datalength: __le32,
    pub CRC: __le32,
}

pub const FW_MAX_DATA_BLK_SIZE: c_int = 600;
// FWData
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwdata {
    pub hdr: fwheader,
    pub seqnum: __le32,
    pub data: [u8; ],
}

// fwsyncheader
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwsyncheader {
    pub cmd: __le32,
    pub seqnum: __le32,
}

pub const FW_HAS_DATA_TO_RECV: c_uint = 0x00000001;
pub const FW_HAS_LAST_BLOCK: c_uint = 0x00000004;
