//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jffs2/os-linux.h
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
//
// Created by David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in this directory.
//
// JFFS2 uses Linux mode bits natively -- no need for conversion

// wbuf.c
extern "C" {
    pub fn jffs2_flash_writev(c: *mut jffs2_sb_info, vecs: *const kvec, count: c_ulong, to: loff_t, retlen: *mut usize, ino: u32) -> c_int;
}
extern "C" {
    pub fn jffs2_flash_write(c: *mut jffs2_sb_info, ofs: loff_t, len: usize, retlen: *mut usize, buf: *const u_char) -> c_int;
}
extern "C" {
    pub fn jffs2_flash_read(c: *mut jffs2_sb_info, ofs: loff_t, len: usize, retlen: *mut usize, buf: *mut u_char) -> c_int;
}
extern "C" {
    pub fn jffs2_check_oob_empty(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, mode: c_int) -> c_int;
}
extern "C" {
    pub fn jffs2_check_nand_cleanmarker(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) -> c_int;
}
extern "C" {
    pub fn jffs2_write_nand_cleanmarker(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) -> c_int;
}
extern "C" {
    pub fn jffs2_write_nand_badblock(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, bad_offset: u32) -> c_int;
}
extern "C" {
    pub fn jffs2_wbuf_timeout(data: c_ulong);
}
extern "C" {
    pub fn jffs2_wbuf_process(data: *mut c_void);
}
extern "C" {
    pub fn jffs2_flush_wbuf_gc(c: *mut jffs2_sb_info, ino: u32) -> c_int;
}
extern "C" {
    pub fn jffs2_flush_wbuf_pad(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_nand_flash_setup(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_nand_flash_cleanup(c: *mut jffs2_sb_info);
}

extern "C" {
    pub fn jffs2_dataflash_setup(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_dataflash_cleanup(c: *mut jffs2_sb_info);
}

extern "C" {
    pub fn jffs2_ubivol_setup(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_ubivol_cleanup(c: *mut jffs2_sb_info);
}

extern "C" {
    pub fn jffs2_nor_wbuf_flash_setup(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_nor_wbuf_flash_cleanup(c: *mut jffs2_sb_info);
}
extern "C" {
    pub fn jffs2_dirty_trigger(c: *mut jffs2_sb_info);
}

// background.c
extern "C" {
    pub fn jffs2_start_garbage_collect_thread(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_stop_garbage_collect_thread(c: *mut jffs2_sb_info);
}
extern "C" {
    pub fn jffs2_garbage_collect_trigger(c: *mut jffs2_sb_info);
}
// dir.c
// file.c
extern "C" {
    pub fn jffs2_fsync(: *mut file, _arg: loff_t, _arg: loff_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn __jffs2_read_folio(file: *mut file, folio: *mut folio) -> c_int;
}
// ioctl.c
extern "C" {
    pub fn jffs2_ioctl(: *mut file, int: unsigned, long: unsigned) -> c_long;
}
// symlink.c
// fs.c
extern "C" {
    pub fn jffs2_setattr(: *mut mnt_idmap, : *mut dentry, : *mut iattr) -> c_int;
}
extern "C" {
    pub fn jffs2_do_setattr(: *mut inode, : *mut iattr) -> c_int;
}
extern "C" {
    pub fn jffs2_evict_inode(: *mut inode);
}
extern "C" {
    pub fn jffs2_dirty_inode(inode: *mut inode, flags: c_int);
}
extern "C" {
    pub fn jffs2_statfs(: *mut dentry, : *mut kstatfs) -> c_int;
}
extern "C" {
    pub fn jffs2_do_remount_fs(sb: *mut super_block, fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn jffs2_do_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn jffs2_flash_cleanup(c: *mut jffs2_sb_info);
}
// writev.c
