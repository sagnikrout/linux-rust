//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_sas.h
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
// Copyright (c) 2004-2008 LSI Corporation.
//
// Name:  mpi_sas.h
// Title:  MPI Serial Attached SCSI structures and definitions
// Creation Date:  August 19, 2004
//
// mpi_sas.h Version:  01.05.05
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 08-19-04  01.05.01  Original release.
// 08-30-05  01.05.02  Added DeviceInfo bit for SEP.
// Added PrimFlags and Primitive field to SAS IO Unit
// Control request, and added a new operation code.
// 03-27-06  01.05.03  Added Force Full Discovery, Transmit Port Select Signal,
// and Remove Device operations to SAS IO Unit Control.
// Added DevHandle field to SAS IO Unit Control request and
// reply.
// 10-11-06  01.05.04  Fixed the name of a define for Operation field of SAS IO
// Unit Control request.
// 01-15-08  01.05.05  Added support for MPI_SAS_OP_SET_IOC_PARAMETER,
// including adding IOCParameter and IOCParameter value
// fields to SAS IO Unit Control Request.
// Added MPI_SAS_DEVICE_INFO_PRODUCT_SPECIFIC define.
// --------------------------------------------------------------------------
//
// Values for SASStatus.
//

//
// Values for the SAS DeviceInfo field used in SAS Device Status Change Event
// data and SAS IO Unit Configuration pages.
//

//
// S e r i a l    A t t a c h e d    S C S I     M e s s a g e s
//
// Serial Management Protocol Passthrough Request
//
// values for PassthroughFlags field

// values for ConnectionRate field

// Serial Management Protocol Passthrough Reply

//
// SATA Passthrough Request
//
// values for PassthroughFlags field

// values for ConnectionRate field

// SATA Passthrough Reply
//
// SAS IO Unit Control Request
//
// values for the Operation field

// values for the PrimFlags field

// SAS IO Unit Control Reply
