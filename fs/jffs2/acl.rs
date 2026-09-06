//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jffs2/acl.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jffs2_acl_entry {
    pub e_tag: jint16_t,
    pub e_perm: jint16_t,
    pub e_id: jint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jffs2_acl_entry_short {
    pub e_tag: jint16_t,
    pub e_perm: jint16_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jffs2_acl_header {
    pub a_version: jint32_t,
    pub a_entries: [jffs2_acl_entry; ],
}

extern "C" {
    pub fn jffs2_init_acl_pre(: *mut inode, : *mut inode, : *mut umode_t) -> c_int;
}
extern "C" {
    pub fn jffs2_init_acl_post(: *mut inode) -> c_int;
}

