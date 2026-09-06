//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bfa_defs_mfg_comm.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

// Manufacturing block version
pub const BFA_MFG_VERSION: c_int = 3;
pub const BFA_MFG_VERSION_UNINIT: c_uint = 0xFF;
// Manufacturing block encrypted version
pub const BFA_MFG_ENC_VER: c_int = 2;
// Manufacturing block version 1 length
pub const BFA_MFG_VER1_LEN: c_int = 128;
// Manufacturing block header length
pub const BFA_MFG_HDR_LEN: c_int = 4;
pub const BFA_MFG_SERIALNUM_SIZE: c_int = 11;

// Manufacturing card type
// Check if Mezz card

// VPD data length
pub const BFA_MFG_VPD_LEN: c_int = 512;
pub const BFA_MFG_VPD_LEN_INVALID: c_int = 0;
pub const BFA_MFG_VPD_PCI_HDR_OFF: c_int = 137;
pub const BFA_MFG_VPD_PCI_VER_MASK: c_uint = 0x07	/*!< version mask 3 bits */;
pub const BFA_MFG_VPD_PCI_VDR_MASK: c_uint = 0xf8	/*!< vendor mask 5 bits */;
// VPD vendor tag
// BFA adapter flash vpd data definition.
//
// All numerical fields are in big-endian format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_mfg_vpd {
    pub /: *mut *mut u8 version; /!< vpd data version,
    pub /: *mut *mut u8 vpd_sig[3]; /!< characters 'V', 'P', 'D',
    pub /: *mut *mut u8 chksum; /!< u8 checksum,
    pub /: *mut *mut u8 vendor; /!< vendor,
    pub /: *mut *mut u8 len; /!< vpd data length excluding header,
    pub rsv: u8,
    pub /: *mut *mut u8 data[BFA_MFG_VPD_LEN]; /!< vpd data,
    pub __packed: },
