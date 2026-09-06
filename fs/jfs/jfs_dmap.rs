//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_dmap.h
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
// maximum l2 number of disk blocks at the various dmapctl levels.
//

//
// maximum number of disk blocks at the various dmapctl levels.
//

//
// determine the maximum free string for four (lower level) nodes
// of the tree.
//
extern "C" {
    pub fn max(_arg: tmp1, _arg: tmp2) -> return;
}
//
// convert disk block number to the logical block number of the dmap
// describing the disk block.  s is the log2(number of logical blocks per page)
//
// The calculation figures out how many logical pages are in front of the dmap.
// - the number of dmaps preceding it
// - the number of L0 pages preceding its L0 page
// - the number of L1 pages preceding its L1 page
// - 3 is added to account for the L2, L1, and L0 page for this dmap
// - 1 is added to account for the control page of the map.
//

//
// convert disk block number to the logical block number of the LEVEL 0
// dmapctl describing the disk block.  s is the log2(number of logical blocks
// per page)
//
// The calculation figures out how many logical pages are in front of the L0.
// - the number of dmap pages preceding it
// - the number of L0 pages preceding it
// - the number of L1 pages preceding its L1 page
// - 2 is added to account for the L2, and L1 page for this L0
// - 1 is added to account for the control page of the map.
//

//
// convert disk block number to the logical block number of the LEVEL 1
// dmapctl describing the disk block.  s is the log2(number of logical blocks
// per page)
//
// The calculation figures out how many logical pages are in front of the L1.
// - the number of dmap pages preceding it
// - the number of L0 pages preceding it
// - the number of L1 pages preceding it
// - 1 is added to account for the L2 page
// - 1 is added to account for the control page of the map.
//

//
// convert disk block number to the logical block number of the dmapctl
// at the specified level which describes the disk block.
//

//
// convert aggregate map size to the zero origin dmapctl level of the
// top dmapctl.
//

// convert disk block number to allocation group number.
//

// convert allocation group number to starting disk block
// number.
//

//
// dmap summary tree
//
// dmaptree must be consistent with dmapctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmaptree {
    pub /: *mut *mut __le32 nleafs; / 4: number of tree leafs,
    pub /: *mut *mut __le32 l2nleafs; / 4: l2 number of tree leafs,
    pub /: *mut *mut __le32 leafidx; / 4: index of first tree leaf,
    pub /: *mut *mut __le32 height; / 4: height of the tree,
    pub /: *mut *mut s8 budmin; / 1: min l2 tree leaf value to combine,
    pub /: *mut *mut s8 stree[TREESIZE]; / TREESIZE: tree,
    pub /: *mut *mut u8 pad[2]; / 2: pad to word boundary,
}

//
// dmap page per 8K blocks bitmap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmap {
    pub /: *mut *mut __le32 nblocks; / 4: num blks covered by this dmap,
    pub /: *mut *mut __le32 nfree; / 4: num of free blks in this dmap,
    pub /: *mut *mut __le64 start; / 8: starting blkno for this dmap,
    pub /: *mut *mut dmaptree tree; / 360: dmap tree,
    pub /: *mut *mut u8 pad[1672]; / 1672: pad to 2048 bytes,
    pub /: *mut *mut __le32 wmap[LPERDMAP]; / 1024: bits of the working map,
    pub /: *mut *mut __le32 pmap[LPERDMAP]; / 1024: bits of the persistent map,
}

//
// disk map control page per level.
//
// dmapctl must be consistent with dmaptree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmapctl {
    pub /: *mut *mut __le32 nleafs; / 4: number of tree leafs,
    pub /: *mut *mut __le32 l2nleafs; / 4: l2 number of tree leafs,
    pub /: *mut *mut __le32 leafidx; / 4: index of the first tree leaf,
    pub /: *mut *mut __le32 height; / 4: height of tree,
    pub /: *mut *mut s8 budmin; / 1: minimum l2 tree leaf value,
    pub /: *mut *mut s8 stree[CTLTREESIZE]; / CTLTREESIZE: dmapctl tree,
    pub /: *mut *mut u8 pad[2714]; / 2714: pad to 4096,
}

//
// common definition for dmaptree within dmap and dmapctl
//
// macros for accessing fields within dmtree

//
// on-disk aggregate disk allocation map descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbmap_disk {
    pub /: *mut *mut __le64 dn_mapsize; / 8: number of blocks in aggregate,
    pub /: *mut *mut __le64 dn_nfree; / 8: num free blks in aggregate map,
    pub /: *mut *mut __le32 dn_l2nbperpage; / 4: number of blks per page,
    pub /: *mut *mut __le32 dn_numag; / 4: total number of ags,
    pub /: *mut *mut __le32 dn_maxlevel; / 4: number of active ags,
    pub /: *mut *mut __le32 dn_maxag; / 4: max active alloc group number,
    pub /: *mut *mut __le32 dn_agpref; / 4: preferred alloc group (hint),
    pub /: *mut *mut __le32 dn_aglevel; / 4: dmapctl level holding the AG,
    pub /: *mut *mut __le32 dn_agheight; / 4: height in dmapctl of the AG,
    pub /: *mut *mut __le32 dn_agwidth; / 4: width in dmapctl of the AG,
    pub /: *mut *mut __le32 dn_agstart; / 4: start tree index at AG height,
    pub /: *mut *mut __le32 dn_agl2size; / 4: l2 num of blks per alloc group,
    pub /: *mut *mut *mut __le64 dn_agfree[MAXAG];/ 8MAXAG: per AG free count,
    pub /: *mut *mut __le64 dn_agsize; / 8: num of blks per alloc group,
    pub /: *mut *mut s8 dn_maxfreebud; / 1: max free buddy system,
    pub /: *mut *mut u8 pad[3007]; / 3007: pad to 4096,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbmap {
    pub /: *mut *mut s64 dn_mapsize; / number of blocks in aggregate,
    pub /: *mut *mut s64 dn_nfree; / num free blks in aggregate map,
    pub /: *mut *mut int dn_l2nbperpage; / number of blks per page,
    pub /: *mut *mut int dn_numag; / total number of ags,
    pub /: *mut *mut int dn_maxlevel; / number of active ags,
    pub /: *mut *mut int dn_maxag; / max active alloc group number,
    pub /: *mut *mut int dn_agpref; / preferred alloc group (hint),
    pub /: *mut *mut int dn_aglevel; / dmapctl level holding the AG,
    pub /: *mut *mut int dn_agheight; / height in dmapctl of the AG,
    pub /: *mut *mut int dn_agwidth; / width in dmapctl of the AG,
    pub /: *mut *mut int dn_agstart; / start tree index at AG height,
    pub /: *mut *mut int dn_agl2size; / l2 num of blks per alloc group,
    pub /: *mut *mut s64 dn_agfree[MAXAG]; / per AG free count,
    pub /: *mut *mut s64 dn_agsize; / num of blks per alloc group,
    pub /: *mut *mut signed char dn_maxfreebud; / max free buddy system,
}

//
// in-memory aggregate disk allocation map descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmap {
    pub /: *mut *mut dbmap db_bmap; / on-disk aggregate map descriptor,
    pub /: *mut *mut *mut inode db_ipbmap; / ptr to aggregate map incore inode,
    pub /: *mut *mut mutex db_bmaplock; / aggregate map lock,
    pub /: *mut *mut atomic_t db_active[MAXAG]; / count of active, open files in AG,
    pub db_DBmap: *mut u32,
}

// macros for accessing fields within in-memory aggregate map descriptor

//
// macros for various conversions needed by the allocators.
// blkstol2(), cntlz(), and cnttz() are operating system dependent functions.
//
// convert number of blocks to log2 number of blocks, rounding up to
// the next log2 value if blocks is not a l2 multiple.
//

// convert number of leafs to log2 leaf value

// convert leaf index to log2 leaf value

// convert a block number to a dmap control leaf index

// convert log2 leaf value to buddy size

//
// external references.
//
extern "C" {
    pub fn dbMount(ipbmap: *mut inode) -> c_int;
}
extern "C" {
    pub fn dbUnmount(ipbmap: *mut inode, mounterror: c_int) -> c_int;
}
extern "C" {
    pub fn dbFree(ipbmap: *mut inode, blkno: i64, nblocks: i64) -> c_int;
}
extern "C" {
    pub fn dbNextAG(ipbmap: *mut inode) -> c_int;
}
extern "C" {
    pub fn dbAlloc(ipbmap: *mut inode, hint: i64, nblocks: i64, results: *mut *mut i64) -> c_int;
}
extern "C" {
    pub fn dbSync(ipbmap: *mut inode) -> c_int;
}
extern "C" {
    pub fn dbAllocBottomUp(ip: *mut inode, blkno: i64, nblocks: i64) -> c_int;
}
extern "C" {
    pub fn dbExtendFS(ipbmap: *mut inode, blkno: i64, nblocks: i64) -> c_int;
}
extern "C" {
    pub fn dbFinalizeBmap(ipbmap: *mut inode);
}
extern "C" {
    pub fn dbMapFileSizeToMapSize(ipbmap: *mut inode) -> i64;
}
extern "C" {
    pub fn dbDiscardAG(ip: *mut inode, agno: c_int, minlen: i64) -> i64;
}
