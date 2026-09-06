//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aha1542.h
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

// I/O Port interface 4.2
// READ

// BIT(1) is reserved

// WRITE

// READ/WRITE

pub const CMD_NOP: c_uint = 0x00	/* No Operation */;
pub const CMD_MBINIT: c_uint = 0x01	/* Mailbox Initialization */;
pub const CMD_START_SCSI: c_uint = 0x02	/* Start SCSI Command */;
pub const CMD_INQUIRY: c_uint = 0x04	/* Adapter Inquiry */;
pub const CMD_EMBOI: c_uint = 0x05	/* Enable MailBox Out Interrupt */;
pub const CMD_BUSON_TIME: c_uint = 0x07	/* Set Bus-On Time */;
pub const CMD_BUSOFF_TIME: c_uint = 0x08	/* Set Bus-Off Time */;
pub const CMD_DMASPEED: c_uint = 0x09	/* Set AT Bus Transfer Speed */;
pub const CMD_RETDEVS: c_uint = 0x0a	/* Return Installed Devices */;
pub const CMD_RETCONF: c_uint = 0x0b	/* Return Configuration Data */;
pub const CMD_RETSETUP: c_uint = 0x0d	/* Return Setup Data */;
pub const CMD_ECHO: c_uint = 0x1f	/* ECHO Command Data */;
pub const CMD_EXTBIOS: c_uint = 0x28    /* Return extend bios information only 1542C */;
pub const CMD_MBENABLE: c_uint = 0x29    /* Set Mailbox Interface enable only 1542C */;
// Mailbox Definition 5.2.1 and 5.2.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mailbox {
    pub /: *mut *mut u8 status; / Command/Status,
    pub /: *mut *mut u8 ccbptr[3]; / msb, .., lsb,
}

// This is used with scatter-gather
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chain {
    pub /: *mut *mut u8 datalen[3]; / Size of this part of chain,
    pub /: *mut *mut u8 dataptr[3]; / Location of data,
}

// These belong in scsi.h also

pub const MAX_CDB: c_int = 12;
pub const MAX_SENSE: c_int = 14;
// Command Control Block (CCB), 5.3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccb {
    pub /: *mut *mut u8 op; / Command Control Block Operation Code:,
// 0x00: SCSI Initiator CCB, 0x01: SCSI Target CCB,
// 0x02: SCSI Initiator CCB with Scatter/Gather,
// 0x81: SCSI Bus Device Reset CCB
    pub /: *mut *mut u8 idlun; / Address and Direction Control:,
// Bits 7-5: op=0, 2: Target ID, op=1: Initiator ID
// Bit	4: Outbound data transfer, length is checked
// Bit	3:  Inbound data transfer, length is checked
// Bits 2-0: Logical Unit Number
    pub /: *mut *mut u8 cdblen; / SCSI Command Length,
    pub /: *mut *mut u8 rsalen; / Request Sense Allocation Length/Disable Auto Sense,
    pub /: *mut *mut u8 datalen[3]; / Data Length (MSB, ..., LSB),
    pub /: *mut *mut u8 dataptr[3]; / Data Pointer (MSB, ..., LSB),
    pub /: *mut *mut u8 linkptr[3]; / Link Pointer (MSB, ..., LSB),
    pub /: *mut *mut u8 commlinkid; / Command Linking Identifier,
    pub /: *mut *mut u8 hastat; / Host Adapter Status (HASTAT),
    pub /: *mut *mut u8 tarstat; / Target Device Status (TARSTAT),
    pub reserved: [u8; 2],
    pub /: *mut *mut u8 cdb[MAX_CDB + MAX_SENSE]; / SCSI Command Descriptor Block,
// followed by the Auto Sense data
}

pub const AHA1542_REGION_SIZE: c_int = 4;
pub const AHA1542_MAILBOXES: c_int = 8;
