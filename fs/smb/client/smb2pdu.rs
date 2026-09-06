//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/smb2pdu.h
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
// Author(s): Steve French (sfrench@us.ibm.com)
// Pavel Shilovsky (pshilovsky@samba.org) 2012
//

// 52 transform hdr + 64 hdr + 88 create rsp
pub const SMB2_TRANSFORM_HEADER_SIZE: c_int = 52;
pub const MAX_SMB2_HDR_SIZE: c_int = 204;
// The total header size for SMB2 read and write

//
// Definitions for SMB2 Protocol Data Units (network frames)
//
// See MS-SMB2.PDF specification for protocol details.
// The Naming convention is the lower case version of the SMB2
// command code name for the struct. Note that structures must be packed.
//
pub const COMPOUND_FID: c_uint = 0xFFFFFFFFFFFFFFFFULL;

pub const SYMLINK_ERROR_TAG: c_uint = 0x4c4d5953;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_symlink_err_rsp {
    pub SymLinkLength: __le32,
    pub SymLinkErrorTag: __le32,
    pub ReparseTag: __le32,
    pub ReparseDataLength: __le16,
    pub UnparsedPathLength: __le16,
    pub SubstituteNameOffset: __le16,
    pub SubstituteNameLength: __le16,
    pub PrintNameOffset: __le16,
    pub PrintNameLength: __le16,
    pub Flags: __le32,
    pub PathBuffer: [__u8; ],
    pub __packed: },
// SMB 3.1.1 and later dialects. See MS-SMB2 section 2.2.2.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_error_context_rsp {
    pub ErrorDataLength: __le32,
    pub ErrorId: __le32,
    pub __counted_by_le(ErrorDataLength): __u8 ErrorContextData[],
    pub __packed: },
// ErrorId values
pub const SMB2_ERROR_ID_DEFAULT: c_uint = 0x00000000;

// Defines for Type field below (see MS-SMB2 2.2.2.2.2.1)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct move_dst_ipaddr {
    pub Type: __le32,
    pub Reserved: __u32,
    pub /: *mut *mut __u8 address[16]; / IPv4 followed by 12 bytes rsvd or IPv6 address,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct share_redirect_error_context_rsp {
    pub StructureSize: __le32,
    pub NotificationType: __le32,
    pub ResourceNameOffset: __le32,
    pub ResourceNameLength: __le32,
    pub Reserved: __le16,
    pub TargetType: __le16,
    pub IPAddrCount: __le32,
    pub IpAddrMoveList: [move_dst_ipaddr; ],
// __u8 ResourceName[] */ /* Name of share as counted Unicode string
    pub __packed: },
//
// Maximum number of iovs we need for an open/create request.
// [0] : struct smb2_create_req
// [1] : path
// [2] : lease context
// [3] : durable context
// [4] : posix context
// [5] : time warp context
// [6] : query id context
// [7] : create ea context
// [8] : compound padding
//
pub const SMB2_CREATE_IOV_SIZE: c_int = 9;
//
// Maximum size of a SMB2_CREATE response is 64 (smb2 header) +
// 88 (fixed part of create response) + 520 (path) + 208 (contexts) +
// 2 bytes of padding.
//
pub const MAX_SMB2_CREATE_RESPONSE_SIZE: c_int = 880;
pub const SMB2_LEASE_READ_CACHING_HE: c_uint = 0x01;
pub const SMB2_LEASE_HANDLE_CACHING_HE: c_uint = 0x02;
pub const SMB2_LEASE_WRITE_CACHING_HE: c_uint = 0x04;
// See MS-SMB2 2.2.13.2.5
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crt_twarp_ctxt {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub Timestamp: __le64,
    pub __packed: },
// See MS-SMB2 2.2.13.2.9
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crt_query_id_ctxt {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crt_sd_ctxt {
    pub ccontext: create_context_hdr,
    pub Name: [__u8; 8],
    pub sd: smb3_sd,
    pub __packed: },
// See MS-FSCC 2.3.29 and 2.3.30
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_retrieval_pointer_count_req {
    pub /: *mut *mut __le64 StartingVcn; / virtual cluster number (signed),
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_retrieval_pointer_count_rsp {
    pub ExtentCount: __le32,
    pub __packed: },
//
// See MS-FSCC 2.3.33 and 2.3.34
// request is the same as get_retrieval_point_count_req struct above
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_extents {
    pub NextVcn: __le64,
    pub /: *mut *mut __le64 Lcn; / logical cluster number,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_retrieval_pointers_refcount_rsp {
    pub ExtentCount: __le32,
    pub Reserved: __u32,
    pub StartingVcn: __le64,
    pub extents: [smb3_extents; ],
    pub __packed: },
// See MS-DFSC 2.2.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsctl_get_dfs_referral_req {
    pub MaxReferralLevel: __le16,
    pub RequestFileName: [__u8; ],
    pub __packed: },
// DFS response is struct get_dfs_refer_rsp
// See MS-SMB2 2.2.31.3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct network_resiliency_req {
    pub Timeout: __le32,
    pub Reserved: __le32,
    pub __packed: },
// There is no buffer for the response ie no struct network_resiliency_rsp
pub const NO_FILE_ID: c_uint = 0xFFFFFFFFFFFFFFFFULL /* general ioctls to srv not to file */;
//
// Maximum number of iovs we need for an ioctl request.
// [0] : struct smb2_ioctl_req
// [1] : in_data
//
pub const SMB2_IOCTL_IOV_SIZE: c_int = 2;
//
// PDU query infolevel structure definitions
// BB consider moving to a different header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_full_ea_info {
    pub next_entry_offset: __le32,
    pub flags: __u8,
    pub ea_name_length: __u8,
    pub ea_value_length: __le16,
    pub /: *mut *mut char ea_data[]; / \0 terminated name plus value,
    pub /: *mut *mut } __packed; / level 15 Set,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_reparse_point_info {
    pub IndexNumber: __le64,
    pub Tag: __le32,
    pub __packed: },
// See MS-FSCC 2.4.26
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_id_information {
    pub VolumeSerialNumber: __le64,
    pub /: *mut *mut __u64 PersistentFileId; / opaque endianness,
    pub /: *mut *mut __u64 VolatileFileId; / opaque endianness,
    pub /: *mut *mut } __packed; / level 59,
// See MS-FSCC 2.4.18
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_file_id_extd_directory_info {
    pub NextEntryOffset: __le32,
    pub FileIndex: __u32,
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub EndOfFile: __le64,
    pub AllocationSize: __le64,
    pub FileAttributes: __le32,
    pub FileNameLength: __le32,
    pub /: *mut *mut __le32 EaSize; / EA size,
    pub /: *mut *mut __le32 ReparsePointTag; / valid if FILE_ATTR_REPARSE_POINT set in FileAttributes,
    pub /: *mut *mut __le64 UniqueId; / inode num - le since Samba puts ino in low 32 bit,
    pub FileName: [c_char; ],
    pub /: *mut *mut } __packed; / level 60,
    pub smb2_padding: [extern char; 7],
//
// See POSIX-SMB2 2.2.14.2.16
// Link: https://gitlab.com/samba-team/smb3-posix-spec/-/blob/master/smb3_posix_extensions.md
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct create_posix_rsp {
    pub nlink: u32,
    pub reparse_tag: u32,
    pub mode: u32,
    pub /: *mut *mut smb_sid owner; / var-sized on the wire,
    pub /: *mut *mut smb_sid group; / var-sized on the wire,
    pub __packed: },
pub const SMB2_QUERY_DIRECTORY_IOV_SIZE: c_int = 2;
//
// SMB2-only POSIX info level for query dir
//
// See posix_info_sid_size(), posix_info_extra_size() and
// posix_info_parse() to help with the handling of this struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_posix_info {
    pub NextEntryOffset: __le32,
    pub Ignored: __u32,
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub EndOfFile: __le64,
    pub AllocationSize: __le64,
    pub DosAttributes: __le32,
    pub Inode: __le64,
    pub DeviceId: __le32,
    pub Zero: __le32,
// beginning of POSIX Create Context Response
    pub HardLinks: __le32,
    pub ReparseTag: __le32,
    pub Mode: __le32,
//
// var sized owner SID
// var sized group SID
// le32 filenamelength
// u8  filename[]
//
    pub __packed: },
//
// Parsed version of the above struct. Allows direct access to the
// variable length fields
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_posix_info_parsed {
    pub base: *const smb2_posix_info,
    pub size: usize,
    pub owner: smb_sid,
    pub group: smb_sid,
    pub name_len: c_int,
    pub name: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb2_create_ea_ctx {
    pub ctx: create_context_hdr,
    pub name: [__u8; 8],
    pub ea: smb2_file_full_ea_info,
    pub __packed: },

pub const SMB2_WSL_XATTR_NAME_LEN: c_int = 6;
pub const SMB2_WSL_NUM_XATTRS: c_int = 4;
pub const SMB2_WSL_XATTR_UID_SIZE: c_int = 4;
pub const SMB2_WSL_XATTR_GID_SIZE: c_int = 4;
pub const SMB2_WSL_XATTR_MODE_SIZE: c_int = 4;
pub const SMB2_WSL_XATTR_DEV_SIZE: c_int = 8;

