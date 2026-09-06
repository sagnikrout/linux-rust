//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/posix_acl.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_acl_entry {
    pub e_tag: c_short,
    pub e_perm: c_ushort,
    pub e_uid: kuid_t,
    pub e_gid: kgid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_acl {
// New members MUST be added within the struct_group() macro below.
    pub a_refcount: refcount_t,
    pub a_count: c_uint,
    pub a_rcu: rcu_head,
    pub __counted_by(a_count): posix_acl_entry a_entries[],
}

//
// Duplicate an ACL handle.
//
// Free an ACL handle.
//
// posix_acl.c
extern "C" {
    pub fn posix_acl_init(: *mut posix_acl, _arg: c_int);
}
extern "C" {
    pub fn posix_acl_equiv_mode(: *const posix_acl, : *mut umode_t) -> c_int;
}
extern "C" {
    pub fn __posix_acl_create(: *mut posix_acl, _arg: gfp_t, : *mut umode_t) -> c_int;
}
extern "C" {
    pub fn __posix_acl_chmod(: *mut posix_acl, _arg: gfp_t, _arg: umode_t) -> c_int;
}

extern "C" {
    pub fn posix_acl_chmod(: *mut mnt_idmap, : *mut dentry, _arg: umode_t) -> c_int;
}
extern "C" {
    pub fn simple_acl_create(: *mut inode, : *mut inode) -> c_int;
}
extern "C" {
    pub fn set_cached_acl(inode: *mut inode, type: c_int, acl: *mut posix_acl);
}
extern "C" {
    pub fn forget_cached_acl(inode: *mut inode, type: c_int);
}
extern "C" {
    pub fn forget_all_cached_acls(inode: *mut inode);
}
extern "C" {
    pub fn posix_acl_valid(: *mut user_namespace, : *const posix_acl) -> c_int;
}

// default_acl = *acl = NULL;
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

