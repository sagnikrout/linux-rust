//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/dirtree.h
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
// Copyright (c) 2023-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// Each of these represents one parent pointer path step in a chain going
// up towards the directory tree root.  These are stored inside an xfarray.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_dirpath_step {
// Directory entry name associated with this parent link.
    pub name_cookie: xfblob_cookie,
    pub name_len: c_uint,
// Handle of the parent directory.
    pub pptr_rec: xfs_parent_rec,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xchk_dirpath_outcome {
    XCHK_DIRPATH_SCANNING = 0,	/* still being put together */
    XCHK_DIRPATH_DELETE,		/* delete this path */
    XCHK_DIRPATH_CORRUPT,		/* corruption detected in path */
    XCHK_DIRPATH_LOOP,		/* cycle detected further up */
    XCHK_DIRPATH_STALE,		/* path is stale */
    XCHK_DIRPATH_OK,		/* path reaches the root */

    XREP_DIRPATH_DELETING,		/* path is being deleted */
    XREP_DIRPATH_DELETED,		/* path has been deleted */
    XREP_DIRPATH_ADOPTING,		/* path is being adopted */
    XREP_DIRPATH_ADOPTED,		/* path has been adopted */
}

//
// Each of these represents one parent pointer path out of the directory being
// scanned.  These exist in-core, and hopefully there aren't more than a
// handful of them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_dirpath {
    pub list: list_head,
// Index of the first step in this path.
    pub first_step: xfarray_idx_t,
// Index of the second step in this path.
    pub second_step: xfarray_idx_t,
// Inodes seen while walking this path.
    pub seen_inodes: xino_bitmap,
// Number of steps in this path.
    pub nr_steps: c_uint,
// Which path is this?
    pub path_nr: c_uint,
// What did we conclude from following this path?
    pub outcome: xchk_dirpath_outcome,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_dirtree_outcomes {
// Number of XCHK_DIRPATH_DELETE
    pub bad: c_uint,
// Number of XCHK_DIRPATH_CORRUPT or XCHK_DIRPATH_LOOP
    pub suspect: c_uint,
// Number of XCHK_DIRPATH_OK
    pub good: c_uint,
// Directory needs to be added to lost+found
    pub needs_adoption: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_dirtree {
    pub sc: *mut xfs_scrub,
// Root inode that we're looking for.
    pub root_ino: xfs_ino_t,
//
// This is the inode that we're scanning.  The live update hook can
// continue to be called after xchk_teardown drops sc->ip but before
// it calls buf_cleanup, so we keep a copy.
//
    pub scan_ino: xfs_ino_t,
//
// If we start deleting redundant paths to this subdirectory, this is
// the inode number of the surviving parent and the dotdot entry will
// be set to this value.  If the value is NULLFSINO, then use @root_ino
// as a stand-in until the orphanage can adopt the subdirectory.
//
    pub parent_ino: xfs_ino_t,
// Scratch buffer for scanning pptr xattrs
    pub pptr_rec: xfs_parent_rec,
    pub pptr_args: xfs_da_args,
// Name buffer
    pub xname: xfs_name,
    pub namebuf: [c_char; MAXNAMELEN],
// Information for reparenting this directory.
    pub adoption: xrep_adoption,
//
// Hook into directory updates so that we can receive live updates
// from other writer threads.
//
    pub dhook: xfs_dir_hook,
// Parent pointer update arguments.
    pub ppargs: xfs_parent_args,
// lock for everything below here
    pub lock: mutex,
// buffer for the live update functions to use for dirent names
    pub hook_xname: xfs_name,
    pub hook_namebuf: [c_uchar; MAXNAMELEN],
//
// All path steps observed during this scan.  Each of the path
// steps for a particular pathwalk are recorded in sequential
// order in the xfarray.  A pathwalk ends either with a step
// pointing to the root directory (success) or pointing to NULLFSINO
// (loop detected, empty dir detected, etc).
//
    pub path_steps: *mut xfarray,
// All names observed during this scan.
    pub path_names: *mut xfblob,
// All paths being tracked by this scanner.
    pub path_list: list_head,
// Number of paths in path_list.
    pub nr_paths: c_uint,
// Number of parents found by a pptr scan.
    pub parents_found: c_uint,
// Have the path data been invalidated by a concurrent update?
    pub stale:1: bool,
// Has the scan been aborted?
    pub aborted:1: bool,
}

extern "C" {
    pub fn xchk_dirtree_parentless(dl: *const xchk_dirtree) -> bool;
}
extern "C" {
    pub fn xchk_dirtree_find_paths_to_root(dl: *mut xchk_dirtree) -> c_int;
}
