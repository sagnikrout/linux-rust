//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ufs/util.h
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
// linux/fs/ufs/util.h
//
// Copyright (C) 1998
// Daniel Pirkl <daniel.pirkl@email.cz>
// Charles University, Faculty of Mathematics and Physics
//

//
// functions used for retyping
//
// macros used for accessing structures
//
extern "C" {
    pub fn fs32_to_cpu(_arg: sb, _arg: usb1->fs_u0.fs_sun.fs_state) -> return;
}
extern "C" {
    pub fn fs32_to_cpu(_arg: sb, _arg: usb3->fs_un2.fs_sun.fs_state) -> return;
}
extern "C" {
    pub fn fs32_to_cpu(_arg: sb, _arg: usb1->fs_u1.fs_sunx86.fs_state) -> return;
}
extern "C" {
    pub fn fs32_to_cpu(_arg: sb, _arg: usb3->fs_un2.fs_44.fs_state) -> return;
}
extern "C" {
    pub fn fs32_to_cpu(_arg: sb, _arg: usb3->fs_un2.fs_sunx86.fs_npsect) -> return;
}
extern "C" {
    pub fn fs32_to_cpu(_arg: sb, _arg: usb1->fs_u1.fs_sun.fs_npsect) -> return;
}
extern "C" {
    pub fn fs64_to_cpu(_arg: sb, _arg: tmp) -> return;
}
extern "C" {
    pub fn fs64_to_cpu(_arg: sb, _arg: tmp) -> return;
}
extern "C" {
    pub fn fs16_to_cpu(_arg: sb, _arg: de->d_u.d_namlen) -> return;
}
//
// TODO turn this into a table lookup
//
extern "C" {
    pub fn fs32_to_cpu(_arg: sb, _arg: inode->ui_u3.ui_44.ui_uid) -> return;
}
extern "C" {
    pub fn fs32_to_cpu(_arg: sb, _arg: inode->ui_u3.ui_sun.ui_uid) -> return;
}
extern "C" {
    pub fn fs16_to_cpu(_arg: sb, _arg: inode->ui_u1.oldids.ui_suid) -> return;
}
extern "C" {
    pub fn fs32_to_cpu(_arg: sb, _arg: inode->ui_u3.ui_44.ui_gid) -> return;
}
extern "C" {
    pub fn fs32_to_cpu(_arg: sb, _arg: inode->ui_u3.ui_sun.ui_gid) -> return;
}
extern "C" {
    pub fn fs16_to_cpu(_arg: sb, _arg: inode->ui_u1.oldids.ui_sgid) -> return;
}
extern "C" {
    pub fn ufs_get_inode_dev(: *mut super_block, : *mut ufs_inode_info) -> dev_t;
}
extern "C" {
    pub fn ufs_set_inode_dev(: *mut super_block, : *mut ufs_inode_info, _arg: dev_t);
}
extern "C" {
    pub fn ufs_prepare_chunk(folio: *mut folio, pos: loff_t, len: unsigned) -> c_int;
}
//
// These functions manipulate ufs buffers
//

extern "C" {
    pub fn _ubh_bread_(: *mut ufs_sb_private_info, : *mut super_block, _arg: u64, _arg: u64) -> *mut ufs_buffer_head;
}
extern "C" {
    pub fn ubh_bread_uspi(: *mut ufs_sb_private_info, : *mut super_block, _arg: u64, _arg: u64) -> *mut ufs_buffer_head;
}
extern "C" {
    pub fn ubh_brelse(: *mut ufs_buffer_head);
}
extern "C" {
    pub fn ubh_brelse_uspi(: *mut ufs_sb_private_info);
}
extern "C" {
    pub fn ubh_mark_buffer_dirty(: *mut ufs_buffer_head);
}
extern "C" {
    pub fn ubh_sync_block(: *mut ufs_buffer_head);
}
extern "C" {
    pub fn ubh_bforget(: *mut ufs_buffer_head);
}
extern "C" {
    pub fn ubh_buffer_dirty(: *mut ufs_buffer_head) -> c_int;
}
// This functions works with cache pages
//
// macros and inline function to get important structures from ufs_sb_private_info
//

//
// Extract byte from ufs_buffer_head
// Extract the bits for a block from a map inside ufs_buffer_head
//

extern "C" {
    pub fn ubh_get_addr64(_arg: ubh, _arg: blk) -> return;
}
extern "C" {
    pub fn ubh_get_addr32(_arg: ubh, _arg: blk) -> return;
}

//
// Macros to access cylinder group array structures
//

//
// Bitmap operations
// These functions work like classical bitmap operations.
// The difference is that we don't have the whole bitmap
// in one contiguous chunk of memory, but in several buffers.
// The parameters of each function are super_block, ufs_buffer_head and
// position of the beginning of the bitmap.
//

// p = 0x00;
// p &= ~(0x0f << (frag & 4));
// p &= ~(0x03 << (frag & 6));
// p &= ~(0x01 << (frag & 7));
// p = 0xff;
// p |= 0x0f << (frag & 4);
// p |= 0x03 << (frag & 6);
// p |= 0x01 << (frag & 7);
// (__fs64 *)p = cpu_to_fs64(sb, val);
// (__fs32 *)p = cpu_to_fs32(sb, val);
// (__fs64 *)p = 0;
// (__fs32 *)p = 0;
// Signed 32-bit interpretation wraps around in 2038, which
// happens in ufs1 inode stamps but not ufs2 using 64-bits
// stamps. For superblock and blockgroup, let's assume
// unsigned 32-bit stamps, which are good until y2106.
// Wrap around rather than clamp here to make the dirty
// file system detection work in the superblock stamp.
//
extern "C" {
    pub fn cpu_to_fs32(_arg: sbp, _arg: lower_32_bits(now)) -> return;
}
