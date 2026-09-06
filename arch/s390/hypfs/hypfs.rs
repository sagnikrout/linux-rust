//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/hypfs/hypfs.h
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
// Hypervisor filesystem for Linux on s390.
//
// Copyright IBM Corp. 2006
// Author(s): Michael Holzheu <holzheu@de.ibm.com>
//

pub const REG_FILE_MODE: c_int = 0440;
pub const UPDATE_FILE_MODE: c_int = 0220;
pub const DIR_MODE: c_int = 0550;
extern "C" {
    pub fn hypfs_create_u64(dir: *mut dentry, name: *const c_char, value: __u64) -> c_int;
}
extern "C" {
    pub fn hypfs_create_str(dir: *mut dentry, name: *const c_char, string: *mut c_char) -> c_int;
}
// LPAR Hypervisor
extern "C" {
    pub fn hypfs_diag_init() -> c_int;
}
extern "C" {
    pub fn hypfs_diag_exit();
}
extern "C" {
    pub fn hypfs_diag_create_files(root: *mut dentry) -> c_int;
}
// VM Hypervisor
extern "C" {
    pub fn hypfs_vm_init() -> c_int;
}
extern "C" {
    pub fn hypfs_vm_exit();
}
extern "C" {
    pub fn hypfs_vm_create_files(root: *mut dentry) -> c_int;
}
// VM diagnose 0c
extern "C" {
    pub fn hypfs_diag0c_init() -> c_int;
}
extern "C" {
    pub fn hypfs_diag0c_exit();
}
// Set Partition-Resource Parameter
extern "C" {
    pub fn hypfs_sprp_init();
}
extern "C" {
    pub fn hypfs_sprp_exit();
}
extern "C" {
    pub fn __hypfs_fs_init() -> c_int;
}
extern "C" {
    pub fn __hypfs_fs_init() -> return;
}
// debugfs interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hypfs_dbfs_data {
    pub buf: *mut c_void,
    pub buf_free_ptr: *mut c_void,
    pub size: usize,
    pub dbfs_file: *mut hypfs_dbfs_file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hypfs_dbfs_file {
    pub name: *const c_char,
    pub size): *mut usize,
    pub buf_free_ptr): *const *const void (data_free)(void,
    pub long): unsigned,
// Private data for hypfs_dbfs.c
    pub lock: mutex,
    pub dentry: *mut dentry,
}

extern "C" {
    pub fn hypfs_dbfs_create_file(df: *mut hypfs_dbfs_file);
}
extern "C" {
    pub fn hypfs_dbfs_remove_file(df: *mut hypfs_dbfs_file);
}
