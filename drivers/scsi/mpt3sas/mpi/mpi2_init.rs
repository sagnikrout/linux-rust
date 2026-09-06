//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpi/mpi2_init.h
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
// Name:  mpi2_init.h
// Title:  MPI SCSI initiator mode messages and structures
// Creation Date:  June 23, 2006
//
// mpi2_init.h Version:  02.00.21
//
// NOTE: Names (typedefs, defines, etc.) beginning with an MPI25 or Mpi25
// prefix are for use only on MPI v2.5 products, and must not be used
// with MPI v2.0 products. Unless otherwise noted, names beginning with
// MPI2 or Mpi2 are for use with both MPI v2.0 and MPI v2.5 products.
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 04-30-07  02.00.00  Corresponds to Fusion-MPT MPI Specification Rev A.
// 10-31-07  02.00.01  Fixed name for pMpi2SCSITaskManagementRequest_t.
// 12-18-07  02.00.02  Modified Task Management Target Reset Method defines.
// 02-29-08  02.00.03  Added Query Task Set and Query Unit Attention.
// 03-03-08  02.00.04  Fixed name of struct _MPI2_SCSI_TASK_MANAGE_REPLY.
// 05-21-08  02.00.05  Fixed typo in name of Mpi2SepRequest_t.
// 10-02-08  02.00.06  Removed Untagged and No Disconnect values from SCSI IO
// Control field Task Attribute flags.
// Moved LUN field defines to mpi2.h becasue they are
// common to many structures.
// 05-06-09  02.00.07  Changed task management type of Query Unit Attention to
// Query Asynchronous Event.
// Defined two new bits in the SlotStatus field of the SCSI
// Enclosure Processor Request and Reply.
// 10-28-09  02.00.08  Added defines for decoding the ResponseInfo bytes for
// both SCSI IO Error Reply and SCSI Task Management Reply.
// Added ResponseInfo field to MPI2_SCSI_TASK_MANAGE_REPLY.
// Added MPI2_SCSITASKMGMT_RSP_TM_OVERLAPPED_TAG define.
// 02-10-10  02.00.09  Removed unused structure that had "#if 0" around it.
// 05-12-10  02.00.10  Added optional vendor-unique region to SCSI IO Request.
// 11-10-10  02.00.11  Added MPI2_SCSIIO_NUM_SGLOFFSETS define.
// 11-18-11  02.00.12  Incorporating additions for MPI v2.5.
// 02-06-12  02.00.13  Added alternate defines for Task Priority / Command
// Priority to match SAM-4.
// Added EEDPErrorOffset to MPI2_SCSI_IO_REPLY.
// 07-10-12  02.00.14  Added MPI2_SCSIIO_CONTROL_SHIFT_DATADIRECTION.
// 04-09-13  02.00.15  Added SCSIStatusQualifier field to MPI2_SCSI_IO_REPLY,
// replacing the Reserved4 field.
// 11-18-14  02.00.16  Updated copyright information.
// 03-16-15  02.00.17  Updated for MPI v2.6.
// Added MPI26_SCSIIO_IOFLAGS_ESCAPE_PASSTHROUGH.
// Added MPI2_SEP_REQ_SLOTSTATUS_DEV_OFF and
// MPI2_SEP_REPLY_SLOTSTATUS_DEV_OFF.
// 08-26-15  02.00.18  Added SCSITASKMGMT_MSGFLAGS for Target Reset.
// 12-18-15  02.00.19  Added EEDPObservedValue added to SCSI IO Reply message.
// 01-04-16  02.00.20  Modified EEDP reported values in SCSI IO Reply message.
// 01-21-16  02.00.21  Modified MPI26_SCSITASKMGMT_MSGFLAGS_PCIE* defines to
// be unique within first 32 characters.
// --------------------------------------------------------------------------
//
// SCSI Initiator Messages
//
// SCSI IO messages and associated structures
//
// MPI v2.0 CDB field
// MPI v2.0 SCSI IO Request Message

// SCSI IO MsgFlags bits
// MsgFlags for SenseBufferAddressSpace

// SCSI IO SGLFlags bits
// base values for Data Location Address Space

// base values for Type

// shift values for each sub-field

// number of SGLOffset fields

// SCSI IO IoFlags bits
// Large CDB Address Space

// SCSI IO EEDPFlags bits

// SCSI IO LUN fields: use MPI2_LUN_ from mpi2.h
// SCSI IO Control bits

// alternate name for the previous field; called Command Priority in SAM-4

// MPI v2.5 CDB field
// MPI v2.5/2.6 SCSI IO Request Message

// use MPI2_SCSIIO_MSGFLAGS_ defines for the MsgFlags field
// Defines for the DMAFlags field
// Each setting affects 4 SGLS, from SGL0 to SGL3.
// D = Data
// C = Cache DIF
// I = Interleaved
// H = Host DIF
//

// number of SGLOffset fields

// defines for the IoFlags field

// MPI v2.5 defines for the EEDPFlags bits
// use MPI2_SCSIIO_EEDPFLAGS_ defines for the other EEDPFlags bits

// use MPI2_LUN_ defines from mpi2.h for the LUN field
// use MPI2_SCSIIO_CONTROL_ defines for the Control field
// NOTE: The SCSI IO Reply is nearly the same for MPI 2.0 and MPI 2.5, so
// MPI2_SCSI_IO_REPLY is used for both.
//
// SCSI IO Error Reply Message
// MPI 2.5+ only; Reserved in MPI 2.0
// SCSI IO Reply MsgFlags bits

// SCSI IO Reply SCSIStatus values (SAM-4 status codes)

// SCSI IO Reply SCSIState flags

// masks and shifts for the ResponseInfo field

//
// SCSI Task Management messages
//
// SCSI Task Management Request Message
// PTR_MPI2_SCSI_TASK_MANAGE_REQUEST,
// pMpi2SCSITaskManagementRequest_t;
// TaskType values

// obsolete TaskType name

// MsgFlags bits

// SCSI Task Management Reply Message
// PTR_MPI2_SCSI_TASK_MANAGE_REPLY,
// ResponseCode values

// masks and shifts for the ResponseInfo field

//
// SCSI Enclosure Processor messages
//
// SCSI Enclosure Processor Request Message
// Action defines

// Flags defines

// SlotStatus defines

// SCSI Enclosure Processor Reply Message
// SlotStatus defines

