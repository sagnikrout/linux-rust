//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/fscache.h
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
// NFS filesystem cache interface definitions
//
// Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Definition of the auxiliary data attached to NFS inode storage objects
// within the cache.
//
// The contents of this struct are recorded in the on-disk local cache in the
// auxiliary data attached to the data storage object backing an inode.  This
// permits coherency to be managed when a new inode binds to an already extant
// cache object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_fscache_inode_auxdata {
    pub mtime_sec: i64,
    pub mtime_nsec: i64,
    pub ctime_sec: i64,
    pub ctime_nsec: i64,
    pub change_attr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_netfs_io_data {
//
// NFS may split a netfs_io_subrequest into multiple RPCs, each
// with their own read completion.  In netfs, we can only call
// netfs_subreq_terminated() once for each subrequest.  Use the
// refcount here to double as a marker of the last RPC completion,
// and only call netfs via netfs_subreq_terminated() once.
//
    pub refcount: refcount_t,
    pub sreq: *mut netfs_io_subrequest,
//
// Final disposition of the netfs_io_subrequest, sent in
// netfs_subreq_terminated()
//
    pub transferred: core::sync::atomic::AtomicI64,
    pub error: c_int,
}

// Only the last RPC completion should call netfs_subreq_terminated()
//
// The NFS pageio interface may read a complete page, even when netfs
// only asked for a partial page.  Specifically, this may be seen when
// one thread is truncating a file while another one is reading the last
// page of the file.
// Correct the final length here to be no larger than the netfs subrequest
// length, and thus avoid netfs's "Subreq overread" warning message.
//
extern "C" {
    pub fn nfs_netfs_initiate_read(hdr: *mut nfs_pgio_header);
}
extern "C" {
    pub fn nfs_netfs_read_completion(hdr: *mut nfs_pgio_header);
}
extern "C" {
    pub fn nfs_netfs_folio_unlock(folio: *mut folio) -> c_int;
}
//
// fscache.c
//
extern "C" {
    pub fn nfs_fscache_get_super_cookie(: *mut super_block, : *const c_char, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nfs_fscache_release_super_cookie(: *mut super_block);
}
extern "C" {
    pub fn nfs_fscache_init_inode(: *mut inode);
}
extern "C" {
    pub fn nfs_fscache_clear_inode(: *mut inode);
}
extern "C" {
    pub fn nfs_fscache_open_file(: *mut inode, : *mut file);
}
extern "C" {
    pub fn nfs_fscache_release_file(: *mut inode, : *mut file);
}
extern "C" {
    pub fn nfs_netfs_readahead(ractl: *mut readahead_control) -> c_int;
}
extern "C" {
    pub fn nfs_netfs_read_folio(file: *mut file, folio: *mut folio) -> c_int;
}
//
// Invalidate the contents of fscache for this inode.  This will not sleep.
//
// indicate the client caching state as readable text
//

