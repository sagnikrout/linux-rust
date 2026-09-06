//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/android/binder/rust_binder_internal.h
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
// rust_binder_internal.h
//
// This file contains internal data structures used by Rust Binder. Mostly,
// these are type definitions used only by binderfs or things that Rust Binder
// define and export to binderfs.
//
// It does not include things exported by binderfs to Rust Binder since this
// file is not included as input to bindgen.
//
// Copyright (C) 2025 Google LLC.
//
pub const RUST_BINDERFS_SUPER_MAGIC: c_uint = 0x6c6f6f71;

//
// The internal data types in the Rust Binder driver are opaque to C, so we use
// void pointer typedefs for these types.
//
// struct binder_device - information about a binder device node
// @minor:     the minor number used by this device
// @ctx:       the Rust Context used by this device, or null for binder-control
//
// This is used as the private data for files directly in binderfs, but not
// files in the binder_logs subdirectory. This struct owns a refcount on `ctx`
// and the entry for `minor` in `binderfs_minors`. For binder-control `ctx` is
// null.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_device {
    pub minor: c_int,
    pub ctx: rust_binder_context,
}

extern "C" {
    pub fn rust_binder_stats_show(m: *mut seq_file, unused: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rust_binder_state_show(m: *mut seq_file, unused: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rust_binder_transactions_show(m: *mut seq_file, unused: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rust_binder_proc_show(m: *mut seq_file, pid: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rust_binder_new_context(name: *mut c_char) -> rust_binder_context;
}
extern "C" {
    pub fn rust_binder_remove_context(device: rust_binder_context);
}
//
// binderfs_mount_opts - mount options for binderfs
// @max: maximum number of allocatable binderfs binder devices
// @stats_mode: enable binder stats in binderfs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binderfs_mount_opts {
    pub max: c_int,
    pub stats_mode: c_int,
}

//
// binderfs_info - information about a binderfs mount
// @ipc_ns:         The ipc namespace the binderfs mount belongs to.
// @control_dentry: This records the dentry of this binderfs mount
// binder-control device.
// @root_uid:       uid that needs to be used when a new binder device is
// created.
// @root_gid:       gid that needs to be used when a new binder device is
// created.
// @mount_opts:     The mount options in use.
// @device_count:   The current number of allocated binder devices.
// @proc_log_dir:   Pointer to the directory dentry containing process-specific
// logs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binderfs_info {
    pub ipc_ns: *mut ipc_namespace,
    pub control_dentry: *mut dentry,
    pub root_uid: kuid_t,
    pub root_gid: kgid_t,
    pub mount_opts: binderfs_mount_opts,
    pub device_count: c_int,
    pub proc_log_dir: *mut dentry,
}
