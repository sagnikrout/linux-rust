//! Automatically rewritten from C Header to Rust Module
//! Source: fs/9p/v9fs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// V9FS definitions.
//
// Copyright (C) 2004-2008 by Eric Van Hensbergen <ericvh@gmail.com>
// Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
//

//
// enum p9_session_flags - option flags for each 9P session
// @V9FS_PROTO_2000U: whether or not to use 9P2000.u extensions
// @V9FS_PROTO_2000L: whether or not to use 9P2000.l extensions
// @V9FS_ACCESS_SINGLE: only the mounting user can access the hierarchy
// @V9FS_ACCESS_USER: a new attach will be issued for every user (default)
// @V9FS_ACCESS_CLIENT: Just like user, but access check is performed on client.
// @V9FS_ACCESS_ANY: use a single attach for all users
// @V9FS_ACCESS_MASK: bit mask of different ACCESS options
// @V9FS_POSIX_ACL: POSIX ACLs are enforced
// @V9FS_NDENTRY_TIMEOUT_SET: Has negative dentry timeout retention time been
// overridden by negtimeout mount option
//
// Session flags reflect options selected by users at mount time
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p9_session_flags {
    V9FS_PROTO_2000U         = 0x01,
    V9FS_PROTO_2000L         = 0x02,
    V9FS_ACCESS_SINGLE       = 0x04,
    V9FS_ACCESS_USER         = 0x08,
    V9FS_ACCESS_CLIENT       = 0x10,
    V9FS_POSIX_ACL           = 0x20,
    V9FS_NO_XATTR            = 0x40,
    V9FS_IGNORE_QV           = 0x80, /* ignore qid.version for cache hints */
    V9FS_DIRECT_IO           = 0x100,
    V9FS_SYNC                = 0x200,
    V9FS_NDENTRY_TIMEOUT_SET = 0x400,
}

//
// enum p9_cache_shortcuts - human readable cache preferences
// @CACHE_SC_NONE: disable all caches
// @CACHE_SC_READAHEAD: only provide caching for readahead
// @CACHE_SC_MMAP: provide caching to enable mmap
// @CACHE_SC_LOOSE: non-coherent caching for files and meta data
// @CACHE_SC_FSCACHE: persistent non-coherent caching for files and meta-data
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p9_cache_shortcuts {
    CACHE_SC_NONE       = 0b00000000,
    CACHE_SC_READAHEAD  = 0b00000001,
    CACHE_SC_MMAP       = 0b00000101,
    CACHE_SC_LOOSE      = 0b00001111,
    CACHE_SC_FSCACHE    = 0b10001111,
}

//
// enum p9_cache_bits - possible values of ->cache
// @CACHE_NONE: caches disabled
// @CACHE_FILE: file caching (open to close)
// @CACHE_META: meta-data and directory caching
// @CACHE_WRITEBACK: write-back caching for files
// @CACHE_LOOSE: don't check cache consistency
// @CACHE_FSCACHE: local persistent caches
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p9_cache_bits {
    CACHE_NONE          = 0b00000000,
    CACHE_FILE          = 0b00000001,
    CACHE_META          = 0b00000010,
    CACHE_WRITEBACK     = 0b00000100,
    CACHE_LOOSE         = 0b00001000,
    CACHE_FSCACHE       = 0b10000000,
}

//
// struct v9fs_session_info - per-instance session information
// @flags: session options of type &p9_session_flags
// @nodev: set to 1 to disable device mapping
// @debug: debug level
// @afid: authentication handle
// @cache: cache mode of type &p9_cache_bits
// @ndentry_timeout: Negative dentry lookup cache retention time in ms
// @cachetag: the tag of the cache associated with this session
// @fscache: session cookie associated with FS-Cache
// @uname: string user name to mount hierarchy as
// @aname: mount specifier for remote hierarchy
// @maxdata: maximum data to be sent/recvd per protocol message
// @dfltuid: default numeric userid to mount hierarchy as
// @dfltgid: default numeric groupid to mount hierarchy as
// @uid: if %V9FS_ACCESS_SINGLE, the numeric uid which mounted the hierarchy
// @clnt: reference to 9P network client instantiated for this session
// @slist: reference to list of registered 9p sessions
// @ndentry_timeout_ms: Negative dentry caching retention time
//
// This structure holds state for each session instance established during
// a sys_mount() .
//
// Bugs: there seems to be a lot of state which could be condensed and/or
// removed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v9fs_session_info {
// options
    pub flags: c_uint,
    pub nodev: c_uchar,
    pub debug: c_ushort,
    pub afid: c_uint,
    pub cache: c_uint,
    pub ndentry_timeout_ms: c_uint,

    pub cachetag: *mut c_char,
    pub fscache: *mut fscache_volume,

    pub /: *mut *mut *mut char uname; / user name to mount as,
    pub /: *mut *mut *mut char aname; / name of remote hierarchy being mounted,
    pub /: *mut *mut unsigned int maxdata; / max data for client interface,
    pub /: *mut *mut kuid_t dfltuid; / default uid/muid for legacy support,
    pub /: *mut *mut kgid_t dfltgid; / default gid for legacy support,
    pub /: *mut *mut kuid_t uid; / if ACCESS_SINGLE, the uid that has access,
    pub /: *mut *mut *mut p9_client clnt; / 9p client,
    pub /: *mut *mut list_head slist; / list of sessions registered with v9fs,
    pub rename_sem: rw_semaphore,
    pub /: *mut *mut long session_lock_timeout; / retry interval for blocking locks,
}

// cache_validity flags
pub const V9FS_INO_INVALID_ATTR: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v9fs_inode {
    pub /: *mut *mut netfs_inode netfs; / Netfslib context and vfs inode,
    pub qid: p9_qid,
    pub cache_validity: c_uint,
    pub v_mutex: mutex,
}

extern "C" {
    pub fn container_of(_arg: inode, v9fs_inode: struct, _arg: netfs.inode) -> return;
}

extern "C" {
    pub fn netfs_i_cookie(_arg: &v9inode->netfs) -> return;
}

extern "C" {
    pub fn v9fs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int;
}
extern "C" {
    pub fn v9fs_show_options(m: *mut seq_file, root: *mut dentry) -> c_int;
}
extern "C" {
    pub fn v9fs_session_close(v9ses: *mut v9fs_session_info);
}
extern "C" {
    pub fn v9fs_session_cancel(v9ses: *mut v9fs_session_info);
}
extern "C" {
    pub fn v9fs_session_begin_cancel(v9ses: *mut v9fs_session_info);
}
extern "C" {
    pub fn v9fs_vfs_unlink(i: *mut inode, d: *mut dentry) -> c_int;
}
extern "C" {
    pub fn v9fs_vfs_rmdir(i: *mut inode, d: *mut dentry) -> c_int;
}
// other default globals
pub const V9FS_PORT: c_int = 564;

//
// v9fs_get_inode_from_fid - Helper routine to populate an inode by
// issuing a attribute request
// @v9ses: session information
// @fid: fid to issue attribute request for
// @sb: superblock on which to create inode
//
extern "C" {
    pub fn v9fs_inode_from_fid_dotl(_arg: v9ses, _arg: fid, _arg: sb, _arg: 0) -> return;
}
extern "C" {
    pub fn v9fs_inode_from_fid(_arg: v9ses, _arg: fid, _arg: sb, _arg: 0) -> return;
}
//
// v9fs_get_new_inode_from_fid - Helper routine to populate an inode by
// issuing a attribute request
// @v9ses: session information
// @fid: fid to issue attribute request for
// @sb: superblock on which to create inode
//
extern "C" {
    pub fn v9fs_inode_from_fid_dotl(_arg: v9ses, _arg: fid, _arg: sb, _arg: 1) -> return;
}
extern "C" {
    pub fn v9fs_inode_from_fid(_arg: v9ses, _arg: fid, _arg: sb, _arg: 1) -> return;
}
