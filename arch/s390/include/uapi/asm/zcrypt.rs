//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/zcrypt.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// include/asm-s390/zcrypt.h
//
// zcrypt 2.2.1 (user-visible header)
//
// Copyright IBM Corp. 2001, 2022
// Author(s): Robert Burroughs
// Eric Rossman (edrossma@us.ibm.com)
//
// Hotplug & misc device support: Jochen Roehrig (roehrig@de.ibm.com)
//
pub const ZCRYPT_VERSION: c_int = 2;
pub const ZCRYPT_RELEASE: c_int = 2;
pub const ZCRYPT_VARIANT: c_int = 1;

// Name of the zcrypt device driver.

//
// struct ica_rsa_modexpo
//
// Requirements:
// - outputdatalength is at least as large as inputdatalength.
// - All key parts are right justified in their fields, padded on
// the left with zeroes.
// - length(b_key) = inputdatalength
// - length(n_modulus) = inputdatalength
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ica_rsa_modexpo {
    pub inputdata: *mut __u8 __user,
    pub inputdatalength: __u32,
    pub outputdata: *mut __u8 __user,
    pub outputdatalength: __u32,
    pub b_key: *mut __u8 __user,
    pub n_modulus: *mut __u8 __user,
}

//
// struct ica_rsa_modexpo_crt
//
// Requirements:
// - inputdatalength is even.
// - outputdatalength is at least as large as inputdatalength.
// - All key parts are right justified in their fields, padded on
// the left with zeroes.
// - length(bp_key)	= inputdatalength/2 + 8
// - length(bq_key)	= inputdatalength/2
// - length(np_key)	= inputdatalength/2 + 8
// - length(nq_key)	= inputdatalength/2
// - length(u_mult_inv) = inputdatalength/2 + 8
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ica_rsa_modexpo_crt {
    pub inputdata: *mut __u8 __user,
    pub inputdatalength: __u32,
    pub outputdata: *mut __u8 __user,
    pub outputdatalength: __u32,
    pub bp_key: *mut __u8 __user,
    pub bq_key: *mut __u8 __user,
    pub np_prime: *mut __u8 __user,
    pub nq_prime: *mut __u8 __user,
    pub u_mult_inv: *mut __u8 __user,
}

//
// CPRBX
// Note that all shorts and ints are big-endian.
// All pointer fields are 16 bytes long, and mean nothing.
//
// A request CPRB is followed by a request_parameter_block.
//
// The request (or reply) parameter block is organized thus:
// function code
// VUD block
// key block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPRBX {
    pub /: *mut *mut __u16 cprb_len; / CPRB length 220,
    pub /: *mut *mut __u8 cprb_ver_id; / CPRB version id. 0x02,
    pub /: *mut *mut __u8 ctfm; / Command Type Filtering Mask,
    pub /: *mut *mut __u8 pad_000[2]; / Alignment pad bytes,
    pub /: *mut *mut __u8 func_id[2]; / function id 0x5432,
    pub /: *mut *mut __u8 cprb_flags[4]; / Flags,
    pub /: *mut *mut __u32 req_parml; / request parameter buffer len,
    pub /: *mut *mut __u32 req_datal; / request data buffer,
    pub /: *mut *mut __u32 rpl_msgbl; / reply message block length,
    pub /: *mut *mut __u32 rpld_parml; / replied parameter block len,
    pub /: *mut *mut __u32 rpl_datal; / reply data block len,
    pub /: *mut *mut __u32 rpld_datal; / replied data block len,
    pub /: *mut *mut __u32 req_extbl; / request extension block len,
    pub /: *mut *mut __u8 _pad_001[4]; / reserved,
    pub /: *mut *mut __u32 rpld_extbl; / replied extension block len,
    pub )]: *mut __u8 _pad_002[16 - sizeof(__u8,
    pub /: *mut *mut *mut __u8 __user req_parmb; / request parm block 'address',
    pub )]: *mut __u8 _pad_003[16 - sizeof(__u8,
    pub /: *mut *mut *mut __u8 __user req_datab; / request data block 'address',
    pub )]: *mut __u8 _pad_004[16 - sizeof(__u8,
    pub /: *mut *mut *mut __u8 __user rpl_parmb; / reply parm block 'address',
    pub )]: *mut __u8 _pad_005[16 - sizeof(__u8,
    pub /: *mut *mut *mut __u8 __user rpl_datab; / reply data block 'address',
    pub )]: *mut __u8 _pad_006[16 - sizeof(__u8,
    pub 'addr'*/: *mut *mut *mut __u8 __user req_extb; / request extension block,
    pub )]: *mut __u8 _pad_007[16 - sizeof(__u8,
    pub 'address'*/: *mut *mut *mut __u8 __user rpl_extb; / reply extension block,
    pub /: *mut *mut __u16 ccp_rtcode; / server return code,
    pub /: *mut *mut __u16 ccp_rscode; / server reason code,
    pub /: *mut *mut __u32 mac_data_len; / Mac Data Length,
    pub /: *mut *mut __u8 logon_id[8]; / Logon Identifier,
    pub /: *mut *mut __u8 mac_value[8]; / Mac Value,
    pub /: *mut *mut __u8 mac_content_flgs; / Mac content flag byte,
    pub /: *mut *mut __u8 _pad_008; / Alignment,
    pub /: *mut *mut __u16 domain; / Domain,
    pub /: *mut *mut __u8 _pad_009[12]; / reserved, checked for zeros,
    pub /: *mut *mut __u8 _pad_010[36]; / reserved,
    pub __attribute__((packed)): },
//
// xcRB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ica_xcRB {
    pub agent_ID: __u16,
    pub user_defined: __u32,
    pub request_ID: __u16,
    pub request_control_blk_length: __u32,
    pub )]: *mut __u8 _padding1[16 - sizeof(__u8,
    pub request_control_blk_addr: *mut __u8 __user,
    pub request_data_length: __u32,
    pub )]: *mut __u8 _padding2[16 - sizeof(__u8,
    pub request_data_address: *mut __u8 __user,
    pub reply_control_blk_length: __u32,
    pub )]: *mut __u8 _padding3[16 - sizeof(__u8,
    pub reply_control_blk_addr: *mut __u8 __user,
    pub reply_data_length: __u32,
    pub )]: *mut __u8 __padding4[16 - sizeof(__u8,
    pub reply_data_addr: *mut __u8 __user,
    pub priority_window: __u16,
    pub status: __u32,
    pub __attribute__((packed)): },
//
// struct ep11_cprb - EP11 connectivity programming request block
// @cprb_len:		CPRB header length [0x0020]
// @cprb_ver_id:	CPRB version id.   [0x04]
// @pad_000:		Alignment pad bytes
// @flags:		Admin bit [0x80], Special bit [0x20]
// @func_id:		Function id / subtype [0x5434] "T4"
// @source_id:		Source id [originator id]
// @target_id:		Target id [usage/ctrl domain id]
// @ret_code:		Return code
// @reserved1:		Reserved
// @reserved2:		Reserved
// @payload_len:	Payload length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep11_cprb {
    pub cprb_len: __u16,
    pub cprb_ver_id: __u8,
    pub pad_000: [__u8; 2],
    pub flags: __u8,
    pub func_id: [__u8; 2],
    pub source_id: __u32,
    pub target_id: __u32,
    pub ret_code: __u32,
    pub reserved1: __u32,
    pub reserved2: __u32,
    pub payload_len: __u32,
    pub __attribute__((packed)): },
//
// struct ep11_target_dev - EP11 target device list
// @ap_id:	AP device id
// @dom_id:	Usage domain id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep11_target_dev {
    pub ap_id: __u16,
    pub dom_id: __u16,
}

//
// struct ep11_urb - EP11 user request block
// @targets_num:	Number of target adapters
// @targets:		Addr to target adapter list
// @weight:		Level of request priority
// @req_no:		Request id/number
// @req_len:		Request length
// @req:		Addr to request block
// @resp_len:		Response length
// @resp:		Addr to response block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep11_urb {
    pub targets_num: __u16,
    pub targets: *mut __u8 __user,
    pub weight: __u64,
    pub req_no: __u64,
    pub req_len: __u64,
    pub req: *mut __u8 __user,
    pub resp_len: __u64,
    pub resp: *mut __u8 __user,
    pub __attribute__((packed)): },
//
// struct zcrypt_device_status_ext
// @hwtype:		raw hardware type
// @qid:		8 bit device index, 8 bit domain
// @functions:		AP device function bit field 'abcdef'
// a, b, c = reserved
// d = CCA coprocessor
// e = Accelerator
// f = EP11 coprocessor
// @online		online status
// @reserved		reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrypt_device_status_ext {
    pub hwtype:8: c_uint,
    pub qid:16: c_uint,
    pub online:1: c_uint,
    pub functions:6: c_uint,
    pub reserved:1: c_uint,
}

pub const MAX_ZDEV_CARDIDS_EXT: c_int = 256;
pub const MAX_ZDEV_DOMAINS_EXT: c_int = 256;
// Maximum number of zcrypt devices

// Device matrix of all zcrypt devices
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrypt_device_matrix_ext {
    pub device: [zcrypt_device_status_ext; MAX_ZDEV_ENTRIES_EXT],
}

pub const AUTOSELECT: c_uint = 0xFFFFFFFF;

//
// Interface notes:
//
// The ioctl()s which are implemented (along with relevant details)
// are:
//
// ICARSAMODEXPO
// Perform an RSA operation using a Modulus-Exponent pair
// This takes an ica_rsa_modexpo struct as its arg.
//
// NOTE: please refer to the comments preceding this structure
// for the implementation details for the contents of the
// block
//
// ICARSACRT
// Perform an RSA operation using a Chinese-Remainder Theorem key
// This takes an ica_rsa_modexpo_crt struct as its arg.
//
// NOTE: please refer to the comments preceding this structure
// for the implementation details for the contents of the
// block
//
// ZSECSENDCPRB
// Send an arbitrary CPRB to a crypto card.
//
// ZSENDEP11CPRB
// Send an arbitrary EP11 CPRB to an EP11 coprocessor crypto card.
//
// ZCRYPT_DEVICE_STATUS
// The given struct zcrypt_device_matrix_ext is updated with
// status information for each currently known apqn.
//
// ZCRYPT_STATUS_MASK
// Return an MAX_ZDEV_CARDIDS_EXT element array of unsigned chars for the
// status of all devices.
// 0x01: PCICA
// 0x02: PCICC
// 0x03: PCIXCC_MCL2
// 0x04: PCIXCC_MCL3
// 0x05: CEX2C
// 0x06: CEX2A
// 0x07: CEX3C
// 0x08: CEX3A
// 0x0a: CEX4
// 0x0b: CEX5
// 0x0c: CEX6, CEX7 or CEX8
// 0x0d: device is disabled
//
// ZCRYPT_QDEPTH_MASK
// Return an MAX_ZDEV_CARDIDS_EXT element array of unsigned chars for the
// queue depth of all devices.
//
// ZCRYPT_PERDEV_REQCNT
// Return an MAX_ZDEV_CARDIDS_EXT element array of unsigned integers for
// the number of successfully completed requests per device since the
// device was detected and made available.
//
// Supported ioctl calls
//

//
// Support for multiple zcrypt device nodes.
//
// Nr of minor device node numbers to allocate.
pub const ZCRYPT_MAX_MINOR_NODES: c_int = 256;
// Max amount of possible ioctls

//
// Only deprecated defines, structs and ioctls below this line.
//
// Deprecated: use MAX_ZDEV_CARDIDS_EXT
pub const MAX_ZDEV_CARDIDS: c_int = 64;
// Deprecated: use MAX_ZDEV_DOMAINS_EXT
pub const MAX_ZDEV_DOMAINS: c_int = 256;
// Deprecated: use MAX_ZDEV_ENTRIES_EXT

// Deprecated: use struct zcrypt_device_status_ext
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrypt_device_status {
    pub hwtype:8: c_uint,
    pub qid:14: c_uint,
    pub online:1: c_uint,
    pub functions:6: c_uint,
    pub reserved:3: c_uint,
}

// Deprecated: use struct zcrypt_device_matrix_ext
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zcrypt_device_matrix {
    pub device: [zcrypt_device_status; MAX_ZDEV_ENTRIES],
}

// Deprecated: use ZCRYPT_DEVICE_STATUS

// Deprecated: use ZCRYPT_STATUS_MASK

// Deprecated: use ZCRYPT_QDEPTH_MASK

// Deprecated: use ZCRYPT_PERDEV_REQCNT

// Deprecated: use sysfs to query these values

//
// The ioctl number ranges 0x40 - 0x42 and 0x4b - 0x4e had been used in the
// past, don't assign new ioctls for these.
//
