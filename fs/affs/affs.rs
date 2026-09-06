//! Automatically rewritten from C Header to Rust Module
//! Source: fs/affs/affs.h
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

// Ugly macros make the code more pretty.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct affs_ext_key {
    pub /: *mut *mut u32 ext; / idx of the extended block,
    pub /: *mut *mut u32 key; / block number,
}

//
// affs fs inode data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct affs_inode_info {
    pub i_opencnt: core::sync::atomic::AtomicI32,
    pub /: *mut *mut mutex i_link_lock; / Protects internal inode access.,
    pub /: *mut *mut mutex i_ext_lock; / Protects internal inode access.,

    pub /: *mut *mut u32 i_blkcnt; / block count,
    pub /: *mut *mut u32 i_extcnt; / extended block count,
    pub /: *mut *mut *mut u32 i_lc; / linear cache of extended blocks,
    pub i_lc_size: u32,
    pub i_lc_shift: u32,
    pub i_lc_mask: u32,
    pub /: *mut *mut *mut affs_ext_key i_ac; / associative cache of extended blocks,
    pub /: *mut *mut u32 i_ext_last; / last accessed extended block,
    pub /: *mut *mut *mut buffer_head i_ext_bh; / bh of last extended block,
    pub mmu_private: loff_t,
    pub /: *mut *mut u32 i_protect; / unused attribute bits,
    pub /: *mut *mut u32 i_lastalloc; / last allocated block,
    pub /: *mut *mut int i_pa_cnt; / number of preallocated blocks,
    pub vfs_inode: inode,
}

// short cut to get to the affs specific inode data
extern "C" {
    pub fn container_of(_arg: inode, affs_inode_info: struct, _arg: vfs_inode) -> return;
}
//
// super-block data in memory
//
// Block numbers are adjusted for their actual size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct affs_bm_info {
    pub /: *mut *mut u32 bm_key; / Disk block number,
    pub /: *mut *mut u32 bm_free; / Free blocks in here,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct affs_sb_info {
    pub /: *mut *mut int s_partition_size; / Partition size in blocks.,
    pub /: *mut *mut int s_reserved; / Number of reserved blocks.,
// u32 s_blksize;			/* Initial device blksize
    pub /: *mut *mut u32 s_data_blksize; / size of the data block w/o header,
    pub /: *mut *mut u32 s_root_block; / FFS root block number.,
    pub /: *mut *mut int s_hashsize; / Size of hash table.,
    pub /: *mut *mut unsigned long s_flags; / See below.,
    pub /: *mut *mut kuid_t s_uid; / uid to override,
    pub /: *mut *mut kgid_t s_gid; / gid to override,
    pub /: *mut *mut umode_t s_mode; / mode to override,
    pub /: *mut *mut *mut buffer_head s_root_bh; / Cached root block.,
    pub /: *mut *mut mutex s_bmlock; / Protects bitmap access.,
    pub /: *mut *mut *mut affs_bm_info s_bitmap; / Bitmap infos.,
    pub /: *mut *mut u32 s_bmap_count; / # of bitmap blocks.,
    pub /: *mut *mut u32 s_bmap_bits; / # of bits in one bitmap blocks,
    pub s_last_bmap: u32,
    pub s_bmap_bh: *mut buffer_head,
    pub /: *mut *mut *mut char s_prefix; / Prefix for volumes and assigns.,
    pub /: *mut *mut char s_volume[32]; / Volume prefix for absolute symlinks.,
    pub /: *mut *mut spinlock_t symlink_lock; / protects the previous two,
    pub /: *mut *mut *mut super_block sb; / the VFS superblock object,
    pub /: *mut *mut int work_queued; / non-zero delayed work is queued,
    pub /: *mut *mut delayed_work sb_work; / superblock flush delayed work,
    pub /: *mut *mut spinlock_t work_lock; / protects sb_work and work_queued,
    pub rcu: rcu_head,
}

pub const AFFS_MOUNT_SF_INTL: c_uint = 0x0001 /* International filesystem. */;
pub const AFFS_MOUNT_SF_BM_VALID: c_uint = 0x0002 /* Bitmap is valid. */;
pub const AFFS_MOUNT_SF_IMMUTABLE: c_uint = 0x0004 /* Protection bits cannot be changed */;
pub const AFFS_MOUNT_SF_QUIET: c_uint = 0x0008 /* chmod errors will be not reported */;
pub const AFFS_MOUNT_SF_SETUID: c_uint = 0x0010 /* Ignore Amiga uid */;
pub const AFFS_MOUNT_SF_SETGID: c_uint = 0x0020 /* Ignore Amiga gid */;
pub const AFFS_MOUNT_SF_SETMODE: c_uint = 0x0040 /* Ignore Amiga protection bits */;
pub const AFFS_MOUNT_SF_MUFS: c_uint = 0x0100 /* Use MUFS uid/gid mapping */;
pub const AFFS_MOUNT_SF_OFS: c_uint = 0x0200 /* Old filesystem */;
pub const AFFS_MOUNT_SF_PREFIX: c_uint = 0x0400 /* Buffer for prefix is allocated */;
pub const AFFS_MOUNT_SF_VERBOSE: c_uint = 0x0800 /* Talk about fs when mounting */;
pub const AFFS_MOUNT_SF_NO_TRUNCATE: c_uint = 0x1000 /* Don't truncate filenames */;

// short cut to get to the affs specific sb data
extern "C" {
    pub fn affs_mark_sb_dirty(sb: *mut super_block);
}
// amigaffs.c
extern "C" {
    pub fn affs_insert_hash(inode: *mut inode, bh: *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn affs_remove_hash(dir: *mut inode, rem_bh: *mut buffer_head) -> c_int;
}
extern "C" {
    pub fn affs_remove_header(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn affs_checksum_block(sb: *mut super_block, bh: *mut buffer_head) -> u32;
}
extern "C" {
    pub fn affs_fix_checksum(sb: *mut super_block, bh: *mut buffer_head);
}
extern "C" {
    pub fn affs_secs_to_datestamp(secs: time64_t, ds: *mut affs_date);
}
extern "C" {
    pub fn affs_prot_to_mode(prot: u32) -> umode_t;
}
extern "C" {
    pub fn affs_mode_to_prot(inode: *mut inode);
}
extern "C" {
    pub fn affs_nofilenametruncate(dentry: *const dentry) -> bool;
}
extern "C" {
    pub fn affs_copy_name(bstr: *mut c_uchar, dentry: *mut dentry) -> c_int;
}
// bitmap. c
extern "C" {
    pub fn affs_count_free_blocks(s: *mut super_block) -> u32;
}
extern "C" {
    pub fn affs_free_block(sb: *mut super_block, block: u32);
}
extern "C" {
    pub fn affs_alloc_block(inode: *mut inode, goal: u32) -> u32;
}
extern "C" {
    pub fn affs_init_bitmap(sb: *mut super_block, flags: *mut c_int) -> c_int;
}
extern "C" {
    pub fn affs_free_bitmap(sb: *mut super_block);
}
// namei.c
extern "C" {
    pub fn affs_hash_name(sb: *mut super_block, name: *const u8, len: c_uint) -> c_int;
}
extern "C" {
    pub fn affs_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn affs_rmdir(dir: *mut inode, dentry: *mut dentry) -> c_int;
}
// inode.c
extern "C" {
    pub fn affs_evict_inode(inode: *mut inode);
}
// file.c
extern "C" {
    pub fn affs_free_prealloc(inode: *mut inode);
}
extern "C" {
    pub fn affs_truncate(: *mut inode);
}
extern "C" {
    pub fn affs_file_fsync(: *mut file, _arg: loff_t, _arg: loff_t, _arg: c_int) -> c_int;
}
// dir.c
extern "C" {
    pub fn affs_dir_truncate(: *mut inode);
}
// jump tables
extern "C" {
    pub fn sb_bread(_arg: sb, _arg: block) -> return;
}
extern "C" {
    pub fn sb_getblk(_arg: sb, _arg: block) -> return;
}
