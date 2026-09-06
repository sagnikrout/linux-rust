//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/layout.h
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
// All NTFS associated on-disk structures.
//
// Copyright (c) 2001-2005 Anton Altaparmakov
// Copyright (c) 2002 Richard Russon
//

// The NTFS oem_id "NTFS    "

//
// Location of bootsector on partition:
// The standard NTFS_BOOT_SECTOR is on sector 0 of the partition.
// On NT4 and above there is one backup copy of the boot sector to
// be found on the last sector of the partition (not normally accessible
// from within Windows as the bootsector contained number of sectors
// value is one less than the actual value!).
// On versions of NT 3.51 and earlier, the backup copy was located at
// number of sectors/2 (integer divide), i.e. in the middle of the volume.
//
// BIOS parameter block (bpb) structure.
//
// @bytes_per_sector:       Size of a sector in bytes (usually 512).
// Matches the logical sector size of the underlying device.
// @sectors_per_cluster:    Size of a cluster in sectors (NTFS cluster size / sector size).
// @reserved_sectors:       Number of reserved sectors at the beginning of the volume.
// Always set to 0 in NTFS.
// @fats:                   Number of FAT tables.
// Always 0 in NTFS (no FAT tables exist).
// @root_entries:           Number of entries in the root directory.
// Always 0 in NTFS.
// @sectors:                Total number of sectors on the volume.
// Always 0 in NTFS (use @large_sectors instead).
// @media_type:             Media descriptor byte.
// 0xF8 for hard disk (fixed media) in NTFS.
// @sectors_per_fat:        Number of sectors per FAT table.
// Always 0 in NTFS.
// @sectors_per_track:      Number of sectors per track.
// Irrelevant for NTFS.
// @heads:                  Number of heads (CHS geometry).
// Irrelevant for NTFS.
// @hidden_sectors:         Number of hidden sectors before the start of the partition.
// Always 0 in NTFS boot sector.
// @large_sectors:          Total number of sectors on the volume.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bios_parameter_block {
    pub bytes_per_sector: __le16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: __le16,
    pub fats: u8,
    pub root_entries: __le16,
    pub sectors: __le16,
    pub media_type: u8,
    pub sectors_per_fat: __le16,
    pub sectors_per_track: __le16,
    pub heads: __le16,
    pub hidden_sectors: __le32,
    pub large_sectors: __le32,
    pub __packed: },
//
// NTFS boot sector structure.
//
// @jump:               3-byte jump instruction to boot code (irrelevant for NTFS).
// Typically 0xEB 0x52 0x90 or similar.
// @oem_id:             OEM identifier string (8 bytes).
// Always "NTFS    " (with trailing spaces) in NTFS volumes.
// @bpb:                Legacy BIOS Parameter Block (see struct bios_parameter_block).
// Mostly zeroed or set to fixed values for NTFS compatibility.
// @unused:             4 bytes, reserved/unused.
// NTFS disk editors show it as:
// - physical_drive (0x80 for fixed disk)
// - current_head (0)
// - extended_boot_signature (0x80 or 0x28)
// - unused (0)
// Always zero in practice for NTFS.
// @number_of_sectors:  Number of sectors in volume. Gives maximum volume
// size of 2^63 sectors. Assuming standard sector
// size of 512 bytes, the maximum byte size is
// approx. 4.7x10^21 bytes. (-;
// @mft_lcn:            Logical cluster number (LCN) of the $MFT data attribute.
// Location of the Master File Table.
// @mftmirr_lcn:        LCN of the $MFTMirr (first 3-4 MFT records copy).
// Mirror for boot-time recovery.
// @clusters_per_mft_record:
// Size of each MFT record in clusters.
// @reserved0:          3 bytes, reserved/zero.
// @clusters_per_index_record:
// Size of each index block/record in clusters.
// @reserved1:          3 bytes, reserved/zero.
// @volume_serial_number:
// 64-bit volume serial number.
// Used for identification (irrelevant for NTFS operation).
// @checksum:           32-bit checksum of the boot sector (excluding this field).
// Used to detect boot sector corruption.
// @bootstrap:          426 bytes of bootstrap code.
// Irrelevant for NTFS (contains x86 boot loader stub).
// @end_of_sector_marker:
// 2-byte end-of-sector signature.
// Always 0xAA55 (little-endian magic number).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_boot_sector {
    pub jump: [u8; 3],
    pub oem_id: __le64,
    pub bpb: bios_parameter_block,
    pub unused: [u8; 4],
    pub number_of_sectors: __le64,
    pub mft_lcn: __le64,
    pub mftmirr_lcn: __le64,
    pub clusters_per_mft_record: i8,
    pub reserved0: [u8; 3],
    pub clusters_per_index_record: i8,
    pub reserved1: [u8; 3],
    pub volume_serial_number: __le64,
    pub checksum: __le32,
    pub bootstrap: [u8; 426],
    pub end_of_sector_marker: __le16,
    pub __packed: },
    pub 512): static_assert(sizeof(struct ntfs_boot_sector) ==,
//
// Magic identifiers present at the beginning of all ntfs record containing
// records (like mft records for example).
//
// magic_FILE:      MFT entry header ("FILE" in ASCII).
// Used in $MFT/$DATA for all master file table records.
// magic_INDX:      Index buffer header ("INDX" in ASCII).
// Used in $INDEX_ALLOCATION attributes (directories, $I30 indexes).
// magic_HOLE:      Hole marker ("HOLE" in ASCII).
// Introduced in NTFS 3.0+, used for sparse/hole regions in some contexts.
// magic_RSTR:      Restart page header ("RSTR" in ASCII).
// Used in LogFile for restart pages (transaction log recovery).
// magic_RCRD:      Log record page header ("RCRD" in ASCII).
// Used in LogFile for individual log record pages.
// magic_CHKD:      Chkdsk modified marker ("CHKD" in ASCII).
// Set by chkdsk when it modifies a record; indicates repair was done.
// magic_BAAD:      Bad record marker ("BAAD" in ASCII).
// Indicates a multi-sector transfer failure was detected.
// The record is corrupted/unusable; often set during I/O errors.
// magic_empty:     Empty/uninitialized page marker (0xffffffff).
// Used in LogFile when a page is filled with 0xff bytes
// and has not yet been initialized. Must be formatted before use.
//
}

//
// Generic magic comparison macros. Finally found a use for the ## preprocessor
// operator! (-8
//

//
// Specialised magic comparison macros for the NTFS_RECORD_TYPEs defined above.
//

//
// struct ntfs_record - Common header for all multi-sector protected NTFS records
//
// @magic:      4-byte magic identifier for the record type and/or status.
// Common values are defined in the magic_* enum (FILE, INDX, RSTR,
// RCRD, CHKD, BAAD, HOLE, empty).
// - "FILE" = MFT record
// - "INDX" = Index allocation block
// - "BAAD" = Record corrupted (multi-sector fixup failed)
// - 0xffffffff = Uninitialized/empty page
// @usa_ofs:    Offset (in bytes) from the start of this record to the Update
// Sequence Array (USA).
// The USA is located at record + usa_ofs.
// @usa_count:  Number of 16-bit entries in the USA array (including the Update
// Sequence Number itself).
// - Number of fixup locations = usa_count - 1
// - Each fixup location is a 16-bit value in the record that needs
// protection against torn writes.
//
// The Update Sequence Array (usa) is an array of the __le16 values which belong
// to the end of each sector protected by the update sequence record in which
// this array is contained. Note that the first entry is the Update Sequence
// Number (usn), a cyclic counter of how many times the protected record has
// been written to disk. The values 0 and -1 (ie. 0xffff) are not used. All
// last le16's of each sector have to be equal to the usn (during reading) or
// are set to it (during writing). If they are not, an incomplete multi sector
// transfer has occurred when the data was written.
// The maximum size for the update sequence array is fixed to:
// maximum size = usa_ofs + (usa_count * 2) = 510 bytes
// The 510 bytes comes from the fact that the last __le16 in the array has to
// (obviously) finish before the last __le16 of the first 512-byte sector.
// This formula can be used as a consistency check in that usa_ofs +
// (usa_count * 2) has to be less than or equal to 510.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_record {
    pub magic: __le32,
    pub usa_ofs: __le16,
    pub usa_count: __le16,
    pub __packed: },
//
// System files mft record numbers. All these files are always marked as used
// in the bitmap attribute of the mft; presumably in order to avoid accidental
// allocation for random other mft records. Also, the sequence number for each
// of the system files is always equal to their mft record number and it is
// never modified.
//
// FILE_MFT:        Master File Table (MFT) itself.
// Data attribute contains all MFT entries;
// Bitmap attribute tracks which records are in use (bit==1).
// FILE_MFTMirr:    MFT mirror: copy of the first four (or more) MFT records
// in its data attribute.
// If cluster size > 4 KiB, copies first N records where
// N = cluster_size / mft_record_size.
// FILE_LogFile:    Journaling log (LogFile) in data attribute.
// Used for transaction logging and recovery.
// FILE_Volume:     Volume information and name.
// Contains $VolumeName (label) and $VolumeInformation
// (flags, NTFS version). Windows calls this the volume DASD.
// FILE_AttrDef:    Attribute definitions array in data attribute.
// Defines all possible attribute types and their properties.
// FILE_root:       Root directory ($Root).
// The top-level directory of the filesystem.
// FILE_Bitmap:     Cluster allocation bitmap ($Bitmap) in data attribute.
// Tracks free/used clusters (LCNs) on the volume.
// FILE_Boot:       Boot sector ($Boot) in data attribute.
// Always located at cluster 0; contains BPB and NTFS parameters.
// FILE_BadClus:    Bad cluster list ($BadClus) in non-resident data attribute.
// Marks all known bad clusters.
// FILE_Secure:     Security descriptors ($Secure).
// Contains shared $SDS (security descriptors) and two indexes
// ($SDH, $SII). Introduced in Windows 2000.
// Before that, it was called $Quota but was unused.
// FILE_UpCase:     Uppercase table ($UpCase) in data attribute.
// Maps all 65536 Unicode characters to their uppercase forms.
// FILE_Extend:     System directory ($Extend).
// Contains additional system files ($ObjId, $Quota, $Reparse,
// $UsnJrnl, etc.). Introduced in NTFS 3.0 (Windows 2000).
// FILE_reserved12: Reserved for future use (MFT records 12–15).
// FILE_reserved13: Reserved.
// FILE_reserved14: Reserved.
// FILE_reserved15: Reserved.
// FILE_first_user: First possible user-created file MFT record number.
// Used as a boundary to distinguish system files from user files.
//
}

//
// enum - Flags for MFT record header
//
// These are the so far known MFT_RECORD_* flags (16-bit) which contain
// information about the mft record in which they are present.
//
// MFT_RECORD_IN_USE:        This MFT record is allocated and in use.
// (bit set = record is valid/used; clear = free)
// MFT_RECORD_IS_DIRECTORY:  This MFT record represents a directory.
// (Used to quickly distinguish files from directories)
// MFT_RECORD_IS_4:          Indicates the record is a special "record 4" type.
// (Rarely used; related to NTFS internal special cases,
// often for $AttrDef or early system files)
// MFT_RECORD_IS_VIEW_INDEX: This MFT record is used as a view index.
// (Specific to NTFS indexed views or object ID indexes)
// MFT_REC_SPACE_FILLER:     Dummy value to force the enum to be 16-bit wide.
// (Not a real flag; just a sentinel to ensure the type
// is __le16 and no higher bits are accidentally used)
//
// mft references (aka file references or file record segment references) are
// used whenever a structure needs to refer to a record in the mft.
//
// A reference consists of a 48-bit index into the mft and a 16-bit sequence
// number used to detect stale references.
//
// For error reporting purposes we treat the 48-bit index as a signed quantity.
//
// The sequence number is a circular counter (skipping 0) describing how many
// times the referenced mft record has been (re)used. This has to match the
// sequence number of the mft record being referenced, otherwise the reference
// is considered stale and removed.
//
// If the sequence number is zero it is assumed that no sequence number
// consistency checking should be performed.
//
// Define two unpacking macros to get to the reference (MREF) and
// sequence number (MSEQNO) respectively.
// The _LE versions are to be applied on little endian MFT_REFs.
// Note: The _LE versions will return a CPU endian formatted value!
//
pub const MFT_REF_MASK_CPU: c_uint = 0x0000ffffffffffffULL;

//
// struct mft_record - NTFS Master File Table (MFT) record header
//
// The mft record header present at the beginning of every record in the mft.
// This is followed by a sequence of variable length attribute records which
// is terminated by an attribute of type AT_END which is a truncated attribute
// in that it only consists of the attribute type code AT_END and none of the
// other members of the attribute structure are present.
//
// magic:               Record magic ("FILE" for valid MFT entries).
// See ntfs_record magic enum for other values.
// usa_ofs:             Offset to Update Sequence Array (see ntfs_record).
// usa_count:           Number of entries in USA (see ntfs_record).
// lsn:                 Log sequence number (LSN) from LogFile.
// Incremented on every modification to this record.
// sequence_number:     Reuse count of this MFT record slot.
// Incremented (skipping zero) when the file is deleted.
// Zero means never reused or special case.
// Part of MFT reference (together with record number).
// link_count:          Number of hard links (directory entries) to this file.
// Only meaningful in base MFT records.
// When deleting a directory entry:
// - If link_count == 1, delete the whole file
// - Else remove only the $FILE_NAME attribute and decrement
// attrs_offset:        Byte offset from start of MFT record to first attribute.
// Must be 8-byte aligned.
// flags:               Bit array of MFT_RECORD_* flags (see MFT_RECORD_IN_USE enum).
// MFT_RECORD_IN_USE cleared when record is freed/deleted.
// bytes_in_use:        Number of bytes actually used in this MFT record.
// Must be 8-byte aligned.
// Includes header + all attributes + padding.
// bytes_allocated:     Total allocated size of this MFT record.
// Usually equal to MFT record size (1024 bytes or cluster size).
// base_mft_record:     MFT reference to the base record.
// 0 for base records.
// Non-zero for extension records → points to base record
// containing the $ATTRIBUTE_LIST that describes this extension.
// next_attr_instance:  Next attribute instance number to assign.
// Incremented after each use.
// Reset to 0 when MFT record is reused.
// First instance is always 0.
// reserved:            Reserved for alignment (NTFS 3.1+).
// mft_record_number:   This MFT record's number (index in $MFT).
// Only present in NTFS 3.1+ (Windows XP and above).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mft_record {
    pub magic: __le32,
    pub usa_ofs: __le16,
    pub usa_count: __le16,
    pub lsn: __le64,
    pub sequence_number: __le16,
    pub link_count: __le16,
    pub attrs_offset: __le16,
    pub flags: __le16,
    pub bytes_in_use: __le32,
    pub bytes_allocated: __le32,
    pub base_mft_record: __le64,
    pub next_attr_instance: __le16,
    pub reserved: __le16,
    pub mft_record_number: __le32,
    pub __packed: },
    pub 48): static_assert(sizeof(struct mft_record) ==,
// x
// struct mft_record_old - Old NTFS MFT record header (pre-NTFS 3.1 / Windows XP)
//
// This is the older version of the MFT record header used in NTFS versions
// prior to 3.1 (Windows XP and later). It lacks the additional fields
// @reserved and @mft_record_number that were added in NTFS 3.1+.
//
// @magic:              Record magic ("FILE" for valid MFT entries).
// See ntfs_record magic enum for other values.
// @usa_ofs:            Offset to Update Sequence Array (see ntfs_record).
// @usa_count:          Number of entries in USA (see ntfs_record).
// @lsn:                Log sequence number (LSN) from LogFile.
// Incremented on every modification to this record.
// @sequence_number:    Reuse count of this MFT record slot.
// Incremented (skipping zero) when the file is deleted.
// Zero means never reused or special case.
// Part of MFT reference (together with record number).
// @link_count:         Number of hard links (directory entries) to this file.
// Only meaningful in base MFT records.
// When deleting a directory entry:
// - If link_count == 1, delete the whole file
// - Else remove only the $FILE_NAME attribute and decrement
// @attrs_offset:       Byte offset from start of MFT record to first attribute.
// Must be 8-byte aligned.
// @flags:              Bit array of MFT_RECORD_* flags (see MFT_RECORD_IN_USE enum).
// MFT_RECORD_IN_USE cleared when record is freed/deleted.
// @bytes_in_use:       Number of bytes actually used in this MFT record.
// Must be 8-byte aligned.
// Includes header + all attributes + padding.
// @bytes_allocated:    Total allocated size of this MFT record.
// Usually equal to MFT record size (1024 bytes or cluster size).
// @base_mft_record:    MFT reference to the base record.
// 0 for base records.
// Non-zero for extension records → points to base record
// containing the $ATTRIBUTE_LIST that describes this extension.
// @next_attr_instance: Next attribute instance number to assign.
// Incremented after each use.
// Reset to 0 when MFT record is reused.
// First instance is always 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mft_record_old {
    pub magic: __le32,
    pub usa_ofs: __le16,
    pub usa_count: __le16,
    pub lsn: __le64,
    pub sequence_number: __le16,
    pub link_count: __le16,
    pub attrs_offset: __le16,
    pub flags: __le16,
    pub bytes_in_use: __le32,
    pub bytes_allocated: __le32,
    pub base_mft_record: __le64,
    pub next_attr_instance: __le16,
    pub __packed: },
    pub 42): static_assert(sizeof(struct mft_record_old) ==,
//
// System defined attributes (32-bit).  Each attribute type has a corresponding
// attribute name (Unicode string of maximum 64 character length) as described
// by the attribute definitions present in the data attribute of the $AttrDef
// system file.  On NTFS 3.0 volumes the names are just as the types are named
// in the below defines exchanging AT_ for the dollar sign ($).  If that is not
// a revealing choice of symbol I do not know what is... (-;
//
}

//
// The collation rules for sorting views/indexes/etc (32-bit).
//
// COLLATION_BINARY - Collate by binary compare where the first byte is most
// significant.
// COLLATION_UNICODE_STRING - Collate Unicode strings by comparing their binary
// Unicode values, except that when a character can be uppercased, the
// upper case value collates before the lower case one.
// COLLATION_FILE_NAME - Collate file names as Unicode strings. The collation
// is done very much like COLLATION_UNICODE_STRING. In fact I have no idea
// what the difference is. Perhaps the difference is that file names
// would treat some special characters in an odd way (see
// unistr.c::ntfs_collate_names() and unistr.c::legal_ansi_char_array[]
// for what I mean but COLLATION_UNICODE_STRING would not give any special
// treatment to any characters at all, but this is speculation.
// COLLATION_NTOFS_ULONG - Sorting is done according to ascending __le32 key
// values. E.g. used for $SII index in FILE_Secure, which sorts by
// security_id (le32).
// COLLATION_NTOFS_SID - Sorting is done according to ascending SID values.
// E.g. used for $O index in FILE_Extend/$Quota.
// COLLATION_NTOFS_SECURITY_HASH - Sorting is done first by ascending hash
// values and second by ascending security_id values. E.g. used for $SDH
// index in FILE_Secure.
// COLLATION_NTOFS_ULONGS - Sorting is done according to a sequence of ascending
// __le32 key values. E.g. used for $O index in FILE_Extend/$ObjId, which
// sorts by object_id (16-byte), by splitting up the object_id in four
// __le32 values and using them as individual keys. E.g. take the following
// two security_ids, stored as follows on disk:
// 1st: a1 61 65 b7 65 7b d4 11 9e 3d 00 e0 81 10 42 59
// 2nd: 38 14 37 d2 d2 f3 d4 11 a5 21 c8 6b 79 b1 97 45
// To compare them, they are split into four __le32 values each, like so:
// 1st: 0xb76561a1 0x11d47b65 0xe0003d9e 0x59421081
// 2nd: 0xd2371438 0x11d4f3d2 0x6bc821a5 0x4597b179
// Now, it is apparent why the 2nd object_id collates after the 1st: the
// first __le32 value of the 1st object_id is less than the first __le32 of
// the 2nd object_id. If the first __le32 values of both object_ids were
// equal then the second __le32 values would be compared, etc.
//
// enum - Attribute definition flags
//
// The flags (32-bit) describing attribute properties in the attribute
// definition structure.
// The INDEXABLE flag is fairly certainly correct as only the file
// name attribute has this flag set and this is the only attribute indexed in
// NT4.
//
// ATTR_DEF_INDEXABLE:      Attribute can be indexed.
// (Used for creating indexes like $I30, $SDH, etc.)
// ATTR_DEF_MULTIPLE:       Attribute type can be present multiple times
// in the MFT record of an inode.
// (e.g., multiple $FILE_NAME, $DATA streams)
// ATTR_DEF_NOT_ZERO:       Attribute value must contain at least one non-zero byte.
// (Prevents empty or all-zero values)
// ATTR_DEF_INDEXED_UNIQUE: Attribute must be indexed and the value must be unique
// for this attribute type across all MFT records of an inode.
// (e.g., security descriptor IDs in $Secure)
// ATTR_DEF_NAMED_UNIQUE:   Attribute must be named and the name must be unique
// for this attribute type across all MFT records of an inode.
// (e.g., named $DATA streams or alternate data streams)
// ATTR_DEF_RESIDENT:       Attribute must be resident (stored in MFT record).
// (Cannot be non-resident/sparse/compressed)
// ATTR_DEF_ALWAYS_LOG:     Always log modifications to this attribute in LogFile,
// regardless of whether it is resident or non-resident.
// Without this flag, modifications are logged only if resident.
// (Used for critical metadata attributes)
//
// struct attr_def - Attribute definition entry ($AttrDef array)
//
// The data attribute of FILE_AttrDef contains a sequence of attribute
// definitions for the NTFS volume. With this, it is supposed to be safe for an
// older NTFS driver to mount a volume containing a newer NTFS version without
// damaging it (that's the theory. In practice it's: not damaging it too much).
// Entries are sorted by attribute type. The flags describe whether the
// attribute can be resident/non-resident and possibly other things, but the
// actual bits are unknown.
//
// @name:           Unicode (UTF-16LE) name of the attribute (e.g. "$DATA", "$FILE_NAME").
// Zero-terminated string, maximum 0x40 characters (128 bytes).
// Used for human-readable display and debugging.
// @type:           Attribute type code (ATTR_TYPE_* constants).
// Defines which attribute this entry describes.
// @display_rule:   Default display rule (usually 0; rarely used in modern NTFS).
// Controls how the attribute is displayed in tools (legacy).
// @collation_rule: Default collation rule for indexing this attribute.
// Determines sort order when indexed (e.g. CASE_SENSITIVE, UNICODE).
// Used in $I30, $SDH, $SII, etc.
// @flags:          Bit array of attribute constraints (ATTR_DEF_* flags).
// See ATTR_DEF_INDEXABLE, ATTR_DEF_MULTIPLE, etc.
// Defines whether the attribute can be indexed, multiple, resident-only, etc.
// @min_size:       Optional minimum size of the attribute value (in bytes).
// 0 means no minimum enforced.
// @max_size:       Maximum allowed size of the attribute value (in bytes).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct attr_def {
    pub name: [__le16; 0x40],
    pub type: __le32,
    pub display_rule: __le32,
    pub collation_rule: __le32,
    pub flags: __le32,
    pub min_size: __le64,
    pub max_size: __le64,
    pub __packed: },
    pub 160): static_assert(sizeof(struct attr_def) ==,
//
// enum - Attribute flags (16-bit) for non-resident attributes
//
// ATTR_IS_COMPRESSED:      Attribute is compressed.
// If set, data is compressed using the method in
// ATTR_COMPRESSION_MASK.
// ATTR_COMPRESSION_MASK:   Mask for compression method.
// Valid values are defined in NTFS compression types
// (e.g., 0x02 = LZNT1, etc.).
// Also serves as the first illegal value for method.
// ATTR_IS_ENCRYPTED:       Attribute is encrypted.
// Data is encrypted using EFS (Encrypting File System).
// ATTR_IS_SPARSE:          Attribute is sparse.
// Contains holes (unallocated regions) that read as zeros.
//
    pub __packed: },
//
// Attribute compression.
//
// Only the data attribute is ever compressed in the current ntfs driver in
// Windows. Further, compression is only applied when the data attribute is
// non-resident. Finally, to use compression, the maximum allowed cluster size
// on a volume is 4kib.
//
// The compression method is based on independently compressing blocks of X
// clusters, where X is determined from the compression_unit value found in the
// non-resident attribute record header (more precisely: X = 2^compression_unit
// clusters). On Windows NT/2k, X always is 16 clusters (compression_unit = 4).
//
// There are three different cases of how a compression block of X clusters
// can be stored:
//
// 1) The data in the block is all zero (a sparse block):
// This is stored as a sparse block in the runlist, i.e. the runlist
// entry has length = X and lcn = -1. The mapping pairs array actually
// uses a delta_lcn value length of 0, i.e. delta_lcn is not present at
// all, which is then interpreted by the driver as lcn = -1.
// NOTE: Even uncompressed files can be sparse on NTFS 3.0 volumes, then
// the same principles apply as above, except that the length is not
// restricted to being any particular value.
//
// 2) The data in the block is not compressed:
// This happens when compression doesn't reduce the size of the block
// in clusters. I.e. if compression has a small effect so that the
// compressed data still occupies X clusters, then the uncompressed data
// is stored in the block.
// This case is recognised by the fact that the runlist entry has
// length = X and lcn >= 0. The mapping pairs array stores this as
// normal with a run length of X and some specific delta_lcn, i.e.
// delta_lcn has to be present.
//
// 3) The data in the block is compressed:
// The common case. This case is recognised by the fact that the run
// list entry has length L < X and lcn >= 0. The mapping pairs array
// stores this as normal with a run length of X and some specific
// delta_lcn, i.e. delta_lcn has to be present. This runlist entry is
// immediately followed by a sparse entry with length = X - L and
// lcn = -1. The latter entry is to make up the vcn counting to the
// full compression block size X.
//
// In fact, life is more complicated because adjacent entries of the same type
// can be coalesced. This means that one has to keep track of the number of
// clusters handled and work on a basis of X clusters at a time being one
// block. An example: if length L > X this means that this particular runlist
// entry contains a block of length X and part of one or more blocks of length
// L - X. Another example: if length L < X, this does not necessarily mean that
// the block is compressed as it might be that the lcn changes inside the block
// and hence the following runlist entry describes the continuation of the
// potentially compressed block. The block would be compressed if the
// following runlist entry describes at least X - L sparse clusters, thus
// making up the compression block length as described in point 3 above. (Of
// course, there can be several runlist entries with small lengths so that the
// sparse entry does not follow the first data containing entry with
// length < X.)
//
// NOTE: At the end of the compressed attribute value, there most likely is not
// just the right amount of data to make up a compression block, thus this data
// is not even attempted to be compressed. It is just stored as is, unless
// the number of clusters it occupies is reduced when compressed in which case
// it is stored as a compressed compression block, complete with sparse
// clusters at the end.
//
// enum - Flags for resident attributes (8-bit)
//
// RESIDENT_ATTR_IS_INDEXED: Attribute is referenced in an index.
// (e.g., part of an index key or entry)
// Has implications for deletion and modification:
// - Cannot be freely removed if indexed
// - Index must be updated when value changes
// - Used for attributes like $FILE_NAME in directories
//
    pub __packed: },
//
// struct attr_record - NTFS attribute record header
//
// Common header for both resident and non-resident attributes.
// Always aligned to an 8-byte boundary on disk.
// Located at attrs_offset in the MFT record (see struct mft_record).
//
// @type:           32-bit attribute type (ATTR_TYPE_* constants).
// Identifies the attribute
// (e.g. 0x10 = $STANDARD_INFORMATION).
// @length:         Total byte size of this attribute record (resident).
// 8-byte aligned; used to locate the next attribute.
// @non_resident:   0 = resident attribute
// 1 = non-resident attribute
// @name_length:    Number of Unicode characters in the attribute name.
// 0 if unnamed (most system attributes are unnamed).
// @name_offset:    Byte offset from start of attribute record to the name.
// 8-byte aligned; when creating, place at end of header.
// @flags:          Attribute flags (see ATTR_IS_COMPRESSED,
// ATTR_IS_ENCRYPTED, etc.).
// For resident: see RESIDENT_ATTR_* flags.
// @instance:       Unique instance number within this MFT record.
// Incremented via next_attr_instance; unique per record.
//
// Resident attributes (when @non_resident == 0):
// @data.resident.value_length:     Byte size of the attribute value.
// @data.resident.value_offset:     Byte offset from start of attribute
// record to the value data.
// 8-byte aligned if name present.
// @data.resident.flags:            Resident-specific flags
// @data.resident.reserved:         Reserved/alignment to 8 bytes.
//
// Non-resident attributes (when @non_resident == 1):
// @data.non_resident.lowest_vcn:   Lowest valid VCN in this extent.
// Usually 0 unless attribute list is used.
// @data.non_resident.highest_vcn:  Highest valid VCN in this extent.
// -1 for zero-length, 0 for single extent.
// @data.non_resident.mapping_pairs_offset:
// Byte offset to mapping pairs array
// (VCN → LCN mappings).
// 8-byte aligned when creating.
// @data.non_resident.compression_unit:
// Log2 of clusters per compression unit.
// 0 = not compressed.
// WinNT4 used 4; sparse files use 0
// on XP SP2+.
// @data.non_resident.reserved:     5 bytes for 8-byte alignment.
// @data.non_resident.allocated_size:
// Allocated disk space in bytes.
// For compressed: logical allocated size.
// @data.non_resident.data_size:    Logical attribute value size in bytes.
// Can be larger than allocated_size if
// compressed/sparse.
// @data.non_resident.initialized_size:
// Initialized portion size in bytes.
// Usually equals data_size.
// @data.non_resident.compressed_size:
// Compressed on-disk size in bytes.
// Only present when compressed or sparse.
// Actual disk usage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct attr_record {
    pub type: __le32,
    pub length: __le32,
    pub non_resident: u8,
    pub name_length: u8,
    pub name_offset: __le16,
    pub flags: __le16,
    pub instance: __le16,
    pub value_length: __le32,
    pub value_offset: __le16,
    pub flags: u8,
    pub reserved: i8,
    pub resident: } __packed,
    pub lowest_vcn: __le64,
    pub highest_vcn: __le64,
    pub mapping_pairs_offset: __le16,
    pub compression_unit: u8,
    pub reserved: [u8; 5],
    pub allocated_size: __le64,
    pub data_size: __le64,
    pub initialized_size: __le64,
    pub compressed_size: __le64,
    pub non_resident: } __packed,
    pub data: } __packed,
    pub __packed: },
//
// enum - NTFS file attribute flags (32-bit)
//
// File attribute flags (32-bit) appearing in the file_attributes fields of the
// STANDARD_INFORMATION attribute of MFT_RECORDs and the FILENAME_ATTR
// attributes of MFT_RECORDs and directory index entries.
//
// All of the below flags appear in the directory index entries but only some
// appear in the STANDARD_INFORMATION attribute whilst only some others appear
// in the FILENAME_ATTR attribute of MFT_RECORDs.  Unless otherwise stated the
// flags appear in all of the above.
//
// FILE_ATTR_READONLY:         File is read-only.
// FILE_ATTR_HIDDEN:           File is hidden (not shown by default).
// FILE_ATTR_SYSTEM:           System file (protected by OS).
// FILE_ATTR_DIRECTORY:        Directory flag (reserved in NT; use MFT flag instead).
// FILE_ATTR_ARCHIVE:          File needs archiving (backup flag).
// FILE_ATTR_DEVICE:           Device file (rarely used).
// FILE_ATTR_NORMAL:           Normal file (no special attributes).
// FILE_ATTR_TEMPORARY:        Temporary file (delete on close).
// FILE_ATTR_SPARSE_FILE:      Sparse file (contains holes).
// FILE_ATTR_REPARSE_POINT:    Reparse point (junction, symlink, mount point).
// FILE_ATTR_COMPRESSED:       File is compressed.
// FILE_ATTR_OFFLINE:          File data is offline (not locally available).
// FILE_ATTR_NOT_CONTENT_INDEXED:
// File is excluded from content indexing.
// FILE_ATTR_ENCRYPTED:        File is encrypted (EFS).
// FILE_ATTR_VALID_FLAGS:      Mask of all valid flags for reading.
// FILE_ATTR_VALID_SET_FLAGS:  Mask of flags that can be set by user.
// FILE_ATTRIBUTE_RECALL_ON_OPEN:
// Recall data on open (cloud/HSM related).
// FILE_ATTR_DUP_FILE_NAME_INDEX_PRESENT:
// $FILE_NAME has duplicate index entry.
// FILE_ATTR_DUP_VIEW_INDEX_PRESENT:
// Duplicate view index present (object ID, quota, etc.).
//
// Old DOS volid. Unused in NT.	= cpu_to_le32(0x00000008),
}

//
// NOTE on times in NTFS: All times are in MS standard time format, i.e. they
// are the number of 100-nanosecond intervals since 1st January 1601, 00:00:00
// universal coordinated time (UTC). (In Linux time starts 1st January 1970,
// 00:00:00 UTC and is stored as the number of 1-second intervals since then.)
//
// struct standard_information - $STANDARD_INFORMATION attribute content
//
// NOTE: Always resident.
// NOTE: Present in all base file records on a volume.
// NOTE: There is conflicting information about the meaning of each of the time
// fields but the meaning as defined below has been verified to be
// correct by practical experimentation on Windows NT4 SP6a and is hence
// assumed to be the one and only correct interpretation.
//
// @creation_time:          File creation time (NTFS timestamp).
// Updated on filename change(?).
// @last_data_change_time:  Last modification time of data streams.
// @last_mft_change_time:   Last modification time of this MFT record.
// @last_access_time:       Last access time (approximate).
// Not updated on read-only volumes; can be disabled.
// @file_attributes:        File attribute flags (FILE_ATTR_* bits).
//
// Union (version-specific fields):
// @ver.v1.reserved12:      12 bytes reserved/alignment (NTFS 1.2 only).
//
// @ver.v3 (NTFS 3.x / Windows 2000+):
// @maximum_versions:       Max allowed file versions (0 = disabled).
// @version_number:         Current version number (0 if disabled).
// @class_id:               Class ID (from bidirectional index?).
// @owner_id:               Owner ID (maps to $Quota via $Q index).
// @security_id:            Security ID (maps to $Secure $SII/$SDS).
// @quota_charged:          Quota charge in bytes (0 if quotas disabled).
// @usn:                    Last USN from $UsnJrnl (0 if disabled).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct standard_information {
    pub creation_time: __le64,
    pub last_data_change_time: __le64,
    pub last_mft_change_time: __le64,
    pub last_access_time: __le64,
    pub file_attributes: __le32,
    pub reserved12: [u8; 12],
    pub v1: } __packed,
    pub maximum_versions: __le32,
    pub version_number: __le32,
    pub class_id: __le32,
    pub owner_id: __le32,
    pub security_id: __le32,
    pub quota_charged: __le64,
    pub usn: __le64,
    pub v3: } __packed,
    pub ver: } __packed,
    pub __packed: },
//
// struct attr_list_entry - Entry in $ATTRIBUTE_LIST attribute.
//
// @type:           Attribute type code (ATTR_TYPE_*).
// @length:         Byte size of this entry (8-byte aligned).
// @name_length:    Unicode char count of attribute name (0 if unnamed).
// @name_offset:    Byte offset from start of entry to name (always set).
// @lowest_vcn:     Lowest VCN of this attribute extent (usually 0).
// Signed value; non-zero when attribute spans extents.
// @mft_reference:  MFT record reference holding this attribute extent.
// @instance:       Attribute instance number (if lowest_vcn == 0); else 0.
// @name:           Variable Unicode name (use @name_offset when reading).
//
// - Can be either resident or non-resident.
// - Value consists of a sequence of variable length, 8-byte aligned,
// ATTR_LIST_ENTRY records.
// - The list is not terminated by anything at all! The only way to know when
// the end is reached is to keep track of the current offset and compare it to
// the attribute value size.
// - The attribute list attribute contains one entry for each attribute of
// the file in which the list is located, except for the list attribute
// itself. The list is sorted: first by attribute type, second by attribute
// name (if present), third by instance number. The extents of one
// non-resident attribute (if present) immediately follow after the initial
// extent. They are ordered by lowest_vcn and have their instance set to zero.
// It is not allowed to have two attributes with all sorting keys equal.
// - Further restrictions:
// - If not resident, the vcn to lcn mapping array has to fit inside the
// base mft record.
// - The attribute list attribute value has a maximum size of 256kb. This
// is imposed by the Windows cache manager.
// - Attribute lists are only used when the attributes of mft record do not
// fit inside the mft record despite all attributes (that can be made
// non-resident) having been made non-resident. This can happen e.g. when:
// - File has a large number of hard links (lots of file name
// attributes present).
// - The mapping pairs array of some non-resident attribute becomes so
// large due to fragmentation that it overflows the mft record.
// - The security descriptor is very complex (not applicable to
// NTFS 3.0 volumes).
// - There are many named streams.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct attr_list_entry {
    pub type: __le32,
    pub length: __le16,
    pub name_length: u8,
    pub name_offset: u8,
    pub lowest_vcn: __le64,
    pub mft_reference: __le64,
    pub instance: __le16,
    pub name: [__le16; ],
    pub __packed: },
//
// The maximum allowed length for a file name.
//
pub const MAXIMUM_FILE_NAME_LENGTH: c_int = 255;
//
// enum - Possible namespaces for filenames in ntfs (8-bit).
//
// FILE_NAME_POSIX        POSIX namespace (case sensitive, most permissive).
// Allows all Unicode except '\0' and '/'.
// WinNT/2k/2003 default utilities ignore case
// differences. SFU (Services For Unix) enables true
// case sensitivity.
// SFU restricts some chars: '"', '/', '<', '>', '\'.
// FILE_NAME_WIN32        Standard WinNT/2k long filename namespace
// (case insensitive).
// Disallows '\0', '"', '*', '/', ':', '<', '>', '?',
// '\', '|'. Names cannot end with '.' or space.
// FILE_NAME_DOS          DOS 8.3 namespace (uppercase only).
// Allows 8-bit chars > space except '"', '*', '+',
// ',', '/', ':', ';', '<', '=', '>', '?', '\'.
// FILE_NAME_WIN32_AND_DOS
// Win32 and DOS names are identical (single record).
// Value 0x03 indicates both are stored in one entry.
//
    pub __packed: },
//
// struct file_name_attr - $FILE_NAME attribute content
//
// NOTE: Always resident.
// NOTE: All fields, except the parent_directory, are only updated when the
// filename is changed. Until then, they just become out of sync with
// reality and the more up to date values are present in the standard
// information attribute.
// NOTE: There is conflicting information about the meaning of each of the time
// fields but the meaning as defined below has been verified to be
// correct by practical experimentation on Windows NT4 SP6a and is hence
// assumed to be the one and only correct interpretation.
//
// @parent_directory:   MFT reference to parent directory.
// @creation_time:      File creation time (NTFS timestamp).
// @last_data_change_time:
// Last data modification time.
// @last_mft_change_time:
// Last MFT record modification time.
// @last_access_time:   Last access time (approximate; may not
// update always).
// @allocated_size:     On-disk allocated size for unnamed $DATA.
// Equals compressed_size if compressed/sparse.
// 0 for directories or no $DATA.
// Multiple of cluster size.
// @data_size:          Logical size of unnamed $DATA.
// 0 for directories or no $DATA.
// @file_attributes:    File attribute flags (FILE_ATTR_* bits).
// @type.ea.packed_ea_size:
// Size needed to pack EAs (if present).
// @type.ea.reserved:   Alignment padding.
// @type.rp.reparse_point_tag:
// Reparse point type (if reparse point, no EAs).
// @file_name_length:   Length of filename in Unicode characters.
// @file_name_type:     Namespace (FILE_NAME_POSIX, WIN32, DOS, etc.).
// @file_name:          Variable-length Unicode filename.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_name_attr {
    pub parent_directory: __le64,
    pub creation_time: __le64,
    pub last_data_change_time: __le64,
    pub last_mft_change_time: __le64,
    pub last_access_time: __le64,
    pub allocated_size: __le64,
    pub data_size: __le64,
    pub file_attributes: __le32,
    pub packed_ea_size: __le16,
    pub reserved: __le16,
    pub ea: } __packed,
    pub reparse_point_tag: __le32,
    pub rp: } __packed,
    pub type: } __packed,
    pub file_name_length: u8,
    pub file_name_type: u8,
    pub file_name: [__le16; ],
    pub __packed: },
//
// struct guid - Globally Unique Identifier (GUID) structure
//
// GUID structures store globally unique identifiers (GUID). A GUID is a
// 128-bit value consisting of one group of eight hexadecimal digits, followed
// by three groups of four hexadecimal digits each, followed by one group of
// twelve hexadecimal digits. GUIDs are Microsoft's implementation of the
// distributed computing environment (DCE) universally unique identifier (UUID).
// Example of a GUID:
// 1F010768-5A73-BC91-0010A52216A7
//
// @data1:      First 32 bits (first 8 hex digits).
// @data2:      Next 16 bits (first group of 4 hex digits).
// @data3:      Next 16 bits (second group of 4 hex digits).
// @data4:      Final 64 bits (third group of 4 + last 12 hex digits).
// data4[0-1]: third group; data4[2-7]: remaining part.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guid {
    pub data1: __le32,
    pub data2: __le16,
    pub data3: __le16,
    pub data4: [u8; 8],
    pub __packed: },
//
// struct object_id_attr - $OBJECT_ID attribute content (NTFS 3.0+)
//
// NOTE: Always resident.
//
// @object_id:          Unique 128-bit GUID assigned to the file.
// Core identifier; always present.
//
// Optional extended info (union; total value size 16–64 bytes):
// @extended_info.birth_volume_id:
// Birth volume GUID (where file was first created).
// @extended_info.birth_object_id:
// Birth object GUID (original ID before copy/move).
// @extended_info.domain_id:
// Domain GUID (usually zero; reserved).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct object_id_attr {
    pub object_id: guid,
    pub birth_volume_id: guid,
    pub birth_object_id: guid,
    pub domain_id: guid,
    pub __packed: },
    pub extended_info: [u8; 48],
    pub __packed: },
    pub __packed: },
//
// enum - RIDs (Relative Identifiers) in Windows/NTFS security
//
// These relative identifiers (RIDs) are used with the above identifier
// authorities to make up universal well-known SIDs.
//
// SECURITY_NULL_RID              S-1-0 (Null authority)
// SECURITY_WORLD_RID             S-1-1 (World/Everyone)
// SECURITY_LOCAL_RID             S-1-2 (Local)
// SECURITY_CREATOR_OWNER_RID     S-1-3-0 (Creator Owner)
// SECURITY_CREATOR_GROUP_RID     S-1-3-1 (Creator Group)
// SECURITY_CREATOR_OWNER_SERVER_RID S-1-3-2 (Server Creator Owner)
// SECURITY_CREATOR_GROUP_SERVER_RID S-1-3-3 (Server Creator Group)
// SECURITY_DIALUP_RID            S-1-5-1 (Dialup)
// SECURITY_NETWORK_RID           S-1-5-2 (Network)
// SECURITY_BATCH_RID             S-1-5-3 (Batch)
// SECURITY_INTERACTIVE_RID       S-1-5-4 (Interactive)
// SECURITY_SERVICE_RID           S-1-5-6 (Service)
// SECURITY_ANONYMOUS_LOGON_RID   S-1-5-7 (Anonymous Logon)
// SECURITY_PROXY_RID             S-1-5-8 (Proxy)
// SECURITY_ENTERPRISE_CONTROLLERS_RID S-1-5-9 (Enterprise DCs)
// SECURITY_SERVER_LOGON_RID      S-1-5-9 (Server Logon alias)
// SECURITY_PRINCIPAL_SELF_RID    S-1-5-10 (Self/PrincipalSelf)
// SECURITY_AUTHENTICATED_USER_RID S-1-5-11 (Authenticated Users)
// SECURITY_RESTRICTED_CODE_RID   S-1-5-12 (Restricted Code)
// SECURITY_TERMINAL_SERVER_RID   S-1-5-13 (Terminal Server)
// SECURITY_LOGON_IDS_RID         S-1-5-5 (Logon session IDs base)
// SECURITY_LOCAL_SYSTEM_RID      S-1-5-18 (Local System)
// SECURITY_NT_NON_UNIQUE         S-1-5-21 (NT non-unique authority)
// SECURITY_BUILTIN_DOMAIN_RID    S-1-5-32 (Built-in domain)
//
// Built-in domain relative RIDs (S-1-5-32-...):
// Users:
// DOMAIN_USER_RID_ADMIN          Administrator
// DOMAIN_USER_RID_GUEST          Guest
// DOMAIN_USER_RID_KRBTGT         krbtgt (Kerberos ticket-granting)
//
// Groups:
// DOMAIN_GROUP_RID_ADMINS        Administrators
// DOMAIN_GROUP_RID_USERS         Users
// DOMAIN_GROUP_RID_GUESTS        Guests
// DOMAIN_GROUP_RID_COMPUTERS     Computers
// DOMAIN_GROUP_RID_CONTROLLERS   Domain Controllers
// DOMAIN_GROUP_RID_CERT_ADMINS   Cert Publishers
// DOMAIN_GROUP_RID_SCHEMA_ADMINS Schema Admins
// DOMAIN_GROUP_RID_ENTERPRISE_ADMINS Enterprise Admins
// DOMAIN_GROUP_RID_POLICY_ADMINS Policy Admins (if present)
//
// Aliases:
// DOMAIN_ALIAS_RID_ADMINS        Administrators alias
// DOMAIN_ALIAS_RID_USERS         Users alias
// DOMAIN_ALIAS_RID_GUESTS        Guests alias
// DOMAIN_ALIAS_RID_POWER_USERS   Power Users
// DOMAIN_ALIAS_RID_ACCOUNT_OPS   Account Operators
// DOMAIN_ALIAS_RID_SYSTEM_OPS    Server Operators
// DOMAIN_ALIAS_RID_PRINT_OPS     Print Operators
// DOMAIN_ALIAS_RID_BACKUP_OPS    Backup Operators
// DOMAIN_ALIAS_RID_REPLICATOR    Replicator
// DOMAIN_ALIAS_RID_RAS_SERVERS   RAS Servers
// DOMAIN_ALIAS_RID_PREW2KCOMPACCESS Pre-Windows 2000 Compatible Access
//
// Note: The relative identifier (RID) refers to the portion of a SID, which
// identifies a user or group in relation to the authority that issued the SID.
// For example, the universal well-known SID Creator Owner ID (S-1-3-0) is
// made up of the identifier authority SECURITY_CREATOR_SID_AUTHORITY (3) and
// the relative identifier SECURITY_CREATOR_OWNER_RID (0).
//
// Well-known domain relative sub-authority values (RIDs).
//
// Users.
// Groups.
// Aliases.
}

//
// The universal well-known SIDs:
//
// NULL_SID			S-1-0-0
// WORLD_SID			S-1-1-0
// LOCAL_SID			S-1-2-0
// CREATOR_OWNER_SID		S-1-3-0
// CREATOR_GROUP_SID		S-1-3-1
// CREATOR_OWNER_SERVER_SID	S-1-3-2
// CREATOR_GROUP_SERVER_SID	S-1-3-3
//
// (Non-unique IDs)		S-1-4
//
// NT well-known SIDs:
//
// NT_AUTHORITY_SID	S-1-5
// DIALUP_SID		S-1-5-1
//
// NETWORD_SID		S-1-5-2
// BATCH_SID		S-1-5-3
// INTERACTIVE_SID		S-1-5-4
// SERVICE_SID		S-1-5-6
// ANONYMOUS_LOGON_SID	S-1-5-7		(aka null logon session)
// PROXY_SID		S-1-5-8
// SERVER_LOGON_SID	S-1-5-9		(aka domain controller account)
// SELF_SID		S-1-5-10	(self RID)
// AUTHENTICATED_USER_SID	S-1-5-11
// RESTRICTED_CODE_SID	S-1-5-12	(running restricted code)
// TERMINAL_SERVER_SID	S-1-5-13	(running on terminal server)
//
// (Logon IDs)		S-1-5-5-X-Y
//
// (NT non-unique IDs)	S-1-5-0x15-...
//
// (Built-in domain)	S-1-5-0x20
//
// struct ntfs_sid - Security Identifier (SID) structure
//
// @revision:            SID revision level (usually 1).
// @sub_authority_count: Number of sub-authorities (1 or more).
// @identifier_authority:
// 48-bit identifier authority (S-1-x-...).
// @parts.high_part: high 16 bits.
// @parts.low_part: low 32 bits.
// @value: raw 6-byte array.
// @sub_authority:       Variable array of 32-bit RIDs.
// At least one; defines the SID relative to authority.
//
// The SID structure is a variable-length structure used to uniquely identify
// users or groups. SID stands for security identifier.
//
// The standard textual representation of the SID is of the form:
// S-R-I-S-S...
// Where:
// - The first "S" is the literal character 'S' identifying the following
// digits as a SID.
// - R is the revision level of the SID expressed as a sequence of digits
// either in decimal or hexadecimal (if the later, prefixed by "0x").
// - I is the 48-bit identifier_authority, expressed as digits as R above.
// - S... is one or more sub_authority values, expressed as digits as above.
//
// Example SID; the domain-relative SID of the local Administrators group on
// Windows NT/2k:
// S-1-5-32-544
// This translates to a SID with:
// revision = 1,
// sub_authority_count = 2,
// identifier_authority = {0,0,0,0,0,5},	// SECURITY_NT_AUTHORITY
// sub_authority[0] = 32,			// SECURITY_BUILTIN_DOMAIN_RID
// sub_authority[1] = 544			// DOMAIN_ALIAS_RID_ADMINS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_sid {
    pub revision: u8,
    pub sub_authority_count: u8,
    pub high_part: u16,
    pub low_part: u32,
    pub parts: } __packed,
    pub value: [u8; 6],
    pub identifier_authority: },
    pub sub_authority: [__le32; ],
    pub __packed: },
//
// enum - Predefined ACE types (8-bit) for NTFS security descriptors
//
// ACCESS_MIN_MS_ACE_TYPE:         Minimum MS ACE type (0).
// ACCESS_ALLOWED_ACE_TYPE:        Allow access (standard ACE).
// ACCESS_DENIED_ACE_TYPE:         Deny access (standard ACE).
// SYSTEM_AUDIT_ACE_TYPE:          Audit successful/failed access.
// SYSTEM_ALARM_ACE_TYPE:          Alarm on access (not in Win2k+).
// ACCESS_MAX_MS_V2_ACE_TYPE:      Max for V2 ACE types.
// ACCESS_ALLOWED_COMPOUND_ACE_TYPE:
// Compound ACE (legacy).
// ACCESS_MAX_MS_V3_ACE_TYPE:      Max for V3 ACE types.
// ACCESS_MIN_MS_OBJECT_ACE_TYPE:  Min for object ACE types (Win2k+).
// ACCESS_ALLOWED_OBJECT_ACE_TYPE: Allow with object-specific rights.
// ACCESS_DENIED_OBJECT_ACE_TYPE:  Deny with object-specific rights.
// SYSTEM_AUDIT_OBJECT_ACE_TYPE:   Audit with object-specific rights.
// SYSTEM_ALARM_OBJECT_ACE_TYPE:   Alarm with object-specific rights.
// ACCESS_MAX_MS_OBJECT_ACE_TYPE:  Max for object ACE types.
// ACCESS_MAX_MS_V4_ACE_TYPE:      Max for V4 ACE types.
// ACCESS_MAX_MS_ACE_TYPE:         Overall max ACE type (WinNT/2k).
//
    pub __packed: },
//
// enum - ACE inheritance and audit flags (8-bit)
//
// OBJECT_INHERIT_ACE:         Object inherit (files inherit this ACE).
// CONTAINER_INHERIT_ACE:      Container inherit (subdirectories inherit).
// NO_PROPAGATE_INHERIT_ACE:   No propagation (stop inheritance after this level).
// INHERIT_ONLY_ACE:           Inherit only (not applied to current object).
// INHERITED_ACE:              ACE was inherited (Win2k+ only).
// VALID_INHERIT_FLAGS:        Mask of all valid inheritance flags (0x1f).
// SUCCESSFUL_ACCESS_ACE_FLAG: Audit successful access (system audit ACE).
// FAILED_ACCESS_ACE_FLAG:     Audit failed access (system audit ACE).
//
// SUCCESSFUL_ACCESS_ACE_FLAG is only used with system audit and alarm ACE
// types to indicate that a message is generated (in Windows!) for successful
// accesses.
//
// FAILED_ACCESS_ACE_FLAG is only used with system audit and alarm ACE types
// to indicate that a message is generated (in Windows!) for failed accesses.
//
    pub __packed: },
//
// enum - NTFS access rights masks (32-bit)
//
// FILE_READ_DATA / FILE_LIST_DIRECTORY:  Read file data / list dir contents.
// FILE_WRITE_DATA / FILE_ADD_FILE:       Write file data / create file in dir.
// FILE_APPEND_DATA / FILE_ADD_SUBDIRECTORY: Append data / create subdir.
// FILE_READ_EA:                          Read extended attributes.
// FILE_WRITE_EA:                         Write extended attributes.
// FILE_EXECUTE / FILE_TRAVERSE:          Execute file / traverse dir.
// FILE_DELETE_CHILD:                     Delete children in dir.
// FILE_READ_ATTRIBUTES:                  Read attributes.
// FILE_WRITE_ATTRIBUTES:                 Write attributes.
//
// Standard rights (object-independent):
// DELETE:                                Delete object.
// READ_CONTROL:                          Read security descriptor/owner.
// WRITE_DAC:                             Modify DACL.
// WRITE_OWNER:                           Change owner.
// SYNCHRONIZE:                           Wait on object signal state.
//
// Combinations:
// STANDARD_RIGHTS_READ / WRITE / EXECUTE: Aliases for READ_CONTROL.
// STANDARD_RIGHTS_REQUIRED:              DELETE + READ_CONTROL +
// WRITE_DAC + WRITE_OWNER.
// STANDARD_RIGHTS_ALL:                   Above + SYNCHRONIZE.
//
// System/access types:
// ACCESS_SYSTEM_SECURITY:                Access system ACL.
// MAXIMUM_ALLOWED:                       Maximum allowed access.
//
// Generic rights (high bits, map to specific/standard):
// GENERIC_ALL:                           Full access.
// GENERIC_EXECUTE:                       Execute/traverse.
// GENERIC_WRITE:                         Write (append, attrs, data, EA, etc.).
// GENERIC_READ:                          Read (attrs, data, EA, etc.).
//
// The specific rights (bits 0 to 15).  These depend on the type of the object
// being secured by the ACE.
//
}

//
// struct ntfs_ace - Access Control Entry (ACE) structure
//
// @type: ACE type (ACCESS_ALLOWED_ACE_TYPE, ACCESS_DENIED_ACE_TYPE, etc.).
// @flags: Inheritance and audit flags (OBJECT_INHERIT_ACE, etc.).
// @size: Total byte size of this ACE (header + SID + variable data).
// @mask: Access rights mask (FILE_READ_DATA, DELETE, GENERIC_ALL, etc.).
// @sid: Security Identifier (SID) this ACE applies to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_ace {
    pub type: u8,
    pub flags: u8,
    pub size: __le16,
    pub mask: __le32,
    pub sid: ntfs_sid,
    pub __packed: },
//
// The object ACE flags (32-bit).
//
}

//
// struct ntfs_acl - NTFS Access Control List (ACL) header
//
// An ACL is an access-control list (ACL).
// An ACL starts with an ACL header structure, which specifies the size of
// the ACL and the number of ACEs it contains. The ACL header is followed by
// zero or more access control entries (ACEs). The ACL as well as each ACE
// are aligned on 4-byte boundaries.
//
// @revision:           ACL revision level (usually 2 or 4).
// @alignment1:         Padding/alignment byte (zero).
// @size:               Total allocated size in bytes (header + all ACEs +
// free space).
// @ace_count:          Number of ACE entries following the header.
// @alignment2:         Padding/alignment (zero).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_acl {
    pub revision: u8,
    pub alignment1: u8,
    pub size: __le16,
    pub ace_count: __le16,
    pub alignment2: __le16,
    pub __packed: },
    pub 8): static_assert(sizeof(struct ntfs_acl) ==,
//
// The security descriptor control flags (16-bit).
//
// SE_OWNER_DEFAULTED - This boolean flag, when set, indicates that the SID
// pointed to by the Owner field was provided by a defaulting mechanism
// rather than explicitly provided by the original provider of the
// security descriptor.  This may affect the treatment of the SID with
// respect to inheritance of an owner.
//
// SE_GROUP_DEFAULTED - This boolean flag, when set, indicates that the SID in
// the Group field was provided by a defaulting mechanism rather than
// explicitly provided by the original provider of the security
// descriptor.  This may affect the treatment of the SID with respect to
// inheritance of a primary group.
//
// SE_DACL_PRESENT - This boolean flag, when set, indicates that the security
// descriptor contains a discretionary ACL.  If this flag is set and the
// Dacl field of the SECURITY_DESCRIPTOR is null, then a null ACL is
// explicitly being specified.
//
// SE_DACL_DEFAULTED - This boolean flag, when set, indicates that the ACL
// pointed to by the Dacl field was provided by a defaulting mechanism
// rather than explicitly provided by the original provider of the
// security descriptor.  This may affect the treatment of the ACL with
// respect to inheritance of an ACL.  This flag is ignored if the
// DaclPresent flag is not set.
//
// SE_SACL_PRESENT - This boolean flag, when set,  indicates that the security
// descriptor contains a system ACL pointed to by the Sacl field.  If this
// flag is set and the Sacl field of the SECURITY_DESCRIPTOR is null, then
// an empty (but present) ACL is being specified.
//
// SE_SACL_DEFAULTED - This boolean flag, when set, indicates that the ACL
// pointed to by the Sacl field was provided by a defaulting mechanism
// rather than explicitly provided by the original provider of the
// security descriptor.  This may affect the treatment of the ACL with
// respect to inheritance of an ACL.  This flag is ignored if the
// SaclPresent flag is not set.
//
// SE_SELF_RELATIVE - This boolean flag, when set, indicates that the security
// descriptor is in self-relative form.  In this form, all fields of the
// security descriptor are contiguous in memory and all pointer fields are
// expressed as offsets from the beginning of the security descriptor.
//
    pub __packed: },
//
// struct security_descriptor_relative - Relative security descriptor
//
// Self-relative security descriptor. Contains the owner and group SIDs as well
// as the sacl and dacl ACLs inside the security descriptor itself.
//
// @revision:          Security descriptor revision (usually 1).
// @alignment:         Padding/alignment byte (zero).
// @control:           Control flags (SE_OWNER_DEFAULTED, SE_DACL_PRESENT,
// SE_SACL_PRESENT, SE_SACL_AUTO_INHERITED, etc.).
// @owner:             Byte offset to owner SID (from start of descriptor).
// 0 if no owner SID present.
// @group:             Byte offset to primary group SID.
// 0 if no group SID present.
// @sacl:              Byte offset to System ACL (SACL).
// Valid only if SE_SACL_PRESENT in @control.
// 0 means NULL SACL.
// @dacl:              Byte offset to Discretionary ACL (DACL).
// Valid only if SE_DACL_PRESENT in @control.
// 0 means NULL DACL (full access granted).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct security_descriptor_relative {
    pub revision: u8,
    pub alignment: u8,
    pub control: __le16,
    pub owner: __le32,
    pub group: __le32,
    pub sacl: __le32,
    pub dacl: __le32,
    pub __packed: },
    pub 20): static_assert(sizeof(struct security_descriptor_relative) ==,
//
// On NTFS 3.0+, all security descriptors are stored in FILE_Secure. Only one
// referenced instance of each unique security descriptor is stored.
//
// FILE_Secure contains no unnamed data attribute, i.e. it has zero length. It
// does, however, contain two indexes ($SDH and $SII) as well as a named data
// stream ($SDS).
//
// Every unique security descriptor is assigned a unique security identifier
// (security_id, not to be confused with a SID). The security_id is unique for
// the NTFS volume and is used as an index into the $SII index, which maps
// security_ids to the security descriptor's storage location within the $SDS
// data attribute. The $SII index is sorted by ascending security_id.
//
// A simple hash is computed from each security descriptor. This hash is used
// as an index into the $SDH index, which maps security descriptor hashes to
// the security descriptor's storage location within the $SDS data attribute.
// The $SDH index is sorted by security descriptor hash and is stored in a B+
// tree. When searching $SDH (with the intent of determining whether or not a
// new security descriptor is already present in the $SDS data stream), if a
// matching hash is found, but the security descriptors do not match, the
// search in the $SDH index is continued, searching for a next matching hash.
//
// When a precise match is found, the security_id coresponding to the security
// descriptor in the $SDS attribute is read from the found $SDH index entry and
// is stored in the $STANDARD_INFORMATION attribute of the file/directory to
// which the security descriptor is being applied. The $STANDARD_INFORMATION
// attribute is present in all base mft records (i.e. in all files and
// directories).
//
// If a match is not found, the security descriptor is assigned a new unique
// security_id and is added to the $SDS data attribute. Then, entries
// referencing the this security descriptor in the $SDS data attribute are
// added to the $SDH and $SII indexes.
//
// Note: Entries are never deleted from FILE_Secure, even if nothing
// references an entry any more.
//
// struct sii_index_key - Key for $SII index in $Secure file
//
// The index entry key used in the $SII index. The collation type is
// COLLATION_NTOFS_ULONG.
//
// @security_id:    32-bit security identifier.
// Unique ID assigned to a security descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sii_index_key {
    pub security_id: __le32,
    pub __packed: },
//
// struct sdh_index_key - Key for $SDH index in $Secure file
//
// The index entry key used in the $SDH index. The keys are sorted first by
// hash and then by security_id. The collation rule is
// COLLATION_NTOFS_SECURITY_HASH.
//
// @hash:           32-bit hash of the security descriptor.
// Used for quick collision checks and indexing.
// @security_id:    32-bit security identifier.
// Unique ID assigned to the descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdh_index_key {
    pub hash: __le32,
    pub security_id: __le32,
    pub __packed: },
//
// enum - NTFS volume flags (16-bit)
//
// These flags are stored in $VolumeInformation attribute.
// They indicate volume state and required actions.
//
// VOLUME_IS_DIRTY:                Volume is dirty (needs chkdsk).
// VOLUME_RESIZE_LOG_FILE:         Resize LogFile on next mount.
// VOLUME_UPGRADE_ON_MOUNT:        Upgrade volume on mount (old NTFS).
// VOLUME_MOUNTED_ON_NT4:          Mounted on NT4 (compatibility flag).
// VOLUME_DELETE_USN_UNDERWAY:     USN journal deletion in progress.
// VOLUME_REPAIR_OBJECT_ID:        Repair $ObjId on next mount.
// VOLUME_CHKDSK_UNDERWAY:         Chkdsk is running.
// VOLUME_MODIFIED_BY_CHKDSK:      Modified by chkdsk.
// VOLUME_FLAGS_MASK:              Mask of all valid flags (0xc03f).
// VOLUME_MUST_MOUNT_RO_MASK:      Flags forcing read-only mount (0xc027).
// If any set, mount read-only.
//
    pub __packed: },
//
// struct volume_information - $VOLUME_INFORMATION (0x70)
//
// @reserved:       Reserved 64-bit field (currently unused).
// @major_ver:      Major NTFS version number (e.g., 3 for NTFS 3.1).
// @minor_ver:      Minor NTFS version number (e.g., 1 for NTFS 3.1).
// @flags:          Volume flags (VOLUME_IS_DIRTY, VOLUME_CHKDSK_UNDERWAY, etc.).
// See volume flags enum for details.
//
// NOTE: Always resident.
// NOTE: Present only in FILE_Volume.
// NOTE: Windows 2000 uses NTFS 3.0 while Windows NT4 service pack 6a uses
// NTFS 1.2. I haven't personally seen other values yet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct volume_information {
    pub reserved: __le64,
    pub major_ver: u8,
    pub minor_ver: u8,
    pub flags: __le16,
    pub __packed: },
//
// enum - Index header flags
//
// These flags are stored in the index header (INDEX_HEADER.flags) for both
// index root ($INDEX_ROOT) and index allocation blocks ($INDEX_ALLOCATION).
//
// For index root ($INDEX_ROOT attribute):
// SMALL_INDEX: Index fits entirely in root attribute (no $INDEX_ALLOCATION).
// LARGE_INDEX: Index too large for root; $INDEX_ALLOCATION present.
//
// For index blocks ($INDEX_ALLOCATION):
// LEAF_NODE:   Leaf node (no child nodes; contains actual entries).
// INDEX_NODE:  Internal node (indexes other nodes; contains keys/pointers).
//
// NODE_MASK:   Mask to extract node type bits (0x01).
//
    pub __packed: },
//
// struct index_header - Common header for index root and index blocks
//
// entries_offset:     Byte offset to first INDEX_ENTRY (8-byte aligned).
// index_length:       Bytes used by index entries (8-byte aligned).
// From entries_offset to end of used data.
// allocated_size:     Total allocated bytes for this index block.
// Fixed size in index allocation; dynamic in root.
// flags:              Index flags (SMALL_INDEX, LARGE_INDEX, LEAF_NODE, etc.).
// See INDEX_HEADER_FLAGS enum.
// reserved:           3 bytes reserved/padding (zero, 8-byte aligned).
//
// This is the header for indexes, describing the INDEX_ENTRY records, which
// follow the index_header. Together the index header and the index entries
// make up a complete index.
//
// IMPORTANT NOTE: The offset, length and size structure members are counted
// relative to the start of the index header structure and not relative to the
// start of the index root or index allocation structures themselves.
//
// For the index root attribute, the above two numbers are always
// equal, as the attribute is resident and it is resized as needed. In
// the case of the index allocation attribute the attribute is not
// resident and hence the allocated_size is a fixed value and must
// equal the index_block_size specified by the INDEX_ROOT attribute
// corresponding to the INDEX_ALLOCATION attribute this INDEX_BLOCK
// belongs to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_header {
    pub entries_offset: __le32,
    pub index_length: __le32,
    pub allocated_size: __le32,
    pub flags: u8,
    pub reserved: [u8; 3],
    pub __packed: },
//
// struct index_root - $INDEX_ROOT attribute (0x90).
//
// @type:               Indexed attribute type ($FILE_NAME for dirs,
// 0 for view indexes).
// @collation_rule:     Collation rule for sorting entries
// (COLLATION_FILE_NAME for $FILE_NAME).
// @index_block_size:   Size of each index block in bytes
// (in $INDEX_ALLOCATION).
// @clusters_per_index_block:
// Clusters per index block (or log2(bytes)
// if < cluster).
// Power of 2; used for encoding block size.
// @reserved:           3 bytes reserved/alignment (zero).
// @index:              Index header for root entries (entries follow
// immediately).
//
// NOTE: Always resident.
//
// This is followed by a sequence of index entries (INDEX_ENTRY structures)
// as described by the index header.
//
// When a directory is small enough to fit inside the index root then this
// is the only attribute describing the directory. When the directory is too
// large to fit in the index root, on the other hand, two additional attributes
// are present: an index allocation attribute, containing sub-nodes of the B+
// directory tree (see below), and a bitmap attribute, describing which virtual
// cluster numbers (vcns) in the index allocation attribute are in use by an
// index block.
//
// NOTE: The root directory (FILE_root) contains an entry for itself. Other
// directories do not contain entries for themselves, though.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_root {
    pub type: __le32,
    pub collation_rule: __le32,
    pub index_block_size: __le32,
    pub clusters_per_index_block: u8,
    pub reserved: [u8; 3],
    pub index: index_header,
    pub __packed: },
//
// struct index_block - Index allocation (0xa0).
//
// @magic:              Magic value "INDX" (see magic_INDX).
// @usa_ofs:            Offset to Update Sequence Array (see ntfs_record).
// @usa_count:          Number of USA entries (see ntfs_record).
// @lsn:                Log sequence number of last modification.
// @index_block_vcn:    VCN of this index block.
// Units: clusters if cluster_size <= index_block_size;
// sectors otherwise.
// @index:              Index header describing entries in this block.
//
// When creating the index block, we place the update sequence array at this
// offset, i.e. before we start with the index entries. This also makes sense,
// otherwise we could run into problems with the update sequence array
// containing in itself the last two bytes of a sector which would mean that
// multi sector transfer protection wouldn't work. As you can't protect data
// by overwriting it since you then can't get it back...
// When reading use the data from the ntfs record header.
//
// NOTE: Always non-resident (doesn't make sense to be resident anyway!).
//
// This is an array of index blocks. Each index block starts with an
// index_block structure containing an index header, followed by a sequence of
// index entries (INDEX_ENTRY structures), as described by the struct index_header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_block {
    pub magic: __le32,
    pub usa_ofs: __le16,
    pub usa_count: __le16,
    pub lsn: __le64,
    pub index_block_vcn: __le64,
    pub index: index_header,
    pub __packed: },
    pub 40): static_assert(sizeof(struct index_block) ==,
//
// struct reparse_index_key - Key for $R reparse index in $Extend/$Reparse
//
// @reparse_tag:    Reparse point type (including flags, REPARSE_TAG_*).
// @file_id:        MFT record number of the file with $REPARSE_POINT
// attribute.
//
// The system file FILE_Extend/$Reparse contains an index named $R listing
// all reparse points on the volume. The index entry keys are as defined
// below. Note, that there is no index data associated with the index entries.
//
// The index entries are sorted by the index key file_id. The collation rule is
// COLLATION_NTOFS_ULONGS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reparse_index_key {
    pub reparse_tag: __le32,
    pub file_id: __le64,
    pub __packed: },
//
// enum - Quota entry flags (32-bit) in $Quota/$Q
//
// These flags are stored in quota control entries ($Quota file).
// They control quota tracking, limits, and state.
//
// User quota flags (mask 0x00000007):
// @QUOTA_FLAG_DEFAULT_LIMITS:   Use default limits.
// @QUOTA_FLAG_LIMIT_REACHED:    Quota limit reached.
// @QUOTA_FLAG_ID_DELETED:       Quota ID deleted.
// @QUOTA_FLAG_USER_MASK:        Mask for user quota flags (0x00000007).
//
// Default entry flags (owner_id = QUOTA_DEFAULTS_ID):
// @QUOTA_FLAG_TRACKING_ENABLED:    Quota tracking enabled.
// @QUOTA_FLAG_ENFORCEMENT_ENABLED: Quota enforcement enabled.
// @QUOTA_FLAG_TRACKING_REQUESTED:  Tracking requested (pending).
// @QUOTA_FLAG_LOG_THRESHOLD:       Log when threshold reached.
// @QUOTA_FLAG_LOG_LIMIT:           Log when limit reached.
// @QUOTA_FLAG_OUT_OF_DATE:         Quota data out of date.
// @QUOTA_FLAG_CORRUPT:             Quota entry corrupt.
// @QUOTA_FLAG_PENDING_DELETES:     Pending quota deletes.
//
}

//
// struct quota_control_entry - Quota entry in $Quota/$Q
//
// @version:        Currently 2.
// @flags:          Quota flags (QUOTA_FLAG_* bits).
// @bytes_used:     Current quota usage in bytes.
// @change_time:    Last modification time (NTFS timestamp).
// @threshold:      Soft quota limit (-1 = unlimited).
// @limit:          Hard quota limit (-1 = unlimited).
// @exceeded_time:  Time soft quota has been exceeded.
// @sid:            SID of user/object (zero for defaults entry).
//
// The system file FILE_Extend/$Quota contains two indexes $O and $Q. Quotas
// are on a per volume and per user basis.
//
// The $Q index contains one entry for each existing user_id on the volume. The
// index key is the user_id of the user/group owning this quota control entry,
// i.e. the key is the owner_id. The user_id of the owner of a file, i.e. the
// owner_id, is found in the standard information attribute. The collation rule
// for $Q is COLLATION_NTOFS_ULONG.
//
// The $O index contains one entry for each user/group who has been assigned
// a quota on that volume. The index key holds the SID of the user_id the
// entry belongs to, i.e. the owner_id. The collation rule for $O is
// COLLATION_NTOFS_SID.
//
// The $O index entry data is the user_id of the user corresponding to the SID.
// This user_id is used as an index into $Q to find the quota control entry
// associated with the SID.
//
// The $Q index entry data is the quota control entry and is defined below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quota_control_entry {
    pub version: __le32,
    pub flags: __le32,
    pub bytes_used: __le64,
    pub change_time: __le64,
    pub threshold: __le64,
    pub limit: __le64,
    pub exceeded_time: __le64,
    pub sid: ntfs_sid,
    pub __packed: },
//
// Predefined owner_id values (32-bit).
//
}

//
// Current constants for quota control entries.
//
// Current version.
//
// enum - Index entry flags (16-bit)
//
// These flags are in INDEX_ENTRY.flags (after key data).
// They describe entry type and status in index blocks/root.
//
// @INDEX_ENTRY_NODE:     Entry points to a sub-node (index block VCN).
// (Not a leaf entry; internal node reference.)
// i.e. a reference to an index block in form of
// a virtual cluster number
// @INDEX_ENTRY_END:      Last entry in index block/root.
// Does not represent a real file; can point to sub-node.
// @INDEX_ENTRY_SPACE_FILLER:
// Dummy value to force enum to 16-bit width.
//
// struct index_entry_header - Common header for all NTFS index entries
//
// This is the fixed header at the start of every INDEX_ENTRY in index
// blocks or index root. It is followed by the variable key, data, and
// sub-node VCN.
//
// Union @data:
// - When INDEX_ENTRY_END is not set:
// @data.dir.indexed_file: MFT reference of the file described by
// this entry. Used in directory indexes ($I30).
// - When INDEX_ENTRY_END is set or for view indexes:
// @data.vi.data_offset: Byte offset from end of this header to
// entry data.
// @data.vi.data_length: Length of data in bytes.
// @data.vi.reservedV:   Reserved (zero).
//
// @length:         Total byte size of this index entry
// (multiple of 8 bytes).
// @key_length:     Byte size of the key (not multiple of 8 bytes).
// Key follows the header immediately.
// @flags:          Bit field of INDEX_ENTRY_* flags (INDEX_ENTRY_NODE, etc.).
// @reserved:       Reserved/padding (zero; align to 8 bytes).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_entry_header {
    pub indexed_file: __le64,
    pub dir: } __packed,
    pub data_offset: __le16,
    pub data_length: __le16,
    pub reservedV: __le32,
    pub vi: } __packed,
    pub data: } __packed,
    pub length: __le16,
    pub key_length: __le16,
    pub flags: __le16,
    pub reserved: __le16,
    pub __packed: },
    pub 16): static_assert(sizeof(struct index_entry_header) ==,
//
// struct index_entry - NTFS index entry structure
//
// This is an index entry. A sequence of such entries follows each index_header
// structure. Together they make up a complete index. The index follows either
// an index root attribute or an index allocation attribute.
//
// Union @data (valid when INDEX_ENTRY_END not set):
// @data.dir.indexed_file: MFT ref of file (for directory indexes).
// @data.vi.data_offset:   Offset to data after key.
// @data.vi.data_length:   Length of data in bytes.
// @data.vi.reservedV:     Reserved (zero).
//
// Fields:
// @length:         Total byte size of entry (multiple of 8 bytes).
// @key_length:     Byte size of key (not multiple of 8).
// @flags:          INDEX_ENTRY_* flags (NODE, END, etc.).
// @reserved:       Reserved/padding (zero).
//
// Union @key (valid when INDEX_ENTRY_END not set)
// The key of the indexed attribute. NOTE: Only present
// if INDEX_ENTRY_END bit in flags is not set. NOTE: On
// NTFS versions before 3.0 the only valid key is the
// struct file_name_attr. On NTFS 3.0+ the following
// additional index keys are defined:
// @key.file_name:  $FILE_NAME attr (for $I30 directory indexes).
// @key.sii:        $SII key (for $Secure $SII index).
// @key.sdh:        $SDH key (for $Secure $SDH index).
// @key.object_id:  GUID (for $ObjId $O index).
// @key.reparse:    Reparse tag + file ID (for $Reparse $R).
// @key.sid:        SID (for $Quota $O index).
// @key.owner_id:   User ID (for $Quota $Q index).
//
// The (optional) index data is inserted here when creating.
// __le64 vcn;     If INDEX_ENTRY_NODE bit in flags is set, the last
// eight bytes of this index entry contain the virtual
// cluster number of the index block that holds the
// entries immediately preceding the current entry (the
// vcn references the corresponding cluster in the data
// of the non-resident index allocation attribute). If
// the key_length is zero, then the vcn immediately
// follows the INDEX_ENTRY_HEADER. Regardless of
// key_length, the address of the 8-byte boundary
// aligned vcn of INDEX_ENTRY{_HEADER} *ie is given by
// (char*)ie + le16_to_cpu(ie*)->length) - sizeof(VCN),
// where sizeof(VCN) can be hardcoded as 8 if wanted.
//
// NOTE: Before NTFS 3.0 only filename attributes were indexed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_entry {
    pub indexed_file: __le64,
    pub dir: } __packed,
    pub data_offset: __le16,
    pub data_length: __le16,
    pub reservedV: __le32,
    pub vi: } __packed,
    pub data: } __packed,
    pub length: __le16,
    pub key_length: __le16,
    pub flags: __le16,
    pub reserved: __le16,
    pub file_name: file_name_attr,
    pub sii: sii_index_key,
    pub sdh: sdh_index_key,
    pub object_id: guid,
    pub reparse: reparse_index_key,
    pub sid: ntfs_sid,
    pub owner_id: __le32,
    pub key: } __packed,
    pub __packed: },
//
// The reparse point tag defines the type of the reparse point. It also
// includes several flags, which further describe the reparse point.
//
// The reparse point tag is an unsigned 32-bit value divided in three parts:
//
// 1. The least significant 16 bits (i.e. bits 0 to 15) specify the type of
// the reparse point.
// 2. The 12 bits after this (i.e. bits 16 to 27) are reserved for future use.
// 3. The most significant four bits are flags describing the reparse point.
// They are defined as follows:
// bit 28: Directory bit. If set, the directory is not a surrogate
// and can be used the usual way.
// bit 29: Name surrogate bit. If set, the filename is an alias for
// another object in the system.
// bit 30: High-latency bit. If set, accessing the first byte of data will
// be slow. (E.g. the data is stored on a tape drive.)
// bit 31: Microsoft bit. If set, the tag is owned by Microsoft. User
// defined tags have to use zero here.
// 4. Moreover, on Windows 10 :
// Some flags may be used in bits 12 to 15 to further describe the
// reparse point.
//
}

pub const SYMLINK_FLAG_RELATIVE: c_int = 1;
//
// struct reparse_point - $REPARSE_POINT attribute content (0xc0)\
//
// @reparse_tag:        Reparse point type (with flags; REPARSE_TAG_*).
// @reparse_data_length: Byte size of @reparse_data.
// @reserved:           Reserved/padding (zero; 8-byte alignment).
// @reparse_data:       Variable reparse data (meaning depends on @reparse_tag).
// - Symbolic link/junction: struct reparse_symlink
// - Mount point: similar symlink structure
// - Other tags: vendor-specific or extended data
//
// NOTE: Can be resident or non-resident.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reparse_point {
    pub reparse_tag: __le32,
    pub reparse_data_length: __le16,
    pub reserved: __le16,
    pub reparse_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mount_point_reparse_data {
    pub substitute_name_offset: __le16,
    pub substitute_name_length: __le16,
    pub print_name_offset: __le16,
    pub print_name_length: __le16,
    pub path_buffer: [__le16; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct symlink_reparse_data {
    pub substitute_name_offset: __le16,
    pub substitute_name_length: __le16,
    pub print_name_offset: __le16,
    pub print_name_length: __le16,
    pub flags: __le32,
    pub path_buffer: [__le16; ],
    pub __packed: },
//
// struct ea_information - $EA_INFORMATION attribute content (0xd0)
//
// @ea_length:      Byte size of packed EAs.
// @need_ea_count:  Number of EAs with NEED_EA bit set.
// @ea_query_length: Byte size needed to unpack/query EAs via ZwQueryEaFile().
// (Unpacked format size.)
//
// NOTE: Always resident. (Is this true???)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ea_information {
    pub ea_length: __le16,
    pub need_ea_count: __le16,
    pub ea_query_length: __le32,
    pub __packed: },
//
// enum - Extended attribute flags (8-bit)
//
// These flags are stored in the EA header of each extended attribute
// (in $EA attribute, type 0xe0).
//
// @NEED_EA:        If set, the file cannot be properly interpreted
// without understanding its associated EAs.
// (Critical EA; applications must process it.)
//
    pub __packed: },
//
// struct ea_attr - Extended attribute (EA) entry (0xe0)
//
// @next_entry_offset:  Byte offset to the next EA_ATTR entry.
// (From start of current entry.)
// @flags:              EA flags (NEED_EA = 0x80 if critical).
// @ea_name_length:     Length of @ea_name in bytes (excluding '\0').
// @ea_value_length:    Byte size of the EA value.
// @ea_name:            ASCII name of the EA (zero-terminated).
// Value immediately follows the name.
// u8 ea_value[];       The value of the EA.  Immediately follows the name.
//
// This is one variable-length record in the $EA attribute value.
// The attribute can be resident or non-resident.
// Sequence of these entries forms the packed EA list.
//
// NOTE: Can be resident or non-resident.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ea_attr {
    pub next_entry_offset: __le32,
    pub flags: u8,
    pub ea_name_length: u8,
    pub ea_value_length: __le16,
    pub ea_name: [u8; ],
    pub __packed: },
