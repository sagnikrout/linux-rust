//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpi/mpi2_image.h
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
// Copyright 2016-2020 Broadcom Limited. All rights reserved.
//
// Name: mpi2_image.h
// Description: Contains definitions for firmware and other component images
// Creation Date: 04/02/2018
// Version: 02.06.04
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 08-01-18  02.06.00  Initial version for MPI 2.6.5.
// 08-14-18  02.06.01  Corrected define for MPI26_IMAGE_HEADER_SIGNATURE0_MPI26
// 08-28-18  02.06.02  Added MPI2_EXT_IMAGE_TYPE_RDE
// 09-07-18  02.06.03  Added MPI26_EVENT_PCIE_TOPO_PI_16_LANES
// 12-17-18  02.06.04  Addd MPI2_EXT_IMAGE_TYPE_PBLP
// Shorten some defines to be compatible with DOS
// 06-24-19  02.06.05  Whitespace adjustments to help with identifier
// checking tool.
// 10-02-19  02.06.06  Added MPI26_IMAGE_HEADER_SIG1_COREDUMP
// Added MPI2_FLASH_REGION_COREDUMP
//
// FW Image Header
// Signature field

// Signature0 field

// Last byte is defined by architecture

// legacy (0x5AEAA55A)

// Signature1 field

// Signature2 field

// defines for using the ProductID field

// SAS ProductID Family bits

// use MPI2_IOCFACTS_PROTOCOL_ defines for ProtocolFlags field
// use MPI2_IOCFACTS_CAPABILITY_ defines for IOCCapabilities field

// This image has a auto-discovery version of SPI

//
// Component Image Format and related defines
//
// Maximum number of Hash Exclusion entries in a Component Image Header

// Hash Exclusion Format
// PTR_MPI26_HASH_EXCLUSION_FORMAT,
// pMpi26HashExclusionFormat_t;
// FW Image Header
// PTR_MPI26_COMPONENT_IMAGE_HEADER,
// pMpi26ComponentImageHeader_t;
// Definitions for Signature0 field

// Definitions for Signature1 field

// little-endian "DUMP"

// Definitions for Signature2 field

// Offsets for Image Header Fields

// Extended Image Header
// useful offsets

// defines for the ImageType field

// FLASH Layout Extended Image Data
//
// Host code (drivers, BIOS, utilities, etc.) should check NumberOfLayouts and
// RegionsPerLayout at runtime before using Layout[] and Region[].
//
// defines for the RegionType field

// ImageRevision

// Supported Devices Extended Image Data
//
// Host code (drivers, BIOS, utilities, etc.) should check NumberOfDevices at
// runtime before using SupportedDevice[].
//
// ImageRevision

// Init Extended Image Data
// defines for the BootFlags field

// defines for the ImageSize field

// defines for the Signature0 field

// defines for the Signature1 field

// defines for the Signature2 field

// Signature fields as individual bytes

// defines for the ResetVector field

// Encrypted Hash Extended Image Data
// values for HashImageType

// values for HashAlgorithm

// values for EncryptionAlgorithm

