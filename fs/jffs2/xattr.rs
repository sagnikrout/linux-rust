//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jffs2/xattr.h
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
// Copyright © 2006  NEC Corporation
//
// Created by KaiGai Kohei <kaigai@ak.jp.nec.com>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

extern "C" {
    pub fn jffs2_init_xattr_subsystem(c: *mut jffs2_sb_info);
}
extern "C" {
    pub fn jffs2_build_xattr_subsystem(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_clear_xattr_subsystem(c: *mut jffs2_sb_info);
}
extern "C" {
    pub fn jffs2_xattr_do_crccheck_inode(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache);
}
extern "C" {
    pub fn jffs2_xattr_delete_inode(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache);
}
extern "C" {
    pub fn jffs2_xattr_free_inode(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache);
}
extern "C" {
    pub fn jffs2_verify_xattr(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_release_xattr_datum(c: *mut jffs2_sb_info, xd: *mut jffs2_xattr_datum);
}
extern "C" {
    pub fn jffs2_release_xattr_ref(c: *mut jffs2_sb_info, ref: *mut jffs2_xattr_ref);
}
extern "C" {
    pub fn jffs2_listxattr(: *mut dentry, : *mut c_char, _arg: usize) -> isize;
}

// Macro flag: #define jffs2_init_xattr_subsystem(c)

// Macro flag: #define jffs2_clear_xattr_subsystem(c)

