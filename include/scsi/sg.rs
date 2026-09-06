//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/sg.h
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
// History:
// Started: Aug 9 by Lawrence Foard (entropy@world.std.com), to allow user
// process control of SCSI devices.
// Development Sponsored by Killy Corp. NY NY
//
// Original driver (sg.h):
// Copyright (C) 1992 Lawrence Foard
// Version 2 and 3 extensions to driver:
// Copyright (C) 1998 - 2014 Douglas Gilbert
//
// Version: 3.5.36 (20140603)
// This version is for 2.6 and 3 series kernels.
//
// Documentation
// =============
// A web site for the SG device driver can be found at:
// http://sg.danny.cz/sg  [alternatively check the MAINTAINERS file]
// The documentation for the sg version 3 driver can be found at:
// http://sg.danny.cz/sg/p/sg_v3_ho.html
// Also see: <kernel_source>/Documentation/scsi/scsi-generic.rst
//
// For utility and test programs see: http://sg.danny.cz/sg/sg3_utils.html
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_sg_io_hdr {
    pub /: *mut *mut compat_int_t interface_id; / [i] 'S' for SCSI generic (required),
    pub /: *mut *mut compat_int_t dxfer_direction; / [i] data transfer direction,
    pub /: *mut *mut unsigned char cmd_len; / [i] SCSI command length ( <= 16 bytes),
    pub /: *mut *mut unsigned char mx_sb_len; / [i] max length to write to sbp,
    pub /: *mut *mut unsigned short iovec_count; / [i] 0 implies no scatter gather,
    pub /: *mut *mut compat_uint_t dxfer_len; / [i] byte count of data transfer,
    pub memory: *mut *mut *mut compat_uint_t dxferp; / [i], [io] points to data transfer,
    pub /: *mut *mut *mut compat_uptr_t cmdp; / [i], [i] points to command to perform,
    pub /: *mut *mut *mut compat_uptr_t sbp; / [i], [o] points to sense_buffer memory,
    pub /: *mut *mut compat_uint_t timeout; / [i] MAX_UINT->no timeout (unit: millisec),
    pub /: *mut *mut compat_uint_t flags; / [i] 0 -> default, see SG_FLAG...,
    pub /: *mut *mut compat_int_t pack_id; / [i->o] unused internally (normally),
    pub /: *mut *mut compat_uptr_t usr_ptr; / [i->o] unused internally,
    pub /: *mut *mut unsigned char status; / [o] scsi status,
    pub /: *mut *mut unsigned char masked_status; / [o] shifted, masked scsi status,
    pub /: *mut *mut unsigned char msg_status; / [o] messaging level data (optional),
    pub /: *mut *mut unsigned char sb_len_wr; / [o] byte count actually written to sbp,
    pub /: *mut *mut unsigned short host_status; / [o] errors from host adapter,
    pub /: *mut *mut unsigned short driver_status; / [o] errors from software driver,
    pub /: *mut *mut compat_int_t resid; / [o] dxfer_len - actual_transferred,
    pub /: *mut *mut compat_uint_t duration; / [o] time taken by cmd (unit: millisec),
    pub /: *mut *mut compat_uint_t info; / [o] auxiliary information,
}

// Use negative values to flag difference from original sg_header structure

// following flag values can be "or"-ed together

// command block (when <= SCSI_2)

pub const SG_FLAG_NO_DXFER: c_uint = 0x10000 /* no transfer of kernel buffers to/from */;
// user space (debug indirect IO)
// defaults:: for sg driver: Q_AT_HEAD; for block layer: Q_AT_TAIL
pub const SG_FLAG_Q_AT_TAIL: c_uint = 0x10;
pub const SG_FLAG_Q_AT_HEAD: c_uint = 0x20;
// following 'info' values are "or"-ed together
pub const SG_INFO_OK_MASK: c_uint = 0x1;
pub const SG_INFO_OK: c_uint = 0x0          /* no sense, host nor driver "noise" */;
pub const SG_INFO_CHECK: c_uint = 0x1       /* something abnormal happened */;
pub const SG_INFO_DIRECT_IO_MASK: c_uint = 0x6;
pub const SG_INFO_INDIRECT_IO: c_uint = 0x0 /* data xfer via kernel buffers (or no xfer) */;
pub const SG_INFO_DIRECT_IO: c_uint = 0x2   /* direct IO requested and performed */;
pub const SG_INFO_MIXED_IO: c_uint = 0x4    /* part direct, part indirect IO */;
//
// Obsolete DRIVER_SENSE driver byte
//
// Originally the SCSI midlayer would set the DRIVER_SENSE driver byte when
// a sense code was generated and a sense buffer was allocated.
// However, as nowadays every scsi command has a sense code allocated this
// distinction became moot as one could check the sense buffer directly.
// Consequently this byte is not set anymore from the midlayer, but SG will
// keep setting this byte to be compatible with previous releases.
//
pub const DRIVER_SENSE: c_uint = 0x08;
// Obsolete driver_byte() declaration

//
// Original linux SCSI Status codes. They are shifted 1 bit right
// from those found in the SCSI standards.
//
pub const GOOD: c_uint = 0x00;
pub const CHECK_CONDITION: c_uint = 0x01;
pub const CONDITION_GOOD: c_uint = 0x02;
pub const BUSY: c_uint = 0x04;
pub const INTERMEDIATE_GOOD: c_uint = 0x08;
pub const INTERMEDIATE_C_GOOD: c_uint = 0x0a;
pub const RESERVATION_CONFLICT: c_uint = 0x0c;
pub const COMMAND_TERMINATED: c_uint = 0x11;
pub const QUEUE_FULL: c_uint = 0x14;
pub const ACA_ACTIVE: c_uint = 0x18;
pub const TASK_ABORTED: c_uint = 0x20;
// Obsolete status_byte() declaration

// IOCTLs: Those ioctls that are relevant to the SG 3.x drivers follow.
pub const SG_EMULATED_HOST: c_uint = 0x2203 /* true for emulated host adapter (ATAPI) */;
// Used to configure SCSI command transformation layer for ATAPI devices
// Only supported by the ide-scsi driver
pub const SG_SET_TRANSFORM: c_uint = 0x2204 /* N.B. 3rd arg is not pointer but value: */;
// 3rd arg = 0 to disable transform, 1 to enable it
pub const SG_GET_TRANSFORM: c_uint = 0x2205;
pub const SG_SET_RESERVED_SIZE: c_uint = 0x2275  /* request a new reserved buffer size */;
pub const SG_GET_RESERVED_SIZE: c_uint = 0x2272  /* actual size of reserved buffer */;
// The following ioctl has a 'sg_scsi_id_t *' object as its 3rd argument.
pub const SG_GET_SCSI_ID: c_uint = 0x2276   /* Yields fd's bus, chan, dev, lun + type */;
// SCSI id information can also be obtained from SCSI_IOCTL_GET_IDLUN
// Override host setting and always DMA using low memory ( <16MB on i386)
pub const SG_SET_FORCE_LOW_DMA: c_uint = 0x2279  /* 0-> use adapter setting, 1-> force */;
pub const SG_GET_LOW_DMA: c_uint = 0x227a   /* 0-> use all ram for dma; 1-> low dma ram */;
// When SG_SET_FORCE_PACK_ID set to 1, pack_id is input to read() which
pub const SG_SET_FORCE_PACK_ID: c_uint = 0x227b;
pub const SG_GET_PACK_ID: c_uint = 0x227c /* Yields oldest readable pack_id (or -1) */;
pub const SG_GET_NUM_WAITING: c_uint = 0x227d /* Number of commands awaiting read() */;
// Yields max scatter gather tablesize allowed by current host adapter
pub const SG_GET_SG_TABLESIZE: c_uint = 0x227F  /* 0 implies can't do scatter gather */;
pub const SG_GET_VERSION_NUM: c_uint = 0x2282 /* Example: version 2.1.34 yields 20134 */;
// Returns -EBUSY if occupied. 3rd argument pointer to int (see next)
pub const SG_SCSI_RESET: c_uint = 0x2284;
// Associated values that can be given to SG_SCSI_RESET follow.
// SG_SCSI_RESET_NO_ESCALATE may be OR-ed to the _DEVICE, _TARGET, _BUS
// or _HOST reset value so only that action is attempted.
pub const SG_SCSI_RESET_NOTHING: c_int = 0;
pub const SG_SCSI_RESET_DEVICE: c_int = 1;
pub const SG_SCSI_RESET_BUS: c_int = 2;
pub const SG_SCSI_RESET_HOST: c_int = 3;
pub const SG_SCSI_RESET_TARGET: c_int = 4;
pub const SG_SCSI_RESET_NO_ESCALATE: c_uint = 0x100;
// synchronous SCSI command ioctl, (only in version 3 interface)
pub const SG_IO: c_uint = 0x2285   /* similar effect as write() followed by read() */;
pub const SG_GET_REQUEST_TABLE: c_uint = 0x2286   /* yields table of active requests */;
// How to treat EINTR during SG_IO ioctl(), only in SG 3.x series
pub const SG_SET_KEEP_ORPHAN: c_uint = 0x2287 /* 1 -> hold for read(), 0 -> drop (def) */;
pub const SG_GET_KEEP_ORPHAN: c_uint = 0x2288;
// yields scsi midlevel's access_count for this SCSI device
pub const SG_GET_ACCESS_COUNT: c_uint = 0x2289;

// Largest size (in bytes) a single scatter-gather list element can have.
pub const SG_DEFAULT_RETRIES: c_int = 0;
// Defaults, commented if they differ from original sg driver
pub const SG_DEF_FORCE_PACK_ID: c_int = 0;
pub const SG_DEF_KEEP_ORPHAN: c_int = 0;

// maximum outstanding requests, write() yields EDOM if exceeded
pub const SG_MAX_QUEUE: c_int = 16;

// Alternate style type names, "..._t" variants preferred
pub type Sg_io_hdr = sg_io_hdr;
pub type Sg_io_vec = sg_io_vec;
pub type Sg_scsi_id = sg_scsi_id;
pub type Sg_req_info = sg_req_info;
// vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
// The older SG interface based on the 'sg_header' structure follows.
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

// [i] Force 12 byte command length for group 6 & 7 commands
// IOCTLs: The following are not required (or ignored) when the sg_io_hdr_t
pub const SG_SET_TIMEOUT: c_uint = 0x2201  /* unit: jiffies (10ms on i386) */;
pub const SG_GET_TIMEOUT: c_uint = 0x2202  /* yield timeout as _return_ value */;
// Get/set command queuing state per fd (default is SG_DEF_COMMAND_Q.
pub const SG_GET_COMMAND_Q: c_uint = 0x2270   /* Yields 0 (queuing off) or 1 (on) */;
pub const SG_SET_COMMAND_Q: c_uint = 0x2271   /* Change queuing state with 0 or 1 */;
// Turn on/off error sense trace (1 and 0 respectively, default is off).
pub const SG_SET_DEBUG: c_uint = 0x227e    /* 0 -> turn off debug */;
pub const SG_NEXT_CMD_LEN: c_uint = 0x2283  /* override SCSI command length with given;
// Defaults, commented if they differ from original sg driver

pub const SG_DEF_UNDERRUN_FLAG: c_int = 0;
