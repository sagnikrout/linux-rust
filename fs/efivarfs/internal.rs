//! Automatically rewritten from C Header to Rust Module
//! Source: fs/efivarfs/internal.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012 Red Hat, Inc.
// Copyright (C) 2012 Jeremy Kerr <jeremy.kerr@canonical.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efivarfs_mount_opts {
    pub uid: kuid_t,
    pub gid: kgid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efivarfs_fs_info {
    pub mount_opts: efivarfs_mount_opts,
    pub sb: *mut super_block,
    pub nb: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_variable {
    pub VariableName: [efi_char16_t; EFI_VAR_NAME_LEN/sizeof(efi_char16_t)],
    pub VendorGuid: efi_guid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efivar_entry {
    pub var: efi_variable,
    pub vfs_inode: inode,
    pub open_count: c_ulong,
    pub removed: bool,
}

extern "C" {
    pub fn container_of(_arg: inode, efivar_entry: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn efivar_entry_delete(entry: *mut efivar_entry) -> c_int;
}
extern "C" {
    pub fn efivar_entry_size(entry: *mut efivar_entry, size: *mut c_ulong) -> c_int;
}
