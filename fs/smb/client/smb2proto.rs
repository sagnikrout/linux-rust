//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/smb2proto.h
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
// Copyright (c) International Business Machines  Corp., 2002, 2011
// Etersoft, 2012
// Author(s): Steve French (sfrench@us.ibm.com)
// Pavel Shilovsky (pshilovsky@samba.org) 2012
//

//
// All Prototypes
//
extern "C" {
    pub fn map_smb2_to_linux_error(buf: *mut c_char, log_err: bool) -> c_int;
}
extern "C" {
    pub fn smb2_init_maperror() -> c_int;
}

extern "C" {
    pub fn smb2_calc_size(buf: *mut c_void) -> c_uint;
}
extern "C" {
    pub fn smb2_get_lease_state(cinode: *mut cifsInodeInfo, oplock: c_uint) -> __le32;
}
extern "C" {
    pub fn smb2_is_valid_oplock_break(buffer: *mut c_char, server: *mut TCP_Server_Info) -> bool;
}
extern "C" {
    pub fn smb2_push_mandatory_locks(cfile: *mut cifsFileInfo) -> c_int;
}
extern "C" {
    pub fn smb2_reconnect_server(work: *mut work_struct);
}
extern "C" {
    pub fn smb3_crypto_aead_allocate(server: *mut TCP_Server_Info) -> c_int;
}
extern "C" {
    pub fn smb2_set_next_command(tcon: *mut cifs_tcon, rqst: *mut smb_rqst);
}
extern "C" {
    pub fn smb2_set_related(rqst: *mut smb_rqst);
}
extern "C" {
    pub fn smb2_set_replay(server: *mut TCP_Server_Info, rqst: *mut smb_rqst);
}
//
// SMB2 Worker functions - most of protocol specific implementation details
// are contained within these calls.
//
extern "C" {
    pub fn SMB2_logoff(xid: c_uint, ses: *mut cifs_ses) -> c_int;
}
extern "C" {
    pub fn SMB2_tdis(xid: c_uint, tcon: *mut cifs_tcon) -> c_int;
}
extern "C" {
    pub fn SMB2_open_free(rqst: *mut smb_rqst);
}
extern "C" {
    pub fn SMB2_ioctl_free(rqst: *mut smb_rqst);
}
extern "C" {
    pub fn SMB2_close_free(rqst: *mut smb_rqst);
}
extern "C" {
    pub fn SMB2_flush_free(rqst: *mut smb_rqst);
}
extern "C" {
    pub fn SMB2_query_info_free(rqst: *mut smb_rqst);
}
extern "C" {
    pub fn smb2_async_readv(rdata: *mut cifs_io_subrequest) -> c_int;
}
extern "C" {
    pub fn smb2_async_writev(wdata: *mut cifs_io_subrequest);
}
extern "C" {
    pub fn SMB2_echo(server: *mut TCP_Server_Info) -> c_int;
}
extern "C" {
    pub fn SMB2_query_directory_free(rqst: *mut smb_rqst);
}
extern "C" {
    pub fn SMB2_set_info_free(rqst: *mut smb_rqst);
}
extern "C" {
    pub fn smb2_cancelled_close_fid(work: *mut work_struct);
}
extern "C" {
    pub fn smb3_validate_negotiate(xid: c_uint, tcon: *mut cifs_tcon) -> c_int;
}
extern "C" {
    pub fn smb3_encryption_required(tcon: *const cifs_tcon) -> c_int;
}
// query path info from the server using SMB311 POSIX extensions
extern "C" {
    pub fn posix_info_sid_size(beg: *const c_void, end: *const c_void) -> c_int;
}
