//! Automatically rewritten from C Header to Rust Module
//! Source: fs/qnx4/qnx4.h
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

pub const QNX4_DEBUG: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx4_sb_info {
    pub /: *mut *mut unsigned int Version; / may be useful,
    pub /: *mut *mut *mut qnx4_inode_entry BitMap; / useful,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx4_inode_info {
    pub raw: qnx4_inode_entry,
    pub mmu_private: loff_t,
    pub vfs_inode: inode,
}

extern "C" {
    pub fn qnx4_count_free_blocks(sb: *mut super_block) -> c_ulong;
}
extern "C" {
    pub fn qnx4_block_map(inode: *mut inode, iblock: c_long) -> c_ulong;
}
extern "C" {
    pub fn qnx4_is_free(sb: *mut super_block, block: c_long) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: inode, qnx4_inode_info: struct, _arg: vfs_inode) -> return;
}
//
// A qnx4 directory entry is an inode entry or link info
// depending on the status field in the last byte. The
// first byte is where the name start either way, and a
// zero means it's empty.
//
// Also, due to a bug in gcc, we don't want to use the
// real (differently sized) name arrays in the inode and
// link entries, but always the 'de_name[]' one in the
// fake struct entry.
//
// See
//
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=99578#c6
//
// for details, but basically gcc will take the size of the
// 'name' array from one of the used union entries randomly.
//
// This use of 'de_name[]' (48 bytes) avoids the false positive
// warnings that would happen if gcc decides to use 'inode.di_name'
// (16 bytes) even when the pointer and size were to come from
// 'link.dl_name' (48 bytes).
//
// In all cases the actual name pointer itself is the same, it's
// only the gcc internal 'what is the size of this field' logic
// that can get confused.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union qnx4_directory_entry {
    pub de_name: [c_char; 48],
    pub de_pad: [u8; 15],
    pub de_status: u8,
}

// Make sure the status byte is in the same place for all structs.
// size = sizeof(de->inode.di_fname);
// size = sizeof(de->link.dl_fname);
// size = strnlen(de->de_name, *size);
