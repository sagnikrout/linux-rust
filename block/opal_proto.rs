//! Automatically rewritten from C Header to Rust Module
//! Source: block/opal_proto.h
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
// Copyright © 2016 Intel Corporation
//
// Authors:
// Rafael Antognolli <rafael.antognolli@intel.com>
// Scott  Bauer      <scott.bauer@intel.com>
//

//
// These constant values come from:
// SPC-4 section
// 6.30 SECURITY PROTOCOL IN command / table 265.
//
// Token defs derived from:
// TCG_Storage_Architecture_Core_Spec_v2.01_r1.00
// 3.2.2 Data Stream Encoding
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_response_token {
    OPAL_DTA_TOKENID_BYTESTRING = 0xe0,
    OPAL_DTA_TOKENID_SINT = 0xe1,
    OPAL_DTA_TOKENID_UINT = 0xe2,
    OPAL_DTA_TOKENID_TOKEN = 0xe3, /* actual token is returned */
    OPAL_DTA_TOKENID_INVALID = 0X0
}

pub const DTAERROR_NO_METHOD_STATUS: c_uint = 0x89;
pub const GENERIC_HOST_SESSION_NUM: c_uint = 0x41;
pub const FIRST_TPER_SESSION_NUM: c_int = 4096;
pub const TPER_SYNC_SUPPORTED: c_uint = 0x01;
// FC_LOCKING features
pub const LOCKING_SUPPORTED_MASK: c_uint = 0x01;
pub const LOCKING_ENABLED_MASK: c_uint = 0x02;
pub const LOCKED_MASK: c_uint = 0x04;
pub const MBR_ENABLED_MASK: c_uint = 0x10;
pub const MBR_DONE_MASK: c_uint = 0x20;
pub const TINY_ATOM_DATA_MASK: c_uint = 0x3F;
pub const TINY_ATOM_SIGNED: c_uint = 0x40;
pub const SHORT_ATOM_ID: c_uint = 0x80;
pub const SHORT_ATOM_BYTESTRING: c_uint = 0x20;
pub const SHORT_ATOM_SIGNED: c_uint = 0x10;
pub const SHORT_ATOM_LEN_MASK: c_uint = 0xF;
pub const MEDIUM_ATOM_ID: c_uint = 0xC0;
pub const MEDIUM_ATOM_BYTESTRING: c_uint = 0x10;
pub const MEDIUM_ATOM_SIGNED: c_uint = 0x8;
pub const MEDIUM_ATOM_LEN_MASK: c_uint = 0x7;
pub const LONG_ATOM_ID: c_uint = 0xe0;
pub const LONG_ATOM_BYTESTRING: c_uint = 0x2;
pub const LONG_ATOM_SIGNED: c_uint = 0x1;
// Derived from TCG Core spec 2.01 Section:
// 3.2.2.1
// Data Type
//
pub const TINY_ATOM_BYTE: c_uint = 0x7F;
pub const SHORT_ATOM_BYTE: c_uint = 0xBF;
pub const MEDIUM_ATOM_BYTE: c_uint = 0xDF;
pub const LONG_ATOM_BYTE: c_uint = 0xE3;
pub const EMPTY_ATOM_BYTE: c_uint = 0xFF;
pub const OPAL_INVAL_PARAM: c_int = 12;
pub const OPAL_MANUFACTURED_INACTIVE: c_uint = 0x08;
pub const OPAL_DISCOVERY_COMID: c_uint = 0x0001;
pub const LOCKING_RANGE_NON_GLOBAL: c_uint = 0x03;
//
// User IDs used in the TCG storage SSCs
// Derived from: TCG_Storage_Architecture_Core_Spec_v2.01_r1.00
// Section: 6.3 Assigned UIDs
//
pub const OPAL_METHOD_LENGTH: c_int = 8;
pub const OPAL_MSID_KEYLEN: c_int = 15;
pub const OPAL_UID_LENGTH_HALF: c_int = 4;
//
// Boolean operators from TCG Core spec 2.01 Section:
// 5.1.3.11
// Table 61
//
pub const OPAL_BOOLEAN_AND: c_int = 0;
pub const OPAL_BOOLEAN_OR: c_int = 1;
pub const OPAL_BOOLEAN_NOT: c_int = 2;
// Enum to index OPALUID array
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_uid {
// users
    OPAL_SMUID_UID,
    OPAL_THISSP_UID,
    OPAL_ADMINSP_UID,
    OPAL_LOCKINGSP_UID,
    OPAL_ENTERPRISE_LOCKINGSP_UID,
    OPAL_ANYBODY_UID,
    OPAL_SID_UID,
    OPAL_ADMIN1_UID,
    OPAL_USER1_UID,
    OPAL_USER2_UID,
    OPAL_PSID_UID,
    OPAL_ENTERPRISE_BANDMASTER0_UID,
    OPAL_ENTERPRISE_ERASEMASTER_UID,
// tables
    OPAL_TABLE_TABLE,
    OPAL_LOCKINGRANGE_GLOBAL,
    OPAL_LOCKINGRANGE_ACE_START_TO_KEY,
    OPAL_LOCKINGRANGE_ACE_RDLOCKED,
    OPAL_LOCKINGRANGE_ACE_WRLOCKED,
    OPAL_MBRCONTROL,
    OPAL_MBR,
    OPAL_AUTHORITY_TABLE,
    OPAL_C_PIN_TABLE,
    OPAL_LOCKING_INFO_TABLE,
    OPAL_ENTERPRISE_LOCKING_INFO_TABLE,
    OPAL_DATASTORE,
    OPAL_LOCKING_TABLE,
// C_PIN_TABLE object ID's
    OPAL_C_PIN_MSID,
    OPAL_C_PIN_SID,
    OPAL_C_PIN_ADMIN1,
// half UID's (only first 4 bytes used)
    OPAL_HALF_UID_AUTHORITY_OBJ_REF,
    OPAL_HALF_UID_BOOLEAN_ACE,
// omitted optional parameter
    OPAL_UID_HEXFF,
}

// Enum for indexing the OPALMETHOD array
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_method {
    OPAL_PROPERTIES,
    OPAL_STARTSESSION,
    OPAL_REVERT,
    OPAL_ACTIVATE,
    OPAL_EGET,
    OPAL_ESET,
    OPAL_NEXT,
    OPAL_EAUTHENTICATE,
    OPAL_GETACL,
    OPAL_GENKEY,
    OPAL_REVERTSP,
    OPAL_GET,
    OPAL_SET,
    OPAL_AUTHENTICATE,
    OPAL_RANDOM,
    OPAL_ERASE,
    OPAL_REACTIVATE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_token {
// Boolean
    OPAL_TRUE = 0x01,
    OPAL_FALSE = 0x00,
    OPAL_BOOLEAN_EXPR = 0x03,
// cellblocks
    OPAL_TABLE = 0x00,
    OPAL_STARTROW = 0x01,
    OPAL_ENDROW = 0x02,
    OPAL_STARTCOLUMN = 0x03,
    OPAL_ENDCOLUMN = 0x04,
    OPAL_VALUES = 0x01,
// table table
    OPAL_TABLE_UID = 0x00,
    OPAL_TABLE_NAME = 0x01,
    OPAL_TABLE_COMMON = 0x02,
    OPAL_TABLE_TEMPLATE = 0x03,
    OPAL_TABLE_KIND = 0x04,
    OPAL_TABLE_COLUMN = 0x05,
    OPAL_TABLE_COLUMNS = 0x06,
    OPAL_TABLE_ROWS = 0x07,
    OPAL_TABLE_ROWS_FREE = 0x08,
    OPAL_TABLE_ROW_BYTES = 0x09,
    OPAL_TABLE_LASTID = 0x0A,
    OPAL_TABLE_MIN = 0x0B,
    OPAL_TABLE_MAX = 0x0C,
// authority table
    OPAL_PIN = 0x03,
// locking tokens
    OPAL_RANGESTART = 0x03,
    OPAL_RANGELENGTH = 0x04,
    OPAL_READLOCKENABLED = 0x05,
    OPAL_WRITELOCKENABLED = 0x06,
    OPAL_READLOCKED = 0x07,
    OPAL_WRITELOCKED = 0x08,
    OPAL_ACTIVEKEY = 0x0A,
// lockingsp table
    OPAL_LIFECYCLE = 0x06,
// locking info table
    OPAL_MAXRANGES = 0x04,
// mbr control
    OPAL_MBRENABLE = 0x01,
    OPAL_MBRDONE = 0x02,
// properties
    OPAL_HOSTPROPERTIES = 0x00,
// atoms
    OPAL_STARTLIST = 0xf0,
    OPAL_ENDLIST = 0xf1,
    OPAL_STARTNAME = 0xf2,
    OPAL_ENDNAME = 0xf3,
    OPAL_CALL = 0xf8,
    OPAL_ENDOFDATA = 0xf9,
    OPAL_ENDOFSESSION = 0xfa,
    OPAL_STARTTRANSACTON = 0xfb,
    OPAL_ENDTRANSACTON = 0xfC,
    OPAL_EMPTYATOM = 0xff,
    OPAL_WHERE = 0x00,
}

// Locking state for a locking range
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_lockingstate {
    OPAL_LOCKING_READWRITE = 0x01,
    OPAL_LOCKING_READONLY = 0x02,
    OPAL_LOCKING_LOCKED = 0x03,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_parameter {
    OPAL_SUM_SET_LIST = 0x060000,
    OPAL_SUM_RANGE_POLICY = 0x060001,
    OPAL_SUM_ADMIN1_PIN = 0x060002,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opal_revertlsp {
    OPAL_KEEP_GLOBAL_RANGE_KEY = 0x060000,
}

// Packets derived from:
// TCG_Storage_Architecture_Core_Spec_v2.01_r1.00
// Secion: 3.2.3 ComPackets, Packets & Subpackets
//
// Comm Packet (header) for transmissions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_compacket {
    pub reserved0: __be32,
    pub extendedComID: [u8; 4],
    pub outstandingData: __be32,
    pub minTransfer: __be32,
    pub length: __be32,
}

// Packet structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_packet {
    pub tsn: __be32,
    pub hsn: __be32,
    pub seq_number: __be32,
    pub reserved0: __be16,
    pub ack_type: __be16,
    pub acknowledgment: __be32,
    pub length: __be32,
}

// Data sub packet header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_data_subpacket {
    pub reserved0: [u8; 6],
    pub kind: __be16,
    pub length: __be32,
}

// header of a response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_header {
    pub cp: opal_compacket,
    pub pkt: opal_packet,
    pub subpkt: opal_data_subpacket,
}

//
// TCG_Storage_Architecture_Core_Spec_v2.01_r1.00
// Section: 3.3.4.7.5 STACK_RESET
//
pub const OPAL_STACK_RESET: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_stack_reset {
    pub extendedComID: [u8; 4],
    pub request_code: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opal_stack_reset_response {
    pub extendedComID: [u8; 4],
    pub request_code: __be32,
    pub reserved0: [u8; 2],
    pub data_length: __be16,
    pub response: __be32,
}

pub const FC_TPER: c_uint = 0x0001;
pub const FC_LOCKING: c_uint = 0x0002;
pub const FC_GEOMETRY: c_uint = 0x0003;
pub const FC_ENTERPRISE: c_uint = 0x0100;
pub const FC_DATASTORE: c_uint = 0x0202;
pub const FC_SINGLEUSER: c_uint = 0x0201;
pub const FC_OPALV100: c_uint = 0x0200;
pub const FC_OPALV200: c_uint = 0x0203;
//
// The Discovery 0 Header. As defined in
// Opal SSC Documentation
// Section: 3.3.5 Capability Discovery
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d0_header {
    pub /: *mut *mut __be32 length; / the length of the header 48 in 2.00.100,
    pub /: *mut *mut *mut __be32 revision; /< revision of the header 1 in 2.00.100,
    pub reserved01: __be32,
    pub reserved02: __be32,
//
// the remainder of the structure is vendor specific and will not be
// addressed now
//
    pub ignored: [u8; 32],
}

//
// TPer Feature Descriptor. Contains flags indicating support for the
// TPer features described in the OPAL specification. The names match the
// OPAL terminology
//
// code == 0x001 in 2.00.100
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d0_tper_features {
//
// supported_features bits:
// bit 7: reserved
// bit 6: com ID management
// bit 5: reserved
// bit 4: streaming support
// bit 3: buffer management
// bit 2: ACK/NACK
// bit 1: async
// bit 0: sync
//
    pub supported_features: u8,
//
// bytes 5 through 15 are reserved, but we represent the first 3 as
// u8 to keep the other two 32bits integers aligned.
//
    pub reserved01: [u8; 3],
    pub reserved02: __be32,
    pub reserved03: __be32,
}

//
// Locking Feature Descriptor. Contains flags indicating support for the
// locking features described in the OPAL specification. The names match the
// OPAL terminology
//
// code == 0x0002 in 2.00.100
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d0_locking_features {
//
// supported_features bits:
// bits 6-7: reserved
// bit 5: MBR done
// bit 4: MBR enabled
// bit 3: media encryption
// bit 2: locked
// bit 1: locking enabled
// bit 0: locking supported
//
    pub supported_features: u8,
//
// bytes 5 through 15 are reserved, but we represent the first 3 as
// u8 to keep the other two 32bits integers aligned.
//
    pub reserved01: [u8; 3],
    pub reserved02: __be32,
    pub reserved03: __be32,
}

//
// Geometry Feature Descriptor. Contains flags indicating support for the
// geometry features described in the OPAL specification. The names match the
// OPAL terminology
//
// code == 0x0003 in 2.00.100
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d0_geometry_features {
//
// skip 32 bits from header, needed to align the struct to 64 bits.
//
    pub header: [u8; 4],
//
// reserved01:
// bits 1-6: reserved
// bit 0: align
//
    pub reserved01: u8,
    pub reserved02: [u8; 7],
    pub logical_block_size: __be32,
    pub alignment_granularity: __be64,
    pub lowest_aligned_lba: __be64,
}

//
// Enterprise SSC Feature
//
// code == 0x0100
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d0_enterprise_ssc {
    pub baseComID: __be16,
    pub numComIDs: __be16,
// range_crossing:
// bits 1-6: reserved
// bit 0: range crossing
//
    pub range_crossing: u8,
    pub reserved01: u8,
    pub reserved02: __be16,
    pub reserved03: __be32,
    pub reserved04: __be32,
}

//
// Opal V1 feature
//
// code == 0x0200
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d0_opal_v100 {
    pub baseComID: __be16,
    pub numComIDs: __be16,
}

//
// Single User Mode feature
//
// code == 0x0201
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d0_single_user_mode {
    pub num_locking_objects: __be32,
// reserved01:
// bit 0: any
// bit 1: all
// bit 2: policy
// bits 3-7: reserved
//
    pub reserved01: u8,
    pub reserved02: u8,
    pub reserved03: __be16,
    pub reserved04: __be32,
}

//
// Additonal Datastores feature
//
// code == 0x0202
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d0_datastore_table {
    pub reserved01: __be16,
    pub max_tables: __be16,
    pub max_size_tables: __be32,
    pub table_size_alignment: __be32,
}

//
// OPAL 2.0 feature
//
// code == 0x0203
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d0_opal_v200 {
    pub baseComID: __be16,
    pub numComIDs: __be16,
// range_crossing:
// bits 1-6: reserved
// bit 0: range crossing
//
    pub range_crossing: u8,
// num_locking_admin_auth:
// not aligned to 16 bits, so use two u8.
// stored in big endian:
// 0: MSB
// 1: LSB
//
    pub num_locking_admin_auth: [u8; 2],
// num_locking_user_auth:
// not aligned to 16 bits, so use two u8.
// stored in big endian:
// 0: MSB
// 1: LSB
//
    pub num_locking_user_auth: [u8; 2],
    pub initialPIN: u8,
    pub revertedPIN: u8,
    pub reserved01: u8,
    pub reserved02: __be32,
}

// Union of features used to parse the discovery 0 response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d0_features {
    pub code: __be16,
//
// r_version bits:
// bits 4-7: version
// bits 0-3: reserved
//
    pub r_version: u8,
    pub length: u8,
    pub features: [u8; ],
}
