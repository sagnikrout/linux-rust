//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/common/smb1pdu.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright (C) International Business Machines  Corp., 2002,2009
// 2018 Samsung Electronics Co., Ltd.
// Author(s): Steve French <sfrench@us.ibm.com>
// Namjae Jeon <linkinjeon@kernel.org>
//

//
// See MS-CIFS 2.2.3.1
// MS-SMB 2.2.3.1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_hdr {
    pub Protocol: [__u8; 4],
    pub Command: __u8,
    pub ErrorClass: __u8,
    pub Reserved: __u8,
    pub Error: __le16,
    pub DosError: } __packed,
    pub CifsError: __le32,
    pub Status: } __packed,
    pub Flags: __u8,
    pub /: *mut *mut __le16 Flags2; / note: le,
    pub PidHigh: __le16,
    pub /: *mut *mut __le32 SequenceNumber; / le,
    pub /: *mut *mut __u32 Reserved; / zero,
    pub Sequence: } __packed,
    pub /: *mut *mut __u8 SecuritySignature[8]; / le,
    pub Signature: } __packed,
    pub pad: [__u8; 2],
    pub Tid: __u16,
    pub Pid: __le16,
    pub Uid: __u16,
    pub Mid: __le16,
    pub WordCount: __u8,
    pub __packed: },
// See MS-CIFS 2.2.4.52.1
    pub /: *mut *mut smb_hdr hdr; / wct = 0,
    pub ByteCount: __le16,
    pub DialectsArray: [c_uchar; ],
    pub SMB_NEGOTIATE_REQ: } __packed,
