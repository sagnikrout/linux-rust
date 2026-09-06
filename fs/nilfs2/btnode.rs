//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nilfs2/btnode.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// NILFS B-tree node cache
//
// Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
//
// Written by Seiji Kihara.
// Revised by Ryusuke Konishi.
//

//
// struct nilfs_btnode_chkey_ctxt - change key context
// @oldkey: old key of block's moving content
// @newkey: new key for block's content
// @bh: buffer head of old buffer
// @newbh: buffer head of new buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nilfs_btnode_chkey_ctxt {
    pub oldkey: __u64,
    pub newkey: __u64,
    pub bh: *mut buffer_head,
    pub newbh: *mut buffer_head,
}

extern "C" {
    pub fn nilfs_init_btnc_inode(btnc_inode: *mut inode);
}
extern "C" {
    pub fn nilfs_btnode_cache_clear(: *mut address_space);
}
extern "C" {
    pub fn nilfs_btnode_delete(: *mut buffer_head);
}
