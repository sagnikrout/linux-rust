//! Automatically rewritten from C Header to Rust Module
//! Source: fs/vboxsf/vfsmod.h
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


// SPDX-License-Identifier: MIT
//
// VirtualBox Guest Shared Folders support: module header.
//
// Copyright (C) 2006-2018 Oracle Corporation
//

// The cast is to prevent assignment of void * to pointers of arbitrary type

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vboxsf_options {
    pub ttl: c_ulong,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub dmode_set: bool,
    pub fmode_set: bool,
    pub dmode: umode_t,
    pub fmode: umode_t,
    pub dmask: umode_t,
    pub fmask: umode_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vboxsf_fs_context {
    pub o: vboxsf_options,
    pub nls_name: *mut c_char,
}

// per-shared folder information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vboxsf_sbi {
    pub o: vboxsf_options,
    pub root_info: shfl_fsobjinfo,
    pub ino_idr: idr,
    pub /: *mut *mut spinlock_t ino_idr_lock; / This protects ino_idr,
    pub nls: *mut nls_table,
    pub next_generation: u32,
    pub root: u32,
    pub bdi_id: c_int,
    pub case_insensitive: bool,
}

// per-inode information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vboxsf_inode {
// some information was changed, update data on next revalidate
    pub force_restat: c_int,
// list of open handles for this inode + lock protecting it
    pub handle_list: list_head,
// This mutex protects handle_list accesses
    pub handle_list_mutex: mutex,
// The VFS inode struct
    pub vfs_inode: inode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vboxsf_dir_info {
    pub info_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vboxsf_dir_buf {
    pub entries: usize,
    pub free: usize,
    pub used: usize,
    pub buf: *mut c_void,
    pub head: list_head,
}

// globals
// from file.c
extern "C" {
    pub fn vboxsf_release_sf_handle(inode: *mut inode, sf_handle: *mut vboxsf_handle);
}
// from utils.c
extern "C" {
    pub fn vboxsf_stat_dentry(dentry: *mut dentry, info: *mut shfl_fsobjinfo) -> c_int;
}
extern "C" {
    pub fn vboxsf_inode_revalidate(dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn vboxsf_dir_info_free(p: *mut vboxsf_dir_info);
}
extern "C" {
    pub fn vboxsf_query_case_sensitive(sbi: *mut vboxsf_sbi) -> c_int;
}
extern "C" {
    pub fn vboxsf_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
// from vboxsf_wrappers.c
extern "C" {
    pub fn vboxsf_connect() -> c_int;
}
extern "C" {
    pub fn vboxsf_disconnect();
}
extern "C" {
    pub fn vboxsf_close(root: u32, handle: u64) -> c_int;
}
extern "C" {
    pub fn vboxsf_remove(root: u32, parsed_path: *mut shfl_string, flags: u32) -> c_int;
}
extern "C" {
    pub fn vboxsf_read(root: u32, handle: u64, offset: u64, buf_len: *mut u32, buf: *mut u8) -> c_int;
}
extern "C" {
    pub fn vboxsf_write(root: u32, handle: u64, offset: u64, buf_len: *mut u32, buf: *mut u8) -> c_int;
}
extern "C" {
    pub fn vboxsf_map_folder(folder_name: *mut shfl_string, root: *mut u32) -> c_int;
}
extern "C" {
    pub fn vboxsf_unmap_folder(root: u32) -> c_int;
}
extern "C" {
    pub fn vboxsf_set_utf8() -> c_int;
}
extern "C" {
    pub fn vboxsf_set_symlinks() -> c_int;
}
