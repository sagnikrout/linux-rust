//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/cifs_ioctl.h
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
// Structure definitions for io control for cifs/smb3
//
// Copyright (c) 2015 Steve French <steve.french@primarydata.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_mnt_fs_info {
    pub /: *mut *mut __u32 version; / 0001,
    pub protocol_id: __u16,
    pub tcon_flags: __u16,
    pub vol_serial_number: __u32,
    pub vol_create_time: __u32,
    pub share_caps: __u32,
    pub share_flags: __u32,
    pub sector_flags: __u32,
    pub optimal_sector_size: __u32,
    pub max_bytes_chunk: __u32,
    pub fs_attributes: __u32,
    pub max_path_component: __u32,
    pub device_type: __u32,
    pub device_characteristics: __u32,
    pub maximal_access: __u32,
    pub cifs_posix_caps: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_mnt_tcon_info {
    pub tid: __u32,
    pub session_id: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_snapshot_array {
    pub number_of_snapshots: __u32,
    pub number_of_snapshots_returned: __u32,
    pub snapshot_array_size: __u32,
// snapshots[];
    pub __packed: },
// query_info flags
pub const PASSTHRU_QUERY_INFO: c_uint = 0x00000000;
pub const PASSTHRU_FSCTL: c_uint = 0x00000001;
pub const PASSTHRU_SET_INFO: c_uint = 0x00000002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb_query_info {
    pub info_type: __u32,
    pub file_info_class: __u32,
    pub additional_information: __u32,
    pub flags: __u32,
    pub input_buffer_length: __u32,
    pub output_buffer_length: __u32,
// char buffer[];
    pub __packed: },
//
// Dumping the commonly used 16 byte (e.g. CCM and GCM128) keys still supported
// for backlevel compatibility, but is not sufficient for dumping the less
// frequently used GCM256 (32 byte) keys (see the newer "CIFS_DUMP_FULL_KEY"
// ioctl for dumping decryption info for GCM256 mounts)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_key_debug_info {
    pub Suid: __u64,
    pub cipher_type: __u16,
    pub auth_key: [__u8; SMB2_NTLMV2_SESSKEY_SIZE],
    pub smb3encryptionkey: [__u8; SMB3_SIGN_KEY_SIZE],
    pub smb3decryptionkey: [__u8; SMB3_SIGN_KEY_SIZE],
    pub __packed: },
//
// Dump variable-sized keys
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_full_key_debug_info {
// INPUT: size of userspace buffer
    pub in_size: __u32,
//
// INPUT: 0 for current user, otherwise session to dump
// OUTPUT: session id that was dumped
//
    pub session_id: __u64,
    pub cipher_type: __u16,
    pub session_key_length: __u8,
    pub server_in_key_length: __u8,
    pub server_out_key_length: __u8,
    pub data: [__u8; ],
//
// return this struct with the keys appended at the end:
// __u8 session_key[session_key_length];
// __u8 server_in_key[server_in_key_length];
// __u8 server_out_key[server_out_key_length];
//
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_notify {
    pub completion_filter: __u32,
    pub watch_tree: bool,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smb3_notify_info {
    pub completion_filter: __u32,
    pub watch_tree: bool,
    pub /: *mut *mut __u32 data_len; / size of notify data below,
    pub notify_data: [__u8; ],
    pub __packed: },
pub const CIFS_IOCTL_MAGIC: c_uint = 0xCF;

//
// Flags for going down operation
//
pub const CIFS_GOING_FLAGS_DEFAULT: c_uint = 0x0     /* going down */;
pub const CIFS_GOING_FLAGS_LOGFLUSH: c_uint = 0x1     /* flush log but not data */;
pub const CIFS_GOING_FLAGS_NOLOGFLUSH: c_uint = 0x2     /* don't flush log nor data */;
