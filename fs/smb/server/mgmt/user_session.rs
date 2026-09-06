//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/mgmt/user_session.h
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

pub const PREAUTH_HASHVALUE_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel {
    pub sess_key: [c_char; CIFS_KEY_SIZE],
    pub smb3signingkey: [__u8; SMB3_SIGN_KEY_SIZE],
    pub conn: *mut ksmbd_conn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct preauth_session {
    pub Preauth_HashValue: [__u8; PREAUTH_HASHVALUE_SIZE],
    pub id: u64,
    pub preauth_entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_session {
    pub id: u64,
    pub dialect: __u16,
    pub ClientGUID: [c_char; SMB2_CLIENT_GUID_SIZE],
    pub user: *mut ksmbd_user,
    pub sequence_number: c_uint,
    pub flags: c_uint,
    pub sign: bool,
    pub enc: bool,
    pub tearing_down: bool,
    pub state: c_int,
    pub Preauth_HashValue: *mut __u8,
    pub sess_key: [c_char; CIFS_KEY_SIZE],
    pub kerberos_expiry: u64,
    pub hlist: hlist_node,
    pub chann_lock: rw_semaphore,
    pub ksmbd_chann_list: xarray,
    pub tree_conns: xarray,
    pub tree_conn_ida: ida,
    pub rpc_handle_list: xarray,
    pub smb3encryptionkey: [__u8; SMB3_ENC_DEC_KEY_SIZE],
    pub smb3decryptionkey: [__u8; SMB3_ENC_DEC_KEY_SIZE],
    pub smb3signingkey: [__u8; SMB3_SIGN_KEY_SIZE],
    pub file_table: ksmbd_file_table,
    pub last_active: c_ulong,
    pub tree_conns_lock: rw_semaphore,

    pub proc_entry: *mut proc_dir_entry,

    pub refcnt: core::sync::atomic::AtomicI32,
    pub rpc_lock: rw_semaphore,
}

extern "C" {
    pub fn ksmbd_session_destroy(sess: *mut ksmbd_session);
}
extern "C" {
    pub fn ksmbd_sessions_deregister(conn: *mut ksmbd_conn);
}
extern "C" {
    pub fn ksmbd_preauth_session_destroy(conn: *mut ksmbd_conn);
}
extern "C" {
    pub fn ksmbd_acquire_tree_conn_id(sess: *mut ksmbd_session) -> c_int;
}
extern "C" {
    pub fn ksmbd_release_tree_conn_id(sess: *mut ksmbd_session, id: c_int);
}
extern "C" {
    pub fn ksmbd_session_rpc_open(sess: *mut ksmbd_session, rpc_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ksmbd_session_rpc_close(sess: *mut ksmbd_session, id: c_int);
}
extern "C" {
    pub fn ksmbd_session_rpc_method(sess: *mut ksmbd_session, id: c_int) -> c_int;
}
extern "C" {
    pub fn ksmbd_user_session_get(sess: *mut ksmbd_session);
}
extern "C" {
    pub fn ksmbd_user_session_put(sess: *mut ksmbd_session);
}
extern "C" {
    pub fn create_proc_sessions() -> c_int;
}
