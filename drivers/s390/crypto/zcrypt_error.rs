//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/crypto/zcrypt_error.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright IBM Corp. 2001, 2006
// Author(s): Robert Burroughs
// Eric Rossman (edrossma@us.ibm.com)
//
// Hotplug & misc device support: Jochen Roehrig (roehrig@de.ibm.com)
// Major cleanup & driver split: Martin Schwidefsky <schwidefsky@de.ibm.com>
//

//
// Reply Messages
//
// Error reply messages are of two types:
// 82:  Error (see below)
// 88:  Error (see below)
// Both type 82 and type 88 have the same structure in the header.
//
// Request reply messages are of three known types:
// 80:  Reply from a Type 50 Request (see CEX2A-RELATED STRUCTS)
// 84:  Reply from a Type 4 Request (see PCICA-RELATED STRUCTS)
// 86:  Reply from a Type 6 Request (see PCICC/PCIXCC/CEX2C-RELATED STRUCTS)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct error_hdr {
    pub /: *mut *mut unsigned char reserved1; / 0x00,
    pub /: *mut *mut unsigned char type; / 0x82 or 0x88,
    pub /: *mut *mut unsigned char reserved2[2]; / 0x0000,
    pub /: *mut *mut unsigned char reply_code; / reply code,
    pub /: *mut *mut unsigned char reserved3[3]; / 0x000000,
}

pub const TYPE82_RSP_CODE: c_uint = 0x82;
pub const TYPE88_RSP_CODE: c_uint = 0x88;
pub const REP82_ERROR_MACHINE_FAILURE: c_uint = 0x10;
pub const REP82_ERROR_PREEMPT_FAILURE: c_uint = 0x12;
pub const REP82_ERROR_CHECKPT_FAILURE: c_uint = 0x14;
pub const REP82_ERROR_MESSAGE_TYPE: c_uint = 0x20;
pub const REP82_ERROR_INVALID_COMM_CD: c_uint = 0x21 /* Type 84	*/;
pub const REP82_ERROR_INVALID_MSG_LEN: c_uint = 0x23;
pub const REP82_ERROR_RESERVD_FIELD: c_uint = 0x24 /* was 0x50	*/;
pub const REP82_ERROR_FORMAT_FIELD: c_uint = 0x29;
pub const REP82_ERROR_INVALID_COMMAND: c_uint = 0x30;
pub const REP82_ERROR_MALFORMED_MSG: c_uint = 0x40;
pub const REP82_ERROR_INVALID_SPECIAL_CMD: c_uint = 0x41;
pub const REP82_ERROR_RESERVED_FIELDO: c_uint = 0x50 /* old value	*/;
pub const REP82_ERROR_WORD_ALIGNMENT: c_uint = 0x60;
pub const REP82_ERROR_MESSAGE_LENGTH: c_uint = 0x80;
pub const REP82_ERROR_OPERAND_INVALID: c_uint = 0x82;
pub const REP82_ERROR_OPERAND_SIZE: c_uint = 0x84;
pub const REP82_ERROR_EVEN_MOD_IN_OPND: c_uint = 0x85;
pub const REP82_ERROR_RESERVED_FIELD: c_uint = 0x88;
pub const REP82_ERROR_INVALID_DOMAIN_PENDING: c_uint = 0x8A;
pub const REP82_ERROR_FILTERED_BY_HYPERVISOR: c_uint = 0x8B;
pub const REP82_ERROR_TRANSPORT_FAIL: c_uint = 0x90;
pub const REP82_ERROR_PACKET_TRUNCATED: c_uint = 0xA0;
pub const REP82_ERROR_ZERO_BUFFER_LEN: c_uint = 0xB0;
pub const REP88_ERROR_MODULE_FAILURE: c_uint = 0x10;
pub const REP88_ERROR_MESSAGE_TYPE: c_uint = 0x20;
pub const REP88_ERROR_MESSAGE_MALFORMD: c_uint = 0x22;
pub const REP88_ERROR_MESSAGE_LENGTH: c_uint = 0x23;
pub const REP88_ERROR_RESERVED_FIELD: c_uint = 0x24;
pub const REP88_ERROR_KEY_TYPE: c_uint = 0x34;
pub const REP88_ERROR_INVALID_KEY: c_uint = 0x82 /* CEX2A	*/;
pub const REP88_ERROR_OPERAND: c_uint = 0x84 /* CEX2A	*/;
pub const REP88_ERROR_OPERAND_EVEN_MOD: c_uint = 0x85 /* CEX2A	*/;
// RY indicates malformed request
//
// Msg to wrong type or card/infrastructure failure. Return
// EAGAIN, the upper layer may do a retry on the request.
//
// For type 86 response show the apfs value (failure reason)
// Assume request is valid and a retry will be worth it
