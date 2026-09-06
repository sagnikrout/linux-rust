//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/nfsfh.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 1995, 1996, 1997 Olaf Kirch <okir@monad.swb.de>
//
// This file describes the layout of the file handles as passed
// over the wire.
//

//
// The file handle starts with a sequence of four-byte words.
// The first word contains a version number (1) and three descriptor bytes
// that tell how the remaining 3 variable length fields should be handled.
// These three bytes are auth_type, fsid_type and fileid_type.
//
// All four-byte values are in host-byte-order.
//
// The auth_type field is deprecated and must be set to 0.
//
// The fsid_type identifies how the filesystem (or export point) is
// encoded.
// Current values:
// 0  - 4 byte device id (ms-2-bytes major, ls-2-bytes minor), 4byte inode number
// NOTE: we cannot use the kdev_t device id value, because kdev_t.h
// says we mustn't.  We must break it up and reassemble.
// 1  - 4 byte user specified identifier
// 2  - 4 byte major, 4 byte minor, 4 byte inode number - DEPRECATED
// 3  - 4 byte device id, encoded for user-space, 4 byte inode number
// 4  - 4 byte inode number and 4 byte uuid
// 5  - 8 byte uuid
// 6  - 16 byte uuid
// 7  - 8 byte inode number and 16 byte uuid
//
// The fileid_type identifies how the file within the filesystem is encoded.
// The values for this field are filesystem specific, exccept that
// filesystems must not use the values '0' or '0xff'. 'See enum fid_type'
// in include/linux/exportfs.h for currently registered values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knfsd_fh {
    pub /*: *mut unsigned int fh_size;,
// Points to the current size while
// building a new file handle.
//
    pub fh_raw: [u8; NFS4_FHSIZE],
}

//
// This is the internal representation of an NFS handle used in knfsd.
// pre_mtime/post_version will be used to support wcc_attr's in NFSv3.
//
// wcc data is not atomic with
// operation
//
// Pre-op attributes saved when inode is locked
//
// pre-op nfsv4 change attr: note must check IS_I_VERSION(inode)
// to find out if it is valid.
//
// Post-op attributes saved in fh_fill_post_attrs()

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfsd_fsid {
    FSID_DEV = 0,
    FSID_NUM,
    FSID_MAJOR_MINOR,
    FSID_ENCODE_DEV,
    FSID_UUID4_INUM,
    FSID_UUID8,
    FSID_UUID16,
    FSID_UUID16_INUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsid_source {
    FSIDSOURCE_DEV,
    FSIDSOURCE_FSID,
    FSIDSOURCE_UUID,
}

extern "C" {
    pub fn fsid_source(fhp: *const svc_fh) -> fsid_source;
}
//
// This might look a little large to "inline" but in all calls except
// one, 'vers' is constant so moste of the function disappears.
//
// In some cases the values are considered to be host endian and in
// others, net endian. fsidv is always considered to be u32 as the
// callers don't know which it will be. So we must use  to keep
// sparse from complaining. Since these values are opaque to the
// client, that shouldn't be a problem.
//
// 4 byte fsid and inode number
// 8 byte fsid
// 16 byte fsid - NFSv3+ only
// 8 byte inode and 16 byte fsid
// (u64*)fsidv = (u64)ino;
//
// Shorthand for dprintk()'s
//
extern "C" {
    pub fn SVCFH_fmt(fhp: *mut svc_fh) -> *mut c_char;
}
//
// Function prototypes
//
extern "C" {
    pub fn fh_verify(: *mut svc_rqst, : *mut svc_fh, _arg: umode_t, _arg: c_int) -> __be32;
}
extern "C" {
    pub fn fh_getattr(fhp: *const svc_fh, stat: *mut kstat) -> __be32;
}
extern "C" {
    pub fn fh_compose(: *mut svc_fh, : *mut svc_export, : *mut dentry, : *mut svc_fh) -> __be32;
}
extern "C" {
    pub fn fh_update(: *mut svc_fh) -> __be32;
}
extern "C" {
    pub fn fh_put(: *mut svc_fh);
}
extern "C" {
    pub fn fh_append_mac(fh: *mut knfsd_fh, fh_maxsize: c_int, net: *mut net) -> bool;
}
// dst = *src;
//
// fh_want_write - Get write access to an export
// @fhp: File handle of file to be written
//
// Caller must invoke fh_drop_write() when its write operation
// is complete.
//
// Returns 0 if the file handle's export can be written to. Otherwise
// the export is not prepared for updates, and the returned negative
// errno value reflects the reason for the failure.
//
// fh_drop_write - Release write access on an export
// @fhp: File handle of file on which fh_want_write() was previously called
//
// knfsd_fh_hash - calculate the crc32 hash for the filehandle
// @fh - pointer to filehandle
//
// returns a crc32 hash for the filehandle that is compatible with
// the one displayed by "wireshark".
//
// fh_clear_pre_post_attrs - Reset pre/post attributes
// @fhp: file handle to be updated
//
extern "C" {
    pub fn nfsd4_change_attribute(stat: *const kstat) -> u64;
}
extern "C" {
    pub fn fh_fill_pre_attrs(fhp: *mut svc_fh) -> __be32 __must_check;
}
extern "C" {
    pub fn fh_fill_post_attrs(fhp: *mut svc_fh) -> __be32;
}
extern "C" {
    pub fn fh_fill_both_attrs(fhp: *mut svc_fh) -> __be32 __must_check;
}
