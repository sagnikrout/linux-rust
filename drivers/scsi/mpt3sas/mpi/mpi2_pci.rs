//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpi/mpi2_pci.h
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


//
// Copyright 2000-2020 Broadcom Inc. All rights reserved.
//
// Name:  mpi2_pci.h
// Title:  MPI PCIe Attached Devices structures and definitions.
// Creation Date:  October 9, 2012
//
// mpi2_pci.h Version:  02.00.04
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
// 03-16-15  02.00.00  Initial version.
// 02-17-16  02.00.01  Removed AHCI support.
// Removed SOP support.
// 07-01-16  02.00.02  Added MPI26_NVME_FLAGS_FORCE_ADMIN_ERR_RESP to
// NVME Encapsulated Request.
// 07-22-18  02.00.03  Updted flags field for NVME Encapsulated req
// 12-17-18  02.00.04  Added MPI26_PCIE_DEVINFO_SCSI
// Shortten some defines to be compatible with DOS
// --------------------------------------------------------------------------
//
// Values for the PCIe DeviceInfo field used in PCIe Device Status Change Event
// data and PCIe Configuration pages.
//

//
// NVMe Encapsulated message
//
// NVME Encapsulated Request Message
// defines for the Flags field

// Submission Queue Type

// Error Response Address Space

// Data Direction

// NVMe Encapuslated Reply Message
// PTR_MPI26_NVME_ENCAPSULATED_ERROR_REPLY,
// pMpi26NVMeEncapsulatedErrorReply_t;
