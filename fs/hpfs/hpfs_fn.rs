//! Automatically rewritten from C Header to Rust Module
//! Source: fs/hpfs/hpfs_fn.h
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
// linux/fs/hpfs/hpfs_fn.h
//
// Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
//
// function headers
//
// #define DBG
// #define DEBUG_LOCKS

pub const ANODE_ALLOC_FWD: c_int = 512;
pub const FNODE_ALLOC_FWD: c_int = 0;
pub const ALLOC_FWD_MIN: c_int = 16;
pub const ALLOC_FWD_MAX: c_int = 128;
pub const ALLOC_M: c_int = 1;
pub const FNODE_RD_AHEAD: c_int = 16;
pub const ANODE_RD_AHEAD: c_int = 0;
pub const DNODE_RD_AHEAD: c_int = 72;
pub const COUNT_RD_AHEAD: c_int = 62;
pub const FREE_DNODES_ADD: c_int = 58;
pub const FREE_DNODES_DEL: c_int = 29;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpfs_inode_info {
    pub mmu_private: loff_t,
    pub /: *mut *mut ino_t i_parent_dir; / (directories) gives fnode of parent dir,
    pub /: *mut *mut unsigned i_dno; / (directories) root dnode,
    pub /: *mut *mut unsigned i_dpos; / (directories) temp for readdir,
    pub /: *mut *mut unsigned i_dsubdno; / (directories) temp for readdir,
    pub /: *mut *mut unsigned i_file_sec; / (files) minimalist cache of alloc info,
    pub /: *mut *mut unsigned i_disk_sec; / (files) minimalist cache of alloc info,
    pub /: *mut *mut unsigned i_n_secs; / (files) minimalist cache of alloc info,
    pub /: *mut *mut unsigned i_ea_size; / size of extended attributes,
    pub /: *mut *mut unsigned i_ea_mode : 1; / file's permission is stored in ea,
    pub /: *mut *mut unsigned i_ea_uid : 1; / file's uid is stored in ea,
    pub /: *mut *mut unsigned i_ea_gid : 1; / file's gid is stored in ea,
    pub 1: unsigned i_dirty :,
    pub i_rddir_off: *mut loff_t,
    pub vfs_inode: inode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpfs_sb_info {
    pub /: *mut *mut mutex hpfs_mutex; / global hpfs lock,
    pub /: *mut *mut ino_t sb_root; / inode number of root dir,
    pub /: *mut *mut unsigned sb_fs_size; / file system size, sectors,
    pub /: *mut *mut unsigned sb_bitmaps; / sector number of bitmap list,
    pub /: *mut *mut unsigned sb_dirband_start; / directory band start sector,
    pub /: *mut *mut unsigned sb_dirband_size; / directory band size, dnodes,
    pub /: *mut *mut unsigned sb_dmap; / sector number of dnode bit map,
    pub /: *mut *mut unsigned sb_n_free; / free blocks for statfs, or -1,
    pub /: *mut *mut unsigned sb_n_free_dnodes; / free dnodes for statfs, or -1,
    pub /: *mut *mut kuid_t sb_uid; / uid from mount options,
    pub /: *mut *mut kgid_t sb_gid; / gid from mount options,
    pub /: *mut *mut umode_t sb_mode; / mode from mount options,
    pub /: *mut *mut unsigned sb_eas : 2; / eas: 0-ignore, 1-ro, 2-rw,
    pub /: *mut *mut unsigned sb_err : 2; / on errs: 0-cont, 1-ro, 2-panic,
    pub /: *mut *mut unsigned sb_chk : 2; / checks: 0-no, 1-normal, 2-strict,
    pub /: *mut *mut unsigned sb_lowercase : 1; / downcase filenames hackery,
    pub /: *mut *mut unsigned sb_was_error : 1; / there was an error, set dirty flag,
    pub /: *mut *mut unsigned sb_chkdsk : 2; / chkdsk: 0-no, 1-on errs, 2-allways,
    pub /: *mut *mut *mut unsigned char sb_cp_table; / code page tables:,
// 128 bytes uppercasing table &
// 128 bytes lowercasing table
    pub /: *mut *mut *mut __le32 sb_bmp_dir; / main bitmap directory,
    pub /: *mut *mut unsigned sb_c_bitmap; / current bitmap,
    pub /: *mut *mut unsigned sb_max_fwd_alloc; / max forwad allocation,
    pub sb_timeshift: c_int,
    pub rcu: rcu_head,
    pub n_hotfixes: unsigned,
    pub hotfix_from: [secno; 256],
    pub hotfix_to: [secno; 256],
}

// Four 512-byte buffers and the 2k block obtained by concatenating them
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quad_buffer_head {
    pub bh: [*mut buffer_head; 4],
    pub data: *mut c_void,
}

// The b-tree down pointer from a dir entry
extern "C" {
    pub fn le32_to_cpu(4): *mut *mut *mut *mut (__le32 ) ((void ) de + le16_to_cpu(de->length) -) -> return;
}
// The first dir entry in a dnode
// The end+1 of the dir entries
// The dir entry after dir entry de
extern "C" {
    pub fn le32_to_cpu(ea->namelen)): *mut *mut *mut get_unaligned((__le32 )((char )ea + 9 +) -> return;
}
extern "C" {
    pub fn le32_to_cpu(ea->namelen)): *mut *mut *mut get_unaligned((__le32 )((char )ea + 5 +) -> return;
}
// alloc.c
extern "C" {
    pub fn hpfs_chk_sectors(: *mut super_block, _arg: secno, _arg: c_int, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn hpfs_alloc_sector(: *mut super_block, _arg: secno, _arg: unsigned, _arg: c_int) -> secno;
}
extern "C" {
    pub fn hpfs_alloc_if_possible(: *mut super_block, _arg: secno) -> c_int;
}
extern "C" {
    pub fn hpfs_free_sectors(: *mut super_block, _arg: secno, _arg: unsigned);
}
extern "C" {
    pub fn hpfs_check_free_dnodes(: *mut super_block, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn hpfs_free_dnode(: *mut super_block, _arg: secno);
}
extern "C" {
    pub fn hpfs_trim_fs(: *mut super_block, _arg: u64, _arg: u64, _arg: u64, : *mut unsigned) -> c_int;
}
// anode.c
extern "C" {
    pub fn hpfs_bplus_lookup(: *mut super_block, : *mut inode, : *mut bplus_header, _arg: unsigned, : *mut buffer_head) -> secno;
}
extern "C" {
    pub fn hpfs_add_sector_to_btree(: *mut super_block, _arg: secno, _arg: c_int, _arg: unsigned) -> secno;
}
extern "C" {
    pub fn hpfs_remove_btree(: *mut super_block, : *mut bplus_header);
}
extern "C" {
    pub fn hpfs_ea_read(: *mut super_block, _arg: secno, _arg: c_int, _arg: unsigned, _arg: unsigned, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn hpfs_ea_write(: *mut super_block, _arg: secno, _arg: c_int, _arg: unsigned, _arg: unsigned, : *const c_char) -> c_int;
}
extern "C" {
    pub fn hpfs_ea_remove(: *mut super_block, _arg: secno, _arg: c_int, _arg: unsigned);
}
extern "C" {
    pub fn hpfs_truncate_btree(: *mut super_block, _arg: secno, _arg: c_int, _arg: unsigned);
}
extern "C" {
    pub fn hpfs_remove_fnode(: *mut super_block, fno: fnode_secno);
}
// buffer.c
extern "C" {
    pub fn hpfs_search_hotfix_map(s: *mut super_block, sec: secno) -> secno;
}
extern "C" {
    pub fn hpfs_search_hotfix_map_for_range(s: *mut super_block, sec: secno, n: unsigned) -> unsigned;
}
extern "C" {
    pub fn hpfs_prefetch_sectors(: *mut super_block, _arg: unsigned, _arg: c_int);
}
extern "C" {
    pub fn hpfs_brelse4(: *mut quad_buffer_head);
}
extern "C" {
    pub fn hpfs_mark_4buffers_dirty(: *mut quad_buffer_head);
}
// dentry.c
// dir.c
// dnode.c
extern "C" {
    pub fn hpfs_add_pos(: *mut inode, : *mut loff_t) -> c_int;
}
extern "C" {
    pub fn hpfs_del_pos(: *mut inode, : *mut loff_t);
}
extern "C" {
    pub fn hpfs_remove_dirent(: *mut inode, _arg: dnode_secno, : *mut hpfs_dirent, : *mut quad_buffer_head, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn hpfs_count_dnodes(: *mut super_block, _arg: dnode_secno, : *mut c_int, : *mut c_int, : *mut c_int);
}
extern "C" {
    pub fn hpfs_de_as_down_as_possible(: *mut super_block, dno: dnode_secno) -> dnode_secno;
}
extern "C" {
    pub fn hpfs_remove_dtree(: *mut super_block, _arg: dnode_secno);
}
// ea.c
extern "C" {
    pub fn hpfs_ea_ext_remove(: *mut super_block, _arg: secno, _arg: c_int, _arg: unsigned);
}
extern "C" {
    pub fn hpfs_read_ea(: *mut super_block, : *mut fnode, : *mut c_char, : *mut c_char, _arg: c_int) -> c_int;
}
// file.c
extern "C" {
    pub fn hpfs_file_fsync(: *mut file, _arg: loff_t, _arg: loff_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn hpfs_truncate(: *mut inode);
}
// inode.c
extern "C" {
    pub fn hpfs_init_inode(: *mut inode);
}
extern "C" {
    pub fn hpfs_read_inode(: *mut inode);
}
extern "C" {
    pub fn hpfs_write_inode(: *mut inode);
}
extern "C" {
    pub fn hpfs_write_inode_nolock(: *mut inode);
}
extern "C" {
    pub fn hpfs_setattr(: *mut mnt_idmap, : *mut dentry, : *mut iattr) -> c_int;
}
extern "C" {
    pub fn hpfs_write_if_changed(: *mut inode);
}
extern "C" {
    pub fn hpfs_evict_inode(: *mut inode);
}
// map.c
extern "C" {
    pub fn hpfs_prefetch_bitmap(: *mut super_block, _arg: unsigned);
}
extern "C" {
    pub fn hpfs_load_hotfix_map(s: *mut super_block, spareblock: *mut hpfs_spare_block);
}
extern "C" {
    pub fn hpfs_fnode_dno(s: *mut super_block, ino: ino_t) -> dnode_secno;
}
// name.c
extern "C" {
    pub fn hpfs_upcase(: *mut c_uchar, char: unsigned) -> c_uchar;
}
extern "C" {
    pub fn hpfs_chk_name(: *const c_uchar, : *mut unsigned) -> c_int;
}
extern "C" {
    pub fn hpfs_is_name_long(: *const c_uchar, _arg: unsigned) -> c_int;
}
extern "C" {
    pub fn hpfs_adjust_length(: *const c_uchar, : *mut unsigned);
}
// namei.c
extern "C" {
    pub fn container_of(_arg: inode, hpfs_inode_info: struct, _arg: vfs_inode) -> return;
}
// super.c
extern "C" {
    pub fn hpfs_error(: *mut super_block, : *const c_char, ...);
}
extern "C" {
    pub fn hpfs_stop_cycles(: *mut super_block, _arg: c_int, : *mut c_int, : *mut c_int, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn hpfs_get_free_dnodes(: *mut super_block) -> unsigned;
}
extern "C" {
    pub fn hpfs_ioctl(file: *mut file, cmd: unsigned, arg: c_ulong) -> c_long;
}
//
// local time (HPFS) to GMT (Unix)
//
extern "C" {
    pub fn gmt_to_local(_arg: s, _arg: ktime_get_real_seconds()) -> return;
}
//
// Locking:
//
// hpfs_lock() locks the whole filesystem. It must be taken
// on any method called by the VFS.
//
// We don't do any per-file locking anymore, it is hard to
// review and HPFS is not performance-sensitive anyway.
//
