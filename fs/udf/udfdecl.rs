//! Automatically rewritten from C Header to Rust Module
//! Source: fs/udf/udfdecl.h
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

pub const UDF_DEFAULT_PREALLOC_BLOCKS: c_int = 8;

pub const UDF_EXTENT_LENGTH_MASK: c_uint = 0x3FFFFFFF;
pub const UDF_EXTENT_FLAG_MASK: c_uint = 0xC0000000;

pub const UDF_NAME_PAD: c_int = 4;
pub const UDF_NAME_LEN: c_int = 254;
pub const UDF_NAME_LEN_CS0: c_int = 255;
extern "C" {
    pub fn sizeof(unallocSpaceEntry: struct) -> return;
}
extern "C" {
    pub fn udf_file_entry_alloc_offset(_arg: inode) -> return;
}
// computes tag checksum
extern "C" {
    pub fn udf_tag_checksum(t: *const tag) -> u8;
}
pub type udf_pblk_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udf_fileident_iter {
    pub /: *mut *mut *mut inode dir; / Directory we are working with,
    pub /: *mut *mut loff_t pos; / Logical position in a dir,
    pub possibly: *mut *mut *mut buffer_head bh[2]; / Buffer containing 'pos' and,
// next buffer if entry straddles
// blocks
    pub /: *mut *mut kernel_lb_addr eloc; / Start of extent containing 'pos',
    pub /: *mut *mut uint32_t elen; / Length of extent containing 'pos',
    pub above: *mut *mut sector_t loffset; / Block offset of 'pos' within,
// extent
    pub /: *mut *mut extent_position epos; / Position after the above extent,
    pub /: *mut *mut fileIdentDesc fi; / Copied directory entry,
    pub /: *mut *mut *mut uint8_t name; / Pointer to entry name,
    pub case: *mut *mut *mut uint8_t namebuf; / Storage for entry name in,
// the name is split between two blocks
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udf_vds_record {
    pub block: u32,
    pub volDescSeqNum: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_desc {
    pub descTag: tag,
    pub volDescSeqNum: __le32,
}

// super.c
extern "C" {
    pub fn lvid_get_unique_id(sb: *mut super_block) -> u64;
}
// namei.c
// file.c
extern "C" {
    pub fn udf_ioctl(: *mut file, int: unsigned, long: unsigned) -> c_long;
}
// inode.c
extern "C" {
    pub fn __udf_iget(_arg: sb, _arg: ino, _arg: true) -> return;
}
extern "C" {
    pub fn __udf_iget(_arg: sb, _arg: ino, _arg: false) -> return;
}
extern "C" {
    pub fn udf_expand_file_adinicb(: *mut inode) -> c_int;
}
extern "C" {
    pub fn udf_setsize(: *mut inode, _arg: loff_t) -> c_int;
}
extern "C" {
    pub fn udf_evict_inode(: *mut inode);
}
extern "C" {
    pub fn udf_write_inode(: *mut inode, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn udf_sync_inode_metadata(: *mut inode, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn udf_get_block(: *mut inode, _arg: sector_t, : *mut buffer_head, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn udf_update_extra_perms(inode: *mut inode, mode: umode_t);
}
// misc.c
extern "C" {
    pub fn udf_update_tag(: *mut c_char, _arg: c_int);
}
extern "C" {
    pub fn udf_new_tag(: *mut c_char, _arg: u16, _arg: u16, _arg: u16, _arg: u32, _arg: c_int);
}
// lowlevel.c
extern "C" {
    pub fn udf_get_last_session(: *mut super_block) -> c_uint;
}
extern "C" {
    pub fn udf_get_last_block(: *mut super_block) -> udf_pblk_t;
}
// partition.c
extern "C" {
    pub fn udf_relocate_blocks(: *mut super_block, _arg: c_long, : *mut c_long) -> c_int;
}
// unicode.c
// ialloc.c
extern "C" {
    pub fn udf_free_inode(: *mut inode);
}
// truncate.c
extern "C" {
    pub fn udf_truncate_tail_extent(: *mut inode);
}
extern "C" {
    pub fn udf_discard_prealloc(: *mut inode);
}
extern "C" {
    pub fn udf_truncate_extents(: *mut inode) -> c_int;
}
// balloc.c
// directory.c
extern "C" {
    pub fn udf_fiiter_advance(iter: *mut udf_fileident_iter) -> c_int;
}
extern "C" {
    pub fn udf_fiiter_release(iter: *mut udf_fileident_iter);
}
extern "C" {
    pub fn udf_fiiter_write_fi(iter: *mut udf_fileident_iter, impuse: *mut u8);
}
extern "C" {
    pub fn udf_fiiter_update_elen(iter: *mut udf_fileident_iter, new_elen: u32);
}
extern "C" {
    pub fn udf_fiiter_append_blk(iter: *mut udf_fileident_iter) -> c_int;
}
// udftime.c
extern "C" {
    pub fn udf_time_to_disk_stamp(dest: *mut timestamp, src: timespec64);
}
