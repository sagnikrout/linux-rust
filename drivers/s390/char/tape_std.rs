//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/char/tape_std.h
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
// standard tape device functions for ibm tapes.
//
// Copyright IBM Corp. 2001, 2006
// Author(s): Carsten Otte <cotte@de.ibm.com>
// Tuan Ngo-Anh <ngoanh@de.ibm.com>
// Martin Schwidefsky <schwidefsky@de.ibm.com>
//
// Biggest block size of 256K to handle.
//
pub const MAX_BLOCKSIZE: c_int = 262144;
//
// The CCW commands for the Tape type of command.
//
pub const BACKSPACEBLOCK: c_uint = 0x27	/* Back Space block */;
pub const BACKSPACEFILE: c_uint = 0x2f	/* Back Space file */;
pub const DATA_SEC_ERASE: c_uint = 0x97	/* Data security erase */;
pub const ERASE_GAP: c_uint = 0x17	/* Erase Gap */;
pub const FORSPACEBLOCK: c_uint = 0x37	/* Forward space block */;
pub const FORSPACEFILE: c_uint = 0x3F	/* Forward Space file */;
pub const NOP: c_uint = 0x03	/* No operation	*/;
pub const READ_FORWARD: c_uint = 0x02	/* Read forward */;
pub const REWIND: c_uint = 0x07	/* Rewind */;
pub const REWIND_UNLOAD: c_uint = 0x0F	/* Rewind and Unload */;
pub const SENSE: c_uint = 0x04	/* Sense */;
pub const WRITE_CMD: c_uint = 0x01	/* Write */;
pub const WRITETAPEMARK: c_uint = 0x1F	/* Write Tape Mark */;
pub const ASSIGN: c_uint = 0xB7	/* Assign */;
pub const LOCATE: c_uint = 0x4F	/* Locate Block */;
pub const MODE_SET_DB: c_uint = 0xDB	/* Mode Set */;
pub const READ_BLOCK_ID: c_uint = 0x22	/* Read Block ID */;
pub const UNASSIGN: c_uint = 0xC7	/* Unassign */;
pub const SENSE_COMMAND_REJECT: c_uint = 0x80;
pub const SENSE_INTERVENTION_REQUIRED: c_uint = 0x40;
pub const SENSE_BUS_OUT_CHECK: c_uint = 0x20;
pub const SENSE_EQUIPMENT_CHECK: c_uint = 0x10;
pub const SENSE_DATA_CHECK: c_uint = 0x08;
pub const SENSE_OVERRUN: c_uint = 0x04;
pub const SENSE_DEFERRED_UNIT_CHECK: c_uint = 0x02;
pub const SENSE_ASSIGNED_ELSEWHERE: c_uint = 0x01;
pub const SENSE_LOCATE_FAILURE: c_uint = 0x80;
pub const SENSE_DRIVE_ONLINE: c_uint = 0x40;
pub const SENSE_RESERVED: c_uint = 0x20;
pub const SENSE_RECORD_SEQUENCE_ERR: c_uint = 0x10;
pub const SENSE_BEGINNING_OF_TAPE: c_uint = 0x08;
pub const SENSE_WRITE_MODE: c_uint = 0x04;
pub const SENSE_WRITE_PROTECT: c_uint = 0x02;
pub const SENSE_NOT_CAPABLE: c_uint = 0x01;
pub const SENSE_CHANNEL_ADAPTER_CODE: c_uint = 0xE0;
pub const SENSE_CHANNEL_ADAPTER_LOC: c_uint = 0x10;
pub const SENSE_REPORTING_CU: c_uint = 0x08;
pub const SENSE_AUTOMATIC_LOADER: c_uint = 0x04;
pub const SENSE_TAPE_SYNC_MODE: c_uint = 0x02;
pub const SENSE_TAPE_POSITIONING: c_uint = 0x01;
// discipline functions
// Some non-mtop commands.
extern "C" {
    pub fn tape_std_assign(: *mut tape_device) -> c_int;
}
extern "C" {
    pub fn tape_std_unassign(: *mut tape_device) -> c_int;
}
extern "C" {
    pub fn tape_std_read_block_id(device: *mut tape_device, id: *mut __u64) -> c_int;
}
extern "C" {
    pub fn tape_std_terminate_write(: *mut tape_device) -> c_int;
}
// Standard magnetic tape commands.
extern "C" {
    pub fn tape_std_mtbsf(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtbsfm(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtbsr(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtcompression(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mteom(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mterase(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtfsf(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtfsfm(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtfsr(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtload(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtnop(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtoffl(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtreset(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtreten(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtrew(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtsetblk(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtunload(: *mut tape_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tape_std_mtweof(: *mut tape_device, _arg: c_int) -> c_int;
}
// Event handlers
extern "C" {
    pub fn tape_std_process_eov(: *mut tape_device);
}
// S390 tape types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s390_tape_type {
    tape_3490,
}
