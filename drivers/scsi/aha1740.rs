//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aha1740.h
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
// $Id$
//
// Header file for the adaptec 1740 driver for Linux
//
// With minor revisions 3/31/93
// Written and (C) 1992,1993 Brad McLean.  See aha1740.c
// for more info
//

pub const SLOTSIZE: c_uint = 0x5c;
// EISA configuration registers & values

pub const HID_PRD: c_int = 0;
pub const HID_REV: c_int = 2;
pub const EBCNTRL_VALUE: c_int = 1;
pub const PORTADDR_ENH: c_uint = 0x80;
// READ

pub const G2INTST_MASK: c_uint = 0xf0	/* isolate the status */;
pub const G2INTST_CCBGOOD: c_uint = 0x10	/* CCB Completed */;
pub const G2INTST_CCBRETRY: c_uint = 0x50	/* CCB Completed with a retry */;
pub const G2INTST_HARDFAIL: c_uint = 0x70	/* Adapter Hardware Failure */;
pub const G2INTST_CMDGOOD: c_uint = 0xa0	/* Immediate command success */;
pub const G2INTST_CCBERROR: c_uint = 0xc0	/* CCB Completed with error */;
pub const G2INTST_ASNEVENT: c_uint = 0xd0	/* Asynchronous Event Notification */;
pub const G2INTST_CMDERROR: c_uint = 0xe0	/* Immediate command error */;

// WRITE (and ReadBack)

pub const ATTN_IMMED: c_uint = 0x10	/* Immediate Command */;
pub const ATTN_START: c_uint = 0x40	/* Start CCB */;
pub const ATTN_ABORT: c_uint = 0x50	/* Abort CCB */;
pub const G2CNTRL_HRST: c_uint = 0x80	/* Hard Reset */;
pub const G2CNTRL_IRST: c_uint = 0x40	/* Clear EISA Interrupt */;
pub const G2CNTRL_HRDY: c_uint = 0x20	/* Sets HOST ready */;
// This is used with scatter-gather
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aha1740_chain {
    pub /: *mut *mut u32 dataptr; / Location of data,
    pub /: *mut *mut u32 datalen; / Size of this part of chain,
}

// These belong in scsi.h

pub const MAX_CDB: c_int = 12;
pub const MAX_SENSE: c_int = 14;
pub const MAX_STATUS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecb {
    pub /: *mut *mut u16 cmdw; / Command Word,
// Flag Word 1
    pub /: *mut *mut ars:1; / Automatic Request Sense,
// Flag Word 2
    pub /: *mut *mut :2, rec:1,:1; / Error Recovery,
    pub /: *mut *mut u16 nil0; / nothing,
    pub /: *mut *mut u32 dataptr; / Data or Scatter List ptr,
    pub /: *mut *mut u32 datalen; / Data or Scatter List len,
    pub /: *mut *mut u32 statusptr; / Status Block ptr,
    pub /: *mut *mut u32 linkptr; / Chain Address,
    pub /: *mut *mut u32 nil1; / nothing,
    pub /: *mut *mut u32 senseptr; / Sense Info Pointer,
    pub /: *mut *mut u8 senselen; / Sense Length,
    pub /: *mut *mut u8 cdblen; / CDB Length,
    pub /: *mut *mut u16 datacheck; / Data checksum,
    pub /: *mut *mut u8 cdb[MAX_CDB]; / CDB area,
// Hardware defined portion ends here, rest is driver defined
    pub /: *mut *mut u8 sense[MAX_SENSE]; / Sense area,
    pub /: *mut *mut u8 status[MAX_STATUS]; / Status area,
    pub /: *mut *mut *mut scsi_cmnd SCpnt; / Link to the SCSI Command Block,
    pub /: *mut *mut *mut *mut void (done) (struct scsi_cmnd ); / Completion Function,
}

pub const AHA1740CMD_NOP: c_uint = 0x00	/* No OP */;
pub const AHA1740CMD_INIT: c_uint = 0x01	/* Initiator SCSI Command */;
pub const AHA1740CMD_DIAG: c_uint = 0x05	/* Run Diagnostic Command */;
pub const AHA1740CMD_SCSI: c_uint = 0x06	/* Initialize SCSI */;
pub const AHA1740CMD_SENSE: c_uint = 0x08	/* Read Sense Information */;
pub const AHA1740CMD_DOWN: c_uint = 0x09	/* Download Firmware (yeah, I bet!) */;
pub const AHA1740CMD_RINQ: c_uint = 0x0a	/* Read Host Adapter Inquiry Data */;
pub const AHA1740CMD_TARG: c_uint = 0x10	/* Target SCSI Command */;
pub const AHA1740_ECBS: c_int = 32;
pub const AHA1740_SCATTER: c_int = 16;
