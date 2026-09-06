//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/smb_common.h
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
// Copyright (C) 2018 Samsung Electronics Co., Ltd.
//

// ksmbd's Specific ERRNO
pub const ESHARE: c_int = 50000;
pub const SMB1_PROT: c_int = 0;
pub const SMB2_PROT: c_int = 1;
pub const SMB21_PROT: c_int = 2;
// multi-protocol negotiate request
pub const SMB2X_PROT: c_int = 3;
pub const SMB30_PROT: c_int = 4;
pub const SMB302_PROT: c_int = 5;
pub const SMB311_PROT: c_int = 6;
pub const BAD_PROT: c_uint = 0xFFFF;

pub const MAX_STREAM_PROT_LEN: c_uint = 0x00FFFFFF;
// Responses when opening a file.
pub const F_SUPERSEDED: c_int = 0;
pub const F_OPENED: c_int = 1;
pub const F_CREATED: c_int = 2;
pub const F_OVERWRITTEN: c_int = 3;
// Combinations of file access permission bits

// generic flags for file open

pub const SMB_COM_NEGOTIATE: c_uint = 0x72 /* See MS-CIFS 2.2.2.1 */;
// See MS-CIFS 2.2.3.1
pub const SMBFLG_RESPONSE: c_uint = 0x80	/* this PDU is a response from server */;
//
// See MS-CIFS 2.2.3.1
// MS-SMB 2.2.3.1
//

// See MS-CIFS 2.2.4.52.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_negotiate_rsp {
    pub /: *mut *mut smb_hdr hdr; / wct = 17,
    pub /: *mut *mut __le16 DialectIndex; / 0xFFFF = no dialect acceptable,
    pub ByteCount: __le16,
    pub __packed: },
pub const EXTENDED_INFO_MAGIC: c_uint = 0x43667364	/* Cfsd */;
pub const STRING_LENGTH: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_extended_info {
    pub magic: __le32,
    pub version: __le32,
    pub release: __le32,
    pub rel_date: __u64,
    pub version_string: [c_char; STRING_LENGTH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct object_id_info {
    pub objid: [c_char; 16],
    pub extended_info: fs_extended_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_names_info {
    pub NextEntryOffset: __le32,
    pub FileIndex: __u32,
    pub FileNameLength: __le32,
    pub FileName: [c_char; ],
    pub /: *mut *mut } __packed; / level 0xc FF resp data,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_id_both_directory_info {
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
    pub ShortNameLength: __u8,
    pub Reserved: __u8,
    pub ShortName: [__u8; 24],
    pub Reserved2: __le16,
    pub UniqueId: __le64,
    pub FileName: [c_char; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_version_ops {
    pub swork): *mut *mut u16 (get_cmd_val)(struct ksmbd_work,
    pub status): *mut *mut void (inc_reqs)(unsigned int cmd, __le32,
    pub swork): *mut *mut int (init_rsp_hdr)(struct ksmbd_work,
    pub err): *mut *mut *mut void (set_rsp_status)(struct ksmbd_work swork, __le32,
    pub work): *mut *mut int (allocate_rsp_buf)(struct ksmbd_work,
    pub work): *mut *mut int (set_rsp_credits)(struct ksmbd_work,
    pub work): *mut *mut int (check_user_session)(struct ksmbd_work,
    pub work): *mut *mut int (get_ksmbd_tcon)(struct ksmbd_work,
    pub command): *mut *mut *mut bool (is_sign_req)(struct ksmbd_work work, unsigned int,
    pub work): *mut *mut int (check_sign_req)(struct ksmbd_work,
    pub work): *mut *mut void (set_sign_rsp)(struct ksmbd_work,
    pub conn): *mut *mut *mut int (generate_signingkey)(struct ksmbd_session sess, struct ksmbd_conn,
    pub sess): *mut *mut *mut void (generate_encryptionkey)(struct ksmbd_conn conn, struct ksmbd_session,
    pub buf): *mut *mut bool (is_transform_hdr)(void,
    pub work): *mut *mut int (decrypt_req)(struct ksmbd_work,
    pub work): *mut *mut int (encrypt_resp)(struct ksmbd_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_version_cmds {
    pub swork): *mut *mut int (proc)(struct ksmbd_work,
}

extern "C" {
    pub fn ksmbd_min_protocol() -> c_int;
}
extern "C" {
    pub fn ksmbd_max_protocol() -> c_int;
}
extern "C" {
    pub fn ksmbd_lookup_protocol_idx(str: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ksmbd_verify_smb_message(work: *mut ksmbd_work) -> c_int;
}
extern "C" {
    pub fn ksmbd_smb_request(conn: *mut ksmbd_conn) -> bool;
}
extern "C" {
    pub fn ksmbd_lookup_dialect_by_id(cli_dialects: *mut __le16, dialects_count: __le16) -> c_int;
}
extern "C" {
    pub fn ksmbd_init_smb_server(conn: *mut ksmbd_conn) -> c_int;
}
extern "C" {
    pub fn ksmbd_smb_negotiate_common(work: *mut ksmbd_work, command: c_uint) -> c_int;
}
extern "C" {
    pub fn ksmbd_smb_check_shared_mode(filp: *mut file, curr_fp: *mut ksmbd_file) -> c_int;
}
extern "C" {
    pub fn ksmbd_override_fsids(work: *mut ksmbd_work) -> c_int;
}
extern "C" {
    pub fn ksmbd_revert_fsids(work: *mut ksmbd_work);
}
extern "C" {
    pub fn ksmbd_server_side_copy_max_chunk_count() -> c_uint;
}
extern "C" {
    pub fn ksmbd_server_side_copy_max_chunk_size() -> c_uint;
}
extern "C" {
    pub fn ksmbd_server_side_copy_max_total_size() -> c_uint;
}
extern "C" {
    pub fn is_asterisk(p: *mut c_char) -> bool;
}
extern "C" {
    pub fn smb_map_generic_desired_access(daccess: __le32) -> __le32;
}
//
// Get the body of the smb message excluding the 4 byte rfc1002 headers
// from request/response buffer.
//
