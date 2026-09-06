//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi.h
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
// This header file contains public constants and structures used by
// the SCSI initiator code.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_timeouts {
    SCSI_DEFAULT_EH_TIMEOUT		= 10 * HZ,
}

//
// DIX-capable adapters effectively support infinite chaining for the
// protection information scatterlist
//
pub const SCSI_MAX_PROT_SG_SEGMENTS: c_uint = 0xFFFF;
//
// Special value for scanning to specify scanning or rescanning of all
// possible channels, (target) ids, or luns on a given shost.
//

//
// standard mode-select header prepended to all mode-select commands
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_modesel_head {
    pub /: *mut *mut __u8 _r1; / reserved,
    pub /: *mut *mut __u8 medium; / device-specific medium type,
    pub /: *mut *mut __u8 _r2; / reserved,
    pub /: *mut *mut __u8 block_desc_length; / block descriptor length,
    pub /: *mut *mut __u8 density; / device-specific density code,
    pub /: *mut *mut __u8 number_blocks_hi; / number of blocks in this block desc,
    pub number_blocks_med: __u8,
    pub number_blocks_lo: __u8,
    pub _r3: __u8,
    pub /: *mut *mut __u8 block_length_hi; / block length for blocks in this desc,
    pub block_length_med: __u8,
    pub block_length_lo: __u8,
}

//
// The Well Known LUNS (SAM-3) in our int representation of a LUN
//
pub const SCSI_W_LUN_BASE: c_uint = 0xc100;

//
// scsi_status_is_check_condition - check the status return.
//
// @status: the status passed up from the driver (including host and
// driver components)
//
// Returns: %true if the status code is SAM_STAT_CHECK_CONDITION.
//
// Extended message codes.
//
pub const EXTENDED_MODIFY_DATA_POINTER: c_uint = 0x00;
pub const EXTENDED_SDTR: c_uint = 0x01;
pub const EXTENDED_EXTENDED_IDENTIFY: c_uint = 0x02    /* SCSI-I only */;
pub const EXTENDED_WDTR: c_uint = 0x03;
pub const EXTENDED_PPR: c_uint = 0x04;
pub const EXTENDED_MODIFY_BIDI_DATA_PTR: c_uint = 0x05;
//
// Internal return values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_disposition {
    NEEDS_RETRY		= 0x2001,
    SUCCESS			= 0x2002,
    FAILED			= 0x2003,
    QUEUED			= 0x2004,
    SOFT_ERROR		= 0x2005,
    ADD_TO_MLQUEUE		= 0x2006,
    TIMEOUT_ERROR		= 0x2007,
    SCSI_RETURN_NOT_HANDLED	= 0x2008,
    FAST_IO_FAIL		= 0x2009,
}

//
// Status values returned by the .queuecommand() callback if a command has not
// been queued.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_qc_status {
    SCSI_MLQUEUE_HOST_BUSY   = 0x1055,
    SCSI_MLQUEUE_DEVICE_BUSY = 0x1056,
    SCSI_MLQUEUE_EH_RETRY    = 0x1057,
    SCSI_MLQUEUE_TARGET_BUSY = 0x1058,
}

//
// Use these to separate status msg and our bytes
//
// These are set by:
//
// status byte = set from target device
// msg_byte    (unused)
// host_byte   = set by low-level driver to indicate status.
//

//
// default timeouts
//

pub const IDENTIFY_BASE: c_uint = 0x80;

//
// struct scsi_device::scsi_level values. For SCSI devices other than those
// prior to SCSI-2 (i.e. over 12 years old) this value is (resp[2] + 1)
// where "resp" is a byte array of the response to an INQUIRY. The scsi_level
// variable is visible to the user via sysfs.
//
pub const SCSI_UNKNOWN: c_int = 0;
pub const SCSI_1: c_int = 1;
pub const SCSI_1_CCS: c_int = 2;
pub const SCSI_2: c_int = 3;

pub const SCSI_SPC_2: c_int = 5;
pub const SCSI_SPC_3: c_int = 6;
pub const SCSI_SPC_4: c_int = 7;
pub const SCSI_SPC_5: c_int = 8;
pub const SCSI_SPC_6: c_int = 14;
//
// INQ PERIPHERAL QUALIFIERS
//
pub const SCSI_INQ_PQ_CON: c_uint = 0x00;
pub const SCSI_INQ_PQ_NOT_CON: c_uint = 0x01;
pub const SCSI_INQ_PQ_NOT_CAP: c_uint = 0x03;
//
// Here are some scsi specific ioctl commands which are sometimes useful.
//
// Note that include/linux/cdrom.h also defines IOCTL 0x5300 - 0x5395
//
// Used to obtain PUN and LUN info.  Conflicts with CDROMAUDIOBUFSIZ
pub const SCSI_IOCTL_GET_IDLUN: c_uint = 0x5382;
// 0x5383 and 0x5384 were used for SCSI_IOCTL_TAGGED_{ENABLE,DISABLE}
// Used to obtain the host number of a device.
pub const SCSI_IOCTL_PROBE_HOST: c_uint = 0x5385;
// Used to obtain the bus number for a device
pub const SCSI_IOCTL_GET_BUS_NUMBER: c_uint = 0x5386;
// Used to obtain the PCI location of a device
pub const SCSI_IOCTL_GET_PCI: c_uint = 0x5387;
//
// scsi_status_is_good - check the status return.
//
// @status: the status passed up from the driver (including host and
// driver components)
//
// Returns: %true for known good conditions that may be treated as
// command completed normally
//
// FIXME: bit0 is listed as reserved in SCSI-2, but is
// significant in SCSI-3.  For now, we follow the SCSI-2
// behaviour and ignore reserved bits.
//
// Next two "intermediate" statuses are obsolete in SAM-4
// FIXME: this is obsolete in SAM-3
