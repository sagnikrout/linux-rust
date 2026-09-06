//! Automatically rewritten from C Header to Rust Module
//! Source: fs/hpfs/hpfs.h
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
// linux/fs/hpfs/hpfs.h
//
// HPFS structures by Chris Smith, 1993
//
// a little bit modified by Mikulas Patocka, 1998-1999
//
// The paper

// Notation
// sector 0
// The boot block is very like a FAT boot block, except that the
pub const BB_MAGIC: c_uint = 0xaa55;
// sector 16
// The super block has the pointer to the root directory.
pub const SB_MAGIC: c_uint = 0xf995e849;
// sector 17
// The spare block has pointers to spare sectors.
pub const SP_MAGIC: c_uint = 0xf9911849;

// The bad block list is 4 sectors long.  The first word must be zero,
pub const BAD_MAGIC: c_int = 0;
// The hotfix map is 4 sectors long.  It looks like
// Sectors 18 and 19 are preallocated and unused.
// The code page info pointed to by the spare block consists of an index
// block pointed to by spareblock->code_page_dir
pub const CP_DIR_MAGIC: c_uint = 0x494521f7;
// blocks pointed to by code_page_directory
pub const CP_DATA_MAGIC: c_uint = 0x894521f7;
// Free space bitmaps are 4 sectors long, which is 16384 bits.
// dnode: directory.  4 sectors long
// A directory is a tree of dnodes.  The fnode for a directory
pub const DNODE_MAGIC: c_uint = 0x77e40aae;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dnode {
    pub /: *mut *mut __le32 magic; / 77e4 0aae,
    pub to: *mut *mut __le32 first_free; / offset from start of dnode,

    pub /: *mut *mut u8 root_dnode: 1; / Is it root dnode?,
    pub /: *mut *mut u8 increment_me: 7; / some kind of activity counter?,
// Neither HPFS.IFS nor CHKDSK cares

    pub /: *mut *mut u8 increment_me: 7; / some kind of activity counter?,
// Neither HPFS.IFS nor CHKDSK cares
    pub /: *mut *mut u8 root_dnode: 1; / Is it root dnode?,
    pub increment_me2: [u8; 3],
    pub fnode: *mut *mut __le32 up; / (root dnode) directory's,
    pub /: *mut *mut __le32 self; / pointer to this dnode,
    pub /: *mut *mut u8 dirent[2028]; / one or more dirents,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpfs_dirent {
    pub /: *mut *mut __le16 length; / offset to next dirent,

    pub /: *mut *mut u8 first: 1; / set on phony ^A^A (".") entry,
    pub 1: u8 has_acl:,
    pub /: *mut *mut u8 down: 1; / down pointer present (after name),
    pub /: *mut *mut u8 last: 1; / set on phony \377 entry,
    pub /: *mut *mut u8 has_ea: 1; / entry has EA,
    pub /: *mut *mut u8 has_xtd_perm: 1; / has extended perm list (???),
    pub 1: u8 has_explicit_acl:,
    pub set: *mut *mut u8 has_needea: 1; / ?? some EA has NEEDEA,

    pub set: *mut *mut u8 has_needea: 1; / ?? some EA has NEEDEA,
    pub 1: u8 has_explicit_acl:,
    pub /: *mut *mut u8 has_xtd_perm: 1; / has extended perm list (???),
    pub /: *mut *mut u8 has_ea: 1; / entry has EA,
    pub /: *mut *mut u8 last: 1; / set on phony \377 entry,
    pub /: *mut *mut u8 down: 1; / down pointer present (after name),
    pub 1: u8 has_acl:,
    pub /: *mut *mut u8 first: 1; / set on phony ^A^A (".") entry,

    pub /: *mut *mut u8 read_only: 1; / dos attrib,
    pub /: *mut *mut u8 hidden: 1; / dos attrib,
    pub /: *mut *mut u8 system: 1; / dos attrib,
    pub /: *mut *mut u8 flag11: 1; / would be volume label dos attrib,
    pub /: *mut *mut u8 directory: 1; / dos attrib,
    pub /: *mut *mut u8 archive: 1; / dos attrib,
    pub /: *mut *mut u8 not_8x3: 1; / name is not 8.3,
    pub 1: u8 flag15:,

    pub 1: u8 flag15:,
    pub /: *mut *mut u8 not_8x3: 1; / name is not 8.3,
    pub /: *mut *mut u8 archive: 1; / dos attrib,
    pub /: *mut *mut u8 directory: 1; / dos attrib,
    pub /: *mut *mut u8 flag11: 1; / would be volume label dos attrib,
    pub /: *mut *mut u8 system: 1; / dos attrib,
    pub /: *mut *mut u8 hidden: 1; / dos attrib,
    pub /: *mut *mut u8 read_only: 1; / dos attrib,

    pub /: *mut *mut __le32 fnode; / fnode giving allocation info,
    pub /: *mut *mut __le32 write_date; / mtime,
    pub /: *mut *mut __le32 file_size; / file length, bytes,
    pub /: *mut *mut __le32 read_date; / atime,
    pub /: *mut *mut __le32 creation_date; / ctime,
    pub /: *mut *mut __le32 ea_size; / total EA length, bytes,
    pub /: *mut *mut u8 no_of_acls; / number of ACL's (low 3 bits),
    pub see: *mut *mut u8 ix; / code page index (of filename),,
    pub /: *mut *mut u8 namelen; / file name length,
    pub /: *mut *mut u8 name[]; / file name,
// dnode_secno down;	  btree down pointer, if present,
}

// B+ tree: allocation info in fnodes and anodes
// dnodes point to fnodes which are responsible for listing the sectors
//
// GET_BTREE_PTR() - Get a pointer to struct bplus_header
//
// Wrapper around container_of() to retrieve a pointer to struct
// bplus_header from a pointer to struct bplus_header_fixed.
//
// @ptr: Pointer to struct bplus_header_fixed.
//

// New members MUST be added within the struct_group() macro below.
// (internal) 2-word entries giving subtree pointers
// (external) 3-word entries giving sector runs
// fnode: root of allocation b+ tree, and EA's
// Every file and every directory has one fnode, pointed to by the directory
pub const FNODE_MAGIC: c_uint = 0xf7e40aae;
// bit 8 set -> directory.  first & only extent
// anode: 99.44% pure allocation tree
pub const ANODE_MAGIC: c_uint = 0x37e40aae;
// extended attributes.
// bit 1 set -> sector is an anode
// bit 7 set -> required ea
//
