//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/oplock.h
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
// Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
// Copyright (C) 2018 Samsung Electronics Co., Ltd.
//

// Oplock states
pub const OPLOCK_STATE_NONE: c_uint = 0x00;
pub const OPLOCK_ACK_WAIT: c_uint = 0x01;
pub const OPLOCK_CLOSING: c_uint = 0x02;
pub const OPLOCK_WRITE_TO_READ: c_uint = 0x01;
pub const OPLOCK_READ_HANDLE_TO_READ: c_uint = 0x02;
pub const OPLOCK_WRITE_TO_NONE: c_uint = 0x04;
pub const OPLOCK_READ_TO_NONE: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lease_ctx_info {
    pub lease_key: [__u8; SMB2_LEASE_KEY_SIZE],
    pub req_state: __le32,
    pub flags: __le32,
    pub duration: __le64,
    pub parent_lease_key: [__u8; SMB2_LEASE_KEY_SIZE],
    pub epoch: __le16,
    pub version: c_int,
    pub is_dir: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lease_table {
    pub client_guid: [c_char; SMB2_CLIENT_GUID_SIZE],
    pub conn: *mut ksmbd_conn,
    pub lease_list: list_head,
    pub l_entry: list_head,
    pub lb_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lease {
    pub lease_key: [__u8; SMB2_LEASE_KEY_SIZE],
    pub state: __le32,
    pub new_state: __le32,
    pub flags: __le32,
    pub duration: __le64,
    pub parent_lease_key: [__u8; SMB2_LEASE_KEY_SIZE],
    pub version: c_int,
    pub epoch: c_ushort,
    pub is_dir: bool,
    pub reuse_epoch: bool,
    pub ci: *mut ksmbd_inode,
    pub l_lb: *mut lease_table,
    pub l_entry: list_head,
    pub open_list: list_head,
    pub lock: spinlock_t,
    pub refcount: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oplock_info {
    pub conn: *mut ksmbd_conn,
    pub sess: *mut ksmbd_session,
    pub work: *mut ksmbd_work,
    pub o_fp: *mut ksmbd_file,
    pub level: c_int,
    pub op_state: c_int,
    pub state_lock: spinlock_t,
    pub pending_break: c_ulong,
    pub fid: u64,
    pub breaking_cnt: core::sync::atomic::AtomicI32,
    pub refcount: core::sync::atomic::AtomicI32,
    pub Tid: __u16,
    pub is_lease: bool,
    pub /: *mut *mut bool open_trunc; / truncate on open,
    pub o_lease: *mut lease,
    pub op_entry: list_head,
    pub lease_entry: list_head,
    pub /: *mut *mut wait_queue_head_t oplock_q; / Other server threads,
    pub /: *mut *mut wait_queue_head_t oplock_brk; / oplock breaking wait,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lease_break_info {
    pub curr_state: __le32,
    pub new_state: __le32,
    pub epoch: __le16,
    pub lease_key: [c_char; SMB2_LEASE_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oplock_break_info {
    pub level: c_int,
    pub open_trunc: c_int,
    pub fid: c_int,
}

extern "C" {
    pub fn smb_break_all_levII_oplock_rename(work: *mut ksmbd_work, fp: *mut ksmbd_file);
}
extern "C" {
    pub fn opinfo_write_to_read(opinfo: *mut oplock_info) -> c_int;
}
extern "C" {
    pub fn opinfo_read_handle_to_read(opinfo: *mut oplock_info) -> c_int;
}
extern "C" {
    pub fn opinfo_write_to_none(opinfo: *mut oplock_info) -> c_int;
}
extern "C" {
    pub fn opinfo_read_to_none(opinfo: *mut oplock_info) -> c_int;
}
extern "C" {
    pub fn close_id_del_oplock(fp: *mut ksmbd_file);
}
extern "C" {
    pub fn smb_break_all_oplock(work: *mut ksmbd_work, fp: *mut ksmbd_file);
}
extern "C" {
    pub fn opinfo_put(opinfo: *mut oplock_info);
}
// Lease related functions
extern "C" {
    pub fn create_lease_buf(rbuf: *mut u8, lease: *mut lease);
}
extern "C" {
    pub fn smb2_map_lease_to_oplock(lease_state: __le32) -> __u8;
}
extern "C" {
    pub fn lease_update_oplock_levels(lease: *mut lease);
}
extern "C" {
    pub fn lease_read_to_write(opinfo: *mut oplock_info) -> c_int;
}
// Durable related functions
extern "C" {
    pub fn create_durable_rsp_buf(cc: *mut c_char);
}
extern "C" {
    pub fn create_durable_v2_rsp_buf(cc: *mut c_char, fp: *mut ksmbd_file);
}
extern "C" {
    pub fn create_mxac_rsp_buf(cc: *mut c_char, maximal_access: c_int);
}
extern "C" {
    pub fn create_disk_id_rsp_buf(cc: *mut c_char, file_id: __u64, vol_id: __u64);
}
extern "C" {
    pub fn create_posix_rsp_buf(cc: *mut c_char, fp: *mut ksmbd_file);
}
extern "C" {
    pub fn destroy_lease_table(conn: *mut ksmbd_conn);
}
extern "C" {
    pub fn smb_lazy_parent_lease_break_close(fp: *mut ksmbd_file);
}
