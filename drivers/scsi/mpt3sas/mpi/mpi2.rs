//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpi/mpi2.h
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
// Name:  mpi2.h
// Title:  MPI Message independent structures and definitions
// including System Interface Register Set and
// scatter/gather formats.
// Creation Date:  June 21, 2006
//
// mpi2.h Version:  02.00.54
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
// 06-04-07  02.00.01  Bumped MPI2_HEADER_VERSION_UNIT.
// 06-26-07  02.00.02  Bumped MPI2_HEADER_VERSION_UNIT.
// 08-31-07  02.00.03  Bumped MPI2_HEADER_VERSION_UNIT.
// Moved ReplyPostHostIndex register to offset 0x6C of the
// MPI2_SYSTEM_INTERFACE_REGS and modified the define for
// MPI2_REPLY_POST_HOST_INDEX_OFFSET.
// Added union of request descriptors.
// Added union of reply descriptors.
// 10-31-07  02.00.04  Bumped MPI2_HEADER_VERSION_UNIT.
// Added define for MPI2_VERSION_02_00.
// Fixed the size of the FunctionDependent5 field in the
// MPI2_DEFAULT_REPLY structure.
// 12-18-07  02.00.05  Bumped MPI2_HEADER_VERSION_UNIT.
// Removed the MPI-defined Fault Codes and extended the
// product specific codes up to 0xEFFF.
// Added a sixth key value for the WriteSequence register
// and changed the flush value to 0x0.
// Added message function codes for Diagnostic Buffer Post
// and Diagnsotic Release.
// New IOCStatus define: MPI2_IOCSTATUS_DIAGNOSTIC_RELEASED
// Moved MPI2_VERSION_UNION from mpi2_ioc.h.
// 02-29-08  02.00.06  Bumped MPI2_HEADER_VERSION_UNIT.
// 03-03-08  02.00.07  Bumped MPI2_HEADER_VERSION_UNIT.
// 05-21-08  02.00.08  Bumped MPI2_HEADER_VERSION_UNIT.
// Added #defines for marking a reply descriptor as unused.
// 06-27-08  02.00.09  Bumped MPI2_HEADER_VERSION_UNIT.
// 10-02-08  02.00.10  Bumped MPI2_HEADER_VERSION_UNIT.
// Moved LUN field defines from mpi2_init.h.
// 01-19-09  02.00.11  Bumped MPI2_HEADER_VERSION_UNIT.
// 05-06-09  02.00.12  Bumped MPI2_HEADER_VERSION_UNIT.
// In all request and reply descriptors, replaced VF_ID
// field with MSIxIndex field.
// Removed DevHandle field from
// MPI2_SCSI_IO_SUCCESS_REPLY_DESCRIPTOR and made those
// bytes reserved.
// Added RAID Accelerator functionality.
// 07-30-09  02.00.13  Bumped MPI2_HEADER_VERSION_UNIT.
// 10-28-09  02.00.14  Bumped MPI2_HEADER_VERSION_UNIT.
// Added MSI-x index mask and shift for Reply Post Host
// Index register.
// Added function code for Host Based Discovery Action.
// 02-10-10  02.00.15  Bumped MPI2_HEADER_VERSION_UNIT.
// Added define for MPI2_FUNCTION_PWR_MGMT_CONTROL.
// Added defines for product-specific range of message
// function codes, 0xF0 to 0xFF.
// 05-12-10  02.00.16  Bumped MPI2_HEADER_VERSION_UNIT.
// Added alternative defines for the SGE Direction bit.
// 08-11-10  02.00.17  Bumped MPI2_HEADER_VERSION_UNIT.
// 11-10-10  02.00.18  Bumped MPI2_HEADER_VERSION_UNIT.
// Added MPI2_IEEE_SGE_FLAGS_SYSTEMPLBCPI_ADDR define.
// 02-23-11  02.00.19  Bumped MPI2_HEADER_VERSION_UNIT.
// Added MPI2_FUNCTION_SEND_HOST_MESSAGE.
// 03-09-11  02.00.20  Bumped MPI2_HEADER_VERSION_UNIT.
// 05-25-11  02.00.21  Bumped MPI2_HEADER_VERSION_UNIT.
// 08-24-11  02.00.22  Bumped MPI2_HEADER_VERSION_UNIT.
// 11-18-11  02.00.23  Bumped MPI2_HEADER_VERSION_UNIT.
// Incorporating additions for MPI v2.5.
// 02-06-12  02.00.24  Bumped MPI2_HEADER_VERSION_UNIT.
// 03-29-12  02.00.25  Bumped MPI2_HEADER_VERSION_UNIT.
// Added Hard Reset delay timings.
// 07-10-12  02.00.26  Bumped MPI2_HEADER_VERSION_UNIT.
// 07-26-12  02.00.27  Bumped MPI2_HEADER_VERSION_UNIT.
// 11-27-12  02.00.28  Bumped MPI2_HEADER_VERSION_UNIT.
// 12-20-12  02.00.29  Bumped MPI2_HEADER_VERSION_UNIT.
// Added MPI25_SUP_REPLY_POST_HOST_INDEX_OFFSET.
// 04-09-13  02.00.30  Bumped MPI2_HEADER_VERSION_UNIT.
// 04-17-13  02.00.31  Bumped MPI2_HEADER_VERSION_UNIT.
// 08-19-13  02.00.32  Bumped MPI2_HEADER_VERSION_UNIT.
// 12-05-13  02.00.33  Bumped MPI2_HEADER_VERSION_UNIT.
// 01-08-14  02.00.34  Bumped MPI2_HEADER_VERSION_UNIT
// 06-13-14  02.00.35  Bumped MPI2_HEADER_VERSION_UNIT.
// 11-18-14  02.00.36  Updated copyright information.
// Bumped MPI2_HEADER_VERSION_UNIT.
// 03-16-15  02.00.37  Bumped MPI2_HEADER_VERSION_UNIT.
// Added Scratchpad registers to
// MPI2_SYSTEM_INTERFACE_REGS.
// Added MPI2_DIAG_SBR_RELOAD.
// 03-19-15  02.00.38  Bumped MPI2_HEADER_VERSION_UNIT.
// 05-25-15  02.00.39  Bumped MPI2_HEADER_VERSION_UNIT.
// 08-25-15  02.00.40  Bumped MPI2_HEADER_VERSION_UNIT.
// 12-15-15  02.00.41  Bumped MPI_HEADER_VERSION_UNIT
// 01-01-16  02.00.42  Bumped MPI_HEADER_VERSION_UNIT
// 04-05-16  02.00.43  Modified  MPI26_DIAG_BOOT_DEVICE_SELECT defines
// to be unique within first 32 characters.
// Removed AHCI support.
// Removed SOP support.
// Bumped MPI2_HEADER_VERSION_UNIT.
// 04-10-16  02.00.44  Bumped MPI2_HEADER_VERSION_UNIT.
// 07-06-16  02.00.45  Bumped MPI2_HEADER_VERSION_UNIT.
// 09-02-16  02.00.46  Bumped MPI2_HEADER_VERSION_UNIT.
// 11-23-16  02.00.47  Bumped MPI2_HEADER_VERSION_UNIT.
// 02-03-17  02.00.48  Bumped MPI2_HEADER_VERSION_UNIT.
// 06-13-17  02.00.49  Bumped MPI2_HEADER_VERSION_UNIT.
// 09-29-17  02.00.50  Bumped MPI2_HEADER_VERSION_UNIT.
// 07-22-18  02.00.51  Added SECURE_BOOT define.
// Bumped MPI2_HEADER_VERSION_UNIT
// 08-15-18  02.00.52  Bumped MPI2_HEADER_VERSION_UNIT.
// 08-28-18  02.00.53  Bumped MPI2_HEADER_VERSION_UNIT.
// Added MPI2_IOCSTATUS_FAILURE
// 12-17-18  02.00.54  Bumped MPI2_HEADER_VERSION_UNIT
// 06-24-19  02.00.55  Bumped MPI2_HEADER_VERSION_UNIT
// 08-01-19  02.00.56  Bumped MPI2_HEADER_VERSION_UNIT
// 10-02-19  02.00.57  Bumped MPI2_HEADER_VERSION_UNIT
// 07-20-20  02.00.58  Bumped MPI2_HEADER_VERSION_UNIT
// 03-30-21  02.00.59  Bumped MPI2_HEADER_VERSION_UNIT
// 06-03-22  02.00.60  Bumped MPI2_HEADER_VERSION_UNIT
// 09-20-23  02.00.61  Bumped MPI2_HEADER_VERSION_UNIT
// 09-13-24  02.00.62  Bumped MPI2_HEADER_VERSION_UNIT
// Added MPI2_FUNCTION_MCTP_PASSTHROUGH
// --------------------------------------------------------------------------
//
// MPI Version Definitions
//

// major version for all MPI v2.x

// minor version for MPI v2.0 compatible products

// minor version for MPI v2.5 compatible products

// minor version for MPI v2.6 compatible products

// Unit and Dev versioning for this MPI header set

//
// IOC State Definitions
//

// Fault state range for prodcut specific codes

//
// System Interface Register Definitions
//
// PTR_MPI2_SYSTEM_INTERFACE_REGS,
// pMpi2SystemInterfaceRegs_t;
//
// Defines for working with the Doorbell register.
//

// IOC --> System values

// System --> IOC values

//
// Defines for the WriteSequence register
//

//
// Defines for the HostDiagnostic register
//

// Defines for V7A/V7R HostDiagnostic Register

//
// Offsets for DiagRWData and address
//

//
// Defines for the HostInterruptStatus register
//

//
// Defines for the HostInterruptMask register
//

//
// Offsets for DCRData and address
//

//
// Offset for the Reply Free Queue
//

//
// Defines for the Reply Descriptor Post Queue
//

//
// Defines for the HCBSize and address
//

//
// Offsets for the Scratchpad registers
//

//
// Offsets for the Request Descriptor Post Queue
//

// Hard Reset delay timings

//
// Message Descriptors
//
// Request Descriptors
// Default Request Descriptor
// PTR_MPI2_DEFAULT_REQUEST_DESCRIPTOR,
// pMpi2DefaultRequestDescriptor_t;
// defines for the RequestFlags field

// High Priority Request Descriptor
// PTR_MPI2_HIGH_PRIORITY_REQUEST_DESCRIPTOR,
// pMpi2HighPriorityRequestDescriptor_t;
// SCSI IO Request Descriptor
// PTR_MPI2_SCSI_IO_REQUEST_DESCRIPTOR,
// pMpi2SCSIIORequestDescriptor_t;
// SCSI Target Request Descriptor
// PTR_MPI2_SCSI_TARGET_REQUEST_DESCRIPTOR,
// pMpi2SCSITargetRequestDescriptor_t;
// RAID Accelerator Request Descriptor
// PTR_MPI2_RAID_ACCEL_REQUEST_DESCRIPTOR,
// pMpi2RAIDAcceleratorRequestDescriptor_t;
// Fast Path SCSI IO Request Descriptor
// PTR_MPI25_FP_SCSI_IO_REQUEST_DESCRIPTOR,
// pMpi25FastPathSCSIIORequestDescriptor_t;
// PCIe Encapsulated Request Descriptor
// PTR_MPI26_PCIE_ENCAPSULATED_REQUEST_DESCRIPTOR,
// pMpi26PCIeEncapsulatedRequestDescriptor_t;
// union of Request Descriptors
// PTR_MPI2_REQUEST_DESCRIPTOR_UNION,
// pMpi2RequestDescriptorUnion_t;
// Atomic Request Descriptors
//
// All Atomic Request Descriptors have the same format, so the following
// structure is used for all Atomic Request Descriptors:
// Atomic Default Request Descriptor
// Atomic High Priority Request Descriptor
// Atomic SCSI IO Request Descriptor
// Atomic SCSI Target Request Descriptor
// Atomic RAID Accelerator Request Descriptor
// Atomic Fast Path SCSI IO Request Descriptor
// Atomic PCIe Encapsulated Request Descriptor
//
// Atomic Request Descriptor
// PTR_MPI26_ATOMIC_REQUEST_DESCRIPTOR,
// pMpi26AtomicRequestDescriptor_t;
// for the RequestFlags field, use the same
// defines as MPI2_DEFAULT_REQUEST_DESCRIPTOR
//
// Reply Descriptors
// Default Reply Descriptor
// PTR_MPI2_DEFAULT_REPLY_DESCRIPTOR,
// pMpi2DefaultReplyDescriptor_t;
// defines for the ReplyFlags field

// values for marking a reply descriptor as unused

// Address Reply Descriptor
// PTR_MPI2_ADDRESS_REPLY_DESCRIPTOR,
// pMpi2AddressReplyDescriptor_t;

// SCSI IO Success Reply Descriptor
// PTR_MPI2_SCSI_IO_SUCCESS_REPLY_DESCRIPTOR,
// pMpi2SCSIIOSuccessReplyDescriptor_t;
// TargetAssist Success Reply Descriptor
// PTR_MPI2_TARGETASSIST_SUCCESS_REPLY_DESCRIPTOR,
// pMpi2TargetAssistSuccessReplyDescriptor_t;
// Target Command Buffer Reply Descriptor
// PTR_MPI2_TARGET_COMMAND_BUFFER_REPLY_DESCRIPTOR,
// pMpi2TargetCommandBufferReplyDescriptor_t;
// defines for Flags field

// RAID Accelerator Success Reply Descriptor
// PTR_MPI2_RAID_ACCELERATOR_SUCCESS_REPLY_DESCRIPTOR,
// pMpi2RAIDAcceleratorSuccessReplyDescriptor_t;
// Fast Path SCSI IO Success Reply Descriptor
// PTR_MPI25_FP_SCSI_IO_SUCCESS_REPLY_DESCRIPTOR,
// pMpi25FastPathSCSIIOSuccessReplyDescriptor_t;
// PCIe Encapsulated Success Reply Descriptor
// PTR_MPI26_PCIE_ENCAPSULATED_SUCCESS_REPLY_DESCRIPTOR,
// pMpi26PCIeEncapsulatedSuccessReplyDescriptor_t;
// union of Reply Descriptors
// PTR_MPI2_REPLY_DESCRIPTORS_UNION,
// pMpi2ReplyDescriptorsUnion_t;
//
// Message Functions
//

// Doorbell functions

//
// IOC Status Values
//
// mask for IOCStatus status value

//
// Common IOCStatus values for all replies
//

// MPI v2.6 and later

//
// Config IOCStatus values
//

//
// SCSI IO Reply
//

//
// For use by SCSI Initiator and SCSI Target end-to-end data protection
//

//
// SCSI Target values
//

//
// Serial Attached SCSI values
//

//
// Diagnostic Buffer Post / Diagnostic Release values
//

//
// RAID Accelerator values
//

//
// IOCStatus flag to indicate that log info is available
//

//
// IOCLogInfo Types
//

//
// Standard Message Structures
//
// Request Message Header for all request messages
//
// Default Reply
//
// common version structure/union used in messages and configuration pages
// LUN field defines, common to many structures

//
// Fusion-MPT MPI Scatter Gather Elements
//
// MPI Simple Element structures
//
// PTR_MPI2_SGE_SIMPLE_UNION,
// pMpi2SGESimpleUnion_t;
//
// MPI Chain Element structures - for MPI v2.0 products only
//
// PTR_MPI2_SGE_CHAIN_UNION,
// pMpi2SGEChainUnion_t;
//
// MPI Transaction Context Element structures - for MPI v2.0 products only
//
// PTR_MPI2_SGE_TRANSACTION32,
// pMpi2SGETransaction32_t;
// PTR_MPI2_SGE_TRANSACTION64,
// pMpi2SGETransaction64_t;
// PTR_MPI2_SGE_TRANSACTION_UNION,
// pMpi2SGETransactionUnion_t;
//
// MPI SGE union for IO SGL's - for MPI v2.0 products only
//
// MPI SGE union for SGL's with Simple and Transaction elements - for MPI v2.0 products only
//
// PTR_MPI2_SGE_TRANS_SIMPLE_UNION,
// pMpi2SGETransSimpleUnion_t;
//
// All MPI SGE types union
//
// MPI SGE field definition and masks
//
// Flags field bit definitions

// Element Type

// Address location

// Direction

// Address Size

// Context Size

//
// MPI SGE operation Macros
//
// SIMPLE FlagsLength manipulations...

// CAUTION - The following are READ-MODIFY-WRITE!

//
// Fusion-MPT IEEE Scatter Gather Elements
//
// IEEE Simple Element structures
//
// MPI2_IEEE_SGE_SIMPLE32 is for MPI v2.0 products only
// PTR_MPI2_IEEE_SGE_SIMPLE_UNION,
// pMpi2IeeeSgeSimpleUnion_t;
//
// IEEE Chain Element structures
//
// MPI2_IEEE_SGE_CHAIN32 is for MPI v2.0 products only
pub type MPI2_IEEE_SGE_CHAIN32 = MPI2_IEEE_SGE_SIMPLE32;
// MPI2_IEEE_SGE_CHAIN64 is for MPI v2.0 products only
pub type MPI2_IEEE_SGE_CHAIN64 = MPI2_IEEE_SGE_SIMPLE64;
// PTR_MPI2_IEEE_SGE_CHAIN_UNION,
// pMpi2IeeeSgeChainUnion_t;
// MPI25_IEEE_SGE_CHAIN64 is for MPI v2.5 and later
// PTR_MPI25_IEEE_SGE_CHAIN64,
// pMpi25IeeeSgeChain64_t;
//
// All IEEE SGE types union
//
// MPI2_IEEE_SGE_UNION is for MPI v2.0 products only
//
// IEEE SGE union for IO SGL's
//
// IEEE SGE field definitions and masks
//
// Flags field bit definitions

// Element Type

// Next Segment Format

// Data Location Address Space

//
// IEEE SGE operation Macros
//
// SIMPLE FlagsLength manipulations...

// CAUTION - The following are READ-MODIFY-WRITE!

//
// Fusion-MPT MPI/IEEE Scatter Gather Unions
//
// Values for SGLFlags field, used in many request messages with an SGL
//
// values for MPI SGL Data Location Address Space subfield

// values for SGL Type subfield

