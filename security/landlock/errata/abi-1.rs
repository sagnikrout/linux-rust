//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/errata/abi-1.h
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
// DOC: erratum_3
//
// Erratum 3: Disconnected directory handling
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// This fix addresses an issue with disconnected directories that occur when a
// directory is moved outside the scope of a bind mount.  The change ensures
// that evaluated access rights include both those from the disconnected file
// hierarchy down to its filesystem root and those from the related mount point
// hierarchy.  This prevents access right widening through rename or link
// actions.
//
// Impact:
//
// Without this fix, it was possible to widen access rights through rename or
// link actions involving disconnected directories, potentially bypassing
// ``LANDLOCK_ACCESS_FS_REFER`` restrictions.  This could allow privilege
// escalation in complex mount scenarios where directories become disconnected
// from their original mount points.
//
// DOC: erratum_4
//
// Erratum 4: Creation of whiteout objects
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// This fix changes the access rights required for the creation of whiteout
// objects through :manpage:`mknod(2)`, :manpage:`renameat2(2)`, or
// :manpage:`link(2)`.  Creating whiteout objects is now guarded by
// ``LANDLOCK_ACCESS_FS_MAKE_REG`` instead of ``LANDLOCK_ACCESS_FS_MAKE_CHAR``.
//
// Whiteout objects are used in OverlayFS to mark the absence of a file in an
// upper file system.  Despite being created with ``S_IFCHR``, whiteout objects
// do not count as character devices.
//
// Impact:
//
// Sandboxed programs that create OverlayFS whiteouts (such as fuse-overlayfs)
// now require ``LANDLOCK_ACCESS_FS_MAKE_REG`` instead of
// ``LANDLOCK_ACCESS_FS_MAKE_CHAR``.
//
