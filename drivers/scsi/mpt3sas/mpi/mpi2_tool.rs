//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpi/mpi2_tool.h
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
// Copyright 2000-2020 Broadcom Inc. All rights reserved.
//
// Name:  mpi2_tool.h
// Title:  MPI diagnostic tool structures and definitions
// Creation Date:  March 26, 2007
//
// mpi2_tool.h Version:  02.00.16
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 04-30-07  02.00.00  Corresponds to Fusion-MPT MPI Specification Rev A.
// 12-18-07  02.00.01  Added Diagnostic Buffer Post and Diagnostic Release
// structures and defines.
// 02-29-08  02.00.02  Modified various names to make them 32-character unique.
// 05-06-09  02.00.03  Added ISTWI Read Write Tool and Diagnostic CLI Tool.
// 07-30-09  02.00.04  Added ExtendedType field to DiagnosticBufferPost request
// and reply messages.
// Added MPI2_DIAG_BUF_TYPE_EXTENDED.
// Incremented MPI2_DIAG_BUF_TYPE_COUNT.
// 05-12-10  02.00.05  Added Diagnostic Data Upload tool.
// 08-11-10  02.00.06  Added defines that were missing for Diagnostic Buffer
// Post Request.
// 05-25-11  02.00.07  Added Flags field and related defines to
// MPI2_TOOLBOX_ISTWI_READ_WRITE_REQUEST.
// 11-18-11  02.00.08  Incorporating additions for MPI v2.5.
// 07-10-12  02.00.09  Add MPI v2.5 Toolbox Diagnostic CLI Tool Request
// message.
// 07-26-12  02.00.10  Modified MPI2_TOOLBOX_DIAGNOSTIC_CLI_REQUEST so that
// it uses MPI Chain SGE as well as MPI Simple SGE.
// 08-19-13  02.00.11  Added MPI2_TOOLBOX_TEXT_DISPLAY_TOOL and related info.
// 01-08-14  02.00.12  Added MPI2_TOOLBOX_CLEAN_BIT26_PRODUCT_SPECIFIC.
// 11-18-14  02.00.13  Updated copyright information.
// 08-25-16  02.00.14  Added new values for the Flags field of Toolbox Clean
// Tool Request Message.
// 07-22-18  02.00.15  Added defines for new TOOLBOX_PCIE_LANE_MARGINING tool.
// Added option for DeviceInfo field in ISTWI tool.
// 12-17-18  02.00.16  Shorten some defines to be compatible with DOS.
// --------------------------------------------------------------------------
//
// Toolbox Messages
//
// defines for the Tools

//
// Toolbox reply
//
// Toolbox Clean Tool request
//
// values for the Flags field

//
// Toolbox Memory Move request
//
// Toolbox Diagnostic Data Upload request
//
// PTR_MPI2_TOOLBOX_DIAG_DATA_UPLOAD_REQUEST,
// pMpi2ToolboxDiagDataUploadRequest_t;
// use MPI2_SGLFLAGS_ defines from mpi2.h for the SGLFlags field
//
// Toolbox ISTWI Read Write Tool
//
// Toolbox ISTWI Read Write Tool request message
// PTR_MPI2_TOOLBOX_ISTWI_READ_WRITE_REQUEST,
// pMpi2ToolboxIstwiReadWriteRequest_t;
// values for the Action field

// use MPI2_SGLFLAGS_ defines from mpi2.h for the SGLFlags field
// values for the Flags field

// MPI26 TOOLBOX Request MsgFlags defines

// Request uses Man Page 43 device index addressing

// Request uses Man Page 43 device info struct addressing

// Toolbox ISTWI Read Write Tool reply message
//
// Toolbox Beacon Tool request
//
// values for the Flags field

//
// Toolbox Diagnostic CLI Tool
//

// MPI v2.0 Toolbox Diagnostic CLI Tool request message
// PTR_MPI2_TOOLBOX_DIAGNOSTIC_CLI_REQUEST,
// pMpi2ToolboxDiagnosticCliRequest_t;
// use MPI2_SGLFLAGS_ defines from mpi2.h for the SGLFlags field
// MPI v2.5 Toolbox Diagnostic CLI Tool request message
// PTR_MPI25_TOOLBOX_DIAGNOSTIC_CLI_REQUEST,
// pMpi25ToolboxDiagnosticCliRequest_t;
// Toolbox Diagnostic CLI Tool reply message
// PTR_MPI2_TOOLBOX_DIAG_CLI_REPLY,
// pMpi2ToolboxDiagnosticCliReply_t;
//
// Toolbox Console Text Display Tool
//
// Toolbox Console Text Display Tool request message
// PTR_MPI2_TOOLBOX_TEXT_DISPLAY_REQUEST,
// pMpi2ToolboxTextDisplayRequest_t;
// defines for the Console field

// defines for the Flags field

//
// Toolbox Backend Lane Margining Tool
//
// Toolbox Backend Lane Margining Tool request message
// PTR_MPI2_TOOLBOX_LANE_MARGINING_REQUEST,
// pMpi2ToolboxLaneMarginingRequest_t;
// defines for the Command field

// Toolbox Backend Lane Margining Tool reply message
// PTR_MPI26_TOOLBOX_LANE_MARGINING_REPLY,
// pMpi26ToolboxLaneMarginingReply_t;
//
// Diagnostic Buffer Messages
//
// Diagnostic Buffer Post request
//
// values for the ExtendedType field

// values for the BufferType field

// count of the number of buffer types

// values for the Flags field

//
// Diagnostic Buffer Post reply
//
// Diagnostic Release request
//
// Diagnostic Buffer Post reply
//
