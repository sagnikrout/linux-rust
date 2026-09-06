//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_parent.h
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
// Copyright (c) 2022-2024 Oracle.
// All Rights Reserved.
//
// Metadata validators
// Initializes a xfs_parent_rec to be stored as an attribute name.
//
// Parent pointer information needed to pass around the deferred xattr update
// machinery.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_parent_args {
    pub rec: xfs_parent_rec,
    pub new_rec: xfs_parent_rec,
    pub args: xfs_da_args,
}

//
// Start a parent pointer update by allocating the context object we need to
// perform a parent pointer update.
//
// ppargsp = NULL;
// ppargsp = kmem_cache_zalloc(xfs_parent_args_cache, GFP_KERNEL);
// Finish a parent pointer update by freeing the context object.
// Repair functions
