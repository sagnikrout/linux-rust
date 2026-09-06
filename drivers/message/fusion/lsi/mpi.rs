//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi.h
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
// Name:  mpi.h
// Title:  MPI Message independent structures and definitions
// Creation Date:  July 27, 2000
//
// mpi.h Version:  01.05.16
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 05-08-00  00.10.01  Original release for 0.10 spec dated 4/26/2000.
// 05-24-00  00.10.02  Added MPI_IOCSTATUS_SCSI_RESIDUAL_MISMATCH definition.
// 06-06-00  01.00.01  Update MPI_VERSION_MAJOR and MPI_VERSION_MINOR.
// 06-22-00  01.00.02  Added MPI_IOCSTATUS_LAN_ definitions.
// Removed LAN_SUSPEND function definition.
// Added MPI_MSGFLAGS_CONTINUATION_REPLY definition.
// 06-30-00  01.00.03  Added MPI_CONTEXT_REPLY_TYPE_LAN definition.
// Added MPI_GET/SET_CONTEXT_REPLY_TYPE macros.
// 07-27-00  01.00.04  Added MPI_FAULT_ definitions.
// Removed MPI_IOCSTATUS_MSG/DATA_XFER_ERROR definitions.
// Added MPI_IOCSTATUS_INTERNAL_ERROR definition.
// Added MPI_IOCSTATUS_TARGET_XFER_COUNT_MISMATCH.
// 11-02-00  01.01.01  Original release for post 1.0 work.
// 12-04-00  01.01.02  Added new function codes.
// 01-09-01  01.01.03  Added more definitions to the system interface section
// Added MPI_IOCSTATUS_TARGET_STS_DATA_NOT_SENT.
// 01-25-01  01.01.04  Changed MPI_VERSION_MINOR from 0x00 to 0x01.
// 02-20-01  01.01.05  Started using MPI_POINTER.
// Fixed value for MPI_DIAG_RW_ENABLE.
// Added defines for MPI_DIAG_PREVENT_IOC_BOOT and
// MPI_DIAG_CLEAR_FLASH_BAD_SIG.
// Obsoleted MPI_IOCSTATUS_TARGET_FC_ defines.
// 02-27-01  01.01.06  Removed MPI_HOST_INDEX_REGISTER define.
// Added function codes for RAID.
// 04-09-01  01.01.07  Added alternate define for MPI_DOORBELL_ACTIVE,
// MPI_DOORBELL_USED, to better match the spec.
// 08-08-01  01.02.01  Original release for v1.2 work.
// Changed MPI_VERSION_MINOR from 0x01 to 0x02.
// Added define MPI_FUNCTION_TOOLBOX.
// 09-28-01  01.02.02  New function code MPI_SCSI_ENCLOSURE_PROCESSOR.
// 11-01-01  01.02.03  Changed name to MPI_FUNCTION_SCSI_ENCLOSURE_PROCESSOR.
// 03-14-02  01.02.04  Added MPI_HEADER_VERSION_ defines.
// 05-31-02  01.02.05  Bumped MPI_HEADER_VERSION_UNIT.
// 07-12-02  01.02.06  Added define for MPI_FUNCTION_MAILBOX.
// 09-16-02  01.02.07  Bumped value for MPI_HEADER_VERSION_UNIT.
// 11-15-02  01.02.08  Added define MPI_IOCSTATUS_TARGET_INVALID_IO_INDEX and
// obsoleted define MPI_IOCSTATUS_TARGET_INVALID_IOCINDEX.
// 04-01-03  01.02.09  New IOCStatus code: MPI_IOCSTATUS_FC_EXCHANGE_CANCELED
// 06-26-03  01.02.10  Bumped MPI_HEADER_VERSION_UNIT value.
// 01-16-04  01.02.11  Added define for MPI_IOCLOGINFO_TYPE_SHIFT.
// 04-29-04  01.02.12  Added function codes for MPI_FUNCTION_DIAG_BUFFER_POST
// and MPI_FUNCTION_DIAG_RELEASE.
// Added MPI_IOCSTATUS_DIAGNOSTIC_RELEASED define.
// Bumped MPI_HEADER_VERSION_UNIT value.
// 05-11-04  01.03.01  Bumped MPI_VERSION_MINOR for MPI v1.3.
// Added codes for Inband.
// 08-19-04  01.05.01  Added defines for Host Buffer Access Control doorbell.
// Added define for offset of High Priority Request Queue.
// Added new function codes and new IOCStatus codes.
// Added a IOCLogInfo type of SAS.
// 12-07-04  01.05.02  Bumped MPI_HEADER_VERSION_UNIT.
// 12-09-04  01.05.03  Bumped MPI_HEADER_VERSION_UNIT.
// 01-15-05  01.05.04  Bumped MPI_HEADER_VERSION_UNIT.
// 02-09-05  01.05.05  Bumped MPI_HEADER_VERSION_UNIT.
// 02-22-05  01.05.06  Bumped MPI_HEADER_VERSION_UNIT.
// 03-11-05  01.05.07  Removed function codes for SCSI IO 32 and
// TargetAssistExtended requests.
// Removed EEDP IOCStatus codes.
// 06-24-05  01.05.08  Added function codes for SCSI IO 32 and
// TargetAssistExtended requests.
// Added EEDP IOCStatus codes.
// 08-03-05  01.05.09  Bumped MPI_HEADER_VERSION_UNIT.
// 08-30-05  01.05.10  Added 2 new IOCStatus codes for Target.
// 03-27-06  01.05.11  Bumped MPI_HEADER_VERSION_UNIT.
// 10-11-06  01.05.12  Bumped MPI_HEADER_VERSION_UNIT.
// 05-24-07  01.05.13  Bumped MPI_HEADER_VERSION_UNIT.
// 08-07-07  01.05.14  Bumped MPI_HEADER_VERSION_UNIT.
// 01-15-08  01.05.15  Bumped MPI_HEADER_VERSION_UNIT.
// 03-28-08  01.05.16  Bumped MPI_HEADER_VERSION_UNIT.
// --------------------------------------------------------------------------
//
// M P I    V e r s i o n    D e f i n i t i o n s
//

// Note: The major versions of 0xe0 through 0xff are reserved
// versioning for this MPI header set

//
// I O C    S t a t e    D e f i n i t i o n s
//

// Fault state codes (product independent range 0x8000-0xFFFF)

//
// P C I    S y s t e m    I n t e r f a c e    R e g i s t e r s
//
// Defines for working with the System Doorbell register.
// Values for doorbell function codes are included in the section that defines
// all the function codes (further on in this file).
//

// values for Host Buffer Access Control doorbell function

//
// M e s s a g e    F r a m e    D e s c r i p t o r s
//

//
// Context Reply macros
//

//
// M e s s a g e    F u n c t i o n s
// 0x80 -> 0x8F reserved for private message use per product
//

// standard version format
//
// S c a t t e r    G a t h e r    E l e m e n t s
//
// Simple element structures
//
// Chain element structures
//
// Transaction Context element
//
// SGE IO types union  for IO SGL's
//
// SGE union for SGL's with Simple and Transaction elements
//
// All SGE types union
//
// SGE field definition and masks
//
// Flags field bit definitions

// Element Type

// Address location

// Direction

// Address Size

// Context Size

//
// SGE operation Macros
//
// SIMPLE FlagsLength manipulations...

// CAUTION - The following are READ-MODIFY-WRITE!

//
// S t a n d a r d    M e s s a g e    S t r u c t u r e s
//
// Standard message request header for all request messages
//
// Default Reply
//
// MsgFlags definition for all replies

//
// I O C    S t a t u s   V a l u e s
//
// Common IOCStatus values for all replies
//

//
// Config IOCStatus values
//

//
// SCSIIO Reply (SPI & FCP) initiator values
//

//
// For use by SCSI Initiator and SCSI Target end-to-end data protection
//

//
// SCSI Target values
//

//
// Additional FCP target values (obsolete)
//

//
// Fibre Channel Direct Access values
//

//
// LAN values
//

//
// Serial Attached SCSI values
//

//
// Inband values
//

//
// Diagnostic Tools values
//

//
// IOCStatus flag to indicate that log info is available
//

//
// LogInfo Types
//

