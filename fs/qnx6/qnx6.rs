//! Automatically rewritten from C Header to Rust Module
//! Source: fs/qnx6/qnx6.h
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
// QNX6 file system, Linux implementation.
//
// Version : 1.0.0
//
// History :
//
// 01-02-2012 by Kai Bankett (chaosman@ontika.net) : first release.
// 16-02-2012 page map extension by Al Viro
//

pub type __fs16 = __u16 ;
pub type __fs32 = __u32 ;
pub type __fs64 = __u64 ;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx6_sb_info {
    pub /: *mut *mut *mut buffer_head sb_buf; / superblock buffer,
    pub /: *mut *mut *mut qnx6_super_block sb; / our superblock,
    pub /: *mut *mut int s_blks_off; / blkoffset fs-startpoint,
    pub /: *mut *mut int s_ptrbits; / indirect pointer bitfield,
    pub /: *mut *mut unsigned long s_mount_opt; / all mount options,
    pub /: *mut *mut int s_bytesex; / holds endianess info,
    pub inodes: *mut *mut inode,
    pub longfile: *mut *mut inode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qnx6_inode_info {
    pub di_block_ptr: [__fs32; QNX6_NO_DIRECT_POINTERS],
    pub di_filelevels: __u8,
    pub i_dir_start_lookup: __u32,
    pub vfs_inode: inode,
}

extern "C" {
    pub fn container_of(_arg: inode, qnx6_inode_info: struct, _arg: vfs_inode) -> return;
}

extern "C" {
    pub fn le64_to_cpu(__le64)n: () -> return;
}
extern "C" {
    pub fn be64_to_cpu(__be64)n: () -> return;
}
extern "C" {
    pub fn le32_to_cpu(__le32)n: () -> return;
}
extern "C" {
    pub fn be32_to_cpu(__be32)n: () -> return;
}
extern "C" {
    pub fn le16_to_cpu(__le16)n: () -> return;
}
extern "C" {
    pub fn be16_to_cpu(__be16)n: () -> return;
}
extern "C" {
    pub fn qnx6_find_ino(len: c_int, dir: *mut inode, name: *const c_char) -> unsigned;
}
