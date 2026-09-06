//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ata.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2003-2004 Red Hat, Inc.  All rights reserved.
// Copyright 2003-2004 Jeff Garzik
//
// libata documentation is available via 'make {ps|pdf}docs',
// as Documentation/driver-api/libata.rst
//
// Hardware documentation available from http://www.t13.org
//

// defines only for the constants which don't work well as enums
pub const ATA_DMA_BOUNDARY: c_uint = 0xffffUL;
pub const ATA_DMA_MASK: c_uint = 0xffffffffULL;
// various global constants
// ATA_UDMA7 is just for completeness... doesn't exist (yet?).
// DMA-related
// bits in ATA command block registers
// ATA command block registers
// ATA device commands
// marked obsolete in the ATA/ATAPI-7 spec
// Subcmds for ATA_CMD_FPDMA_RECV
// Subcmds for ATA_CMD_FPDMA_SEND
// Subcmds for ATA_CMD_NCQ_NON_DATA
// Subcmds for ATA_CMD_ZAC_MGMT_IN
// Subcmds for ATA_CMD_ZAC_MGMT_OUT
// READ_LOG_EXT pages
// Identify device log pages:
// Identify device SATA settings log:
// NCQ send and receive log
// NCQ Non-Data log
// READ/WRITE LONG (obsolete)
// SETFEATURES stuff
// Enable/Disable Automatic Acoustic Management
// SETFEATURE Sector counts for SATA features
// feature values for SET_MAX
// feature values for DEVICE CONFIGURATION OVERLAY
// feature values for SMART
// feature values for Data Set Management
// password used in LBA Mid / LBA High for executing SMART commands
// ATAPI stuff
// PMP stuff
// cable types
// SATA Status and Control Registers
// SError bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ata_prot_flags {
// protocol flags
    ATA_PROT_FLAG_PIO	= (1 << 0), /* is PIO */
    ATA_PROT_FLAG_DMA	= (1 << 1), /* is DMA */
    ATA_PROT_FLAG_NCQ	= (1 << 2), /* is NCQ */
    ATA_PROT_FLAG_ATAPI	= (1 << 3), /* is ATAPI */

// taskfile protocols
    ATA_PROT_UNKNOWN	= (u8)-1,
    ATA_PROT_NODATA		= 0,
    ATA_PROT_PIO		= ATA_PROT_FLAG_PIO,
    ATA_PROT_DMA		= ATA_PROT_FLAG_DMA,
    ATA_PROT_NCQ_NODATA	= ATA_PROT_FLAG_NCQ,
    ATA_PROT_NCQ		= ATA_PROT_FLAG_DMA | ATA_PROT_FLAG_NCQ,
    ATAPI_PROT_NODATA	= ATA_PROT_FLAG_ATAPI,
    ATAPI_PROT_PIO		= ATA_PROT_FLAG_ATAPI | ATA_PROT_FLAG_PIO,
    ATAPI_PROT_DMA		= ATA_PROT_FLAG_ATAPI | ATA_PROT_FLAG_DMA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ata_ioctls {
    ATA_IOC_GET_IO32	= 0x309, /* HDIO_GET_32BIT */
    ATA_IOC_SET_IO32	= 0x324, /* HDIO_SET_32BIT */
}

// core structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_bmdma_prd {
    pub addr: __le32,
    pub flags_len: __le32,
}

//
// id tests
//

// T13/1699-D Revision 6a, Sep 6, 2008. Page 128.
// IDENTIFY DEVICE data, word 117-118.
// 0xd000 ignores bit 13 (logical:physical > 1)
//
// T13/1699-D Revision 6a, Sep 6, 2008. Page 128.
// IDENTIFY DEVICE data, word 106.
// 0xe000 ignores bit 12 (logical sector > 512 bytes)
//
// Offset of logical sectors relative to physical sectors.
//
// If device has more than one logical sector per physical sector
// (aka 512 byte emulation), vendors might offset the "sector 0" address
// so sector 63 is "naturally aligned" - e.g. FAT partition table.
// This avoids Read/Mod/Write penalties when using FAT partition table
// and updating "well aligned" (FS perspective) physical sectors on every
// transaction.
//
// Yes children, word 83 valid bits cover word 82 data
// And 87 covers 85-87
// Check command sets enabled as well as supported
// Yes children, word 83 valid bits cover word 82 data
// Word 86 must have bit 15 set
// READ LOG DMA EXT support can be signaled either from word 119
// or from word 120. The format is the same for both words: Bit
// 15 must be cleared, bit 14 set and bit 3 set.
//
// ata_id_has_sense_reporting() == true, word 86 must have bit 15 set
//
// Word: 206 - SCT Command Transport
// 15:12 - Vendor Specific
// 11:6 - Reserved
// 5 - SCT Command Transport Data Tables supported
// 4 - SCT Command Transport Features Control supported
// 3 - SCT Command Transport Error Recovery Control supported
// 2 - SCT Command Transport Write Same supported
// 1 - SCT Command Transport Long Sector Access supported
// 0 - SCT Command Transport supported
//
// ata_id_major_version	-	get ATA level of drive
// @id: Identify data
//
// Caveats:
// ATA-1 considers identify optional
// ATA-2 introduces mandatory identify
// ATA-3 introduces word 80 and accurate reporting
//
// The practical impact of this is that ata_id_major_version cannot
// reliably report on drives below ATA3.
//
// Returns: major version of ATA drive level or %0 if unknown
//
// See if word 93 is 0 AND drive is at least ATA-5 compatible
// verifying that word 80 by casting it to a signed type --
// this trick allows us to filter out the reserved values of
// 0x0000 and 0xffff along with the earlier ATA revisions...
//
// The TPM bits are only valid on ATA8
// ATA 8 reuses this flag for "trusted" computing
//
// IDENTIFY DEVICE word 105: MAX PAGES PER DSM COMMAND. Maximum number
// of 512-byte pages of LBA Range Entries the device accepts in a
// single DATA SET MANAGEMENT command. Zero means the device does not
// specify a limit. The field is reserved unless TRIM is supported, so
// callers must gate on ata_id_has_trim().
//
// DSM supported, deterministic read, and read zero after trim set
// For ATA-1 devices, if the INITIALIZE DEVICE PARAMETERS command
//
// CF specs don't require specific value in the word 0 anymore and yet
// they forbid to report the ATA version in the word 80 and require the
// CFA feature set support to be indicated in the word 83 in this case.
// Unfortunately, some cards only follow either of this requirements,
// and while those that don't indicate CFA feature support need some
// sort of quirk list, it seems impractical for the ones that do...
//
// CF spec. r4.1 Table 22 says no IORDY on PIO5 and PIO6.
// For PIO3 and higher it is mandatory.
// Turn it on when possible.
extern "C" {
    pub fn ata_id_has_iordy(_arg: id) -> return;
}
extern "C" {
    pub fn ata_id_major_version(0x8000: dev_id) >= 7 && (dev_id[62] &) -> return;
}
// check the ending block number: must be LESS THAN 0x0fffffff
// check the ending block number

