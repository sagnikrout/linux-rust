//! Automatically rewritten from C Header to Rust Module
//! Source: fs/afs/afs_fs.h
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
// AFS File Service definitions
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AFS_FS_Operations {
    FSFETCHDATA		= 130,	/* AFS Fetch file data */
    FSFETCHACL		= 131,	/* AFS Fetch file ACL */
    FSFETCHSTATUS		= 132,	/* AFS Fetch file status */
    FSSTOREDATA		= 133,	/* AFS Store file data */
    FSSTOREACL		= 134,	/* AFS Store file ACL */
    FSSTORESTATUS		= 135,	/* AFS Store file status */
    FSREMOVEFILE		= 136,	/* AFS Remove a file */
    FSCREATEFILE		= 137,	/* AFS Create a file */
    FSRENAME		= 138,	/* AFS Rename or move a file or directory */
    FSSYMLINK		= 139,	/* AFS Create a symbolic link */
    FSLINK			= 140,	/* AFS Create a hard link */
    FSMAKEDIR		= 141,	/* AFS Create a directory */
    FSREMOVEDIR		= 142,	/* AFS Remove a directory */
    FSGIVEUPCALLBACKS	= 147,	/* AFS Discard callback promises */
    FSGETVOLUMEINFO		= 148,	/* AFS Get information about a volume */
    FSGETVOLUMESTATUS	= 149,	/* AFS Get volume status information */
    FSGETROOTVOLUME		= 151,	/* AFS Get root volume name */
    FSBULKSTATUS		= 155,	/* AFS Fetch multiple file statuses */
    FSSETLOCK		= 156,	/* AFS Request a file lock */
    FSEXTENDLOCK		= 157,	/* AFS Extend a file lock */
    FSRELEASELOCK		= 158,	/* AFS Release a file lock */
    FSLOOKUP		= 161,	/* AFS lookup file in directory */
    FSINLINEBULKSTATUS	= 65536, /* AFS Fetch multiple file statuses with inline errors */
    FSFETCHDATA64		= 65537, /* AFS Fetch file data */
    FSSTOREDATA64		= 65538, /* AFS Store file data */
    FSGIVEUPALLCALLBACKS	= 65539, /* AFS Give up all outstanding callbacks on a server */
    FSGETCAPABILITIES	= 65540, /* Probe and get the capabilities of a fileserver */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AFS_FS_Errors {
    VRESTARTING	= -100,	/* Server is restarting */
    VSALVAGE	= 101,	/* volume needs salvaging */
    VNOVNODE	= 102,	/* no such file/dir (vnode) */
    VNOVOL		= 103,	/* no such volume or volume unavailable */
    VVOLEXISTS	= 104,	/* volume name already exists */
    VNOSERVICE	= 105,	/* volume not currently in service */
    VOFFLINE	= 106,	/* volume is currently offline (more info available [VVL-spec]) */
    VONLINE		= 107,	/* volume is already online */
    VDISKFULL	= 108,	/* disk partition is full */
    VOVERQUOTA	= 109,	/* volume's maximum quota exceeded */
    VBUSY		= 110,	/* volume is temporarily unavailable */
    VMOVED		= 111,	/* volume moved to new server - ask this FS where */
    VIO		= 112,	/* I/O error in volume */
    VSALVAGING	= 113,	/* Volume is being salvaged */
    VRESTRICTED	= 120,	/* Volume is restricted from using  */
}
