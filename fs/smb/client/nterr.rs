//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/nterr.h
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
// NT status -> dos error map
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntstatus_to_dos_err {
    pub dos_class: __u8,
    pub dos_code: __u16,
    pub ntstatus: __u32,
    pub nt_errstr: *const c_char,
}

// Win32 Error Codes.
pub const NT_ERROR_INVALID_PARAMETER: c_uint = 0x0057;
pub const NT_ERROR_INSUFFICIENT_BUFFER: c_uint = 0x007a;
pub const NT_ERROR_INVALID_DATATYPE: c_uint = 0x070c;
//
// NTSTATUS Values extracted using a loop in smbclient then printing a netmon
// sniff to a file.
//
// The comment at the end of each definition indicates `dos_class`
// and `dos_code` fields of the `ntstatus_to_dos_map` array; it is
// used to generate the `smb1_mapping_table.c` file.
//
pub const NT_STATUS_OK: c_uint = 0x0000	// SUCCESS, 0;
pub const NT_STATUS_PENDING: c_uint = 0x0103	// ERRHRD, ERRgeneral;
pub const NT_STATUS_MORE_ENTRIES: c_uint = 0x0105	// ERRHRD, ERRgeneral;
pub const NT_STATUS_SOME_NOT_MAPPED: c_uint = 0x0107	// ERRHRD, ERRgeneral;
pub const NT_STATUS_NOTIFY_ENUM_DIR: c_uint = 0x010c	// ERRSRV, ERR_NOTIFY_ENUM_DIR;
pub const NT_STATUS_BUFFER_OVERFLOW: c_uint = 0x80000005	// ERRDOS, ERRmoredata;
pub const NT_STATUS_NO_MORE_ENTRIES: c_uint = 0x8000001a	// ERRHRD, ERRgeneral;
pub const NT_STATUS_MEDIA_CHANGED: c_uint = 0x8000001c	// ERRHRD, ERRgeneral;
pub const NT_STATUS_END_OF_MEDIA: c_uint = 0x8000001e	// ERRHRD, ERRgeneral;
pub const NT_STATUS_MEDIA_CHECK: c_uint = 0x80000020	// ERRHRD, ERRgeneral;
pub const NT_STATUS_NO_DATA_DETECTED: c_uint = 0x80000022	// ERRHRD, ERRgeneral;
pub const NT_STATUS_STOPPED_ON_SYMLINK: c_uint = 0x8000002d	// ERRDOS, ERRsymlink;
pub const NT_STATUS_DEVICE_REQUIRES_CLEANING: c_uint = 0x80000288	// ERRHRD, ERRgeneral;
pub const NT_STATUS_DEVICE_DOOR_OPEN: c_uint = 0x80000289	// ERRHRD, ERRgeneral;

// scheduler

pub const NT_STATUS_OS2_INVALID_LEVEL: c_uint = 0x007c0001	// ERRDOS, ERRunknownlevel;
