//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/common/fscc.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright (c) International Business Machines  Corp., 2009, 2013
// Etersoft, 2012
// 2018 Samsung Electronics Co., Ltd.
// Author(s): Steve French (sfrench@us.ibm.com)
// Pavel Shilovsky (pshilovsky@samba.org) 2012
// Namjae Jeon (linkinjeon@kernel.org)
//
// Reparse structures - see MS-FSCC 2.1.2
// struct fsctl_reparse_info_req is empty, only response structs (see below)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reparse_data_buffer {
    pub ReparseTag: __le32,
    pub ReparseDataLength: __le16,
    pub Reserved: __u16,
    pub /: *mut *mut __u8 DataBuffer[]; / Variable Length,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reparse_guid_data_buffer {
    pub ReparseTag: __le32,
    pub ReparseDataLength: __le16,
    pub Reserved: __u16,
    pub ReparseGuid: [__u8; 16],
    pub /: *mut *mut __u8 DataBuffer[]; / Variable Length,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reparse_mount_point_data_buffer {
    pub ReparseTag: __le32,
    pub ReparseDataLength: __le16,
    pub Reserved: __u16,
    pub SubstituteNameOffset: __le16,
    pub SubstituteNameLength: __le16,
    pub PrintNameOffset: __le16,
    pub PrintNameLength: __le16,
    pub /: *mut *mut __u8 PathBuffer[]; / Variable Length,
    pub __packed: },
pub const SYMLINK_FLAG_RELATIVE: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reparse_symlink_data_buffer {
    pub ReparseTag: __le32,
    pub ReparseDataLength: __le16,
    pub Reserved: __u16,
    pub SubstituteNameOffset: __le16,
    pub SubstituteNameLength: __le16,
    pub PrintNameOffset: __le16,
    pub PrintNameLength: __le16,
    pub Flags: __le32,
    pub /: *mut *mut __u8 PathBuffer[]; / Variable Length,
    pub __packed: },
// For IO_REPARSE_TAG_NFS - see MS-FSCC 2.1.2.6
pub const NFS_SPECFILE_LNK: c_uint = 0x00000000014B4E4C;
pub const NFS_SPECFILE_CHR: c_uint = 0x0000000000524843;
pub const NFS_SPECFILE_BLK: c_uint = 0x00000000004B4C42;
pub const NFS_SPECFILE_FIFO: c_uint = 0x000000004F464946;
pub const NFS_SPECFILE_SOCK: c_uint = 0x000000004B434F53;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reparse_nfs_data_buffer {
    pub ReparseTag: __le32,
    pub ReparseDataLength: __le16,
    pub Reserved: __u16,
    pub /: *mut *mut *mut __le64 InodeType; / NFS_SPECFILE_,
    pub DataBuffer: [__u8; ],
    pub __packed: },
// For IO_REPARSE_TAG_LX_SYMLINK - see MS-FSCC 2.1.2.7
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reparse_wsl_symlink_data_buffer {
    pub ReparseTag: __le32,
    pub ReparseDataLength: __le16,
    pub Reserved: __u16,
    pub /: *mut *mut __le32 Version; / Always 2,
    pub /: *mut *mut __u8 Target[]; / Variable Length UTF-8 string without nul-term,
    pub __packed: },
// See MS-FSCC 2.3.7
#[repr(C)]
#[derive(Copy, Clone)]
pub struct duplicate_extents_to_file {
    pub /: *mut *mut __u64 PersistentFileHandle; / source file handle, opaque endianness,
    pub VolatileFileHandle: __u64,
    pub SourceFileOffset: __le64,
    pub TargetFileOffset: __le64,
    pub /: *mut *mut __le64 ByteCount; / Bytes to be copied,
    pub __packed: },
// See MS-FSCC 2.3.9
pub const DUPLICATE_EXTENTS_DATA_EX_SOURCE_ATOMIC: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct duplicate_extents_to_file_ex {
    pub /: *mut *mut __le64 StructureSize; / MUST be set to 0x30,
    pub /: *mut *mut __u64 PersistentFileHandle; / source file handle, opaque endianness,
    pub VolatileFileHandle: __u64,
    pub SourceFileOffset: __le64,
    pub TargetFileOffset: __le64,
    pub /: *mut *mut __le64 ByteCount; / Bytes to be copied,
    pub Flags: __le32,
    pub Reserved: __le32,
    pub __packed: },
//
// compression state flags
// See MS-FSCC 2.3.18
// MS-FSCC 2.3.67
// MS-FSCC 2.4.9
//
pub const COMPRESSION_FORMAT_NONE: c_uint = 0x0000;
pub const COMPRESSION_FORMAT_DEFAULT: c_uint = 0x0001;
pub const COMPRESSION_FORMAT_LZNT1: c_uint = 0x0002;
//
// See MS-FSCC 2.3.18
// MS-FSCC 2.3.67
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compress_ioctl {
    pub CompressionState: __le16,
    pub __packed: },
// See MS-FSCC 2.3.20
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsctl_get_integrity_information_rsp {
    pub ChecksumAlgorithm: __le16,
    pub Reserved: __le16,
    pub Flags: __le32,
    pub ChecksumChunkSizeInBytes: __le32,
    pub ClusterSizeInBytes: __le32,
    pub __packed: },
// See MS-FSCC 2.3.52
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_allocated_range_buffer {
    pub file_offset: __le64,
    pub length: __le64,
    pub __packed: },
// See MS-FSCC 2.3.55
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsctl_query_file_regions_req {
    pub FileOffset: __le64,
    pub Length: __le64,
    pub DesiredUsage: __le32,
    pub Reserved: __le32,
    pub __packed: },
// DesiredUsage flags see MS-FSCC 2.3.56.1
pub const FILE_USAGE_INVALID_RANGE: c_uint = 0x00000000;
pub const FILE_USAGE_VALID_CACHED_DATA: c_uint = 0x00000001;
pub const FILE_USAGE_NONCACHED_DATA: c_uint = 0x00000002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_region_info {
    pub FileOffset: __le64,
    pub Length: __le64,
    pub DesiredUsage: __le32,
    pub Reserved: __le32,
    pub __packed: },
// See MS-FSCC 2.3.56
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsctl_query_file_region_rsp {
    pub Flags: __le32,
    pub TotalRegionEntryCount: __le32,
    pub RegionEntryCount: __le32,
    pub Reserved: __u32,
    pub Regions: [file_region_info; ],
    pub __packed: },
// See MS-FSCC 2.3.58
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsctl_query_on_disk_vol_info_rsp {
    pub DirectoryCount: __le64,
    pub FileCount: __le64,
    pub FsFormatMajVersion: __le16,
    pub FsFormatMinVersion: __le16,
    pub FsFormatName: [__u8; 24],
    pub FormatTime: __le64,
    pub LastUpdateTime: __le64,
    pub CopyrightInfo: [__u8; 68],
    pub AbstractInfo: [__u8; 68],
    pub FormatImplInfo: [__u8; 68],
    pub LastModifyImplInfo: [__u8; 68],
    pub __packed: },
// See MS-FSCC 2.3.73
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsctl_set_integrity_information_req {
    pub ChecksumAlgorithm: __le16,
    pub Reserved: __le16,
    pub Flags: __le32,
    pub __packed: },
// See MS-FSCC 2.3.75
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsctl_set_integrity_info_ex_req {
    pub EnableIntegrity: __u8,
    pub KeepState: __u8,
    pub Reserved: __u16,
    pub Flags: __le32,
    pub Version: __u8,
    pub Reserved2: [__u8; 7],
    pub __packed: },
//
// this goes in the ioctl buffer when doing FSCTL_SET_ZERO_DATA
// See MS-FSCC 2.3.85
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_zero_data_information {
    pub FileOffset: __le64,
    pub BeyondFinalZero: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_level_trim_range {
    pub Offset: __le64,
    pub Length: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_level_trim {
    pub Key: __le32,
    pub NumRanges: __le32,
    pub Ranges: [file_level_trim_range; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_level_trim_output {
    pub NumRangesProcessed: __le32,
    pub __packed: },
//
// This level 18, although with struct with same name is different from cifs
// level 0x107. Level 0x107 has an extra u64 between AccessFlags and
// CurrentByteOffset.
// See MS-FSCC 2.4.2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_all_info {
    pub /: *mut *mut __le64 CreationTime; / Beginning of FILE_BASIC_INFO equivalent,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub Attributes: __le32,
    pub /: *mut *mut __u32 Pad1; / End of FILE_BASIC_INFO_INFO equivalent,
    pub /: *mut *mut __le64 AllocationSize; / Beginning of FILE_STANDARD_INFO equivalent,
    pub /: *mut *mut __le64 EndOfFile; / size ie offset to first free byte in file,
    pub /: *mut *mut __le32 NumberOfLinks; / hard links,
    pub DeletePending: __u8,
    pub Directory: __u8,
    pub /: *mut *mut __u16 Pad2; / End of FILE_STANDARD_INFO equivalent,
    pub IndexNumber: __le64,
    pub EASize: __le32,
    pub AccessFlags: __le32,
    pub CurrentByteOffset: __le64,
    pub Mode: __le32,
    pub AlignmentRequirement: __le32,
    pub FileNameLength: __le32,
    pub /: *mut *mut char __pad; / Legacy structure padding,
    pub FileName): DECLARE_FLEX_ARRAY(char,,
}

// See MS-FSCC 2.4.7
// See MS-FSCC 2.4.8
// See MS-FSCC 2.4.10
// See MS-FSCC 2.4.14
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_eof_info {
    pub /: *mut *mut __le64 EndOfFile; / new end of file value,
    pub /: *mut *mut } __packed; / level 20 Set,
// See MS-FSCC 2.4.4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_alloc_info {
    pub AllocationSize: __le64,
    pub __packed: },
// See MS-FSCC 2.4.15
    pub NextEntryOffset: __le32,
    pub FileIndex: __u32,
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub EndOfFile: __le64,
    pub AllocationSize: __le64,
    pub ExtFileAttributes: __le32,
    pub FileNameLength: __le32,
    pub /: *mut *mut __le32 EaSize; / length of the xattrs,
    pub FileName: [c_char; ],
    pub /: *mut *mut } __packed FILE_FULL_DIRECTORY_INFO; / level 0x102 rsp data,
// See MS-FSCC 2.4.24
    pub NextEntryOffset: __le32,
    pub FileIndex: __u32,
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub EndOfFile: __le64,
    pub AllocationSize: __le64,
    pub ExtFileAttributes: __le32,
    pub FileNameLength: __le32,
    pub /: *mut *mut __le32 EaSize; / EA size,
    pub Reserved: __le32,
    pub bit*/: *mut *mut __le64 UniqueId; / inode num - le since Samba puts ino in low 32,
    pub FileName: [c_char; ],
    pub /: *mut *mut } __packed FILE_ID_FULL_DIR_INFO; / level 0x105 FF rsp data,
// See MS-FSCC 2.4.27
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_internal_info {
    pub IndexNumber: __le64,
    pub /: *mut *mut } __packed; / level 6 Query,
// See MS-FSCC 2.4.28.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_link_info {
// New members MUST be added within the struct_group() macro below.
    pub /: *mut *mut __u8 ReplaceIfExists; / 1 = replace existing link with new,
// 0 = fail if link already exists
    pub Reserved: [__u8; 7],
    pub /: *mut *mut __u64 RootDirectory; / MBZ for network operations (why says spec?),
    pub FileNameLength: __le32,
    pub /: *mut *mut char FileName[]; / Name to be assigned to new link,
    pub /: *mut *mut } __packed; / level 11 Set,
    pub __struct_group()"): "struct member likely outside of,
// See MS-FSCC 2.4.34
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_network_open_info {
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub AllocationSize: __le64,
    pub EndOfFile: __le64,
    pub Attributes: __le32,
    pub Reserved: __le32,
    pub /: *mut *mut } __packed; / level 34 Query also similar returned in close rsp and open rsp,
// See MS-FSCC 2.4.42.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_rename_info {
// New members MUST be added within the struct_group() macro below.
    pub /: *mut *mut __u8 ReplaceIfExists; / 1 = replace existing target with new,
// 0 = fail if target already exists
    pub Reserved: [__u8; 7],
    pub /: *mut *mut __u64 RootDirectory; / MBZ for network operations (why says spec?),
    pub FileNameLength: __le32,
    pub /: *mut *mut char FileName[]; / New name to be assigned,
// padding - overall struct size must be >= 24 so filename + pad >= 6
    pub /: *mut *mut } __packed; / level 10 Set,
    pub __struct_group()"): "struct member likely outside of,
// File System Information Classes
// See MS-FSCC 2.5

// See POSIX Extensions to MS-FSCC 2.3.1.1

// See MS-FSCC 2.5.1
pub const MAX_FS_NAME_LEN: c_int = 52;
    pub Attributes: __le32,
    pub MaxPathNameComponentLength: __le32,
    pub FileSystemNameLen: __le32,
    pub /: *mut *mut __le16 FileSystemName[]; / do not have to save this - get subset?,
    pub FILE_SYSTEM_ATTRIBUTE_INFO: } __packed,
// List of FileSystemAttributes - see MS-FSCC 2.5.1
pub const FILE_SUPPORTS_SPARSE_VDL: c_uint = 0x10000000 /* faster nonsparse extend */;
pub const FILE_SUPPORTS_BLOCK_REFCOUNTING: c_uint = 0x08000000 /* allow ioctl dup extents */;
pub const FILE_SUPPORT_INTEGRITY_STREAMS: c_uint = 0x04000000;
pub const FILE_SUPPORTS_USN_JOURNAL: c_uint = 0x02000000;
pub const FILE_SUPPORTS_OPEN_BY_FILE_ID: c_uint = 0x01000000;
pub const FILE_SUPPORTS_EXTENDED_ATTRIBUTES: c_uint = 0x00800000;
pub const FILE_SUPPORTS_HARD_LINKS: c_uint = 0x00400000;
pub const FILE_SUPPORTS_TRANSACTIONS: c_uint = 0x00200000;
pub const FILE_SEQUENTIAL_WRITE_ONCE: c_uint = 0x00100000;
pub const FILE_READ_ONLY_VOLUME: c_uint = 0x00080000;
pub const FILE_NAMED_STREAMS: c_uint = 0x00040000;
pub const FILE_SUPPORTS_ENCRYPTION: c_uint = 0x00020000;
pub const FILE_SUPPORTS_OBJECT_IDS: c_uint = 0x00010000;
pub const FILE_VOLUME_IS_COMPRESSED: c_uint = 0x00008000;
pub const FILE_SUPPORTS_POSIX_UNLINK_RENAME: c_uint = 0x00000400;
pub const FILE_RETURNS_CLEANUP_RESULT_INFO: c_uint = 0x00000200;
pub const FILE_SUPPORTS_REMOTE_STORAGE: c_uint = 0x00000100;
pub const FILE_SUPPORTS_REPARSE_POINTS: c_uint = 0x00000080;
pub const FILE_SUPPORTS_SPARSE_FILES: c_uint = 0x00000040;
pub const FILE_VOLUME_QUOTAS: c_uint = 0x00000020;
pub const FILE_FILE_COMPRESSION: c_uint = 0x00000010;
pub const FILE_PERSISTENT_ACLS: c_uint = 0x00000008;
pub const FILE_UNICODE_ON_DISK: c_uint = 0x00000004;
pub const FILE_CASE_PRESERVED_NAMES: c_uint = 0x00000002;
pub const FILE_CASE_SENSITIVE_SEARCH: c_uint = 0x00000001;
//
// File System Control Information
// See MS-FSCC 2.5.2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_fs_control_info {
    pub FreeSpaceStartFiltering: __le64,
    pub FreeSpaceThreshold: __le64,
    pub FreeSpaceStopFiltering: __le64,
    pub DefaultQuotaThreshold: __le64,
    pub DefaultQuotaLimit: __le64,
    pub FileSystemControlFlags: __le32,
    pub Padding: __le32,
    pub __packed: },
// See MS-FSCC 2.5.4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_fs_full_size_info {
    pub TotalAllocationUnits: __le64,
    pub CallerAvailableAllocationUnits: __le64,
    pub ActualAvailableAllocationUnits: __le64,
    pub SectorsPerAllocationUnit: __le32,
    pub BytesPerSector: __le32,
    pub __packed: },
// See MS-FSCC 2.5.7
pub const SSINFO_FLAGS_ALIGNED_DEVICE: c_uint = 0x00000001;
pub const SSINFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: c_uint = 0x00000002;
pub const SSINFO_FLAGS_NO_SEEK_PENALTY: c_uint = 0x00000004;
pub const SSINFO_FLAGS_TRIM_ENABLED: c_uint = 0x00000008;
// sector size info struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_fs_ss_info {
    pub LogicalBytesPerSector: __le32,
    pub PhysicalBytesPerSectorForAtomicity: __le32,
    pub PhysicalBytesPerSectorForPerf: __le32,
    pub FSEffPhysicalBytesPerSectorForAtomicity: __le32,
    pub Flags: __le32,
    pub ByteOffsetForSectorAlignment: __le32,
    pub ByteOffsetForPartitionAlignment: __le32,
    pub __packed: },
// See MS-FSCC 2.5.8
    pub TotalAllocationUnits: __le64,
    pub AvailableAllocationUnits: __le64,
    pub SectorsPerAllocationUnit: __le32,
    pub BytesPerSector: __le32,
    pub /: *mut *mut } __packed FILE_SYSTEM_SIZE_INFO; / size info, level 0x103,
// volume info struct - see MS-FSCC 2.5.9
pub const MAX_VOL_LABEL_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filesystem_vol_info {
    pub VolumeCreationTime: __le64,
    pub VolumeSerialNumber: __le32,
    pub /: *mut *mut __le32 VolumeLabelLength; / includes trailing null,
    pub /: *mut *mut __u8 SupportsObjects; / True if eg like NTFS, supports objects,
    pub Reserved: __u8,
    pub /: *mut *mut __u8 VolumeLabel[]; / variable len,
    pub __packed: },
// See MS-FSCC 2.5.10
    pub DeviceType: __le32,
    pub DeviceCharacteristics: __le32,
    pub /: *mut *mut } __packed FILE_SYSTEM_DEVICE_INFO; / device info level 0x104,
//
// File Attributes
// See MS-FSCC 2.6
//
pub const FILE_ATTRIBUTE_READONLY: c_uint = 0x00000001;
pub const FILE_ATTRIBUTE_HIDDEN: c_uint = 0x00000002;
pub const FILE_ATTRIBUTE_SYSTEM: c_uint = 0x00000004;
pub const FILE_ATTRIBUTE_DIRECTORY: c_uint = 0x00000010;
pub const FILE_ATTRIBUTE_ARCHIVE: c_uint = 0x00000020;
pub const FILE_ATTRIBUTE_NORMAL: c_uint = 0x00000080;
pub const FILE_ATTRIBUTE_TEMPORARY: c_uint = 0x00000100;
pub const FILE_ATTRIBUTE_SPARSE_FILE: c_uint = 0x00000200;
pub const FILE_ATTRIBUTE_REPARSE_POINT: c_uint = 0x00000400;
pub const FILE_ATTRIBUTE_COMPRESSED: c_uint = 0x00000800;
pub const FILE_ATTRIBUTE_OFFLINE: c_uint = 0x00001000;
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: c_uint = 0x00002000;
pub const FILE_ATTRIBUTE_ENCRYPTED: c_uint = 0x00004000;
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: c_uint = 0x00008000;
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: c_uint = 0x00020000;

//
// SMB2 Notify Action Flags
// See MS-FSCC 2.7.1
//
pub const FILE_ACTION_ADDED: c_uint = 0x00000001;
pub const FILE_ACTION_REMOVED: c_uint = 0x00000002;
pub const FILE_ACTION_MODIFIED: c_uint = 0x00000003;
pub const FILE_ACTION_RENAMED_OLD_NAME: c_uint = 0x00000004;
pub const FILE_ACTION_RENAMED_NEW_NAME: c_uint = 0x00000005;
pub const FILE_ACTION_ADDED_STREAM: c_uint = 0x00000006;
pub const FILE_ACTION_REMOVED_STREAM: c_uint = 0x00000007;
pub const FILE_ACTION_MODIFIED_STREAM: c_uint = 0x00000008;
pub const FILE_ACTION_REMOVED_BY_DELETE: c_uint = 0x00000009;
pub const FILE_ACTION_ID_NOT_TUNNELLED: c_uint = 0x0000000A;
pub const FILE_ACTION_TUNNELLED_ID_COLLISION: c_uint = 0x0000000B;
//
// Response contains array of the following structures
// See MS-FSCC 2.7.1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_notify_information {
    pub NextEntryOffset: __le32,
    pub Action: __le32,
    pub FileNameLength: __le32,
    pub FileName: [__u8; ],
    pub __packed: },
//
// See POSIX Extensions to MS-FSCC 2.3.2.1
// Link: https://gitlab.com/samba-team/smb3-posix-spec/-/blob/master/fscc_posix_extensions.md
//
// For undefined recommended transfer size return -1 in that field
    pub /: *mut *mut __le32 OptimalTransferSize; / bsize on some os, iosize on other os,
    pub BlockSize: __le32,
// The next three fields are in terms of the block size.
// (above). If block size is unknown, 4096 would be a
// reasonable block size for a server to report.
// Note that returning the blocks/blocksavail removes need
// to make a second call (to QFSInfo level 0x103 to get this info.
// UserBlockAvail is typically less than or equal to BlocksAvail,
// if no distinction is made return the same value in each
//
    pub TotalBlocks: __le64,
    pub /: *mut *mut __le64 BlocksAvail; / bfree,
    pub /: *mut *mut __le64 UserBlocksAvail; / bavail,
// For undefined Node fields or FSID return -1
    pub TotalFileNodes: __le64,
    pub FreeFileNodes: __le64,
    pub /: *mut *mut __le64 FileSysIdentifier; / fsid,
// NB Namelen comes from FILE_SYSTEM_ATTRIBUTE_INFO call
// NB flags can come from FILE_SYSTEM_DEVICE_INFO call
    pub FILE_SYSTEM_POSIX_INFO: } __packed,
