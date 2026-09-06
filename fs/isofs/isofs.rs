//! Automatically rewritten from C Header to Rust Module
//! Source: fs/isofs/isofs.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isofs_file_format {
    isofs_file_normal = 0,
    isofs_file_sparse = 1,
    isofs_file_compressed = 2,
}

//
// iso fs inode data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iso_inode_info {
    pub i_iget5_block: c_ulong,
    pub i_iget5_offset: c_ulong,
    pub i_first_extent: c_uint,
    pub i_file_format: c_uchar,
    pub i_format_parm: [c_uchar; 3],
    pub i_next_section_block: c_ulong,
    pub i_next_section_offset: c_ulong,
    pub i_section_size: off_t,
    pub vfs_inode: inode,
}

//
// iso9660 super-block data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isofs_sb_info {
    pub s_ninodes: c_ulong,
    pub s_nzones: c_ulong,
    pub s_firstdatazone: c_ulong,
    pub s_log_zone_size: c_ulong,
    pub s_max_size: c_ulong,
    pub /: *mut *mut int s_rock_offset; / offset of SUSP fields within SU area,
    pub s_sbsector: i32,
    pub s_joliet_level: c_uchar,
    pub s_mapping: c_uchar,
    pub s_check: c_uchar,
    pub s_session: c_uchar,
    pub s_high_sierra:1: c_uint,
    pub s_rock:2: c_uint,
    pub length: *mut *mut unsigned int s_cruft:1; / Broken disks with high byte of,
// containing junk
    pub s_nocompress:1: c_uint,
    pub s_hide:1: c_uint,
    pub s_showassoc:1: c_uint,
    pub s_overriderockperm:1: c_uint,
    pub s_uid_set:1: c_uint,
    pub s_gid_set:1: c_uint,
    pub s_fmode: umode_t,
    pub s_dmode: umode_t,
    pub s_gid: kgid_t,
    pub s_uid: kuid_t,
    pub /: *mut *mut *mut nls_table s_nls_iocharset; / Native language support table,
}

extern "C" {
    pub fn container_of(_arg: inode, iso_inode_info: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn get_unaligned_le16(_arg: p) -> return;
}
extern "C" {
    pub fn get_unaligned_be16(_arg: p) -> return;
}
// Ignore bigendian datum due to broken mastering programs
extern "C" {
    pub fn get_unaligned_le16(_arg: p) -> return;
}
extern "C" {
    pub fn get_unaligned_le32(_arg: p) -> return;
}
extern "C" {
    pub fn get_unaligned_be32(_arg: p) -> return;
}
// Ignore bigendian datum due to broken mastering programs
extern "C" {
    pub fn get_unaligned_le32(_arg: p) -> return;
}

extern "C" {
    pub fn iso_date(p: *mut u8, flags: c_int) -> timespec64;
}
extern "C" {
    pub fn parse_rock_ridge_inode(: *mut iso_directory_record, : *mut inode, relocated: c_int) -> c_int;
}
extern "C" {
    pub fn get_rock_ridge_filename(: *mut iso_directory_record, : *mut c_char, : *mut inode) -> c_int;
}
extern "C" {
    pub fn isofs_name_translate(: *mut iso_directory_record, : *mut c_char, : *mut inode) -> c_int;
}
extern "C" {
    pub fn get_joliet_filename(: *mut iso_directory_record, : *mut c_uchar, : *mut inode) -> c_int;
}
extern "C" {
    pub fn get_acorn_filename(: *mut iso_directory_record, : *mut c_char, : *mut inode) -> c_int;
}
extern "C" {
    pub fn isofs_get_blocks(: *mut inode, _arg: sector_t, : *mut buffer_head, long: unsigned) -> c_int;
}
extern "C" {
    pub fn __isofs_iget(_arg: sb, _arg: block, _arg: offset, _arg: 0) -> return;
}
extern "C" {
    pub fn __isofs_iget(_arg: sb, _arg: block, _arg: offset, _arg: 1) -> return;
}
// Because the inode number is no longer relevant to finding the
// underlying meta-data for an inode, we are free to choose a more
// convenient 32-bit number as the inode number.  The inode numbering
// scheme was recommended by Sergey Vlasov and Eric Lammerts.
// Every directory can have many redundant directory entries scattered
// throughout the directory tree.  First there is the directory entry
// with the name of the directory stored in the parent directory.
// Then, there is the "." directory entry stored in the directory
// itself.  Finally, there are possibly many ".." directory entries
// stored in all the subdirectories.
//
// In order for the NFS get_parent() method to work and for the
// general consistency of the dcache, we need to make sure the
// "i_iget5_block" and "i_iget5_offset" all point to exactly one of
// the many redundant entries for each directory.  We normalize the
// block and offset by always making them point to the "."  directory.
//
// Notice that we do not use the entry for the directory with the name
// that is located in the parent directory.  Even though choosing this
// first directory is more natural, it is much easier to find the "."
// entry in the NFS get_parent() method because it is implicitly
// encoded in the "extent + ext_attr_length" fields of _all_ the
// redundant entries for the directory.  Thus, it can always be
// reached regardless of which directory entry you have in hand.
//
// This works because the "." entry is simply the first directory
// record when you start reading the file that holds all the directory
// records, and this file starts at "extent + ext_attr_length" blocks.
// Because the "." entry is always the first entry listed in the
// directories file, the normalized "offset" value is always 0.
//
// You should pass the directory entry in "de".  On return, "block"
// and "offset" will hold normalized values.  Only directories are
// affected making it safe to call even for non-directory file
// types.
// Only directories are normalized.
// offset = 0;
// block = (unsigned long)isonum_733(de->extent)
extern "C" {
    pub fn isofs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
