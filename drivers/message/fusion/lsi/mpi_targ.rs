//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_targ.h
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
// Copyright (c) 2000-2008 LSI Corporation.
//
// Name:  mpi_targ.h
// Title:  MPI Target mode messages and structures
// Creation Date:  June 22, 2000
//
// mpi_targ.h Version:  01.05.06
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 05-08-00  00.10.01  Original release for 0.10 spec dated 4/26/2000.
// 06-06-00  01.00.01  Update version number for 1.0 release.
// 06-22-00  01.00.02  Added _MSG_TARGET_CMD_BUFFER_POST_REPLY structure.
// Corrected DECSRIPTOR typo to DESCRIPTOR.
// 11-02-00  01.01.01  Original release for post 1.0 work
// Modified target mode to use IoIndex instead of
// HostIndex and IocIndex. Added Alias.
// 01-09-01  01.01.02  Added defines for TARGET_ASSIST_FLAGS_REPOST_CMD_BUFFER
// and TARGET_STATUS_SEND_FLAGS_REPOST_CMD_BUFFER.
// 02-20-01  01.01.03  Started using MPI_POINTER.
// Added structures for MPI_TARGET_SCSI_SPI_CMD_BUFFER and
// MPI_TARGET_FCP_CMD_BUFFER.
// 03-27-01  01.01.04  Added structure offset comments.
// 08-08-01  01.02.01  Original release for v1.2 work.
// 09-28-01  01.02.02  Added structure for MPI_TARGET_SCSI_SPI_STATUS_IU.
// Added PriorityReason field to some replies and
// defined more PriorityReason codes.
// Added some defines for to support previous version
// of MPI.
// 10-04-01  01.02.03  Added PriorityReason to MSG_TARGET_ERROR_REPLY.
// 11-01-01  01.02.04  Added define for TARGET_STATUS_SEND_FLAGS_HIGH_PRIORITY.
// 03-14-02  01.02.05  Modified MPI_TARGET_FCP_RSP_BUFFER to get the proper
// byte ordering.
// 05-31-02  01.02.06  Modified TARGET_MODE_REPLY_ALIAS_MASK to only include
// one bit.
// Added AliasIndex field to MPI_TARGET_FCP_CMD_BUFFER.
// 09-16-02  01.02.07  Added flags for confirmed completion.
// Added PRIORITY_REASON_TARGET_BUSY.
// 11-15-02  01.02.08  Added AliasID field to MPI_TARGET_SCSI_SPI_CMD_BUFFER.
// 04-01-03  01.02.09  Added OptionalOxid field to MPI_TARGET_FCP_CMD_BUFFER.
// 05-11-04  01.03.01  Original release for MPI v1.3.
// 08-19-04  01.05.01  Added new request message structures for
// MSG_TARGET_CMD_BUF_POST_BASE_REQUEST,
// MSG_TARGET_CMD_BUF_POST_LIST_REQUEST, and
// MSG_TARGET_ASSIST_EXT_REQUEST.
// Added new structures for SAS SSP Command buffer, SSP
// Task buffer, and SSP Status IU.
// 10-05-04  01.05.02  MSG_TARGET_CMD_BUFFER_POST_BASE_LIST_REPLY added.
// 02-22-05  01.05.03  Changed a comment.
// 03-11-05  01.05.04  Removed TargetAssistExtended Request.
// 06-24-05  01.05.05  Added TargetAssistExtended structures and defines.
// 03-27-06  01.05.06  Added a comment.
// --------------------------------------------------------------------------
//
// S C S I    T a r g e t    M e s s a g e s
//
// Target Command Buffer Post Request
//

// the following structure is obsolete as of MPI v1.2

//
// Target Command Buffer Post Base Request
//

//
// Target Command Buffer Post List Request
//
// Command Buffer Formats (with 16 byte CDB)
//
// SPI L_Q information unit
// SPI command information unit
// Alias ID
// COMMAND information unit starts here
// Additional CDB bytes extend past the CDB field
// TASK information unit starts here
//
// Target Assist Request
//

// Standard Target Mode Reply message
//
// Target Assist Extended Request
//
// see the defines after MSG_TARGET_ASSIST_REQUEST for TargetAssistFlags
// defines for the MsgFlags field

// defines for the EEDPFlags field

//
// Target Status Send Request
//

//
// NOTE: FCP_RSP data is big-endian. When used on a little-endian system, this
// structure properly orders the bytes.
//
// NOTE: The SPI status IU is big-endian. When used on a little-endian system,
// this structure properly orders the bytes.
//
// NOTE: The SSP status IU is big-endian. When used on a little-endian system,
// this structure properly orders the bytes.
//
// start of RESPONSE information unit
//
// Target Mode Abort Request
//

// Target Mode Abort Reply
//
// Target Mode Context Reply
//

// the following obsolete values are for MPI v1.0 support

