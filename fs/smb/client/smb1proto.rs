//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/smb1proto.h
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
// Copyright (c) International Business Machines  Corp., 2002,2008
// Author(s): Steve French (sfrench@us.ibm.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cifs_unix_set_info_args {
    pub ctime: __u64,
    pub atime: __u64,
    pub mtime: __u64,
    pub mode: __u64,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub device: dev_t,
}

//
// cifssmb.c
//
extern "C" {
    pub fn CIFSSMBTDis(xid: c_uint, tcon: *mut cifs_tcon) -> c_int;
}
extern "C" {
    pub fn CIFSSMBEcho(server: *mut TCP_Server_Info) -> c_int;
}
extern "C" {
    pub fn CIFSSMBLogoff(xid: c_uint, ses: *mut cifs_ses) -> c_int;
}
extern "C" {
    pub fn cifs_async_readv(rdata: *mut cifs_io_subrequest) -> c_int;
}
extern "C" {
    pub fn cifs_async_writev(wdata: *mut cifs_io_subrequest);
}
extern "C" {
    pub fn CIFSSMBQFSAttributeInfo(xid: c_uint, tcon: *mut cifs_tcon) -> c_int;
}
extern "C" {
    pub fn CIFSSMBQFSDeviceInfo(xid: c_uint, tcon: *mut cifs_tcon) -> c_int;
}
extern "C" {
    pub fn CIFSSMBQFSUnixInfo(xid: c_uint, tcon: *mut cifs_tcon) -> c_int;
}
//
// smb1debug.c
//
// smb1encrypt.c
//
// smb1maperror.c
//
extern "C" {
    pub fn map_smb_to_linux_error(buf: *mut c_char, logErr: bool) -> c_int;
}
extern "C" {
    pub fn smb1_init_maperror() -> c_int;
}

//
// smb1misc.c
//
extern "C" {
    pub fn is_valid_oplock_break(buffer: *mut c_char, srv: *mut TCP_Server_Info) -> bool;
}
extern "C" {
    pub fn smbCalcSize(buf: *mut c_void) -> c_uint;
}
//
// smb1ops.c
//
// smb1session.c
//
// smb1transport.c
//
extern "C" {
    pub fn le16_to_cpu(_arg: smb->Mid) -> return;
}

// given a pointer to an smb_hdr, retrieve a void pointer to the ByteCount
// given a pointer to an smb_hdr retrieve the pointer to the byte area

// get the unconverted ByteCount for a SMB packet and return it
extern "C" {
    pub fn get_unaligned_le16(_arg: bc_ptr) -> return;
}
// set the ByteCount for a SMB packet in little-endian

