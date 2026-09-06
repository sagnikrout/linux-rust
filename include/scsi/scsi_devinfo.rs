//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_devinfo.h
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
// Flags for SCSI devices that need special treatment
//
// Only scan LUN 0

// Known to have LUNs, force scanning.
// DEPRECATED: Use max_luns=N

// Flag for broken handshaking

// unlock by special command

// Do not use LUNs in parallel

// Buggy Tagged Command Queuing

// Non consecutive LUN numbering

// Avoid LUNS >= 5

// Treat as (removable) CD-ROM

// LUNs past 7 on a SCSI-2 device

// override additional length field

// ignore MEDIA CHANGE unit attention after resuming from runtime suspend

// do not do automatic start on add

// do not ask for VPD page size first on some broken targets

// try REPORT_LUNS even for SCSI-2 devs (if HBA supports more than 8 LUNs)

// don't try REPORT_LUNS scan (SCSI-3 devs)

// don't use PREVENT-ALLOW commands

// device is actually for RAID config

// select without ATN

// retry HARDWARE_ERROR

// maximum 512 sector cdb length

// Disable T10 PI (DIF)

// Ignore SBC-3 VPD pages

// Attempt to read VPD pages

// don't try to issue RSOC

// maximum 1024 sector cdb length

// Use UNMAP limit for WRITE SAME

// Always retry ABORTED_COMMAND with Internal Target Failure

// Always retry ABORTED_COMMAND with ASC 0xc1

// Do not query the IO Advice Hints Grouping mode page

