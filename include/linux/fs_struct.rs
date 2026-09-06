//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fs_struct.h
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
pub struct fs_struct {
    pub users: c_int,
    pub seq: seqlock_t,
    pub umask: c_int,
    pub in_exec: c_int,
    pub pwd: path root,,
    pub __randomize_layout: },
    pub fs_cachep: *mut extern struct kmem_cache,
    pub userspace_init_fs: *mut extern struct fs_struct,
    pub ): *mut extern void exit_fs(struct task_struct,
    pub ): *const *const extern void set_fs_root(struct fs_struct , struct path,
    pub ): *const *const extern void set_fs_pwd(struct fs_struct , struct path,
    pub ): *mut *mut extern struct fs_struct copy_fs_struct(struct fs_struct,
    pub ): *mut extern void free_fs_struct(struct fs_struct,
    pub unshare_fs_struct(void): extern int,
// root = fs->root;
// pwd = fs->pwd;
    pub new_fs): *mut *mut fs_switch_fs_struct(fs_struct,
    pub current_chrooted(void): extern bool,
    pub current->fs->umask: return,
//
// Temporarily use userspace_init_fs for path resolution in kthreads.
// Callers should use scoped_with_init_fs() which automatically
// restores the original fs_struct at scope exit.
//
    pub old_fs: *mut fs_struct,
    pub current->fs: old_fs =,
    pub userspace_init_fs): WRITE_ONCE(current->fs,,
    pub old_fs: return,
    pub userspace_init_fs): VFS_WARN_ON_ONCE(current->fs !=,
    pub old_fs): WRITE_ONCE(current->fs,,

    pub init_userspace_fs(void): void __init,
