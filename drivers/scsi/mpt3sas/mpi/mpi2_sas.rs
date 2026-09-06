//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpi/mpi2_sas.h
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
// Name:  mpi2_sas.h
// Title:  MPI Serial Attached SCSI structures and definitions
// Creation Date:  February 9, 2007
//
// mpi2_sas.h Version:  02.00.10
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
// 06-26-07  02.00.01  Added Clear All Persistent Operation to SAS IO Unit
// Control Request.
// 10-02-08  02.00.02  Added Set IOC Parameter Operation to SAS IO Unit Control
// Request.
// 10-28-09  02.00.03  Changed the type of SGL in MPI2_SATA_PASSTHROUGH_REQUEST
// to MPI2_SGE_IO_UNION since it supports chained SGLs.
// 05-12-10  02.00.04  Modified some comments.
// 08-11-10  02.00.05  Added NCQ operations to SAS IO Unit Control.
// 11-18-11  02.00.06  Incorporating additions for MPI v2.5.
// 07-10-12  02.00.07  Added MPI2_SATA_PT_SGE_UNION for use in the SATA
// Passthrough Request message.
// 08-19-13  02.00.08  Made MPI2_SAS_OP_TRANSMIT_PORT_SELECT_SIGNAL obsolete
// for anything newer than MPI v2.0.
// 11-18-14  02.00.09  Updated copyright information.
// 03-16-15  02.00.10  Updated for MPI v2.6.
// Added MPI2_SATA_PT_REQ_PT_FLAGS_FPDMA.
// --------------------------------------------------------------------------
//
// Values for SASStatus.
//

//
// Values for the SAS DeviceInfo field used in SAS Device Status Change Event
// data and SAS Configuration pages.
//

//
// SAS Messages
//
// SMP Passthrough messages
//
// SMP Passthrough Request Message
// values for PassthroughFlags field

// MPI v2.0: use MPI2_SGLFLAGS_ defines from mpi2.h for the SGLFlags field
// SMP Passthrough Reply Message
// values for PassthroughFlags field

// values for SASStatus field are at the top of this file
//
// SATA Passthrough messages
//
// SATA Passthrough Request Message
// pMpi2SataPassthroughRequest_t;
// values for PassthroughFlags field

// MPI v2.0: use MPI2_SGLFLAGS_ defines from mpi2.h for the SGLFlags field
// SATA Passthrough Reply Message
// values for SASStatus field are at the top of this file
//
// SAS IO Unit Control messages
// (MPI v2.5 and earlier only.
// Replaced by IO Unit Control messages in MPI v2.6 and later.)
//
// SAS IO Unit Control Request Message
// PTR_MPI2_SAS_IOUNIT_CONTROL_REQUEST,
// pMpi2SasIoUnitControlRequest_t;
// values for the Operation field

// values for the PrimFlags field

// values for the LookupMethod field

// SAS IO Unit Control Reply Message
// PTR_MPI2_SAS_IOUNIT_CONTROL_REPLY,
