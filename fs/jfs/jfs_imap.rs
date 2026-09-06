//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_imap.h
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
// Copyright (C) International Business Machines Corp., 2000-2002
//

//
// jfs_imap.h: disk inode manager
//

// convert inode number to iag number

// convert iag number to logical block number of the iag page

// get the starting block number of the 4K page of an inode extent
// that contains ino.
//

//
// inode allocation map:
//
// inode allocation map consists of
// . the inode map control page and
// . inode allocation group pages (per 4096 inodes)
// which are addressed by standard JFS xtree.
//
// inode allocation group page (per 4096 inodes of an AG)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iag {
    pub /: *mut *mut __le64 agstart; / 8: starting block of ag,
    pub /: *mut *mut __le32 iagnum; / 4: inode allocation group number,
    pub /: *mut *mut __le32 inofreefwd; / 4: ag inode free list forward,
    pub /: *mut *mut __le32 inofreeback; / 4: ag inode free list back,
    pub /: *mut *mut __le32 extfreefwd; / 4: ag inode extent free list forward,
    pub /: *mut *mut __le32 extfreeback; / 4: ag inode extent free list back,
    pub /: *mut *mut __le32 iagfree; / 4: iag free list,
// summary map: 1 bit per inode extent
    pub inodes: *mut *mut __le32 inosmap[SMAPSZ]; / 16: sum map of mapwords w/ free,
// note: this indicates free and backed
// inodes, if the extent is not backed the
// value will be 1.  if the extent is
// backed but all inodes are being used the
// value will be 1.  if the extent is
// backed but at least one of the inodes is
// free the value will be 0.
//
    pub /: *mut *mut __le32 extsmap[SMAPSZ]; / 16: sum map of mapwords w/ free extents,
    pub /: *mut *mut __le32 nfreeinos; / 4: number of free inodes,
    pub /: *mut *mut __le32 nfreeexts; / 4: number of free extents,
// (72)
    pub /: *mut *mut u8 pad[1976]; / 1976: pad to 2048 bytes,
// allocation bit map: 1 bit per inode (0 - free, 1 - allocated)
    pub /: *mut *mut __le32 wmap[EXTSPERIAG]; / 512: working allocation map,
    pub /: *mut *mut __le32 pmap[EXTSPERIAG]; / 512: persistent allocation map,
    pub /: *mut *mut pxd_t inoext[EXTSPERIAG]; / 1024: inode extent addresses,
}

//
// per AG control information (in inode map control page)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iagctl_disk {
    pub /: *mut *mut __le32 inofree; / 4: free inode list anchor,
    pub /: *mut *mut __le32 extfree; / 4: free extent list anchor,
    pub /: *mut *mut __le32 numinos; / 4: number of backed inodes,
    pub /: *mut *mut __le32 numfree; / 4: number of free inodes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iagctl {
    pub /: *mut *mut int inofree; / free inode list anchor,
    pub /: *mut *mut int extfree; / free extent list anchor,
    pub /: *mut *mut int numinos; / number of backed inodes,
    pub /: *mut *mut int numfree; / number of free inodes,
}

//
// per fileset/aggregate inode map control page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dinomap_disk {
    pub /: *mut *mut __le32 in_freeiag; / 4: free iag list anchor,
    pub /: *mut *mut __le32 in_nextiag; / 4: next free iag number,
    pub /: *mut *mut __le32 in_numinos; / 4: num of backed inodes,
    pub /: *mut *mut __le32 in_numfree; / 4: num of free backed inodes,
    pub /: *mut *mut __le32 in_nbperiext; / 4: num of blocks per inode extent,
    pub /: *mut *mut __le32 in_l2nbperiext; / 4: l2 of in_nbperiext,
    pub /: *mut *mut __le32 in_diskblock; / 4: for standalone test driver,
    pub /: *mut *mut __le32 in_maxag; / 4: for standalone test driver,
    pub /: *mut *mut u8 pad[2016]; / 2016: pad to 2048,
    pub /: *mut *mut iagctl_disk in_agctl[MAXAG]; / 2048: AG control information,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dinomap {
    pub /: *mut *mut int in_freeiag; / free iag list anchor,
    pub /: *mut *mut int in_nextiag; / next free iag number,
    pub /: *mut *mut int in_numinos; / num of backed inodes,
    pub /: *mut *mut int in_numfree; / num of free backed inodes,
    pub /: *mut *mut int in_nbperiext; / num of blocks per inode extent,
    pub /: *mut *mut int in_l2nbperiext; / l2 of in_nbperiext,
    pub /: *mut *mut int in_diskblock; / for standalone test driver,
    pub /: *mut *mut int in_maxag; / for standalone test driver,
    pub /: *mut *mut iagctl in_agctl[MAXAG]; / AG control information,
}

//
// In-core inode map control page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inomap {
    pub /: *mut *mut dinomap im_imap; / 4096: inode allocation control,
    pub /: *mut *mut *mut inode im_ipimap; / 4: ptr to inode for imap,
    pub /: *mut *mut mutex im_freelock; / 4: iag free list lock,
    pub /: *mut *mut mutex im_aglock[MAXAG]; / 512: per AG locks,
    pub im_DBGdimap: *mut u32,
    pub /: *mut *mut atomic_t im_numinos; / num of backed inodes,
    pub /: *mut *mut atomic_t im_numfree; / num of free backed inodes,
}

// for standalone testdriver
//

extern "C" {
    pub fn diFree(: *mut inode) -> c_int;
}
extern "C" {
    pub fn diAlloc(: *mut inode, _arg: bool, : *mut inode) -> c_int;
}
extern "C" {
    pub fn diSync(: *mut inode) -> c_int;
}
// external references
extern "C" {
    pub fn diExtendFS(ipimap: *mut inode, ipbmap: *mut inode) -> c_int;
}
extern "C" {
    pub fn diMount(: *mut inode) -> c_int;
}
extern "C" {
    pub fn diUnmount(: *mut inode, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn diRead(: *mut inode) -> c_int;
}
extern "C" {
    pub fn diWriteSpecial(: *mut inode, _arg: c_int);
}
extern "C" {
    pub fn diFreeSpecial(: *mut inode);
}
extern "C" {
    pub fn diWrite(tid: tid_t, : *mut inode) -> c_int;
}
