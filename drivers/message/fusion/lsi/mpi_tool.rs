//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_tool.h
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
// Copyright (c) 2001-2008 LSI Corporation.
//
// Name:  mpi_tool.h
// Title:  MPI Toolbox structures and definitions
// Creation Date:  July 30, 2001
//
// mpi_tool.h Version:  01.05.03
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 08-08-01  01.02.01  Original release.
// 08-29-01  01.02.02  Added DIAG_DATA_UPLOAD_HEADER and related defines.
// 01-16-04  01.02.03  Added defines and structures for new tools
// .                     MPI_TOOLBOX_ISTWI_READ_WRITE_TOOL and
// MPI_TOOLBOX_FC_MANAGEMENT_TOOL.
// 04-29-04  01.02.04  Added message structures for Diagnostic Buffer Post and
// Diagnostic Release requests and replies.
// 05-11-04  01.03.01  Original release for MPI v1.3.
// 08-19-04  01.05.01  Original release for MPI v1.5.
// 10-06-04  01.05.02  Added define for MPI_DIAG_BUF_TYPE_COUNT.
// 02-09-05  01.05.03  Added frame size option to FC management tool.
// Added Beacon tool to the Toolbox.
// --------------------------------------------------------------------------
//

//
// Toolbox reply
//
// Toolbox Clean Tool request
//

//
// Toolbox Memory Move request
//
// Toolbox Diagnostic Data Upload request
//

//
// Toolbox ISTWI Read Write request
//

//
// Toolbox FC Management request
//
// ActionInfo for Bus and TargetId
// ActionInfo for port identifier
// ActionInfo for set max frame size
// union of ActionInfo
// defines for the Action field

//
// Toolbox Beacon Tool request
//

//
// Diagnostic Buffer Post request
//

// count of the number of buffer types

// Diagnostic Buffer Post reply
//
// Diagnostic Release request
//
// Diagnostic Release reply
