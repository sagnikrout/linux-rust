//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/common/smbfsctl.h
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


// SPDX-License-Identifier: LGPL-2.1+
//
// SMB, CIFS, SMB2 FSCTL definitions
//
// Copyright (c) International Business Machines  Corp., 2002,2013
// Author(s): Steve French (sfrench@us.ibm.com)
//
// IOCTL information
//
// List of ioctl/fsctl function codes that are or could be useful in the
// future to remote clients like cifs or SMB2/SMB3 client.  This is probably
// a slightly larger set of fsctls that NTFS local filesystem could handle,
// including the seven below that we do not have struct definitions for.
// Even with protocol definitions for most of these now available, we still
// need to do some experimentation to identify which are practical to do
// remotely.  Some of the following, such as the encryption/compression ones
// could be invoked from tools via a specialized hook into the VFS rather
// than via the standard vfs entry points
//
// See MS-SMB2 Section 2.2.31 (last checked September 2021, all of that list are
// below). Additional detail on less common ones can be found in MS-FSCC
// section 2.3.
//
// FSCTL values are 32 bits and are constructed as
// <device 16bits> <access 2bits> <function 12bits> <method 2bits>
//
// Device

pub const FSCTL_DEVICE_MASK: c_uint = 0xffff0000;
// Access

pub const FSCTL_DEVICE_ACCESS_MASK: c_uint = 0x0000c000;
// Function
pub const FSCTL_DEVICE_FUNCTION_MASK: c_uint = 0x00003ffc;
// Method
pub const FSCTL_DEVICE_METHOD_BUFFERED: c_uint = 0x00;
pub const FSCTL_DEVICE_METHOD_IN_DIRECT: c_uint = 0x01;
pub const FSCTL_DEVICE_METHOD_OUT_DIRECT: c_uint = 0x02;
pub const FSCTL_DEVICE_METHOD_NEITHER: c_uint = 0x03;
pub const FSCTL_DEVICE_METHOD_MASK: c_uint = 0x00000003;
pub const FSCTL_DFS_GET_REFERRALS: c_uint = 0x00060194;
pub const FSCTL_DFS_GET_REFERRALS_EX: c_uint = 0x000601B0;
pub const FSCTL_REQUEST_OPLOCK_LEVEL_1: c_uint = 0x00090000;
pub const FSCTL_REQUEST_OPLOCK_LEVEL_2: c_uint = 0x00090004;
pub const FSCTL_REQUEST_BATCH_OPLOCK: c_uint = 0x00090008;
pub const FSCTL_LOCK_VOLUME: c_uint = 0x00090018;
pub const FSCTL_UNLOCK_VOLUME: c_uint = 0x0009001C;
pub const FSCTL_IS_PATHNAME_VALID: c_uint = 0x0009002C /* BB add struct */;
pub const FSCTL_GET_COMPRESSION: c_uint = 0x0009003C;
pub const FSCTL_SET_COMPRESSION: c_uint = 0x0009C040;
pub const FSCTL_QUERY_FAT_BPB: c_uint = 0x00090058 /* BB add struct */;
// Verify the next FSCTL number, we had it as 0x00090090 before
pub const FSCTL_FILESYSTEM_GET_STATS: c_uint = 0x00090060 /* BB add struct */;
pub const FSCTL_GET_NTFS_VOLUME_DATA: c_uint = 0x00090064 /* BB add struct */;
pub const FSCTL_GET_RETRIEVAL_POINTERS: c_uint = 0x00090073 /* BB add struct */;
pub const FSCTL_IS_VOLUME_DIRTY: c_uint = 0x00090078 /* BB add struct */;
pub const FSCTL_ALLOW_EXTENDED_DASD_IO: c_uint = 0x00090083 /* BB add struct */;
pub const FSCTL_REQUEST_FILTER_OPLOCK: c_uint = 0x0009008C;
pub const FSCTL_FIND_FILES_BY_SID: c_uint = 0x0009008F /* BB add struct */;
pub const FSCTL_SET_OBJECT_ID: c_uint = 0x00090098 /* BB add struct */;
pub const FSCTL_GET_OBJECT_ID: c_uint = 0x0009009C /* BB add struct */;
pub const FSCTL_DELETE_OBJECT_ID: c_uint = 0x000900A0 /* BB add struct */;
pub const FSCTL_SET_REPARSE_POINT: c_uint = 0x000900A4 /* BB add struct */;
pub const FSCTL_GET_REPARSE_POINT: c_uint = 0x000900A8 /* BB add struct */;
pub const FSCTL_DELETE_REPARSE_POINT: c_uint = 0x000900AC /* BB add struct */;
pub const FSCTL_SET_OBJECT_ID_EXTENDED: c_uint = 0x000900BC /* BB add struct */;
pub const FSCTL_CREATE_OR_GET_OBJECT_ID: c_uint = 0x000900C0 /* BB add struct */;
pub const FSCTL_SET_SPARSE: c_uint = 0x000900C4 /* BB add struct */;
pub const FSCTL_SET_ZERO_DATA: c_uint = 0x000980C8;
pub const FSCTL_SET_ENCRYPTION: c_uint = 0x000900D7 /* BB add struct */;
pub const FSCTL_ENCRYPTION_FSCTL_IO: c_uint = 0x000900DB /* BB add struct */;
pub const FSCTL_WRITE_RAW_ENCRYPTED: c_uint = 0x000900DF /* BB add struct */;
pub const FSCTL_READ_RAW_ENCRYPTED: c_uint = 0x000900E3 /* BB add struct */;
pub const FSCTL_READ_FILE_USN_DATA: c_uint = 0x000900EB /* BB add struct */;
pub const FSCTL_WRITE_USN_CLOSE_RECORD: c_uint = 0x000900EF /* BB add struct */;
pub const FSCTL_MARK_HANDLE: c_uint = 0x000900FC /* BB add struct */;
pub const FSCTL_SIS_COPYFILE: c_uint = 0x00090100 /* BB add struct */;
pub const FSCTL_RECALL_FILE: c_uint = 0x00090117 /* BB add struct */;
pub const FSCTL_QUERY_SPARING_INFO: c_uint = 0x00090138 /* BB add struct */;
pub const FSCTL_QUERY_ON_DISK_VOLUME_INFO: c_uint = 0x0009013C;
pub const FSCTL_SET_ZERO_ON_DEALLOC: c_uint = 0x00090194 /* BB add struct */;
pub const FSCTL_SET_SHORT_NAME_BEHAVIOR: c_uint = 0x000901B4 /* BB add struct */;
pub const FSCTL_GET_INTEGRITY_INFORMATION: c_uint = 0x0009027C;
pub const FSCTL_QUERY_FILE_REGIONS: c_uint = 0x00090284;
pub const FSCTL_GET_REFS_VOLUME_DATA: c_uint = 0x000902D8 /* See MS-FSCC 2.3.24 */;
pub const FSCTL_SET_INTEGRITY_INFORMATION_EXT: c_uint = 0x00090380;
pub const FSCTL_GET_RETRIEVAL_POINTERS_AND_REFCOUNT: c_uint = 0x000903d3;
pub const FSCTL_GET_RETRIEVAL_POINTER_COUNT: c_uint = 0x0009042b;
pub const FSCTL_REFS_STREAM_SNAPSHOT_MANAGEMENT: c_uint = 0x00090440;
pub const FSCTL_QUERY_ALLOCATED_RANGES: c_uint = 0x000940CF;
pub const FSCTL_OFFLOAD_READ: c_uint = 0x00094264 /* BB add struct */;
pub const FSCTL_OFFLOAD_WRITE: c_uint = 0x00098268 /* BB add struct */;
pub const FSCTL_SET_DEFECT_MANAGEMENT: c_uint = 0x00098134 /* BB add struct */;
pub const FSCTL_FILE_LEVEL_TRIM: c_uint = 0x00098208 /* BB add struct */;
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE: c_uint = 0x00098344;
pub const FSCTL_DUPLICATE_EXTENTS_TO_FILE_EX: c_uint = 0x000983E8;
pub const FSCTL_SIS_LINK_FILES: c_uint = 0x0009C104;
pub const FSCTL_SET_INTEGRITY_INFORMATION: c_uint = 0x0009C280;
pub const FSCTL_PIPE_PEEK: c_uint = 0x0011400C /* BB add struct */;
pub const FSCTL_PIPE_TRANSCEIVE: c_uint = 0x0011C017 /* BB add struct */;
// strange that the number for this op is not sequential with previous op
pub const FSCTL_PIPE_WAIT: c_uint = 0x00110018 /* BB add struct */;
// Enumerate previous versions of a file
pub const FSCTL_SRV_ENUMERATE_SNAPSHOTS: c_uint = 0x00144064;
// Retrieve an opaque file reference for server-side data movement ie copy
pub const FSCTL_SRV_REQUEST_RESUME_KEY: c_uint = 0x00140078;
pub const FSCTL_SRV_ENUM_SNAPS: c_uint = 0x00144064;
pub const FSCTL_LMR_REQUEST_RESILIENCY: c_uint = 0x001401D4;
pub const FSCTL_LMR_GET_LINK_TRACK_INF: c_uint = 0x001400E8 /* BB add struct */;
pub const FSCTL_LMR_SET_LINK_TRACK_INF: c_uint = 0x001400EC /* BB add struct */;
pub const FSCTL_VALIDATE_NEGOTIATE_INFO: c_uint = 0x00140204;
// Perform server-side data movement
pub const FSCTL_SRV_COPYCHUNK: c_uint = 0x001440F2;
pub const FSCTL_SRV_COPYCHUNK_WRITE: c_uint = 0x001480F2;
pub const FSCTL_QUERY_NETWORK_INTERFACE_INFO: c_uint = 0x001401FC /* BB add struct */;
pub const FSCTL_SRV_READ_HASH: c_uint = 0x001441BB /* BB add struct */;
// See FSCC 2.1.2.5
pub const IO_REPARSE_TAG_MOUNT_POINT: c_uint = 0xA0000003;
pub const IO_REPARSE_TAG_HSM: c_uint = 0xC0000004;
pub const IO_REPARSE_TAG_SIS: c_uint = 0x80000007;
pub const IO_REPARSE_TAG_HSM2: c_uint = 0x80000006;
pub const IO_REPARSE_TAG_DRIVER_EXTENDER: c_uint = 0x80000005;
// Used by the DFS filter. See MS-DFSC
pub const IO_REPARSE_TAG_DFS: c_uint = 0x8000000A;
// Used by the DFS filter See MS-DFSC
pub const IO_REPARSE_TAG_DFSR: c_uint = 0x80000012;
pub const IO_REPARSE_TAG_FILTER_MANAGER: c_uint = 0x8000000B;
// Native SMB symlinks since Windows Vista, see MS-FSCC 2.1.2.4
pub const IO_REPARSE_TAG_SYMLINK: c_uint = 0xA000000C;
pub const IO_REPARSE_TAG_DEDUP: c_uint = 0x80000013;
pub const IO_REPARSE_APPXSTREAM: c_uint = 0xC0000014;
// NFS special files used by Windows NFS server since Windows Server 2012, see MS-FSCC 2.1.2.6
pub const IO_REPARSE_TAG_NFS: c_uint = 0x80000014;
//
// AzureFileSync - see
// https://docs.microsoft.com/en-us/azure/storage/files/storage-sync-cloud-tiering
//
pub const IO_REPARSE_TAG_AZ_FILE_SYNC: c_uint = 0x8000001e;
// Native Win32 AF_UNIX sockets since Windows 10 April 2018 Update, used also by WSL
pub const IO_REPARSE_TAG_AF_UNIX: c_uint = 0x80000023;
// WSL reparse tags
pub const IO_REPARSE_TAG_LX_SYMLINK: c_uint = 0xA000001D;
pub const IO_REPARSE_TAG_LX_FIFO: c_uint = 0x80000024;
pub const IO_REPARSE_TAG_LX_CHR: c_uint = 0x80000025;
pub const IO_REPARSE_TAG_LX_BLK: c_uint = 0x80000026;

// If Name Surrogate Bit is set, the file or directory represents another named entity in the system.

// fsctl flags
// If Flags is set to this value, the request is an FSCTL not ioctl request
pub const SMB2_0_IOCTL_IS_FSCTL: c_uint = 0x00000001;
