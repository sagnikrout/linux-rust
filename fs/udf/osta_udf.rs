//! Automatically rewritten from C Header to Rust Module
//! Source: fs/udf/osta_udf.h
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
// osta_udf.h
//
// This file is based on OSTA UDF(tm) 2.60 (March 1, 2005)
// http://www.osta.org
//
// Copyright (c) 2001-2004  Ben Fennema
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
// OSTA-UDF defines and structure definitions
//

pub const _OSTA_UDF_H: c_int = 1;
// OSTA CS0 Charspec (UDF 2.60 2.1.2)
pub const UDF_CHAR_SET_TYPE: c_int = 0;

// Entity Identifier (UDF 2.60 2.1.5)
// Identifiers (UDF 2.60 2.1.5.2)
// Implementation Use Extended Attribute (UDF 2.60 3.3.4.5)
// Virtual Allocation Table (UDF 1.50 2.2.10)
// Logical Volume Extended Information (UDF 1.50 Errata, DCN 5003, 3.3.4.5.1.3)
// OS2EA (UDF 1.50 3.3.4.5.3.1)
// MacUniqueIDTable (UDF 1.50 3.3.4.5.4.3)
// MacResourceFork (UDF 1.50 3.3.4.5.4.4)

// Identifier Suffix (UDF 2.60 2.1.5.3)
pub const DOMAIN_FLAGS_HARD_WRITE_PROTECT: c_uint = 0x01;
pub const DOMAIN_FLAGS_SOFT_WRITE_PROTECT: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct domainIdentSuffix {
    pub UDFRevision: __le16,
    pub domainFlags: u8,
    pub reserved: [u8; 5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UDFIdentSuffix {
    pub UDFRevision: __le16,
    pub OSClass: u8,
    pub OSIdentifier: u8,
    pub reserved: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct impIdentSuffix {
    pub OSClass: u8,
    pub OSIdentifier: u8,
    pub impUse: [u8; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct appIdentSuffix {
    pub impUse: [u8; 8],
    pub __packed: },
// Logical Volume Integrity Descriptor (UDF 2.60 2.2.6)
// Implementation Use (UDF 2.60 2.2.6.4)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicalVolIntegrityDescImpUse {
    pub impIdent: regid,
    pub numFiles: __le32,
    pub numDirs: __le32,
    pub minUDFReadRev: __le16,
    pub minUDFWriteRev: __le16,
    pub maxUDFWriteRev: __le16,
    pub impUse: [u8; ],
    pub __packed: },
// Implementation Use Volume Descriptor (UDF 2.60 2.2.7)
// Implementation Use (UDF 2.60 2.2.7.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct impUseVolDescImpUse {
    pub LVICharset: charspec,
    pub logicalVolIdent: [dstring; 128],
    pub LVInfo1: [dstring; 36],
    pub LVInfo2: [dstring; 36],
    pub LVInfo3: [dstring; 36],
    pub impIdent: regid,
    pub impUse: [u8; 128],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udfPartitionMap2 {
    pub partitionMapType: u8,
    pub partitionMapLength: u8,
    pub reserved1: [u8; 2],
    pub partIdent: regid,
    pub volSeqNum: __le16,
    pub partitionNum: __le16,
    pub __packed: },
// Virtual Partition Map (UDF 2.60 2.2.8)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtualPartitionMap {
    pub partitionMapType: u8,
    pub partitionMapLength: u8,
    pub reserved1: [u8; 2],
    pub partIdent: regid,
    pub volSeqNum: __le16,
    pub partitionNum: __le16,
    pub reserved2: [u8; 24],
    pub __packed: },
// Sparable Partition Map (UDF 2.60 2.2.9)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparablePartitionMap {
    pub partitionMapType: u8,
    pub partitionMapLength: u8,
    pub reserved1: [u8; 2],
    pub partIdent: regid,
    pub volSeqNum: __le16,
    pub partitionNum: __le16,
    pub packetLength: __le16,
    pub numSparingTables: u8,
    pub reserved2: [u8; 1],
    pub sizeSparingTable: __le32,
    pub locSparingTable: [__le32; 4],
    pub __packed: },
// Metadata Partition Map (UDF 2.60 2.2.10)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct metadataPartitionMap {
    pub partitionMapType: u8,
    pub partitionMapLength: u8,
    pub reserved1: [u8; 2],
    pub partIdent: regid,
    pub volSeqNum: __le16,
    pub partitionNum: __le16,
    pub metadataFileLoc: __le32,
    pub metadataMirrorFileLoc: __le32,
    pub metadataBitmapFileLoc: __le32,
    pub allocUnitSize: __le32,
    pub alignUnitSize: __le16,
    pub flags: u8,
    pub reserved2: [u8; 5],
    pub __packed: },
// Virtual Allocation Table (UDF 2.60 2.2.11)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtualAllocationTable20 {
    pub lengthHeader: __le16,
    pub lengthImpUse: __le16,
    pub logicalVolIdent: [dstring; 128],
    pub previousVATICBLoc: __le32,
    pub numFiles: __le32,
    pub numDirs: __le32,
    pub minUDFReadRev: __le16,
    pub minUDFWriteRev: __le16,
    pub maxUDFWriteRev: __le16,
    pub reserved: __le16,
    pub impUse: [u8; ],
// __le32	vatEntry[];
    pub __packed: },
pub const ICBTAG_FILE_TYPE_VAT20: c_uint = 0xF8U;
// Sparing Table (UDF 2.60 2.2.12)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparingEntry {
    pub origLocation: __le32,
    pub mappedLocation: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparingTable {
    pub descTag: tag,
    pub sparingIdent: regid,
    pub reallocationTableLen: __le16,
    pub reserved: __le16,
    pub sequenceNum: __le32,
    pub mapEntry: [sparingEntry; ],
    pub __packed: },
// Metadata File (and Metadata Mirror File) (UDF 2.60 2.2.13.1)
pub const ICBTAG_FILE_TYPE_MAIN: c_uint = 0xFA;
pub const ICBTAG_FILE_TYPE_MIRROR: c_uint = 0xFB;
pub const ICBTAG_FILE_TYPE_BITMAP: c_uint = 0xFC;
// struct long_ad ICB - ADImpUse (UDF 2.60 2.2.4.3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct allocDescImpUse {
    pub flags: __le16,
    pub impUse: [u8; 4],
    pub __packed: },
pub const AD_IU_EXT_ERASED: c_uint = 0x0001;
// Real-Time Files (UDF 2.60 6.11)
pub const ICBTAG_FILE_TYPE_REALTIME: c_uint = 0xF9U;
// Implementation Use Extended Attribute (UDF 2.60 3.3.4.5)
// FreeEASpace (UDF 2.60 3.3.4.5.1.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct freeEaSpace {
    pub headerChecksum: __le16,
    pub freeEASpace: [u8; ],
    pub __packed: },
// DVD Copyright Management Information (UDF 2.60 3.3.4.5.1.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DVDCopyrightImpUse {
    pub headerChecksum: __le16,
    pub CGMSInfo: u8,
    pub dataType: u8,
    pub protectionSystemInfo: [u8; 4],
    pub __packed: },
// Logical Volume Extended Information (UDF 1.50 Errata, DCN 5003, 3.3.4.5.1.3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LVExtensionEA {
    pub headerChecksum: __le16,
    pub verificationID: __le64,
    pub numFiles: __le32,
    pub numDirs: __le32,
    pub logicalVolIdent: [dstring; 128],
    pub __packed: },
// Application Use Extended Attribute (UDF 2.60 3.3.4.6)
// FreeAppEASpace (UDF 2.60 3.3.4.6.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct freeAppEASpace {
    pub headerChecksum: __le16,
    pub freeEASpace: [u8; ],
    pub __packed: },
// UDF Defined System Stream (UDF 2.60 3.3.7)

// UDF Defined Non-System Streams (UDF 2.60 3.3.8)

// #define UDF_ID_OS2_EA		"*UDF OS/2 EA"

// Operating System Identifiers (UDF 2.60 6.3)
pub const UDF_OS_CLASS_UNDEF: c_uint = 0x00U;
pub const UDF_OS_CLASS_DOS: c_uint = 0x01U;
pub const UDF_OS_CLASS_OS2: c_uint = 0x02U;
pub const UDF_OS_CLASS_MAC: c_uint = 0x03U;
pub const UDF_OS_CLASS_UNIX: c_uint = 0x04U;
pub const UDF_OS_CLASS_WIN9X: c_uint = 0x05U;
pub const UDF_OS_CLASS_WINNT: c_uint = 0x06U;
pub const UDF_OS_CLASS_OS400: c_uint = 0x07U;
pub const UDF_OS_CLASS_BEOS: c_uint = 0x08U;
pub const UDF_OS_CLASS_WINCE: c_uint = 0x09U;
pub const UDF_OS_ID_UNDEF: c_uint = 0x00U;
pub const UDF_OS_ID_DOS: c_uint = 0x00U;
pub const UDF_OS_ID_OS2: c_uint = 0x00U;
pub const UDF_OS_ID_MAC: c_uint = 0x00U;
pub const UDF_OS_ID_MAX_OSX: c_uint = 0x01U;
pub const UDF_OS_ID_UNIX: c_uint = 0x00U;
pub const UDF_OS_ID_AIX: c_uint = 0x01U;
pub const UDF_OS_ID_SOLARIS: c_uint = 0x02U;
pub const UDF_OS_ID_HPUX: c_uint = 0x03U;
pub const UDF_OS_ID_IRIX: c_uint = 0x04U;
pub const UDF_OS_ID_LINUX: c_uint = 0x05U;
pub const UDF_OS_ID_MKLINUX: c_uint = 0x06U;
pub const UDF_OS_ID_FREEBSD: c_uint = 0x07U;
pub const UDF_OS_ID_NETBSD: c_uint = 0x08U;
pub const UDF_OS_ID_WIN9X: c_uint = 0x00U;
pub const UDF_OS_ID_WINNT: c_uint = 0x00U;
pub const UDF_OS_ID_OS400: c_uint = 0x00U;
pub const UDF_OS_ID_BEOS: c_uint = 0x00U;
pub const UDF_OS_ID_WINCE: c_uint = 0x00U;
