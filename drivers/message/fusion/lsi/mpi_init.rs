//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_init.h
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
// Name:  mpi_init.h
// Title:  MPI initiator mode messages and structures
// Creation Date:  June 8, 2000
//
// mpi_init.h Version:  01.05.09
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 05-08-00  00.10.01  Original release for 0.10 spec dated 4/26/2000.
// 05-24-00  00.10.02  Added SenseBufferLength to _MSG_SCSI_IO_REPLY.
// 06-06-00  01.00.01  Update version number for 1.0 release.
// 06-08-00  01.00.02  Added MPI_SCSI_RSP_INFO_ definitions.
// 11-02-00  01.01.01  Original release for post 1.0 work.
// 12-04-00  01.01.02  Added MPI_SCSIIO_CONTROL_NO_DISCONNECT.
// 02-20-01  01.01.03  Started using MPI_POINTER.
// 03-27-01  01.01.04  Added structure offset comments.
// 04-10-01  01.01.05  Added new MsgFlag for MSG_SCSI_TASK_MGMT.
// 08-08-01  01.02.01  Original release for v1.2 work.
// 08-29-01  01.02.02  Added MPI_SCSITASKMGMT_TASKTYPE_LOGICAL_UNIT_RESET.
// Added MPI_SCSI_STATE_QUEUE_TAG_REJECTED for
// MSG_SCSI_IO_REPLY.
// 09-28-01  01.02.03  Added structures and defines for SCSI Enclosure
// Processor messages.
// 10-04-01  01.02.04  Added defines for SEP request Action field.
// 05-31-02  01.02.05  Added MPI_SCSIIO_MSGFLGS_CMD_DETERMINES_DATA_DIR define
// for SCSI IO requests.
// 11-15-02  01.02.06  Added special extended SCSI Status defines for FCP.
// 06-26-03  01.02.07  Added MPI_SCSI_STATUS_FCPEXT_UNASSIGNED define.
// 05-11-04  01.03.01  Original release for MPI v1.3.
// 08-19-04  01.05.01  Added MsgFlags defines for EEDP to SCSI IO request.
// Added new word to MSG_SCSI_IO_REPLY to add TaskTag field
// and a reserved U16.
// Added new MSG_SCSI_IO32_REQUEST structure.
// Added a TaskType of Clear Task Set to SCSI
// Task Management request.
// 12-07-04  01.05.02  Added support for Task Management Query Task.
// 01-15-05  01.05.03  Modified SCSI Enclosure Processor Request to support
// WWID addressing.
// 03-11-05  01.05.04  Removed EEDP flags from SCSI IO Request.
// Removed SCSI IO 32 Request.
// Modified SCSI Enclosure Processor Request and Reply to
// support Enclosure/Slot addressing rather than WWID
// addressing.
// 06-24-05  01.05.05  Added SCSI IO 32 structures and defines.
// Added four new defines for SEP SlotStatus.
// 08-03-05  01.05.06  Fixed some MPI_SCSIIO32_MSGFLGS_ defines to make them
// unique in the first 32 characters.
// 03-27-06  01.05.07  Added Task Management type of Clear ACA.
// 10-11-06  01.05.08  Shortened define for Task Management type of Clear ACA.
// 02-28-07  01.05.09  Defined two new MsgFlags bits for SCSI Task Management
// Request: Do Not Send Task IU and Soft Reset Option.
// --------------------------------------------------------------------------
//
// S C S I    I n i t i a t o r    M e s s a g e s
//
// SCSI IO messages and associated structures
//
// SCSI IO MsgFlags bits

// SCSI IO LUN fields

// SCSI IO Control bits

// SCSI IO reply structure
// SCSI IO Reply SCSIStatus values (SAM-2 status codes)

// SCSI IO Reply SCSIState values

// SCSI IO Reply ResponseInfo values
// (FCP-1 RSP_CODE values and SPI-3 Packetized Failure codes)

//
// SCSI IO 32 messages and associated structures
//
// SCSI IO 32 MsgFlags bits

// SCSI IO 32 Flags bits

// SCSI IO 32 LUN fields

// SCSI IO 32 Control bits

// SCSI IO 32 EEDPFlags

// SCSIIO32 IO reply structure
//
// SCSI Task Management messages
//
// TaskType values

// MsgFlags bits

// SCSI Task Management Reply
// ResponseCode values

//
// SCSI Enclosure Processor messages
//
// Action defines

// Flags defines

// SlotStatus bits for MSG_SEP_REQUEST

// SlotStatus bits for MSG_SEP_REPLY

