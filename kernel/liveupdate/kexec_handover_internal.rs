//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/liveupdate/kexec_handover_internal.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_debugfs {
    pub dir: *mut dentry,
    pub sub_fdt_dir: *mut dentry,
    pub fdt_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_debugfs {

    pub kho_scratch: *mut extern struct kho_scratch,
    pub kho_scratch_cnt: extern unsigned int,

    pub kho_debugfs_init(void): c_int,
    pub fdt): *const *const void kho_in_debugfs_init(struct kho_debugfs dbg, void,
    pub dbg): *mut int kho_out_debugfs_init(struct kho_debugfs,
    pub root): *const *const void blob, size_t size, bool,
    pub blob): *mut *mut void kho_debugfs_blob_remove(struct kho_debugfs dbg, void,

    pub }: static inline int kho_debugfs_init(void) { return 0;,
    pub }: *mut *mut static inline int kho_out_debugfs_init(struct kho_debugfs dbg) { return 0;,
    pub }: size_t size, bool root) { return 0;,

