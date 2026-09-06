//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs3/ntfs.h
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
// Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
//
// on-disk ntfs structs
//
// clang-format off

// TODO: Check 4K MFT record and 512 bytes cluster.
// Check each run for marked clusters.
// Macro flag: #define NTFS3_CHECK_FREE_CLST
pub const NTFS_NAME_LEN: c_int = 255;
//
// ntfs.sys used 500 maximum links on-disk struct allows up to 0xffff.
// xfstest generic/041 creates 3003 hardlinks.
//
pub const NTFS_LINK_MAX: c_int = 4000;
//
// Activate to use 64 bit clusters instead of 32 bits in ntfs.sys.
// Logical and virtual cluster number if needed, may be
// redefined to use 64 bit value.
//
// #define CONFIG_NTFS3_64BIT_CLUSTER
pub const NTFS_LZNT_MAX_CLUSTER: c_int = 4096;
pub const NTFS_LZNT_CUNIT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GUID {
    pub Data1: __le32,
    pub Data2: __le16,
    pub Data3: __le16,
    pub Data4: [u8; 8],
}

//
// This struct repeats layout of ATTR_FILE_NAME
// at offset 0x40.
// It used to store global constants NAME_MFT/NAME_MIRROR...
// most constant names are shorter than 10.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_str {
    pub len: u8,
    pub ads_len: u8,
    pub name: [u16; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct le_str {
    pub len: u8,
    pub unused: u8,
    pub name: [__le16; ],
}

pub type CLST = u64;

pub type CLST = u32;

// On-disk sparsed cluster is marked as -1.

// Below is virtual (not on-disk) values.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RECORD_NUM {
    MFT_REC_MFT		= 0,
    MFT_REC_MIRR		= 1,
    MFT_REC_LOG		= 2,
    MFT_REC_VOL		= 3,
    MFT_REC_ATTR		= 4,
    MFT_REC_ROOT		= 5,
    MFT_REC_BITMAP		= 6,
    MFT_REC_BOOT		= 7,
    MFT_REC_BADCLUST	= 8,
    MFT_REC_SECURE		= 9,
    MFT_REC_UPCASE		= 10,
    MFT_REC_EXTEND		= 11,
    MFT_REC_RESERVED	= 12,
    MFT_REC_FREE		= 16,
    MFT_REC_USER		= 24,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATTR_TYPE {
    ATTR_ZERO		= cpu_to_le32(0x00),
    ATTR_STD		= cpu_to_le32(0x10),
    ATTR_LIST		= cpu_to_le32(0x20),
    ATTR_NAME		= cpu_to_le32(0x30),
    ATTR_ID			= cpu_to_le32(0x40),
    ATTR_SECURE		= cpu_to_le32(0x50),
    ATTR_LABEL		= cpu_to_le32(0x60),
    ATTR_VOL_INFO		= cpu_to_le32(0x70),
    ATTR_DATA		= cpu_to_le32(0x80),
    ATTR_ROOT		= cpu_to_le32(0x90),
    ATTR_ALLOC		= cpu_to_le32(0xA0),
    ATTR_BITMAP		= cpu_to_le32(0xB0),
    ATTR_REPARSE		= cpu_to_le32(0xC0),
    ATTR_EA_INFO		= cpu_to_le32(0xD0),
    ATTR_EA			= cpu_to_le32(0xE0),
    ATTR_PROPERTYSET	= cpu_to_le32(0xF0),
    ATTR_LOGGED_UTILITY_STREAM = cpu_to_le32(0x100),
    ATTR_END		= cpu_to_le32(0xFFFFFFFF)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FILE_ATTRIBUTE {
    FILE_ATTRIBUTE_READONLY		= cpu_to_le32(0x00000001),
    FILE_ATTRIBUTE_HIDDEN		= cpu_to_le32(0x00000002),
    FILE_ATTRIBUTE_SYSTEM		= cpu_to_le32(0x00000004),
    FILE_ATTRIBUTE_ARCHIVE		= cpu_to_le32(0x00000020),
    FILE_ATTRIBUTE_DEVICE		= cpu_to_le32(0x00000040),
    FILE_ATTRIBUTE_TEMPORARY	= cpu_to_le32(0x00000100),
    FILE_ATTRIBUTE_SPARSE_FILE	= cpu_to_le32(0x00000200),
    FILE_ATTRIBUTE_REPARSE_POINT	= cpu_to_le32(0x00000400),
    FILE_ATTRIBUTE_COMPRESSED	= cpu_to_le32(0x00000800),
    FILE_ATTRIBUTE_OFFLINE		= cpu_to_le32(0x00001000),
    FILE_ATTRIBUTE_NOT_CONTENT_INDEXED = cpu_to_le32(0x00002000),
    FILE_ATTRIBUTE_ENCRYPTED	= cpu_to_le32(0x00004000),
    FILE_ATTRIBUTE_VALID_FLAGS	= cpu_to_le32(0x00007fb7),
    FILE_ATTRIBUTE_DIRECTORY	= cpu_to_le32(0x10000000),
    FILE_ATTRIBUTE_INDEX		= cpu_to_le32(0x20000000)
}

// MFT record number structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MFT_REF {
    pub number.: __le32 low; // The low part of the,
    pub number.: __le16 high; // The high part of the,
    pub record.: __le16 seq; // The sequence number of MFT,
}

extern "C" {
    pub fn le32_to_cpu(32: ref->low) | ((u64)le16_to_cpu(ref->high) <<) -> return;
}

extern "C" {
    pub fn le32_to_cpu(_arg: ref->low) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NTFS_BOOT {
    pub code.: u8 jump_code[3]; // 0x00: Jump to boot,
    pub ": u8 system_id[8]; // 0x03: System ID, equals "NTFS,
// NOTE: This member is not aligned(!)
// bytes_per_sector[0] must be 0.
// bytes_per_sector[1] must be multiplied by 256.
    pub sector.: u8 bytes_per_sector[2]; // 0x0B: Bytes per,
    pub cluster.: u8 sectors_per_clusters;// 0x0D: Sectors per,
    pub unused1: [u8; 7],
    pub harddisk): u8 media_type; // 0x15: Media type (0xF8 -,
    pub unused2: [u8; 2],
    pub track.: __le16 sct_per_track; // 0x18: number of sectors per,
    pub cylinder.: __le16 heads; // 0x1A: number of heads per,
    pub sectors.: __le32 hidden_sectors; // 0x1C: number of 'hidden',
    pub unused3: [u8; 4],
    pub =0x80.: u8 bios_drive_num; // 0x24: BIOS drive number,
    pub unused4: u8,
    pub =0x80.: u8 signature_ex; // 0x26: Extended BOOT signature,
    pub unused5: u8,
    pub sectors.: __le64 sectors_per_volume;// 0x28: Size of volume in,
    pub $MFT: __le64 mft_clst; // 0x30: First cluster of,
    pub $MFTMirr: __le64 mft2_clst; // 0x38: First cluster of,
    pub clusters(sectors).: s8 record_size; // 0x40: Size of MFT record in,
    pub unused6: [u8; 3],
    pub clusters(sectors).: s8 index_size; // 0x44: Size of INDX record in,
    pub unused7: [u8; 3],
    pub number: __le64 serial_num; // 0x48: Volume serial,
    pub all: __le32 check_sum; // 0x50: Simple additive checksum of,
// of the u32's which precede the 'check_sum'.
    pub 0x54:: u8 boot_code[0x200 - 0x50 - 2 - 4]; //,
    pub 0xAA: u8 boot_magic[2]; // 0x1FE: Boot signature =0x55 +,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NTFS_SIGNATURE {
    NTFS_FILE_SIGNATURE = cpu_to_le32(0x454C4946), // 'FILE'
    NTFS_INDX_SIGNATURE = cpu_to_le32(0x58444E49), // 'INDX'
    NTFS_CHKD_SIGNATURE = cpu_to_le32(0x444B4843), // 'CHKD'
    NTFS_RSTR_SIGNATURE = cpu_to_le32(0x52545352), // 'RSTR'
    NTFS_RCRD_SIGNATURE = cpu_to_le32(0x44524352), // 'RCRD'
    NTFS_BAAD_SIGNATURE = cpu_to_le32(0x44414142), // 'BAAD'
    NTFS_HOLE_SIGNATURE = cpu_to_le32(0x454C4F48), // 'HOLE'
    NTFS_FFFF_SIGNATURE = cpu_to_le32(0xffffffff),
}

// MFT Record header structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NTFS_RECORD_HEADER {
// Record magic number, equals 'FILE'/'INDX'/'RSTR'/'RCRD'.
    pub 0x00:: NTFS_SIGNATURE sign; //,
    pub 0x04:: __le16 fix_off; //,
    pub 0x06:: __le16 fix_num; //,
    pub number,: __le64 lsn; // 0x08: Log file sequence,
}

// Possible bits in struct MFT_REC.flags.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RECORD_FLAG {
    RECORD_FLAG_IN_USE	= cpu_to_le16(0x0001),
    RECORD_FLAG_DIR		= cpu_to_le16(0x0002),
    RECORD_FLAG_SYSTEM	= cpu_to_le16(0x0004),
    RECORD_FLAG_INDEX	= cpu_to_le16(0x0008),
}

// MFT Record structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MFT_REC {
    pub 'FILE': NTFS_RECORD_HEADER rhdr; //,
    pub record.: __le16 seq; // 0x10: Sequence number for this,
    pub record.: __le16 hard_links; // 0x12: The number of hard links to,
    pub attributes.: __le16 attr_off; // 0x14: Offset to,
    pub RECORD_FLAG.: __le16 flags; // 0x16: See,
    pub part.: __le32 used; // 0x18: The size of used,
    pub size.: __le32 total; // 0x1C: Total record,
    pub record.: MFT_REF parent_ref; // 0x20: Parent MFT,
    pub Id.: __le16 next_attr_id; // 0x28: The next attribute,
    pub record?: __le16 res; // 0x2A: High part of MFT,
    pub number.: __le32 mft_record; // 0x2C: Current MFT record,
    pub 0x30:: __le16 fixups[]; //,
}

//
// define MFTRECORD_FIXUP_OFFSET as MFTRECORD_FIXUP_OFFSET_3 (0x30)
// to format new mft records with bigger header (as current ntfs.sys does)
//
// define MFTRECORD_FIXUP_OFFSET as MFTRECORD_FIXUP_OFFSET_1 (0x2A)
// to format new mft records with smaller header (as old ntfs.sys did)
// Both variants are valid.
//

// Possible values of ATTR_RESIDENT.flags
pub const RESIDENT_FLAG_INDEXED: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ATTR_RESIDENT {
    pub data.: __le32 data_size; // 0x10: The size of,
    pub data.: __le16 data_off; // 0x14: Offset to,
    pub ).: u8 flags; // 0x16: Resident flags ( 1 - indexed,
    pub 0x17:: u8 res; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ATTR_NONRESIDENT {
    pub segment.: __le64 svcn; // 0x10: Starting VCN of this,
    pub segment.: __le64 evcn; // 0x18: End VCN of this,
    pub runs.: __le16 run_off; // 0x20: Offset to packed,
// Unit of Compression size for this stream, expressed
// as a log of the cluster size.
//
// 0 means file is not compressed
// 1, 2, 3, and 4 are potentially legal values if the
// stream is compressed, however the implementation
// may only choose to use 4, or possibly 3.
// Note that 4 means cluster size time 16.
// If convenient the implementation may wish to accept a
// reasonable range of legal values here (1-5?),
// even if the implementation only generates
// a smaller set of values itself.
    pub 0x22:: u8 c_unit; //,
    pub 0x23:: u8 res1[5]; //,
    pub bytes.: __le64 alloc_size; // 0x28: The allocated size of attribute in,
// (multiple of cluster size)
    pub alloc_size.: __le64 data_size; // 0x30: The size of attribute in bytes <=,
    pub data_size.: __le64 valid_size; // 0x38: The size of valid part in bytes <=,
    pub file.: __le64 total_size; // 0x40: The sum of the allocated clusters for a,
// (present only for the first segment (0 == vcn)
// of compressed attribute)
}

// Possible values of ATTRIB.flags:

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ATTRIB {
    pub attribute.: ATTR_TYPE type; // 0x00: The type of this,
    pub attribute.: __le32 size; // 0x04: The size of this,
    pub non-resident?: u8 non_res; // 0x08: Is this attribute,
    pub length.: u8 name_len; // 0x09: This attribute name,
    pub name.: __le16 name_off; // 0x0A: Offset to the attribute,
    pub ATTR_FLAG_XXX.: __le16 flags; // 0x0C: See,
    pub record).: __le16 id; // 0x0E: Unique id (per,
    pub 0x10: ATTR_RESIDENT res; //,
    pub 0x10: ATTR_NONRESIDENT nres; //,
}

// Define attribute sizes.
pub const SIZEOF_RESIDENT: c_uint = 0x18;
pub const SIZEOF_NONRESIDENT_EX: c_uint = 0x48;
pub const SIZEOF_NONRESIDENT: c_uint = 0x40;

extern "C" {
    pub fn Add2Ptr(_arg: attr, _arg: le16_to_cpu(attr->name_off)) -> return;
}
extern "C" {
    pub fn Add2Ptr(_arg: attr, _arg: off) -> return;
}
extern "C" {
    pub fn Add2Ptr(_arg: attr, _arg: le16_to_cpu(attr->res.data_off)) -> return;
}
extern "C" {
    pub fn Add2Ptr(_arg: attr, _arg: le16_to_cpu(attr->nres.run_off)) -> return;
}
// Standard information attribute (0x10).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ATTR_STD_INFO {
    pub file.: __le64 cr_time; // 0x00: File creation,
    pub time.: __le64 m_time; // 0x08: File modification,
    pub modified.: __le64 c_time; // 0x10: Last time any attribute was,
    pub time.: __le64 a_time; // 0x18: File last access,
    pub more.: FILE_ATTRIBUTE fa; // 0x20: Standard DOS attributes &,
    pub Versions.: __le32 max_ver_num; // 0x24: Maximum Number of,
    pub Number.: __le32 ver_num; // 0x28: Version,
    pub index.: __le32 class_id; // 0x2C: Class Id from bidirectional Class Id,
}

pub const SECURITY_ID_INVALID: c_uint = 0x00000000;
pub const SECURITY_ID_FIRST: c_uint = 0x00000100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ATTR_STD_INFO5 {
    pub file.: __le64 cr_time; // 0x00: File creation,
    pub time.: __le64 m_time; // 0x08: File modification,
    pub modified.: __le64 c_time; // 0x10: Last time any attribute was,
    pub time.: __le64 a_time; // 0x18: File last access,
    pub more.: FILE_ATTRIBUTE fa; // 0x20: Standard DOS attributes &,
    pub Versions.: __le32 max_ver_num; // 0x24: Maximum Number of,
    pub Number.: __le32 ver_num; // 0x28: Version,
    pub index.: __le32 class_id; // 0x2C: Class Id from bidirectional Class Id,
    pub file.: __le32 owner_id; // 0x30: Owner Id of the user owning the,
    pub $SDS.: __le32 security_id; // 0x34: The Security Id is a key in the $SII Index and,
    pub 0x38:: __le64 quota_charge; //,
    pub direct: __le64 usn; // 0x40: Last Update Sequence Number of the file. This is a,
// index into the file $UsnJrnl. If zero, the USN Journal is
// disabled.
}

// Attribute list entry structure (0x20)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ATTR_LIST_ENTRY {
    pub attribute.: ATTR_TYPE type; // 0x00: The type of,
    pub record.: __le16 size; // 0x04: The size of this,
    pub name.: u8 name_len; // 0x06: The length of attribute,
    pub name.: u8 name_off; // 0x07: The offset to attribute,
    pub attribute.: __le64 vcn; // 0x08: Starting VCN of this,
    pub attribute.: MFT_REF ref; // 0x10: MFT record number with,
    pub ID.: __le16 id; // 0x18: struct ATTRIB,
    pub name_off.: __le16 name[]; // 0x1A: To get real name use,
}

extern "C" {
    pub fn sizeof(_arg: short), _arg: 8) -> *mut name_len;
}
// Returns 0 if 'attr' has the same type and name.
extern "C" {
    pub fn Add2Ptr(_arg: le, _arg: le->name_off) -> return;
}
// File name types (the field type in struct ATTR_FILE_NAME).
pub const FILE_NAME_POSIX: c_int = 0;
pub const FILE_NAME_UNICODE: c_int = 1;
pub const FILE_NAME_DOS: c_int = 2;

// Filename attribute structure (0x30).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NTFS_DUP_INFO {
    pub file.: __le64 cr_time; // 0x00: File creation,
    pub time.: __le64 m_time; // 0x08: File modification,
    pub modified.: __le64 c_time; // 0x10: Last time any attribute was,
    pub time.: __le64 a_time; // 0x18: File last access,
    pub size.: __le64 alloc_size; // 0x20: Data attribute allocated size, multiple of cluster,
    pub Dataalloc_size.: __le64 data_size; // 0x28: Data attribute size <=,
    pub more.: FILE_ATTRIBUTE fa; // 0x30: Standard DOS attributes &,
    pub data.: __le32 extend_data; // 0x34: Extended,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ATTR_FILE_NAME {
    pub directory.: MFT_REF home; // 0x00: MFT record for,
    pub 0x08:: NTFS_DUP_INFO dup;//,
    pub words.: u8 name_len; // 0x40: File name length in,
    pub type.: u8 type; // 0x41: File name,
    pub name.: __le16 name[]; // 0x42: File,
}

pub const SIZEOF_ATTRIBUTE_FILENAME: c_uint = 0x44;

// Don't return struct_size(fname, name, fname->name_len);
// Index entry defines ( the field flags in NtfsDirEntry ).

// Directory entry structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NTFS_DE {
    pub file.: MFT_REF ref; // 0x00: MFT record number with this,
    pub 0x00:: __le16 data_off; //,
    pub 0x02:: __le16 data_size; //,
    pub 0.: __le32 res; // 0x04: Must be,
    pub view: },
}

// Here any indexed attribute can be placed.
// One of them is:
// struct ATTR_FILE_NAME AttrFileName;
//
// The last 8 bytes of this structure contains
// the VBN of subnode.
// !!! Note !!!
// This field is presented only if (flags & NTFS_IE_HAS_SUBNODES)
// __le64 vbn;
// v = vcn;
// v = cpu_to_le64(vcn);
extern "C" {
    pub fn le64_to_cpu(_arg: *mut v) -> return;
}
extern "C" {
    pub fn Add2Ptr(_arg: e, _arg: le16_to_cpu(e->size)) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct INDEX_HDR {
    pub structure: __le32 de_off; // 0x00: The offset from the start of this,
// to the first NTFS_DE.
    pub all: __le32 used; // 0x04: The size of this structure plus,
// entries (quad-word aligned).
    pub entries.: __le32 total; // 0x08: The allocated size of for this structure plus all,
    pub directory.: __le32 flags; // 0x0C: 0x00 = Small directory, 0x01 = Large,
//
// de_off + used <= total
//
}

extern "C" {
    pub fn Add2Ptr(_arg: e, _arg: esize) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct INDEX_BUFFER {
    pub 'INDX': NTFS_RECORD_HEADER rhdr; //,
    pub cluster: __le64 vbn; // 0x10: vcn if index >= cluster or vsn id index <,
    pub 0x18:: INDEX_HDR ihdr; //,
}

// Index root structure ( 0x90 ).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum COLLATION_RULE {
    NTFS_COLLATION_TYPE_BINARY	= cpu_to_le32(0),
// $I30
    NTFS_COLLATION_TYPE_FILENAME	= cpu_to_le32(0x01),
// $SII of $Secure and $Q of Quota
    NTFS_COLLATION_TYPE_UINT	= cpu_to_le32(0x10),
// $O of Quota
    NTFS_COLLATION_TYPE_SID		= cpu_to_le32(0x11),
// $SDH of $Secure
    NTFS_COLLATION_TYPE_SECURITY_HASH = cpu_to_le32(0x12),
// $O of ObjId and "$R" for Reparse
    NTFS_COLLATION_TYPE_UINTS	= cpu_to_le32(0x13)
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct INDEX_ROOT {
    pub on.: ATTR_TYPE type; // 0x00: The type of attribute to index,
    pub rule.: COLLATION_RULE rule; // 0x04: The,
    pub record.: __le32 index_block_size;// 0x08: The size of index,
    pub index.: u8 index_block_clst; // 0x0C: The number of clusters or sectors per,
    pub res: [u8; 3],
    pub 0x10:: INDEX_HDR ihdr; //,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VOLUME_INFO {
    pub 0x00: __le64 res1; //,
    pub .): u8 major_ver; // 0x08: NTFS major version number (before,
    pub .): u8 minor_ver; // 0x09: NTFS minor version number (after,
    pub VOLUME_FLAG_XXX: __le16 flags; // 0x0A: Volume flags, see,
}

pub const SIZEOF_ATTRIBUTE_VOLUME_INFO: c_uint = 0xc;

// $AttrDef file entry.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ATTR_DEF_ENTRY {
    pub name.: __le16 name[0x40]; // 0x00: Attr,
    pub type.: ATTR_TYPE type; // 0x80: struct ATTRIB,
    pub 0x84:: __le32 res; //,
    pub 0x88:: COLLATION_RULE rule; //,
    pub above).: __le32 flags; // 0x8C: NTFS_ATTR_XXX (see,
    pub size.: __le64 min_sz; // 0x90: Minimum attribute data,
    pub size.: __le64 max_sz; // 0x98: Maximum attribute data,
}

// Object ID (0x40)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OBJECT_ID {
    pub file.: GUID ObjId; // 0x00: Unique Id assigned to,
// Birth Volume Id is the Object Id of the Volume on.
// which the Object Id was allocated. It never changes.
    pub //0x10:: GUID BirthVolumeId;,
// Birth Object Id is the first Object Id that was
// ever assigned to this MFT Record. I.e. If the Object Id
// is changed for some reason, this field will reflect the
// original value of the Object Id.
    pub 0x20:: GUID BirthObjectId; //,
// Domain Id is currently unused but it is intended to be
// used in a network environment where the local machine is
// part of a Windows 2000 Domain. This may be used in a Windows
// 2000 Advanced Server managed domain.
    pub 0x30:: GUID DomainId; //,
}

// O Directory entry structure ( rule = 0x13 )
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NTFS_DE_O {
    pub de: NTFS_DE,
    pub file.: GUID ObjId; // 0x10: Unique Id assigned to,
    pub file.: MFT_REF ref; // 0x20: MFT record number with this,
// Birth Volume Id is the Object Id of the Volume on
// which the Object Id was allocated. It never changes.
    pub 0x28:: GUID BirthVolumeId; //,
// Birth Object Id is the first Object Id that was
// ever assigned to this MFT Record. I.e. If the Object Id
// is changed for some reason, this field will reflect the
// original value of the Object Id.
// This field is valid if data_size == 0x48.
    pub 0x38:: GUID BirthObjectId; //,
// Domain Id is currently unused but it is intended
// to be used in a network environment where the local
// machine is part of a Windows 2000 Domain. This may be
// used in a Windows 2000 Advanced Server managed domain.
    pub 0x48:: GUID BirthDomainId; //,
}

// Q Directory entry structure ( rule = 0x11 )
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NTFS_DE_Q {
    pub de: NTFS_DE,
    pub file: __le32 owner_id; // 0x10: Unique Id assigned to,
// here is 0x30 bytes of user quota. NOTE: 4 byte aligned!
    pub 0x02: __le32 Version; // 0x14:,
    pub above: __le32 Flags; // 0x18: Quota flags, see,
    pub 0x1C:: __le64 BytesUsed; //,
    pub 0x24:: __le64 ChangeTime; //,
    pub 0x28:: __le64 WarningLimit; //,
    pub 0x34:: __le64 HardLimit; //,
    pub 0x3C:: __le64 ExceededTime; //,
// SID is placed here
    pub 0x44: }__packed; // sizeof() =,
    pub 0x44): static_assert(sizeof(struct NTFS_DE_Q) ==,
pub const SecurityDescriptorsBlockSize: c_uint = 0x40000 // 256K;
pub const SecurityDescriptorMaxSize: c_uint = 0x20000 // 128K;
pub const Log2OfSecurityDescriptorsBlockSize: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SECURITY_KEY {
    pub descriptor: __le32 hash; // Hash value for,
    pub unique): __le32 sec_id; // Security Id (guaranteed,
}

// Security descriptors (the content of $Secure::SDS data stream)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SECURITY_HDR {
    pub Key.: SECURITY_KEY key; // 0x00: Security,
    pub file.: __le64 off; // 0x08: Offset of this entry in the,
    pub aligned.: __le32 size; // 0x10: Size of this entry, 8 byte,
//
// Security descriptor itself is placed here.
// Total size is 16 byte aligned.
//
    pub __packed: },
    pub 0x14): static_assert(sizeof(struct SECURITY_HDR) ==,
// SII Directory entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NTFS_DE_SII {
    pub de: NTFS_DE,
    pub wKeySize: __le32 sec_id; // 0x10: Key: sizeof(security_id) =,
    pub 0x14:: SECURITY_HDR sec_hdr; //,
    pub __packed: },
    pub 0x14): static_assert(offsetof(struct NTFS_DE_SII, sec_hdr) ==,
    pub 0x28): static_assert(sizeof(struct NTFS_DE_SII) ==,
// SDH Directory entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NTFS_DE_SDH {
    pub de: NTFS_DE,
    pub Key: SECURITY_KEY key; // 0x10:,
    pub Data: SECURITY_HDR sec_hdr; // 0x18:,
    pub I": __le16 magic[2]; // 0x2C: 0x00490049 "I,
}

pub const SIZEOF_SDH_DIRENTRY: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct REPARSE_KEY {
    pub Tag: __le32 ReparseTag; // 0x00: Reparse,
    pub file: MFT_REF ref; // 0x04: MFT record number with this,
}

pub const SIZEOF_REPARSE_KEY: c_uint = 0x0C;
// Reparse Directory entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NTFS_DE_R {
    pub de: NTFS_DE,
    pub Key.: REPARSE_KEY key; // 0x10: Reparse,
    pub 0x1c:: u32 zero; //,
}

// CompressReparseBuffer.WofVersion

// CompressReparseBuffer.WofProvider

// CompressReparseBuffer.WofProvider

// CompressReparseBuffer.ProviderVer

//
// ATTR_REPARSE (0xC0)
//
// The reparse struct GUID structure is used by all 3rd party layered drivers to
// store data in a reparse point. For non-Microsoft tags, The struct GUID field
// cannot be GUID_NULL.
// The constraints on reparse tags are defined below.
// Microsoft tags can also be used with this format of the reparse point buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct REPARSE_POINT {
    pub 0x00:: __le32 ReparseTag; //,
    pub 0x04:: __le16 ReparseDataLength;//,
    pub Reserved: __le16,
    pub 0x08:: GUID Guid; //,
//
// Here GenericReparseBuffer is placed
//
}

//
// The value of the following constant needs to satisfy the following
// conditions:
// (1) Be at least as large as the largest of the reserved tags.
// (2) Be strictly smaller than all the tags in use.
//
pub const IO_REPARSE_TAG_RESERVED_RANGE: c_int = 1;
//
// The reparse tags are a ULONG. The 32 bits are laid out as follows:
//
// 3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-----------------------+-------------------------------+
// |M|R|N|R|	  Reserved bits     |	    Reparse Tag Value	    |
// +-+-+-+-+-----------------------+-------------------------------+
//
// M is the Microsoft bit. When set to 1, it denotes a tag owned by Microsoft.
// All ISVs must use a tag with a 0 in this position.
// Note: If a Microsoft tag is used by non-Microsoft software, the
// behavior is not defined.
//
// R is reserved.  Must be zero for non-Microsoft tags.
//
// N is name surrogate. When set to 1, the file represents another named
// entity in the system.
//
// The M and N bits are OR-able.
// The following macros check for the M and N bit values:
//
// Macro to determine whether a reparse point tag corresponds to a tag
// owned by Microsoft.
//

// Macro to determine whether a reparse point tag is a name surrogate.

//
// The following constant represents the bits that are valid to use in
// reparse tags.
//
pub const IO_REPARSE_TAG_VALID_VALUES: c_uint = 0xF000FFFF;
//
// Macro to determine whether a reparse tag is a valid tag.
//

// Microsoft tags for reparse points.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IO_REPARSE_TAG {
    IO_REPARSE_TAG_SYMBOLIC_LINK	= cpu_to_le32(0),
    IO_REPARSE_TAG_NAME_SURROGATE	= cpu_to_le32(0x20000000),
    IO_REPARSE_TAG_MICROSOFT	= cpu_to_le32(0x80000000),
    IO_REPARSE_TAG_MOUNT_POINT	= cpu_to_le32(0xA0000003),
    IO_REPARSE_TAG_SYMLINK		= cpu_to_le32(0xA000000C),
    IO_REPARSE_TAG_HSM		= cpu_to_le32(0xC0000004),
    IO_REPARSE_TAG_SIS		= cpu_to_le32(0x80000007),
    IO_REPARSE_TAG_DEDUP		= cpu_to_le32(0x80000013),
    IO_REPARSE_TAG_COMPRESS		= cpu_to_le32(0x80000017),

//
// The reparse tag 0x80000008 is reserved for Microsoft internal use.
// May be published in the future.
//

// Microsoft reparse tag reserved for DFS
    IO_REPARSE_TAG_DFS	= cpu_to_le32(0x8000000A),

// Microsoft reparse tag reserved for the file system filter manager.
    IO_REPARSE_TAG_FILTER_MANAGER	= cpu_to_le32(0x8000000B),

// Non-Microsoft tags for reparse points

// Tag allocated to CONGRUENT, May 2000. Used by IFSTEST.
    IO_REPARSE_TAG_IFSTEST_CONGRUENT = cpu_to_le32(0x00000009),

// Tag allocated to ARKIVIO.
    IO_REPARSE_TAG_ARKIVIO	= cpu_to_le32(0x0000000C),

// Tag allocated to SOLUTIONSOFT.
    IO_REPARSE_TAG_SOLUTIONSOFT	= cpu_to_le32(0x2000000D),

// Tag allocated to COMMVAULT.
    IO_REPARSE_TAG_COMMVAULT	= cpu_to_le32(0x0000000E),

// OneDrive??
    IO_REPARSE_TAG_CLOUD	= cpu_to_le32(0x9000001A),
    IO_REPARSE_TAG_CLOUD_1	= cpu_to_le32(0x9000101A),
    IO_REPARSE_TAG_CLOUD_2	= cpu_to_le32(0x9000201A),
    IO_REPARSE_TAG_CLOUD_3	= cpu_to_le32(0x9000301A),
    IO_REPARSE_TAG_CLOUD_4	= cpu_to_le32(0x9000401A),
    IO_REPARSE_TAG_CLOUD_5	= cpu_to_le32(0x9000501A),
    IO_REPARSE_TAG_CLOUD_6	= cpu_to_le32(0x9000601A),
    IO_REPARSE_TAG_CLOUD_7	= cpu_to_le32(0x9000701A),
    IO_REPARSE_TAG_CLOUD_8	= cpu_to_le32(0x9000801A),
    IO_REPARSE_TAG_CLOUD_9	= cpu_to_le32(0x9000901A),
    IO_REPARSE_TAG_CLOUD_A	= cpu_to_le32(0x9000A01A),
    IO_REPARSE_TAG_CLOUD_B	= cpu_to_le32(0x9000B01A),
    IO_REPARSE_TAG_CLOUD_C	= cpu_to_le32(0x9000C01A),
    IO_REPARSE_TAG_CLOUD_D	= cpu_to_le32(0x9000D01A),
    IO_REPARSE_TAG_CLOUD_E	= cpu_to_le32(0x9000E01A),
    IO_REPARSE_TAG_CLOUD_F	= cpu_to_le32(0x9000F01A),

}

pub const SYMLINK_FLAG_RELATIVE: c_int = 1;
// Microsoft reparse buffer. (see DDK for details)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct REPARSE_DATA_BUFFER {
    pub 0x00:: __le32 ReparseTag; //,
    pub 0x04:: __le16 ReparseDataLength; //,
    pub Reserved: __le16,
// If ReparseTag == 0xA0000003 (IO_REPARSE_TAG_MOUNT_POINT)
    pub 0x08: __le16 SubstituteNameOffset; //,
    pub 0x0A: __le16 SubstituteNameLength; //,
    pub 0x0C: __le16 PrintNameOffset; //,
    pub 0x0E: __le16 PrintNameLength; //,
    pub 0x10: __le16 PathBuffer[]; //,
    pub MountPointReparseBuffer: },
//
// If ReparseTag == 0xA000000C (IO_REPARSE_TAG_SYMLINK)
// https://msdn.microsoft.com/en-us/library/cc232006.aspx
//
    pub 0x08: __le16 SubstituteNameOffset; //,
    pub 0x0A: __le16 SubstituteNameLength; //,
    pub 0x0C: __le16 PrintNameOffset; //,
    pub 0x0E: __le16 PrintNameLength; //,
// 0-absolute path 1- relative path, SYMLINK_FLAG_RELATIVE
    pub 0x10: __le32 Flags; //,
    pub 0x14: __le16 PathBuffer[]; //,
    pub SymbolicLinkReparseBuffer: },
// If ReparseTag == 0x80000017U
    pub 1: __le32 WofVersion; // 0x08 ==,
//
// 1 - WIM backing provider ("WIMBoot"),
// 2 - System compressed file provider
//
    pub 0x0C:: __le32 WofProvider; //,
    pub 1: __le32 ProviderVer; // 0x10: == 1 WOF_FILE_PROVIDER_CURRENT_VERSION ==,
    pub WOF_COMPRESSION_XXX: __le32 CompressionFormat; // 0x14: 0, 1, 2, 3. See,
    pub CompressReparseBuffer: },
    pub 0x08:: u8 DataBuffer[1]; //,
    pub GenericReparseBuffer: },
}

// ATTR_EA_INFO (0xD0)
pub const FILE_NEED_EA: c_uint = 0x80 // See ntifs.h;
//
// FILE_NEED_EA, indicates that the file to which the EA belongs cannot be
// interpreted without understanding the associated extended attributes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EA_INFO {
    pub form.: __le16 size_pack; // 0x00: Size of buffer to hold in packed,
    pub set.: __le16 count; // 0x02: Count of EA's with FILE_NEED_EA bit,
    pub form.: __le32 size; // 0x04: Size of buffer to hold in unpacked,
}

// ATTR_EA (0xE0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EA_FULL {
    pub packed): __le32 size; // 0x00: (not in,
    pub 0x04:: u8 flags; //,
    pub 0x05:: u8 name_len; //,
    pub 0x06:: __le16 elength; //,
    pub 0x08:: u8 name[]; //,
}

pub const ACL_REVISION: c_int = 2;
pub const ACL_REVISION_DS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SECURITY_DESCRIPTOR_RELATIVE {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: __le16,
    pub Owner: __le32,
    pub Group: __le32,
    pub Sacl: __le32,
    pub Dacl: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_HEADER {
    pub AceType: u8,
    pub AceFlags: u8,
    pub AceSize: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACL {
    pub AclRevision: u8,
    pub Sbz1: u8,
    pub AclSize: __le16,
    pub AceCount: __le16,
    pub Sbz2: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SID {
    pub Revision: u8,
    pub SubAuthorityCount: u8,
    pub IdentifierAuthority: [u8; 6],
    pub SubAuthority: [__le32; ],
}

// clang-format on
