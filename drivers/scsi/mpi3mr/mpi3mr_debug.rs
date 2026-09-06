//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpi3mr/mpi3mr_debug.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Broadcom MPI3 Storage Controllers
//
// Copyright (C) 2017-2023 Broadcom Inc.
// (mailto: mpi3mr-linuxdrv.pdl@broadcom.com)
//

// Macro flag: #define MPI3SAS_DEBUG_H_INCLUDED
//
// debug levels
//
pub const MPI3_DEBUG_EVENT: c_uint = 0x00000001;
pub const MPI3_DEBUG_EVENT_WORK_TASK: c_uint = 0x00000002;
pub const MPI3_DEBUG_INIT: c_uint = 0x00000004;
pub const MPI3_DEBUG_EXIT: c_uint = 0x00000008;
pub const MPI3_DEBUG_TM: c_uint = 0x00000010;
pub const MPI3_DEBUG_RESET: c_uint = 0x00000020;
pub const MPI3_DEBUG_SCSI_ERROR: c_uint = 0x00000040;
pub const MPI3_DEBUG_REPLY: c_uint = 0x00000080;
pub const MPI3_DEBUG_CFG_ERROR: c_uint = 0x00000100;
pub const MPI3_DEBUG_TRANSPORT_ERROR: c_uint = 0x00000200;
pub const MPI3_DEBUG_BSG_ERROR: c_uint = 0x00008000;
pub const MPI3_DEBUG_BSG_INFO: c_uint = 0x00010000;
pub const MPI3_DEBUG_SCSI_INFO: c_uint = 0x00020000;
pub const MPI3_DEBUG_CFG_INFO: c_uint = 0x00040000;
pub const MPI3_DEBUG_TRANSPORT_INFO: c_uint = 0x00080000;
pub const MPI3_DEBUG: c_uint = 0x01000000;
pub const MPI3_DEBUG_SG: c_uint = 0x02000000;
//
// debug macros
//

//
// dprint_dump - print contents of a memory buffer
// @req: Pointer to a memory buffer
// @sz: Memory buffer size
// @namestr: Name String to identify the buffer type
//
// dprint_dump_req - print message frame contents
// @req: pointer to message frame
// @sz: number of dwords
//
