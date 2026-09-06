//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/crypto/zcrypt_msgtype6.h
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
// Copyright IBM Corp. 2001, 2012
// Author(s): Robert Burroughs
// Eric Rossman (edrossma@us.ibm.com)
//
// Hotplug & misc device support: Jochen Roehrig (roehrig@de.ibm.com)
// Major cleanup & driver split: Martin Schwidefsky <schwidefsky@de.ibm.com>
// MSGTYPE restruct:		  Holger Dengler <hd@linux.vnet.ibm.com>
//

pub const MSGTYPE06_VARIANT_DEFAULT: c_int = 0;
pub const MSGTYPE06_VARIANT_NORNG: c_int = 1;
pub const MSGTYPE06_VARIANT_EP11: c_int = 2;
//
// The type 6 message family is associated with CEXxC/CEXxP cards.
//
// It contains a message header followed by a CPRB, both of which
// are described below.
//
// Note that all reserved fields must be zeroes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type6_hdr {
    pub /: *mut *mut unsigned char reserved1; / 0x00,
    pub /: *mut *mut unsigned char type; / 0x06,
    pub /: *mut *mut unsigned char reserved2[2]; / 0x0000,
    pub /: *mut *mut unsigned char right[4]; / 0x00000000,
    pub /: *mut *mut unsigned char reserved3[2]; / 0x0000,
    pub /: *mut *mut unsigned char reserved4[2]; / 0x0000,
    pub /: *mut *mut unsigned int apfs; / 0x00000000,
    pub /: *mut *mut unsigned int offset1; / 0x00000058 (offset to CPRB),
    pub /: *mut *mut unsigned int offset2; / 0x00000000,
    pub /: *mut *mut unsigned int offset3; / 0x00000000,
    pub /: *mut *mut unsigned int offset4; / 0x00000000,
    pub /: *mut *mut unsigned char agent_id[16]; / 0x4341000000000000,
// 0x0000000000000000
    pub /: *mut *mut unsigned char rqid[2]; / rqid. internal to 603,
    pub /: *mut *mut unsigned char reserved5[2]; / 0x0000,
    pub /: *mut *mut unsigned char function_code[2]; / for PKD, 0x5044 (ascii 'PD'),
    pub /: *mut *mut unsigned char reserved6[2]; / 0x0000,
    pub /: *mut *mut unsigned int tocardlen1; / (request CPRB len + 3) & -4,
    pub /: *mut *mut unsigned int tocardlen2; / db len 0x00000000 for PKD,
    pub /: *mut *mut unsigned int tocardlen3; / 0x00000000,
    pub /: *mut *mut unsigned int tocardlen4; / 0x00000000,
    pub /: *mut *mut unsigned int fromcardlen1; / response buffer length,
    pub /: *mut *mut unsigned int fromcardlen2; / db len 0x00000000 for PKD,
    pub /: *mut *mut unsigned int fromcardlen3; / 0x00000000,
    pub /: *mut *mut unsigned int fromcardlen4; / 0x00000000,
    pub __packed: },
//
// The type 86 message family is associated with CEXxC/CEXxP cards.
//
// It contains a message header followed by a CPRB.  The CPRB is
// the same as the request CPRB, which is described above.
//
// If format is 1, an error condition exists and no data beyond
// the 8-byte message header is of interest.
//
// The non-error message is shown below.
//
// Note that all reserved fields must be zeroes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type86_hdr {
    pub /: *mut *mut unsigned char reserved1; / 0x00,
    pub /: *mut *mut unsigned char type; / 0x86,
    pub /: *mut *mut unsigned char format; / 0x01 (error) or 0x02 (ok),
    pub /: *mut *mut unsigned char reserved2; / 0x00,
    pub /: *mut *mut unsigned char reply_code; / reply code (see above),
    pub /: *mut *mut unsigned char reserved3[3]; / 0x000000,
    pub __packed: },
pub const TYPE86_RSP_CODE: c_uint = 0x86;
pub const TYPE87_RSP_CODE: c_uint = 0x87;
pub const TYPE86_FMT2: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type86_fmt2_ext {
    pub /: *mut *mut unsigned char reserved[4]; / 0x00000000,
    pub /: *mut *mut unsigned int apfs; / final status,
    pub /: *mut *mut unsigned int count1; / length of CPRB + parameters,
    pub /: *mut *mut unsigned int offset1; / offset to CPRB,
    pub /: *mut *mut unsigned int count2; / 0x00000000,
    pub /: *mut *mut unsigned int offset2; / db offset 0x00000000 for PKD,
    pub /: *mut *mut unsigned int count3; / 0x00000000,
    pub /: *mut *mut unsigned int offset3; / 0x00000000,
    pub /: *mut *mut unsigned int count4; / 0x00000000,
    pub /: *mut *mut unsigned int offset4; / 0x00000000,
    pub __packed: },
    pub dom): *mut *mut unsigned int fc, unsigned int,
    pub dom): *mut *mut unsigned int fc, unsigned int,
    pub dom): *mut *mut int fc, unsigned int,
pub const LOW: c_int = 10;
pub const MEDIUM: c_int = 100;
pub const HIGH: c_int = 500;
    pub speed_idx_cca(int): c_int,
    pub speed_idx_ep11(int): c_int,
    pub zcrypt_msgtype6_init(void): c_void,
    pub zcrypt_msgtype6_exit(void): c_void,
