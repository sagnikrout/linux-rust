//! Automatically rewritten from C Header to Rust Module
//! Source: fs/udf/ecma_167.h
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
// ecma_167.h
//
// This file is based on ECMA-167 3rd edition (June 1997)
// https://www.ecma.ch
//
// Copyright (c) 2001-2002  Ben Fennema
// Copyright (c) 2017-2019  Pali Rohár <pali@kernel.org>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. The name of the author may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU Public License ("GPL").
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//
// @file
// ECMA-167r3 defines and structure definitions
//

pub const _ECMA_167_H: c_int = 1;
// Character sets and coding - d-characters (ECMA 167r3 1/7.2)
pub type dchars = u8;
// Character set specification (ECMA 167r3 1/7.2.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct charspec {
    pub charSetType: u8,
    pub charSetInfo: [u8; 63],
    pub __packed: },
// Character Set Type (ECMA 167r3 1/7.2.1.1)
pub const CHARSPEC_TYPE_CS0: c_uint = 0x00	/* (1/7.2.2) */;
pub const CHARSPEC_TYPE_CS1: c_uint = 0x01	/* (1/7.2.3) */;
pub const CHARSPEC_TYPE_CS2: c_uint = 0x02	/* (1/7.2.4) */;
pub const CHARSPEC_TYPE_CS3: c_uint = 0x03	/* (1/7.2.5) */;
pub const CHARSPEC_TYPE_CS4: c_uint = 0x04	/* (1/7.2.6) */;
pub const CHARSPEC_TYPE_CS5: c_uint = 0x05	/* (1/7.2.7) */;
pub const CHARSPEC_TYPE_CS6: c_uint = 0x06	/* (1/7.2.8) */;
pub const CHARSPEC_TYPE_CS7: c_uint = 0x07	/* (1/7.2.9) */;
pub const CHARSPEC_TYPE_CS8: c_uint = 0x08	/* (1/7.2.10) */;
// Fixed-length character fields - d-string (EMCA 167r3 1/7.2.12)
pub type dstring = u8;
// Timestamp (ECMA 167r3 1/7.3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timestamp {
    pub typeAndTimezone: __le16,
    pub year: __le16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub centiseconds: u8,
    pub hundredsOfMicroseconds: u8,
    pub microseconds: u8,
    pub __packed: },
// Type and Time Zone (ECMA 167r3 1/7.3.1)
pub const TIMESTAMP_TYPE_MASK: c_uint = 0xF000;
pub const TIMESTAMP_TYPE_CUT: c_uint = 0x0000;
pub const TIMESTAMP_TYPE_LOCAL: c_uint = 0x1000;
pub const TIMESTAMP_TYPE_AGREEMENT: c_uint = 0x2000;
pub const TIMESTAMP_TIMEZONE_MASK: c_uint = 0x0FFF;
// Entity identifier (ECMA 167r3 1/7.4)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regid {
    pub flags: u8,
    pub ident: [u8; 23],
    pub identSuffix: [u8; 8],
    pub __packed: },
// Flags (ECMA 167r3 1/7.4.1)
pub const ENTITYID_FLAGS_DIRTY: c_uint = 0x01;
pub const ENTITYID_FLAGS_PROTECTED: c_uint = 0x02;
// Volume Structure Descriptor (ECMA 167r3 2/9.1)
pub const VSD_STD_ID_LEN: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct volStructDesc {
    pub structType: u8,
    pub stdIdent: [u8; VSD_STD_ID_LEN],
    pub structVersion: u8,
    pub structData: [u8; 2041],
    pub __packed: },
// Standard Identifier (EMCA 167r2 2/9.1.2)

// Standard Identifier (ECMA 167r3 2/9.1.2)

// Beginning Extended Area Descriptor (ECMA 167r3 2/9.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct beginningExtendedAreaDesc {
    pub structType: u8,
    pub stdIdent: [u8; VSD_STD_ID_LEN],
    pub structVersion: u8,
    pub structData: [u8; 2041],
    pub __packed: },
// Terminating Extended Area Descriptor (ECMA 167r3 2/9.3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct terminatingExtendedAreaDesc {
    pub structType: u8,
    pub stdIdent: [u8; VSD_STD_ID_LEN],
    pub structVersion: u8,
    pub structData: [u8; 2041],
    pub __packed: },
// Boot Descriptor (ECMA 167r3 2/9.4)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootDesc {
    pub structType: u8,
    pub stdIdent: [u8; VSD_STD_ID_LEN],
    pub structVersion: u8,
    pub reserved1: u8,
    pub archType: regid,
    pub bootIdent: regid,
    pub bootExtLocation: __le32,
    pub bootExtLength: __le32,
    pub loadAddress: __le64,
    pub startAddress: __le64,
    pub descCreationDateAndTime: timestamp,
    pub flags: __le16,
    pub reserved2: [u8; 32],
    pub bootUse: [u8; 1906],
    pub __packed: },
// Flags (ECMA 167r3 2/9.4.12)
pub const BOOT_FLAGS_ERASE: c_uint = 0x01;
// Extent Descriptor (ECMA 167r3 3/7.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_ad {
    pub extLength: __le32,
    pub extLocation: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_extent_ad {
    pub extLength: u32,
    pub extLocation: u32,
}

// Descriptor Tag (ECMA 167r3 3/7.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag {
    pub tagIdent: __le16,
    pub descVersion: __le16,
    pub tagChecksum: u8,
    pub reserved: u8,
    pub tagSerialNum: __le16,
    pub descCRC: __le16,
    pub descCRCLength: __le16,
    pub tagLocation: __le32,
    pub __packed: },
// Tag Identifier (ECMA 167r3 3/7.2.1)
pub const TAG_IDENT_PVD: c_uint = 0x0001;
pub const TAG_IDENT_AVDP: c_uint = 0x0002;
pub const TAG_IDENT_VDP: c_uint = 0x0003;
pub const TAG_IDENT_IUVD: c_uint = 0x0004;
pub const TAG_IDENT_PD: c_uint = 0x0005;
pub const TAG_IDENT_LVD: c_uint = 0x0006;
pub const TAG_IDENT_USD: c_uint = 0x0007;
pub const TAG_IDENT_TD: c_uint = 0x0008;
pub const TAG_IDENT_LVID: c_uint = 0x0009;
// NSR Descriptor (ECMA 167r3 3/9.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSRDesc {
    pub structType: u8,
    pub stdIdent: [u8; VSD_STD_ID_LEN],
    pub structVersion: u8,
    pub reserved: u8,
    pub structData: [u8; 2040],
    pub __packed: },
// Generic Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genericDesc {
    pub descTag: tag,
    pub volDescSeqNum: __le32,
    pub reserved: [u8; 492],
    pub __packed: },
// Primary Volume Descriptor (ECMA 167r3 3/10.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct primaryVolDesc {
    pub descTag: tag,
    pub volDescSeqNum: __le32,
    pub primaryVolDescNum: __le32,
    pub volIdent: [dstring; 32],
    pub volSeqNum: __le16,
    pub maxVolSeqNum: __le16,
    pub interchangeLvl: __le16,
    pub maxInterchangeLvl: __le16,
    pub charSetList: __le32,
    pub maxCharSetList: __le32,
    pub volSetIdent: [dstring; 128],
    pub descCharSet: charspec,
    pub explanatoryCharSet: charspec,
    pub volAbstract: extent_ad,
    pub volCopyright: extent_ad,
    pub appIdent: regid,
    pub recordingDateAndTime: timestamp,
    pub impIdent: regid,
    pub impUse: [u8; 64],
    pub predecessorVolDescSeqLocation: __le32,
    pub flags: __le16,
    pub reserved: [u8; 22],
    pub __packed: },
// Flags (ECMA 167r3 3/10.1.21)
pub const PVD_FLAGS_VSID_COMMON: c_uint = 0x0001;
// Anchor Volume Descriptor Pointer (ECMA 167r3 3/10.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct anchorVolDescPtr {
    pub descTag: tag,
    pub mainVolDescSeqExt: extent_ad,
    pub reserveVolDescSeqExt: extent_ad,
    pub reserved: [u8; 480],
    pub __packed: },
// Volume Descriptor Pointer (ECMA 167r3 3/10.3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct volDescPtr {
    pub descTag: tag,
    pub volDescSeqNum: __le32,
    pub nextVolDescSeqExt: extent_ad,
    pub reserved: [u8; 484],
    pub __packed: },
// Implementation Use Volume Descriptor (ECMA 167r3 3/10.4)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct impUseVolDesc {
    pub descTag: tag,
    pub volDescSeqNum: __le32,
    pub impIdent: regid,
    pub impUse: [u8; 460],
    pub __packed: },
// Partition Descriptor (ECMA 167r3 3/10.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct partitionDesc {
    pub descTag: tag,
    pub volDescSeqNum: __le32,
    pub partitionFlags: __le16,
    pub partitionNumber: __le16,
    pub partitionContents: regid,
    pub partitionContentsUse: [u8; 128],
    pub accessType: __le32,
    pub partitionStartingLocation: __le32,
    pub partitionLength: __le32,
    pub impIdent: regid,
    pub impUse: [u8; 128],
    pub reserved: [u8; 156],
    pub __packed: },
// Partition Flags (ECMA 167r3 3/10.5.3)
pub const PD_PARTITION_FLAGS_ALLOC: c_uint = 0x0001;
// Partition Contents (ECMA 167r2 3/10.5.3)

// Partition Contents (ECMA 167r3 3/10.5.5)

// Access Type (ECMA 167r3 3/10.5.7)
pub const PD_ACCESS_TYPE_NONE: c_uint = 0x00000000;
pub const PD_ACCESS_TYPE_READ_ONLY: c_uint = 0x00000001;
pub const PD_ACCESS_TYPE_WRITE_ONCE: c_uint = 0x00000002;
pub const PD_ACCESS_TYPE_REWRITABLE: c_uint = 0x00000003;
pub const PD_ACCESS_TYPE_OVERWRITABLE: c_uint = 0x00000004;
// Logical Volume Descriptor (ECMA 167r3 3/10.6)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicalVolDesc {
    pub descTag: tag,
    pub volDescSeqNum: __le32,
    pub descCharSet: charspec,
    pub logicalVolIdent: [dstring; 128],
    pub logicalBlockSize: __le32,
    pub domainIdent: regid,
    pub logicalVolContentsUse: [u8; 16],
    pub mapTableLength: __le32,
    pub numPartitionMaps: __le32,
    pub impIdent: regid,
    pub impUse: [u8; 128],
    pub integritySeqExt: extent_ad,
    pub partitionMaps: [u8; ],
    pub __packed: },
// Generic Partition Map (ECMA 167r3 3/10.7.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genericPartitionMap {
    pub partitionMapType: u8,
    pub partitionMapLength: u8,
    pub partitionMapping: [u8; ],
    pub __packed: },
// Partition Map Type (ECMA 167r3 3/10.7.1.1)
pub const GP_PARTITION_MAP_TYPE_UNDEF: c_uint = 0x00;
pub const GP_PARTITION_MAP_TYPE_1: c_uint = 0x01;
pub const GP_PARTITION_MAP_TYPE_2: c_uint = 0x02;
// Type 1 Partition Map (ECMA 167r3 3/10.7.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genericPartitionMap1 {
    pub partitionMapType: u8,
    pub partitionMapLength: u8,
    pub volSeqNum: __le16,
    pub partitionNum: __le16,
    pub __packed: },
// Type 2 Partition Map (ECMA 167r3 3/10.7.3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genericPartitionMap2 {
    pub partitionMapType: u8,
    pub partitionMapLength: u8,
    pub partitionIdent: [u8; 62],
    pub __packed: },
// Unallocated Space Descriptor (ECMA 167r3 3/10.8)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unallocSpaceDesc {
    pub descTag: tag,
    pub volDescSeqNum: __le32,
    pub numAllocDescs: __le32,
    pub allocDescs: [extent_ad; ],
    pub __packed: },
// Terminating Descriptor (ECMA 167r3 3/10.9)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct terminatingDesc {
    pub descTag: tag,
    pub reserved: [u8; 496],
    pub __packed: },
// Logical Volume Integrity Descriptor (ECMA 167r3 3/10.10)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicalVolIntegrityDesc {
    pub descTag: tag,
    pub recordingDateAndTime: timestamp,
    pub integrityType: __le32,
    pub nextIntegrityExt: extent_ad,
    pub logicalVolContentsUse: [u8; 32],
    pub numOfPartitions: __le32,
    pub lengthOfImpUse: __le32,
    pub freeSpaceTable: [__le32; ],
// __le32		sizeTable[];
// uint8_t		impUse[];
    pub __packed: },
// Integrity Type (ECMA 167r3 3/10.10.3)
pub const LVID_INTEGRITY_TYPE_OPEN: c_uint = 0x00000000;
pub const LVID_INTEGRITY_TYPE_CLOSE: c_uint = 0x00000001;
// Recorded Address (ECMA 167r3 4/7.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_addr {
    pub logicalBlockNum: __le32,
    pub partitionReferenceNum: __le16,
    pub __packed: },
// ... and its in-core analog
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_lb_addr {
    pub logicalBlockNum: u32,
    pub partitionReferenceNum: u16,
}

// Short Allocation Descriptor (ECMA 167r3 4/14.14.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct short_ad {
    pub extLength: __le32,
    pub extPosition: __le32,
    pub __packed: },
// Long Allocation Descriptor (ECMA 167r3 4/14.14.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct long_ad {
    pub extLength: __le32,
    pub extLocation: lb_addr,
    pub impUse: [u8; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_long_ad {
    pub extLength: u32,
    pub extLocation: kernel_lb_addr,
    pub impUse: [u8; 6],
}

// Extended Allocation Descriptor (ECMA 167r3 4/14.14.3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_ad {
    pub extLength: __le32,
    pub recordedLength: __le32,
    pub informationLength: __le32,
    pub extLocation: lb_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_ext_ad {
    pub extLength: u32,
    pub recordedLength: u32,
    pub informationLength: u32,
    pub extLocation: kernel_lb_addr,
}

// Descriptor Tag (ECMA 167r3 4/7.2 - See 3/7.2)
// Tag Identifier (ECMA 167r3 4/7.2.1)
pub const TAG_IDENT_FSD: c_uint = 0x0100;
pub const TAG_IDENT_FID: c_uint = 0x0101;
pub const TAG_IDENT_AED: c_uint = 0x0102;
pub const TAG_IDENT_IE: c_uint = 0x0103;
pub const TAG_IDENT_TE: c_uint = 0x0104;
pub const TAG_IDENT_FE: c_uint = 0x0105;
pub const TAG_IDENT_EAHD: c_uint = 0x0106;
pub const TAG_IDENT_USE: c_uint = 0x0107;
pub const TAG_IDENT_SBD: c_uint = 0x0108;
pub const TAG_IDENT_PIE: c_uint = 0x0109;
pub const TAG_IDENT_EFE: c_uint = 0x010A;
// File Set Descriptor (ECMA 167r3 4/14.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fileSetDesc {
    pub descTag: tag,
    pub recordingDateAndTime: timestamp,
    pub interchangeLvl: __le16,
    pub maxInterchangeLvl: __le16,
    pub charSetList: __le32,
    pub maxCharSetList: __le32,
    pub fileSetNum: __le32,
    pub fileSetDescNum: __le32,
    pub logicalVolIdentCharSet: charspec,
    pub logicalVolIdent: [dstring; 128],
    pub fileSetCharSet: charspec,
    pub fileSetIdent: [dstring; 32],
    pub copyrightFileIdent: [dstring; 32],
    pub abstractFileIdent: [dstring; 32],
    pub rootDirectoryICB: long_ad,
    pub domainIdent: regid,
    pub nextExt: long_ad,
    pub streamDirectoryICB: long_ad,
    pub reserved: [u8; 32],
    pub __packed: },
// Partition Header Descriptor (ECMA 167r3 4/14.3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct partitionHeaderDesc {
    pub unallocSpaceTable: short_ad,
    pub unallocSpaceBitmap: short_ad,
    pub partitionIntegrityTable: short_ad,
    pub freedSpaceTable: short_ad,
    pub freedSpaceBitmap: short_ad,
    pub reserved: [u8; 88],
    pub __packed: },
// File Identifier Descriptor (ECMA 167r3 4/14.4)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fileIdentDesc {
    pub descTag: tag,
    pub fileVersionNum: __le16,
    pub fileCharacteristics: u8,
    pub lengthFileIdent: u8,
    pub icb: long_ad,
    pub lengthOfImpUse: __le16,
// uint8_t	impUse[];
// uint8_t	fileIdent[];
// uint8_t	padding[];
    pub __packed: },
// File Characteristics (ECMA 167r3 4/14.4.3)
pub const FID_FILE_CHAR_HIDDEN: c_uint = 0x01;
pub const FID_FILE_CHAR_DIRECTORY: c_uint = 0x02;
pub const FID_FILE_CHAR_DELETED: c_uint = 0x04;
pub const FID_FILE_CHAR_PARENT: c_uint = 0x08;
pub const FID_FILE_CHAR_METADATA: c_uint = 0x10;
// Allocation Ext Descriptor (ECMA 167r3 4/14.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocExtDesc {
    pub descTag: tag,
    pub previousAllocExtLocation: __le32,
    pub lengthAllocDescs: __le32,
    pub __packed: },
// ICB Tag (ECMA 167r3 4/14.6)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icbtag {
    pub priorRecordedNumDirectEntries: __le32,
    pub strategyType: __le16,
    pub strategyParameter: __le16,
    pub numEntries: __le16,
    pub reserved: u8,
    pub fileType: u8,
    pub parentICBLocation: lb_addr,
    pub flags: __le16,
    pub __packed: },
// Strategy Type (ECMA 167r3 4/14.6.2)
pub const ICBTAG_STRATEGY_TYPE_UNDEF: c_uint = 0x0000;
pub const ICBTAG_STRATEGY_TYPE_1: c_uint = 0x0001;
pub const ICBTAG_STRATEGY_TYPE_2: c_uint = 0x0002;
pub const ICBTAG_STRATEGY_TYPE_3: c_uint = 0x0003;
pub const ICBTAG_STRATEGY_TYPE_4: c_uint = 0x0004;
// File Type (ECMA 167r3 4/14.6.6)
pub const ICBTAG_FILE_TYPE_UNDEF: c_uint = 0x00;
pub const ICBTAG_FILE_TYPE_USE: c_uint = 0x01;
pub const ICBTAG_FILE_TYPE_PIE: c_uint = 0x02;
pub const ICBTAG_FILE_TYPE_IE: c_uint = 0x03;
pub const ICBTAG_FILE_TYPE_DIRECTORY: c_uint = 0x04;
pub const ICBTAG_FILE_TYPE_REGULAR: c_uint = 0x05;
pub const ICBTAG_FILE_TYPE_BLOCK: c_uint = 0x06;
pub const ICBTAG_FILE_TYPE_CHAR: c_uint = 0x07;
pub const ICBTAG_FILE_TYPE_EA: c_uint = 0x08;
pub const ICBTAG_FILE_TYPE_FIFO: c_uint = 0x09;
pub const ICBTAG_FILE_TYPE_SOCKET: c_uint = 0x0A;
pub const ICBTAG_FILE_TYPE_TE: c_uint = 0x0B;
pub const ICBTAG_FILE_TYPE_SYMLINK: c_uint = 0x0C;
pub const ICBTAG_FILE_TYPE_STREAMDIR: c_uint = 0x0D;
// Flags (ECMA 167r3 4/14.6.8)
pub const ICBTAG_FLAG_AD_MASK: c_uint = 0x0007;
pub const ICBTAG_FLAG_AD_SHORT: c_uint = 0x0000;
pub const ICBTAG_FLAG_AD_LONG: c_uint = 0x0001;
pub const ICBTAG_FLAG_AD_EXTENDED: c_uint = 0x0002;
pub const ICBTAG_FLAG_AD_IN_ICB: c_uint = 0x0003;
pub const ICBTAG_FLAG_SORTED: c_uint = 0x0008;
pub const ICBTAG_FLAG_NONRELOCATABLE: c_uint = 0x0010;
pub const ICBTAG_FLAG_ARCHIVE: c_uint = 0x0020;
pub const ICBTAG_FLAG_SETUID: c_uint = 0x0040;
pub const ICBTAG_FLAG_SETGID: c_uint = 0x0080;
pub const ICBTAG_FLAG_STICKY: c_uint = 0x0100;
pub const ICBTAG_FLAG_CONTIGUOUS: c_uint = 0x0200;
pub const ICBTAG_FLAG_SYSTEM: c_uint = 0x0400;
pub const ICBTAG_FLAG_TRANSFORMED: c_uint = 0x0800;
pub const ICBTAG_FLAG_MULTIVERSIONS: c_uint = 0x1000;
pub const ICBTAG_FLAG_STREAM: c_uint = 0x2000;
// Indirect Entry (ECMA 167r3 4/14.7)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct indirectEntry {
    pub descTag: tag,
    pub icbTag: icbtag,
    pub indirectICB: long_ad,
    pub __packed: },
// Terminal Entry (ECMA 167r3 4/14.8)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct terminalEntry {
    pub descTag: tag,
    pub icbTag: icbtag,
    pub __packed: },
// File Entry (ECMA 167r3 4/14.9)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fileEntry {
    pub descTag: tag,
    pub icbTag: icbtag,
    pub uid: __le32,
    pub gid: __le32,
    pub permissions: __le32,
    pub fileLinkCount: __le16,
    pub recordFormat: u8,
    pub recordDisplayAttr: u8,
    pub recordLength: __le32,
    pub informationLength: __le64,
    pub logicalBlocksRecorded: __le64,
    pub accessTime: timestamp,
    pub modificationTime: timestamp,
    pub attrTime: timestamp,
    pub checkpoint: __le32,
    pub extendedAttrICB: long_ad,
    pub impIdent: regid,
    pub uniqueID: __le64,
    pub lengthExtendedAttr: __le32,
    pub lengthAllocDescs: __le32,
    pub extendedAttr: [u8; ],
// uint8_t		allocDescs[];
    pub __packed: },
// Permissions (ECMA 167r3 4/14.9.5)
pub const FE_PERM_O_EXEC: c_uint = 0x00000001U;
pub const FE_PERM_O_WRITE: c_uint = 0x00000002U;
pub const FE_PERM_O_READ: c_uint = 0x00000004U;
pub const FE_PERM_O_CHATTR: c_uint = 0x00000008U;
pub const FE_PERM_O_DELETE: c_uint = 0x00000010U;
pub const FE_PERM_G_EXEC: c_uint = 0x00000020U;
pub const FE_PERM_G_WRITE: c_uint = 0x00000040U;
pub const FE_PERM_G_READ: c_uint = 0x00000080U;
pub const FE_PERM_G_CHATTR: c_uint = 0x00000100U;
pub const FE_PERM_G_DELETE: c_uint = 0x00000200U;
pub const FE_PERM_U_EXEC: c_uint = 0x00000400U;
pub const FE_PERM_U_WRITE: c_uint = 0x00000800U;
pub const FE_PERM_U_READ: c_uint = 0x00001000U;
pub const FE_PERM_U_CHATTR: c_uint = 0x00002000U;
pub const FE_PERM_U_DELETE: c_uint = 0x00004000U;
// Record Format (ECMA 167r3 4/14.9.7)
pub const FE_RECORD_FMT_UNDEF: c_uint = 0x00;
pub const FE_RECORD_FMT_FIXED_PAD: c_uint = 0x01;
pub const FE_RECORD_FMT_FIXED: c_uint = 0x02;
pub const FE_RECORD_FMT_VARIABLE8: c_uint = 0x03;
pub const FE_RECORD_FMT_VARIABLE16: c_uint = 0x04;
pub const FE_RECORD_FMT_VARIABLE16_MSB: c_uint = 0x05;
pub const FE_RECORD_FMT_VARIABLE32: c_uint = 0x06;
pub const FE_RECORD_FMT_PRINT: c_uint = 0x07;
pub const FE_RECORD_FMT_LF: c_uint = 0x08;
pub const FE_RECORD_FMT_CR: c_uint = 0x09;
pub const FE_RECORD_FMT_CRLF: c_uint = 0x0A;
pub const FE_RECORD_FMT_LFCR: c_uint = 0x0B;
// Record Display Attributes (ECMA 167r3 4/14.9.8)
pub const FE_RECORD_DISPLAY_ATTR_UNDEF: c_uint = 0x00;
pub const FE_RECORD_DISPLAY_ATTR_1: c_uint = 0x01;
pub const FE_RECORD_DISPLAY_ATTR_2: c_uint = 0x02;
pub const FE_RECORD_DISPLAY_ATTR_3: c_uint = 0x03;
// Extended Attribute Header Descriptor (ECMA 167r3 4/14.10.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extendedAttrHeaderDesc {
    pub descTag: tag,
    pub impAttrLocation: __le32,
    pub appAttrLocation: __le32,
    pub __packed: },
// Generic Format (ECMA 167r3 4/14.10.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genericFormat {
    pub attrType: __le32,
    pub attrSubtype: u8,
    pub reserved: [u8; 3],
    pub attrLength: __le32,
    pub attrData: [u8; ],
    pub __packed: },
// Character Set Information (ECMA 167r3 4/14.10.3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct charSetInfo {
    pub attrType: __le32,
    pub attrSubtype: u8,
    pub reserved: [u8; 3],
    pub attrLength: __le32,
    pub escapeSeqLength: __le32,
    pub charSetType: u8,
    pub escapeSeq: [u8; ],
    pub __packed: },
// Alternate Permissions (ECMA 167r3 4/14.10.4)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altPerms {
    pub attrType: __le32,
    pub attrSubtype: u8,
    pub reserved: [u8; 3],
    pub attrLength: __le32,
    pub ownerIdent: __le16,
    pub groupIdent: __le16,
    pub permission: __le16,
    pub __packed: },
// File Times Extended Attribute (ECMA 167r3 4/14.10.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fileTimesExtAttr {
    pub attrType: __le32,
    pub attrSubtype: u8,
    pub reserved: [u8; 3],
    pub attrLength: __le32,
    pub dataLength: __le32,
    pub fileTimeExistence: __le32,
    pub fileTimes: u8,
    pub __packed: },
// FileTimeExistence (ECMA 167r3 4/14.10.5.6)
pub const FTE_CREATION: c_uint = 0x00000001;
pub const FTE_DELETION: c_uint = 0x00000004;
pub const FTE_EFFECTIVE: c_uint = 0x00000008;
pub const FTE_BACKUP: c_uint = 0x00000002;
// Information Times Extended Attribute (ECMA 167r3 4/14.10.6)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct infoTimesExtAttr {
    pub attrType: __le32,
    pub attrSubtype: u8,
    pub reserved: [u8; 3],
    pub attrLength: __le32,
    pub dataLength: __le32,
    pub infoTimeExistence: __le32,
    pub infoTimes: [u8; ],
    pub __packed: },
// Device Specification (ECMA 167r3 4/14.10.7)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct deviceSpec {
    pub attrType: __le32,
    pub attrSubtype: u8,
    pub reserved: [u8; 3],
    pub attrLength: __le32,
    pub impUseLength: __le32,
    pub majorDeviceIdent: __le32,
    pub minorDeviceIdent: __le32,
    pub impUse: [u8; ],
    pub __packed: },
// Implementation Use Extended Attr (ECMA 167r3 4/14.10.8)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct impUseExtAttr {
    pub attrType: __le32,
    pub attrSubtype: u8,
    pub reserved: [u8; 3],
    pub attrLength: __le32,
    pub impUseLength: __le32,
    pub impIdent: regid,
    pub impUse: [u8; ],
    pub __packed: },
// Application Use Extended Attribute (ECMA 167r3 4/14.10.9)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct appUseExtAttr {
    pub attrType: __le32,
    pub attrSubtype: u8,
    pub reserved: [u8; 3],
    pub attrLength: __le32,
    pub appUseLength: __le32,
    pub appIdent: regid,
    pub appUse: [u8; ],
    pub __packed: },
pub const EXTATTR_CHAR_SET: c_int = 1;
pub const EXTATTR_ALT_PERMS: c_int = 3;
pub const EXTATTR_FILE_TIMES: c_int = 5;
pub const EXTATTR_INFO_TIMES: c_int = 6;
pub const EXTATTR_DEV_SPEC: c_int = 12;
pub const EXTATTR_IMP_USE: c_int = 2048;
pub const EXTATTR_APP_USE: c_int = 65536;
pub const EXTATTR_SUBTYPE: c_int = 1;
// Unallocated Space Entry (ECMA 167r3 4/14.11)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unallocSpaceEntry {
    pub descTag: tag,
    pub icbTag: icbtag,
    pub lengthAllocDescs: __le32,
    pub allocDescs: [u8; ],
    pub __packed: },
// Space Bitmap Descriptor (ECMA 167r3 4/14.12)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spaceBitmapDesc {
    pub descTag: tag,
    pub numOfBits: __le32,
    pub numOfBytes: __le32,
    pub bitmap: [u8; ],
    pub __packed: },
// Partition Integrity Entry (ECMA 167r3 4/14.13)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct partitionIntegrityEntry {
    pub descTag: tag,
    pub icbTag: icbtag,
    pub recordingDateAndTime: timestamp,
    pub integrityType: u8,
    pub reserved: [u8; 175],
    pub impIdent: regid,
    pub impUse: [u8; 256],
    pub __packed: },
// Short Allocation Descriptor (ECMA 167r3 4/14.14.1)
// Extent Length (ECMA 167r3 4/14.14.1.1)
pub const EXT_LENGTH_MASK: c_uint = 0x3FFFFFFF;
pub const EXT_TYPE_MASK: c_uint = 0xC0000000;
pub const EXT_RECORDED_ALLOCATED: c_uint = 0x00000000;
pub const EXT_NOT_RECORDED_ALLOCATED: c_uint = 0x40000000;
pub const EXT_NOT_RECORDED_NOT_ALLOCATED: c_uint = 0x80000000;
pub const EXT_NEXT_EXTENT_ALLOCDESCS: c_uint = 0xC0000000;
// Long Allocation Descriptor (ECMA 167r3 4/14.14.2)
// Extended Allocation Descriptor (ECMA 167r3 4/14.14.3)
// Logical Volume Header Descriptor (ECMA 167r3 4/14.15)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicalVolHeaderDesc {
    pub uniqueID: __le64,
    pub reserved: [u8; 24],
    pub __packed: },
// Path Component (ECMA 167r3 4/14.16.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pathComponent {
    pub componentType: u8,
    pub lengthComponentIdent: u8,
    pub componentFileVersionNum: __le16,
    pub componentIdent: [dchars; ],
    pub __packed: },
// File Entry (ECMA 167r3 4/14.17)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extendedFileEntry {
    pub descTag: tag,
    pub icbTag: icbtag,
    pub uid: __le32,
    pub gid: __le32,
    pub permissions: __le32,
    pub fileLinkCount: __le16,
    pub recordFormat: u8,
    pub recordDisplayAttr: u8,
    pub recordLength: __le32,
    pub informationLength: __le64,
    pub objectSize: __le64,
    pub logicalBlocksRecorded: __le64,
    pub accessTime: timestamp,
    pub modificationTime: timestamp,
    pub createTime: timestamp,
    pub attrTime: timestamp,
    pub checkpoint: __le32,
    pub reserved: __le32,
    pub extendedAttrICB: long_ad,
    pub streamDirectoryICB: long_ad,
    pub impIdent: regid,
    pub uniqueID: __le64,
    pub lengthExtendedAttr: __le32,
    pub lengthAllocDescs: __le32,
    pub extendedAttr: [u8; ],
// uint8_t		allocDescs[];
    pub __packed: },
