//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpf_lsm.h
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
// Copyright (C) 2020 Google LLC.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_storage_blob {
    pub storage: *mut bpf_local_storage __rcu,
}

extern "C" {
    pub fn bpf_lsm_is_sleepable_hook(btf_id: u32) -> bool;
}
extern "C" {
    pub fn bpf_lsm_is_trusted(prog: *const bpf_prog) -> bool;
}
extern "C" {
    pub fn bpf_inode_storage_free(inode: *mut inode);
}
extern "C" {
    pub fn bpf_lsm_find_cgroup_shim(prog: *const bpf_prog, bpf_func: *mut bpf_func_t);
}
extern "C" {
    pub fn bpf_remove_dentry_xattr_locked(dentry: *mut dentry, name__str: *const c_char) -> c_int;
}
extern "C" {
    pub fn bpf_lsm_has_d_inode_locked(prog: *const bpf_prog) -> bool;
}
extern "C" {
    pub fn bpf_lsm_hook_returns_errno(btf_id: u32) -> bool;
}

