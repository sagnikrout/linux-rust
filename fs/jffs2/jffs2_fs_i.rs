//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jffs2/jffs2_fs_i.h
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


//
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2001-2007 Red Hat, Inc.
// Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
//
// Created by David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jffs2_inode_info {
// We need an internal mutex similar to inode->i_rwsem.
    pub sem: mutex,
// The highest (datanode) version number used for this ino
    pub highest_version: u32,
// List of data fragments which make up the file
    pub fragtree: rb_root,
// There may be one datanode which isn't referenced by any of the
// This also holds the _only_ dnode for symlinks/device nodes,
    pub metadata: *mut jffs2_full_dnode,
// Directory entries
    pub dents: *mut jffs2_full_dirent,
// The target path if this is the inode of a symlink
    pub target: *mut c_uchar,
// Some stuff we just have to keep in-core at all times, for each inode.
    pub inocache: *mut jffs2_inode_cache,
    pub flags: u16,
    pub usercompr: u8,
    pub vfs_inode: inode,
}
