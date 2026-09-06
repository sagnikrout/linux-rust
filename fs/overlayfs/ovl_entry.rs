//! Automatically rewritten from C Header to Rust Module
//! Source: fs/overlayfs/ovl_entry.h
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
// Copyright (C) 2011 Novell Inc.
// Copyright (C) 2016 Red Hat, Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_config {
    pub upperdir: *mut c_char,
    pub workdir: *mut c_char,
    pub lowerdirs: *mut c_char,
    pub default_permissions: bool,
    pub redirect_mode: c_int,
    pub verity_mode: c_int,
    pub index: bool,
    pub uuid: c_int,
    pub nfs_export: bool,
    pub xino: c_int,
    pub metacopy: bool,
    pub userxattr: bool,
    pub fsync_mode: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_sb {
    pub sb: *mut super_block,
    pub pseudo_dev: dev_t,
// Unusable (conflicting) uuid
    pub bad_uuid: bool,
// Used as a lower layer (but maybe also as upper)
    pub is_lower: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_layer {
// ovl_free_fs() relies on @mnt being the first member!
    pub mnt: *mut vfsmount,
// Trap in ovl inode cache
    pub trap: *mut inode,
    pub fs: *mut ovl_sb,
// Index of this layer in fs root (upper idx == 0)
    pub idx: c_int,
// One fsid per unique underlying sb (upper fsid == 0)
    pub fsid: c_int,
// xwhiteouts were found on this layer
    pub has_xwhiteouts: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_path {
    pub layer: *const ovl_layer,
    pub dentry: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_entry {
    pub __numlower: c_uint,
    pub __counted_by(__numlower): ovl_path __lowerstack[],
}

// private information held for overlayfs's superblock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_fs {
    pub numlayer: c_uint,
// Number of unique fs among layers including upper fs
    pub numfs: c_uint,
// Number of data-only lower layers
    pub numdatalayer: c_uint,
    pub layers: *mut ovl_layer,
    pub fs: *mut ovl_sb,
// workbasedir is the path at workdir= mount option
    pub workbasedir: *mut dentry,
// workdir is the 'work' or 'index' directory under workbasedir
    pub workdir: *mut dentry,
    pub namelen: c_long,
// pathnames of lower and upper dirs, for show_options
    pub config: ovl_config,
// creds of process who forced instantiation of super block
    pub creator_cred: *const cred,
    pub tmpfile: bool,
    pub noxattr: bool,
    pub nofh: bool,
// Did we take the inuse lock?
    pub upperdir_locked: bool,
    pub workdir_locked: bool,
// Traps in ovl inode cache
    pub workbasedir_trap: *mut inode,
    pub workdir_trap: *mut inode,
// -1: disabled, 0: same fs, 1..32: number of unused ino bits
    pub xino_mode: c_int,
// For allocation of non-persistent inode numbers
    pub last_ino: atomic_long_t,
// Shared whiteout cache
    pub whiteout: *mut dentry,
    pub no_shared_whiteout: bool,
    pub whiteout_lock: mutex,
// r/o snapshot of upperdir sb's only taken on volatile mounts
    pub errseq: errseq_t,
    pub casefold: bool,
}

// Number of lower layers, not including data-only layers
extern "C" {
    pub fn mnt_idmap(_arg: ovl_upper_mnt(ofs)) -> return;
}
extern "C" {
    pub fn ovl_lowerstack(_arg: oe) -> return;
}
// May return NULL if lazy lookup of lowerdata is needed
// private information held for every overlayfs dentry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovl_inode {
    pub /: *mut *mut *mut ovl_dir_cache cache; / directory,
    pub /: *const *const *const char lowerdata_redirect; / regular file,
}

// synchronize copy up and more
extern "C" {
    pub fn container_of(_arg: inode, ovl_inode: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn OVL_I_E(_arg: d_inode(dentry)) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: oi->__upperdentry) -> return;
}
