//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/storage.h
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


// SPDX-License-Identifier: GPL-2.0
//
// linux/usb/storage.h
//
// Copyright Matthew Wilcox for Intel Corp, 2010
//
// This file contains definitions taken from the
// USB Mass Storage Class Specification Overview
//
// Storage subclass codes
pub const USB_SC_RBC: c_uint = 0x01		/* Typically, flash devices */;
pub const USB_SC_8020: c_uint = 0x02		/* CD-ROM */;
pub const USB_SC_QIC: c_uint = 0x03		/* QIC-157 Tapes */;
pub const USB_SC_UFI: c_uint = 0x04		/* Floppy */;
pub const USB_SC_8070: c_uint = 0x05		/* Removable media */;
pub const USB_SC_SCSI: c_uint = 0x06		/* Transparent */;
pub const USB_SC_LOCKABLE: c_uint = 0x07		/* Password-protected */;
pub const USB_SC_ISD200: c_uint = 0xf0		/* ISD200 ATA */;
pub const USB_SC_CYP_ATACB: c_uint = 0xf1	/* Cypress ATACB */;
pub const USB_SC_DEVICE: c_uint = 0xff		/* Use device's value */;
// Storage protocol codes
pub const USB_PR_CBI: c_uint = 0x00		/* Control/Bulk/Interrupt */;
pub const USB_PR_CB: c_uint = 0x01		/* Control/Bulk w/o interrupt */;
pub const USB_PR_BULK: c_uint = 0x50		/* bulk only */;
pub const USB_PR_UAS: c_uint = 0x62		/* USB Attached SCSI */;
pub const USB_PR_USBAT: c_uint = 0x80		/* SCM-ATAPI bridge */;
pub const USB_PR_EUSB_SDDR09: c_uint = 0x81	/* SCM-SCSI bridge for SDDR-09 */;
pub const USB_PR_SDDR55: c_uint = 0x82		/* SDDR-55 (made up) */;
pub const USB_PR_DPCM_USB: c_uint = 0xf0		/* Combination CB/SDDR09 */;
pub const USB_PR_FREECOM: c_uint = 0xf1		/* Freecom */;
pub const USB_PR_DATAFAB: c_uint = 0xf2		/* Datafab chipsets */;
pub const USB_PR_JUMPSHOT: c_uint = 0xf3		/* Lexar Jumpshot */;
pub const USB_PR_ALAUDA: c_uint = 0xf4		/* Alauda chipsets */;
pub const USB_PR_KARMA: c_uint = 0xf5		/* Rio Karma */;
pub const USB_PR_DEVICE: c_uint = 0xff		/* Use device's value */;
//
// Bulk only data structures
//
// command block wrapper
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bulk_cb_wrap {
    pub /: *mut *mut __le32 Signature; / contains 'USBC',
    pub /: *mut *mut __u32 Tag; / unique per command id,
    pub /: *mut *mut __le32 DataTransferLength; / size of data,
    pub /: *mut *mut __u8 Flags; / direction in bit 7,
    pub /: *mut *mut __u8 Lun; / LUN normally 0,
    pub /: *mut *mut __u8 Length; / length of the CDB,
    pub /: *mut *mut __u8 CDB[16]; / max command,
}

pub const US_BULK_CB_WRAP_LEN: c_int = 31;
pub const US_BULK_CB_SIGN: c_uint = 0x43425355	/* spells out 'USBC' */;

pub const US_BULK_FLAG_OUT: c_int = 0;
// command status wrapper
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bulk_cs_wrap {
    pub /: *mut *mut __le32 Signature; / contains 'USBS',
    pub /: *mut *mut __u32 Tag; / same as original command,
    pub /: *mut *mut __le32 Residue; / amount not transferred,
    pub /: *mut *mut __u8 Status; / see below,
}

pub const US_BULK_CS_WRAP_LEN: c_int = 13;
pub const US_BULK_CS_SIGN: c_uint = 0x53425355      /* spells out 'USBS' */;
pub const US_BULK_STAT_OK: c_int = 0;
pub const US_BULK_STAT_FAIL: c_int = 1;
pub const US_BULK_STAT_PHASE: c_int = 2;
// bulk-only class specific requests
pub const US_BULK_RESET_REQUEST: c_uint = 0xff;
pub const US_BULK_GET_MAX_LUN: c_uint = 0xfe;
//
// If 4 LUNs are supported then the LUNs would be
// numbered from 0 to 3, and the return value for
// US_BULK_GET_MAX_LUN request would be 3. The valid
// LUN field is 4 bits wide, the upper limit is 0x0f.
//
pub const US_BULK_MAX_LUN_LIMIT: c_uint = 0x0f;
