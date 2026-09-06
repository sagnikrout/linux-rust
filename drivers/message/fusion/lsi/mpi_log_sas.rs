//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_log_sas.h
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
// Copyright (c) 2000-2008 LSI Corporation.  All rights reserved.
//
// Description
// ------------
// This include file contains SAS firmware interface IOC Log Info codes
//
// -------------------------------------------------------------------------
//

// Macro flag: #define IOPI_IOCLOGINFO_H_INCLUDED
pub const SAS_LOGINFO_NEXUS_LOSS: c_uint = 0x31170000;
pub const SAS_LOGINFO_MASK: c_uint = 0xFFFF0000;
//
// IOC LOGINFO defines, 0x00000000 - 0x0FFFFFFF
// Format:
// Bits 31-28: MPI_IOCLOGINFO_TYPE_SAS (3)
// Bits 27-24: IOC_LOGINFO_ORIGINATOR: 0=IOP, 1=PL, 2=IR
// Bits 23-16: LOGINFO_CODE
// Bits 15-0:  LOGINFO_CODE Specific
//
// IOC_LOGINFO_ORIGINATOR defines
//

//
// LOGINFO_CODE defines
//

//
// IOP LOGINFO_CODE defines, valid if IOC_LOGINFO_ORIGINATOR = IOP
//

//
// PL LOGINFO_CODE defines, valid if IOC_LOGINFO_ORIGINATOR = PL
//

// Bits 0-3 encode Transport Status Register (offset 0x08)
// Bit 0 is Status Bit 0: FrameXferErr
// Bit 1 & 2 are Status Bits 16 and 17: FrameXmitErrStatus
// Bit 3 is Status Bit 18 WriteDataLenghtGTDataLengthErr

// not currently used in mainline

//
// IR LOGINFO_CODE defines, valid if IOC_LOGINFO_ORIGINATOR = IR
//

// Amount of information passed down for Create Volume is too large

// Creation of duplicate volume attempted (Bus/Target ID checked)

// Creation failed due to maximum number of supported volumes exceeded

// Creation failed due to DMA error in trying to read from host

// Creation failed due to invalid volume type passed down

// Creation failed due to error reading MFG Page 4

// Creation failed when trying to create internal structures

// Activation failed due to trying to activate an already active volume

// Activation failed due to trying to active unsupported volume type

// Activation failed due to trying to active too many volumes

// Activation failed due to Volume ID in use already

// Activation failed call to activateVolume returned failure

// Activation failed trying to import the volume

// Activation failed trying to import the volume

// Phys Disk failed, too many phys disks

// Amount of information passed down for Create Pnysdisk is too large

// Creation failed due to DMA error in trying to read from host

// Creation failed due to invalid Bus TargetID passed down

// Creation failed due to error in creating RAID Phys Disk Config Page

// Compatibility Error : IR Disabled

// Compatibility Error : Inquiry Command failed

// Compatibility Error : Device not direct access device

// Compatibility Error : Removable device found

// Compatibility Error : Device SCSI Version not 2 or higher

// Compatibility Error : SATA device, 48 BIT LBA not supported

// Compatibility Error : Device does not have 512 byte block sizes

// Compatibility Error : Volume Type check failed

// Compatibility Error : Volume Type is unsupported by FW

// Compatibility Error : Disk drive too small for use in volume

// Compatibility Error : Phys disk for Create Volume not found

// Compatibility Error : membership count error, too many or too few disks for volume type

// Compatibility Error : Disk stripe sizes must be 64KB

// Compatibility Error : IME size limited to < 2TB

// Device Firmware Update: DFU can only be started once

// Device Firmware Update: Volume must be Optimal/Active/non-Quiesced

// Device Firmware Update: DFU Timeout cannot be zero

// Device Firmware Update: CREATE TIMER FAILED

// Device Firmware Update: Failed to read SAS_IO_UNIT_PG_1

// Device Firmware Update: Invalid SAS_IO_UNIT_PG_1 value(s)

// Device Firmware Update: Unable to allocate memory for page

//
// Defines for convenience
//

