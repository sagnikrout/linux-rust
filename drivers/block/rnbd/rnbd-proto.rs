//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/rnbd/rnbd-proto.h
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
// RDMA Network Block Driver
//
// Copyright (c) 2014 - 2018 ProfitBricks GmbH. All rights reserved.
// Copyright (c) 2018 - 2019 1&1 IONOS Cloud GmbH. All rights reserved.
// Copyright (c) 2019 - 2020 1&1 IONOS SE. All rights reserved.
//

pub const RNBD_PROTO_VER_MAJOR: c_int = 2;
pub const RNBD_PROTO_VER_MINOR: c_int = 2;
// The default port number the RTRS server is listening on.
pub const RTRS_PORT: c_int = 1234;
//
// enum rnbd_msg_type - RNBD message types
// @RNBD_MSG_SESS_INFO:	initial session info from client to server
// @RNBD_MSG_SESS_INFO_RSP:	initial session info from server to client
// @RNBD_MSG_OPEN:		open (map) device request
// @RNBD_MSG_OPEN_RSP:		response to an @RNBD_MSG_OPEN
// @RNBD_MSG_IO:		block IO request operation
// @RNBD_MSG_CLOSE:		close (unmap) device request
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rnbd_msg_type {
    RNBD_MSG_SESS_INFO,
    RNBD_MSG_SESS_INFO_RSP,
    RNBD_MSG_OPEN,
    RNBD_MSG_OPEN_RSP,
    RNBD_MSG_IO,
    RNBD_MSG_CLOSE,
}

//
// struct rnbd_msg_hdr - header of RNBD messages
// @type:	Message type, valid values see: enum rnbd_msg_types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_hdr {
    pub type: __le16,
// private:
    pub __padding: __le16,
}

//
// We allow to map RO many times and RW only once. We allow to map yet another
// time RW, if MIGRATION is provided (second RW export can be required for
// example for VM migration)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rnbd_access_mode {
    RNBD_ACCESS_RO,
    RNBD_ACCESS_RW,
    RNBD_ACCESS_MIGRATION,
}

//
// struct rnbd_msg_sess_info - initial session info from client to server
// @hdr:		message header
// @ver:		RNBD protocol version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_sess_info {
    pub hdr: rnbd_msg_hdr,
    pub ver: u8,
// private:
    pub reserved: [u8; 31],
}

//
// struct rnbd_msg_sess_info_rsp - initial session info from server to client
// @hdr:		message header
// @ver:		RNBD protocol version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_sess_info_rsp {
    pub hdr: rnbd_msg_hdr,
    pub ver: u8,
// private:
    pub reserved: [u8; 31],
}

//
// struct rnbd_msg_open - request to open a remote device.
// @hdr:		message header
// @access_mode:	the mode to open remote device, valid values see:
// enum rnbd_access_mode
// @dev_name:		device path on remote side
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_open {
    pub hdr: rnbd_msg_hdr,
    pub access_mode: u8,
// private:
    pub resv1: u8,
// public:
    pub dev_name: [i8; NAME_MAX],
// private:
    pub reserved: [u8; 3],
}

//
// struct rnbd_msg_close - request to close a remote device.
// @hdr:	message header
// @device_id:	device_id on server side to identify the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_close {
    pub hdr: rnbd_msg_hdr,
    pub device_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rnbd_cache_policy {
    RNBD_FUA = 1 << 0,
    RNBD_WRITEBACK = 1 << 1,
}

//
// struct rnbd_msg_open_rsp - response message to RNBD_MSG_OPEN
// @hdr:		message header
// @device_id:		device_id on server side to identify the device
// @nsectors:		number of sectors in the usual 512b unit
// @max_hw_sectors:	max hardware sectors in the usual 512b unit
// @max_write_zeroes_sectors: max sectors for WRITE ZEROES in the 512b unit
// @max_discard_sectors: max. sectors that can be discarded at once in 512b
// unit.
// @discard_granularity: size of the internal discard allocation unit in bytes
// @discard_alignment: offset from internal allocation assignment in bytes
// @physical_block_size: physical block size device supports in bytes
// @logical_block_size: logical block size device supports in bytes
// @max_segments:	max segments hardware support in one transfer
// @secure_discard:	supports secure discard
// @obsolete_rotational: obsolete, not in used.
// @cache_policy: 	support write-back caching or FUA?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_open_rsp {
    pub hdr: rnbd_msg_hdr,
    pub device_id: __le32,
    pub nsectors: __le64,
    pub max_hw_sectors: __le32,
    pub max_write_zeroes_sectors: __le32,
    pub max_discard_sectors: __le32,
    pub discard_granularity: __le32,
    pub discard_alignment: __le32,
    pub physical_block_size: __le16,
    pub logical_block_size: __le16,
    pub max_segments: __le16,
    pub secure_discard: __le16,
    pub obsolete_rotational: u8,
    pub cache_policy: u8,
// private:
    pub reserved: [u8; 10],
}

//
// struct rnbd_msg_io - message for I/O read/write
// @hdr:	message header
// @device_id:	device_id on server side to find the right device
// @sector:	bi_sector attribute from struct bio
// @rw:		valid values are defined in enum rnbd_io_flags
// @bi_size:    number of bytes for I/O read/write
// @prio:       priority
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rnbd_msg_io {
    pub hdr: rnbd_msg_hdr,
    pub device_id: __le32,
    pub sector: __le64,
    pub rw: __le32,
    pub bi_size: __le32,
    pub prio: __le16,
}

pub const RNBD_OP_BITS: c_int = 8;

//
// enum rnbd_io_flags - RNBD request types from rq_flag_bits
// @RNBD_OP_READ:	     read sectors from the device
// @RNBD_OP_WRITE:	     write sectors to the device
// @RNBD_OP_FLUSH:	     flush the volatile write cache
// @RNBD_OP_DISCARD:        discard sectors
// @RNBD_OP_SECURE_ERASE:   securely erase sectors
// @RNBD_OP_WRITE_ZEROES:   write zeroes sectors
//
// @RNBD_F_SYNC:	     request is sync (sync write or read)
// @RNBD_F_FUA:             forced unit access
// @RNBD_F_PREFLUSH:	    request for cache flush
// @RNBD_F_NOUNMAP:	    do not free blocks when zeroing
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rnbd_io_flags {

// Operations
    RNBD_OP_READ		= 0,
    RNBD_OP_WRITE		= 1,
    RNBD_OP_FLUSH		= 2,
    RNBD_OP_DISCARD	= 3,
    RNBD_OP_SECURE_ERASE	= 4,
    RNBD_OP_WRITE_ZEROES	= 5,

// Flags
    RNBD_F_SYNC  = 1<<(RNBD_OP_BITS + 0),
    RNBD_F_FUA   = 1<<(RNBD_OP_BITS + 1),
    RNBD_F_PREFLUSH = 1<<(RNBD_OP_BITS + 2),
    RNBD_F_NOUNMAP = 1<<(RNBD_OP_BITS + 3)
}
