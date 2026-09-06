//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/fs.h
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
// Landlock - Filesystem management and hooks
//
// Copyright © 2017-2020 Mickaël Salaün <mic@digikod.net>
// Copyright © 2018-2020 ANSSI
// Copyright © 2024-2025 Microsoft Corporation
//

//
// struct landlock_inode_security - Inode security blob
//
// Enable to reference a &struct landlock_object tied to an inode (i.e.
// underlying object).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_inode_security {
//
// @object: Weak pointer to an allocated object.  All assignments of a
// new object are protected by the underlying inode->i_lock.  However,
// atomically disassociating @object from the inode is only protected
// by @object->lock, from the time @object's usage refcount drops to
// zero to the time this pointer is nulled out (cf. release_inode() and
// hook_sb_delete()).  Indeed, such disassociation doesn't require
// inode->i_lock thanks to the careful rcu_access_pointer() check
// performed by get_inode_object().
//
    pub object: *mut landlock_object __rcu,
}

//
// struct landlock_file_security - File security blob
//
// This information is populated when opening a file in hook_file_open, and
// tracks the relevant Landlock access rights that were available at the time
// of opening the file. Other LSM hooks use these rights in order to authorize
// operations on already opened files.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_file_security {
//
// @allowed_access: Access rights that were available at the time of
// opening the file. This is not necessarily the full set of access
// rights available at that time, but it's the necessary subset as
// needed to authorize later operations on the open file.
//
    pub allowed_access: access_mask_t,

//
// @deny_masks: Domain layer levels that deny an optional access (see
// _LANDLOCK_ACCESS_FS_OPTIONAL).
//
    pub deny_masks: deny_masks_t,
//
// @quiet_optional_accesses: Stores which optional accesses are covered
// by quiet rules within the layer referred to in deny_masks, one access
// per bit.  Does not take into account whether the quiet access bits
// are actually set in the layer's corresponding landlock_hierarchy.
//
    pub quiet_optional_accesses: optional_access_t,
//
// @fown_layer: Layer level of @fown_subject->domain with
// LANDLOCK_SCOPE_SIGNAL.
//
    pub fown_layer: u8,

//
// @fown_subject: Landlock credential of the task that set the PID that
// may receive a signal e.g., SIGURG when writing MSG_OOB to the
// related socket.  This pointer is protected by the related
// file->f_owner->lock, as for fown_struct's members: pid, uid, and
// euid.
//
    pub fown_subject: landlock_cred_security,
//
// @fown_tg: Thread group of the task that set the file owner, pinned
// while @fown_subject holds a domain.  It lets
// hook_file_send_sigiotask() always allow a SIGIO delivered to the
// owner's own process -- e.g. the thread-group leader reached through a
// process-group owner -- matching the same-process exemption of
// hook_task_kill().  NULL when no domain is recorded.  Protected by
// file->f_owner->lock, like @fown_subject.
//
    pub fown_tg: *mut pid,
}

// Makes sure all layers can be identified.
// clang-format off
// clang-format on
//
// Make sure quiet_optional_accesses has enough bits to cover all optional
// accesses.
//

//
// struct landlock_superblock_security - Superblock security blob
//
// Enable hook_sb_delete() to wait for concurrent calls to release_inode().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_superblock_security {
//
// @inode_refs: Number of pending inodes (from this superblock) that
// are being released by release_inode().
// Cf. struct super_block->s_fsnotify_inode_refs .
//
    pub inode_refs: atomic_long_t,
}

extern "C" {
    pub fn landlock_add_fs_hooks() -> __init void;
}
//
// resolve_path_for_trace - Resolve a path for tracepoint display
//
// @path: The path to resolve.
// @buf: A buffer of at least PATH_MAX bytes for the resolved path.
//
// Uses d_absolute_path() to produce a namespace-independent absolute path,
// unlike d_path() which resolves relative to the process's chroot.  This
// ensures trace output is deterministic regardless of the tracer's mount
// namespace.
//
// Return: A pointer into @buf with the resolved path, or an error string
// ("<too_long>", "<unreachable>").
//
